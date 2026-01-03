pub use std::collections::HashSet;
use std::{collections::HashMap, error::Error};
#[derive(Clone, Debug, PartialEq)]
pub struct ShallowType {
    pub name: String,
    pub index: u64,
    pub array_count: u64,
    pub is_ptr: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Void,
    Integer,
    Float,
    Bool,
    String,
    Ptr {
        to: ShallowType,
    },
    Struct {
        name: String,
        fields: Vec<(String, ShallowType)>,
    },
    Function {
        from: Vec<Type>,
        to: Box<Type>,
        name: String,
    },
}
impl Type {
    pub fn is_primitive(&self) -> bool {
        match self {
            Self::Void => true,
            Self::Integer => true,
            Self::Float => true,
            Self::Bool => true,
            Self::String => false,

            _ => false,
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub enum Var {
    Stack {
        vtype: ShallowType,
        index: usize,
        name: String,
    },
    ConstInt {
        value: i64,
    },
    ConstFloat {
        value: f64,
    },
    ConstString {
        value: String,
    },
    ConstBool {
        value: bool,
    },
    Unit,
    FieldAccess {
        of: Box<Var>,
        index: usize,
        return_type: ShallowType,
    },
    FunctionLiteral {
        name: String,
    },
    OperatorNew {
        new_type: ShallowType,
    },
}
#[derive(Clone, Debug, PartialEq)]
pub enum Binop {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    NotEqual,
    Less,
    Greater,
    And,
    Or,
}
#[derive(Clone, Debug, PartialEq)]
pub enum Cmd {
    Binop {
        l: Var,
        r: Var,
        out: Var,
        op: Binop,
    },
    Assign {
        l: Var,
        r: Var,
    },
    Jmp {
        to: String,
    },
    JmpCond {
        cond: Var,
        to: String,
    },
    DeclareVariables {
        values: Vec<Type>,
    },
    Call {
        to_call: Var,
        returned: Var,
        args: Vec<Var>,
    },
    Return {
        to_return: Var,
    },
}
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Unit,
    Integer { v: i64 },
    Float { v: f64 },
    String { v: String },
    Bool { v: bool },
    Object { ptr: u64 },
    ObjectHeader { information: u32, size: u32 },
}
#[derive(Clone, Debug)]
pub struct Frame {
    pub ip: u64,
    pub v_start: u64,
    pub v_end: u64,
    pub to_return: Option<Var>,
}
#[derive(Clone, Debug)]
pub struct Machine {
    pub cmds: Vec<Cmd>,
    pub ip: u64,
    pub v_start: u64,
    pub v_end: u64,
    pub frames: Vec<Frame>,
    pub to_return: Option<Var>,
    pub heap: Heap,
    pub type_table: Vec<(String, Type)>,
    pub symbol_table: HashMap<String, usize>,
    pub stack: Vec<Value>,
    pub done: bool,
}
#[derive(Clone, Debug)]
pub struct Function {
    pub arguments: Vec<(String, ShallowType)>,
    pub return_type: ShallowType,
    pub cmds: Vec<Cmd>,
    pub labels: HashMap<String, usize>,
    pub display_name: String,
}
#[derive(Clone, Debug)]
pub struct Program {
    pub types: Vec<(String, Type)>,
    pub functions: HashMap<String, Function>,
}
#[derive(Clone, Debug)]
pub struct Heap {
    pub values: Box<[Value; 65536]>,
    pub tracking: Box<[bool; 65536]>,
    pub free_list: Vec<(u32, u32)>,
    pub allocations: HashSet<(u32, u32)>,
}
impl Default for Heap {
    fn default() -> Self {
        Self::new()
    }
}

impl Heap {
    pub fn new() -> Self {
        let values = Box::new([const { Value::Integer { v: 0 } }; 65536]);
        let tracking = Box::new([false; 65536]);
        let free_list = vec![(16, 65536)];
        Self {
            values,
            tracking,
            free_list,
            allocations: HashSet::new(),
        }
    }
    pub fn cleanup(&mut self) {
        self.free_list.clear();
        let mut prev = 0;
        let mut idx = 16;
        while idx < self.tracking.len() {
            if prev != 0 {
                if self.tracking[idx] {
                    self.free_list.push((prev, idx as u32));
                    prev = 0;
                }
            } else {
                if !self.tracking[idx] {
                    prev = idx as u32;
                }
            }
            idx += 1;
        }
        if prev != 0 {
            self.free_list.push((prev, idx as u32));
        }
    }
    pub fn allocate(&mut self, count: usize, typeheader: u32) -> Option<u32> {
        let needed = count as u32 + 1;
        let mut base = 0;
        let mut end = 0;
        let mut idx = -1;
        for i in 0..self.free_list.len() {
            if self.free_list[i].1 - self.free_list[i].0 >= needed {
                base = self.free_list[i].0;
                end = self.free_list[i].1;
                idx = i as i64;
            }
        }
        if base == 0 || idx == -1 {
            self.cleanup();
            for i in 0..self.free_list.len() {
                if self.free_list[i].1 - self.free_list[i].0 >= needed {
                    base = self.free_list[i].0;
                    end = self.free_list[i].1;
                    idx = i as i64;
                }
            }
            if base == 0 || idx == -1 {
                return None;
            }
        }
        for i in base..end {
            self.tracking[i as usize] = true;
        }
        let udx = idx as usize;
        let v = &mut self.free_list[udx];
        if v.1 - v.0 - needed > 1 {
            v.0 += needed;
        } else {
            self.free_list.swap_remove(udx);
        }
        self.values[base as usize] = Value::ObjectHeader {
            information: typeheader,
            size: count as u32,
        };
        self.allocations.insert((base + 1, count as u32));
        Some(base + 1)
    }
    pub fn free(&mut self, start: u32) -> Result<(), u32> {
        let base = start - 1;
        if !self.tracking[base as usize] {
            println!("attempted to free unallocated object:{start}");
            return Err(start);
        }
        let Value::ObjectHeader {
            information: _,
            size,
        } = self.values[base as usize]
        else {
            println!("attempted to free unallocated object:{start}");
            return Err(start);
        };
        for i in base..base + size + 1 {
            self.tracking[i as usize] = false;
        }
        self.free_list.push((base, base + size + 1));
        Ok(())
    }
}
impl Type {
    pub fn as_default(&self, _types: &[(String, Type)]) -> Result<Value, Box<dyn Error>> {
        match self {
            Type::Void => Ok(Value::Unit),
            Type::Integer => Ok(Value::Integer { v: 0 }),
            Type::Float => Ok(Value::Float { v: 0.0 }),
            Type::Bool => Ok(Value::Bool { v: false }),
            Type::String => Ok(Value::String { v: "".into() }),
            Type::Ptr { to: _ } => todo!(),
            Type::Struct { name: _, fields: _ } => todo!(),
            Type::Function {
                from: _,
                to: _,
                name: _,
            } => todo!(),
        }
    }
}
impl ShallowType {
    pub fn as_type(&self, type_table: &[(String, Type)]) -> Type {
        if self.is_ptr {
            let mut tmp = self.clone();
            tmp.is_ptr = false;
            return Type::Ptr { to: tmp };
        } else {
            return type_table[self.index as usize].clone().1;
        }
    }
}
impl Var {
    pub fn get_type(&self, type_table: &[(String, Type)]) -> Type {
        match self {
            Var::Stack {
                vtype,
                index: _,
                name: _,
            } => {
                if vtype.is_ptr {
                    let mut vt = vtype.clone();
                    vt.is_ptr = false;
                    Type::Ptr { to: vt }
                } else {
                    type_table[vtype.index as usize].1.clone()
                }
            }
            Var::ConstInt { value: _ } => Type::Integer,
            Var::ConstFloat { value: _ } => Type::Float,
            Var::ConstString { value: _ } => Type::String,
            Var::ConstBool { value: _ } => Type::Bool,
            Var::Unit => Type::Void,
            Var::FieldAccess {
                of: _,
                index: _,
                return_type,
            } => type_table[return_type.index as usize].1.clone(),
            Var::FunctionLiteral { name } => Type::Function {
                from: Vec::new(),
                to: Box::new(Type::Void),
                name: name.clone(),
            },
            Var::OperatorNew { new_type } => Type::Ptr {
                to: new_type.clone(),
            },
        }
    }
}
impl Machine {
    pub fn get_l_value(&mut self, var: Var) -> Result<&mut Value, String> {
        match var {
            Var::Stack {
                vtype: _,
                index,
                name: _,
            } => {
                if index + self.v_start as usize >= self.stack.len() {
                    println!(
                        "index:{:#?}, self.v_start:{:#?}, self.stack_len:{:#?}",
                        index,
                        self.v_start,
                        self.stack.len()
                    );
                    todo!()
                }
                return Ok(&mut self.stack[self.v_start as usize + index]);
            }
            Var::FieldAccess {
                of,
                index,
                return_type: _,
            } => {
                let v = self.get_l_value(*of)?;
                match v {
                    Value::Object { ptr } => {
                        let var = &mut self.heap.values[*ptr as usize + index + 1];
                        return Ok(var);
                    }
                    _ => {
                        todo!()
                    }
                }
            }
            _ => {}
        }
        Err("".into())
    }
    pub fn get_value(&self, var: Var) -> Result<Value, String> {
        match var {
            Var::Stack {
                vtype: _,
                index,
                name: _,
            } => {
                return Ok(self.stack[self.v_start as usize + index].clone());
            }
            Var::ConstInt { value } => {
                return Ok(Value::Integer { v: value });
            }
            Var::ConstFloat { value } => {
                return Ok(Value::Float { v: value });
            }
            Var::ConstString { value } => {
                return Ok(Value::String { v: value });
            }
            Var::ConstBool { value } => return Ok(Value::Bool { v: value }),
            Var::Unit => {
                return Ok(Value::Unit);
            }
            Var::FieldAccess {
                of,
                index,
                return_type: _,
            } => {
                let v = self.get_value(*of)?;
                match v {
                    Value::Object { ptr } => {
                        return Ok(self.heap.values[ptr as usize + index + 1].clone());
                    }
                    _ => {
                        todo!();
                    }
                }
            }
            _ => {
                todo!()
            }
        }
    }
    pub fn get_bool(&self, var: Var) -> Result<bool, String> {
        match self.get_value(var)? {
            Value::Bool { v } => Ok(v),
            _ => Err("accessed non bool as bool".into()),
        }
    }
    pub fn get_float(&self, var: Var) -> Result<f64, String> {
        match self.get_value(var)? {
            Value::Float { v } => Ok(v),
            _ => Err("accessed non float as float".into()),
        }
    }
    pub fn get_int(&self, var: Var) -> Result<i64, String> {
        match self.get_value(var)? {
            Value::Integer { v } => Ok(v),
            _ => Err("accessed non int as int".into()),
        }
    }
    pub fn get_string(&self, var: Var) -> Result<String, String> {
        match self.get_value(var)? {
            Value::String { v } => Ok(v),
            _ => Err("accessed non string as string".into()),
        }
    }
    pub fn update(&mut self) -> Result<(), Box<dyn Error>> {
        let ins = self.cmds[self.ip as usize].clone();
        self.ip += 1;
        //println!("{:#?}",ins);
        match ins {
            Cmd::Binop { l, r, out, op } => {
                let lt = l.get_type(&self.type_table);
                let ot = out.get_type(&self.type_table);
                match lt.clone() {
                    Type::Void => {
                        return Err("binop not supported on void".into());
                    }
                    Type::Integer => {
                        let lv = self.get_int(l)?;
                        let rv = self.get_int(r)?;
                        let output = self.get_l_value(out)?;
                        match op {
                            Binop::Add => {
                                if lt != ot {
                                    todo!();
                                }
                                *output = Value::Integer { v: lv + rv };
                            }
                            Binop::Sub => {
                                if lt != ot {
                                    todo!();
                                }
                                *output = Value::Integer { v: lv - rv };
                            }
                            Binop::Mul => {
                                if lt != ot {
                                    todo!();
                                }
                                *output = Value::Integer { v: lv * rv }
                            }
                            Binop::Div => {
                                if lt != ot {
                                    todo!();
                                }
                                *output = Value::Integer { v: lv / rv };
                            }
                            Binop::Equal => {
                                if ot != Type::Bool {
                                    todo!();
                                }
                                *output = Value::Bool { v: lv == rv };
                            }
                            Binop::NotEqual => {
                                if ot != Type::Bool {
                                    todo!();
                                }
                                *output = Value::Bool { v: lv != rv };
                            }
                            Binop::Less => {
                                if ot != Type::Bool {
                                    todo!();
                                }
                                *output = Value::Bool { v: lv < rv };
                            }
                            Binop::Greater => {
                                if ot != Type::Bool {
                                    todo!();
                                }
                                *output = Value::Bool { v: lv > rv };
                            }
                            Binop::And => return Err("and not supported on integers".into()),
                            Binop::Or => return Err("or not supported on integers".into()),
                        }
                    }
                    Type::Float => {
                        let lv = self.get_float(l)?;
                        let rv = self.get_float(r)?;
                        let output = self.get_l_value(out)?;
                        match op {
                            Binop::Add => {
                                if lt != ot {
                                    todo!();
                                }
                                *output = Value::Float { v: lv + rv };
                            }
                            Binop::Sub => {
                                if lt != ot {
                                    todo!();
                                }
                                *output = Value::Float { v: lv - rv };
                            }
                            Binop::Mul => {
                                if lt != ot {
                                    todo!();
                                }
                                *output = Value::Float { v: lv * rv }
                            }
                            Binop::Div => {
                                if lt != ot {
                                    todo!();
                                }
                                *output = Value::Float { v: lv / rv };
                            }
                            Binop::Equal => {
                                if ot != Type::Bool {
                                    todo!();
                                }
                                *output = Value::Bool { v: lv == rv };
                            }
                            Binop::NotEqual => {
                                if ot != Type::Bool {
                                    todo!();
                                }
                                *output = Value::Bool { v: lv != rv };
                            }
                            Binop::Less => {
                                if ot != Type::Bool {
                                    todo!();
                                }
                                *output = Value::Bool { v: lv < rv };
                            }
                            Binop::Greater => {
                                if ot != Type::Bool {
                                    todo!();
                                }
                                *output = Value::Bool { v: lv > rv };
                            }
                            Binop::And => return Err("and not supported on floats".into()),
                            Binop::Or => return Err("or not supported on floats".into()),
                        }
                    }
                    Type::Bool => {
                        let lv = self.get_bool(l)?;
                        let rv = self.get_bool(r)?;
                        let output = self.get_l_value(out)?;
                        match op {
                            Binop::Equal => {
                                if ot != Type::Bool {
                                    todo!();
                                }
                                *output = Value::Bool { v: lv == rv }
                            }
                            Binop::NotEqual => {
                                if ot != Type::Bool {
                                    todo!();
                                }
                                *output = Value::Bool { v: lv != rv }
                            }
                            Binop::And => {
                                if ot != Type::Bool {
                                    todo!();
                                }
                                *output = Value::Bool { v: lv && rv }
                            }
                            Binop::Or => {
                                if ot != Type::Bool {
                                    todo!();
                                }
                                *output = Value::Bool { v: lv || rv }
                            }
                            _ => return Err(" arithmetic not supported on bools".into()),
                        }
                    }
                    Type::String => match op {
                        Binop::Add => {
                            let lv = self.get_string(l)?;
                            let rv = self.get_string(r)?;
                            let output = self.get_l_value(out)?;
                            if ot != Type::String {
                                todo!()
                            }
                            *output = Value::String { v: lv + &rv };
                        }
                        Binop::Mul => {
                            let lv = self.get_string(l)?;
                            let rv = self.get_int(r)?;
                            let output = self.get_l_value(out)?;
                            if rv < 0 {
                                todo!();
                            }
                            let mut out = "".to_string();
                            for _i in 0..rv {
                                out += &lv;
                            }
                            if ot != Type::String {
                                todo!()
                            }
                            *output = Value::String { v: out };
                        }
                        Binop::Equal => {
                            let lv = self.get_string(l)?;
                            let rv = self.get_string(r)?;
                            let output = self.get_l_value(out)?;
                            if ot != Type::Bool {
                                todo!();
                            }
                            *output = Value::Bool { v: lv == rv }
                        }
                        Binop::NotEqual => {
                            let lv = self.get_string(l)?;
                            let rv = self.get_string(r)?;
                            let output = self.get_l_value(out)?;
                            if ot != Type::Bool {
                                todo!();
                            }
                            *output = Value::Bool { v: lv != rv }
                        }

                        _ => {
                            todo!()
                        }
                    },
                    Type::Ptr { to: _ } => {
                        return Err("binop not supported on pointers".into());
                    }
                    Type::Struct { name: _, fields: _ } => {
                        return Err("binop not supported on structures".into());
                    }
                    Type::Function {
                        from: _,
                        to: _,
                        name: _,
                    } => {
                        return Err("binop not supported on structures".into());
                    }
                }
            }
            Cmd::Assign { l, r } => {
                let lt = l.get_type(&self.type_table);
                let rt = l.get_type(&self.type_table);
                if lt != rt {
                    todo!()
                }
                let rv = self.get_value(r)?;
                let lv = self.get_l_value(l)?;
                *lv = rv;
            }
            Cmd::Jmp { to } => {
                if let Some(loc) = self.symbol_table.get(&to) {
                    self.ip = (*loc) as u64;
                } else {
                    return Err(format!("undefined symbol:{}", to).into());
                }
            }
            Cmd::JmpCond { cond, to } => {
                let b = self.get_bool(cond)?;
                if let Some(loc) = self.symbol_table.get(&to) {
                    if b {
                        self.ip = (*loc) as u64;
                    }
                } else {
                    return Err(format!("undefined symbol:{}", to).into());
                }
            }
            Cmd::DeclareVariables { values } => {
                for i in values {
                    if self.stack.len() as u64 > self.v_end + 1 {
                        self.stack[self.v_end as usize] = i.as_default(&self.type_table)?;
                    } else {
                        self.stack.push(i.as_default(&self.type_table)?);
                    }
                    self.v_end += 1;
                    // println!("v_end:{:#?}", self.v_end);
                }
            }
            Cmd::Call {
                to_call,
                returned,
                args,
            } => {
                let f = Frame {
                    ip: self.ip,
                    v_start: self.v_start,
                    v_end: self.v_end,
                    to_return: self.to_return.take(),
                };
                let loc = match to_call {
                    Var::FunctionLiteral { name } => self.symbol_table.get(&name),
                    _ => {
                        todo!()
                    }
                };
                if let Some(loc) = loc {
                    let old = self.v_end;
                    for i in args {
                        if self.stack.len() as u64 > self.v_end + 1 {
                            self.stack[self.v_end as usize] = self.get_value(i)?;
                        } else {
                            self.stack.push(self.get_value(i)?);
                        }
                        self.v_end += 1;
                    }
                    self.ip = *loc as u64;
                    self.v_start = old;
                }
                self.frames.push(f);
                self.to_return = if returned != Var::Unit {
                    Some(returned)
                } else {
                    None
                };
            }
            Cmd::Return { to_return } => {
                let Some(base) = self.frames.pop() else {
                    println!("exited with:{:#?}", self.get_value(to_return));
                    self.done = true;
                    return Ok(());
                };
                let rv = self.get_value(to_return)?;
                self.ip = base.ip;
                self.v_start = base.v_start;
                self.v_end = base.v_end;
                if self.to_return.is_none() {
                    self.to_return = base.to_return;
                    return Ok(());
                }
                let ret = self.to_return.take().unwrap();
                if let Ok(t) = self.get_l_value(ret) {
                    *t = rv;
                    self.to_return = base.to_return;
                } else {
                    todo!()
                }
            }
        }
        Ok(())
    }
}
