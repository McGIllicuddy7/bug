pub use std::collections::HashSet;
pub use std::rc::Rc;
use std::{cell::UnsafeCell, collections::HashMap, error::Error, sync::Arc};
#[derive(Clone, Debug, PartialEq)]
pub struct ShallowType {
    pub name: Rc<str>,
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
        name: Rc<str>,
        fields: Rc<[(Rc<str>, ShallowType)]>,
    },
    Function {
        from: Vec<Type>,
        to: Box<Type>,
        name: Rc<str>,
    },
}
impl Type {
    pub fn is_primitive(&self) -> bool {
        match self {
            Self::Void => true,
            Self::Integer => true,
            Self::Float => true,
            Self::Bool => true,
            Self::String => true,
            _ => false,
        }
    }
}
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum Var {
    Stack {
        vtype: ShallowType,
        index: usize,
        name: Rc<str>,
    },
    ConstInt {
        value: i64,
    },
    ConstFloat {
        value: f64,
    },
    ConstString {
        value: Rc<str>,
    },
    ConstBool {
        value: bool,
    },
    Unit,
    FieldAccess {
        of: Rc<Var>,
        index: usize,
        return_type: ShallowType,
    },
    FunctionLiteral {
        name: Rc<str>,
        idx: usize,
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
        to: Rc<str>,
        to_idx: usize,
    },
    JmpCond {
        cond: Var,
        to: Rc<str>,
        to_idx: usize,
    },
    DeclareVariables {
        values: Rc<[Type]>,
    },
    Call {
        to_call: Var,
        returned: Var,
        args: Rc<[Var]>,
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
    String { v: Rc<str> },
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
    pub type_table: Vec<(Rc<str>, Type)>,
    pub symbol_table: HashMap<String, usize>,
    pub stack: Vec<Value>,
    pub done: bool,
}
#[derive(Clone, Debug)]
pub struct Function {
    pub arguments: Rc<[(Rc<str>, ShallowType)]>,
    pub return_type: ShallowType,
    pub cmds: Vec<Cmd>,
    pub labels: HashMap<String, usize>,
    pub display_name: String,
}
#[derive(Clone, Debug)]
pub struct Program {
    pub types: Vec<(Rc<str>, Type)>,
    pub functions: HashMap<String, Function>,
}
#[derive(Clone, Debug)]
pub struct HeapInternal {
    pub values: Box<[Value; 65536]>,
    pub tracking: Box<[bool; 65536]>,
    pub free_list: Vec<(u32, u32)>,
    pub allocations: HashSet<(u32, u32)>,
}
#[derive(Clone, Debug)]
pub struct Heap {
    pub v: Arc<UnsafeCell<HeapInternal>>,
}
impl Default for HeapInternal {
    fn default() -> Self {
        Self::new()
    }
}

impl HeapInternal {
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
impl Default for Heap {
    fn default() -> Self {
        Self::new()
    }
}

impl Heap {
    pub fn new() -> Self {
        Self {
            v: Arc::new(UnsafeCell::new(HeapInternal::new())),
        }
    }
    pub fn cleanup(&self) {
        unsafe {
            (*self.v.get()).cleanup();
        }
    }
    pub fn allocate(&self, count: usize, typeheader: u32) -> Option<u32> {
        unsafe { (*self.v.get()).allocate(count, typeheader) }
    }
    pub fn free(&self, start: u32) -> Result<(), u32> {
        unsafe { (*self.v.get()).free(start) }
    }
    pub fn get(&self, ptr: usize) -> Value {
        unsafe { (*self.v.get()).values[ptr].clone() }
    }
    pub fn get_mut(&self, ptr: usize) -> &mut Value {
        unsafe { &mut (*self.v.get()).values[ptr] }
    }
    pub fn debug(&self) {
        unsafe {
            println!(
                "allocations:{:#?}\n, free_list:{:#?}",
                (*self.v.get()).allocations,
                (*self.v.get()).free_list
            );
        }
    }
}
impl Type {
    pub fn as_default(&self, _types: &[(Rc<str>, Type)]) -> Result<Value, Box<dyn Error>> {
        match self {
            Type::Void => Ok(Value::Unit),
            Type::Integer => Ok(Value::Integer { v: 0 }),
            Type::Float => Ok(Value::Float { v: 0.0 }),
            Type::Bool => Ok(Value::Bool { v: false }),
            Type::String => Ok(Value::String { v: "".into() }),
            Type::Ptr { to: _ } => Ok(Value::Object { ptr: 0 }),
            Type::Struct { name: _, fields: _ } => todo!(),
            Type::Function {
                from: _,
                to: _,
                name: _,
            } => todo!(),
        }
    }
    pub fn get_size(&self, types: &[(Rc<str>, Type)]) -> usize {
        match self {
            Type::Struct { name: _, fields } => {
                let mut out = 0;
                for i in fields.iter() {
                    out += i.1.as_type(types).get_size(types);
                }
                out
            }
            _ => 1,
        }
    }
}
impl ShallowType {
    pub fn as_type(&self, type_table: &[(Rc<str>, Type)]) -> Type {
        if self.is_ptr {
            let mut tmp = self.clone();
            tmp.is_ptr = false;
            Type::Ptr { to: tmp }
        } else {
            type_table[self.index as usize].clone().1
        }
    }
}
impl Var {
    pub fn get_type(&self, type_table: &[(Rc<str>, Type)]) -> Type {
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
            } => {
                if return_type.is_ptr {
                    let mut vt = return_type.clone();
                    vt.is_ptr = false;
                    Type::Ptr { to: vt }
                } else {
                    type_table[return_type.index as usize].1.clone()
                }
            }
            Var::FunctionLiteral { name, idx: _ } => Type::Function {
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
                let v = self.get_value((*of).clone())?;
                match v {
                    Value::Object { ptr } => {
                        let var = self.heap.get_mut(ptr as usize + index + 1);
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
            } => Ok(self.stack[self.v_start as usize + index].clone()),
            Var::ConstInt { value } => Ok(Value::Integer { v: value }),
            Var::ConstFloat { value } => Ok(Value::Float { v: value }),
            Var::ConstString { value } => Ok(Value::String { v: value }),
            Var::ConstBool { value } => Ok(Value::Bool { v: value }),
            Var::Unit => Ok(Value::Unit),
            Var::FieldAccess {
                of,
                index,
                return_type: _,
            } => {
                let v = self.get_value((*of).clone())?;
                match v {
                    Value::Object { ptr } => Ok(self.heap.get(ptr as usize + index + 1)),
                    _ => {
                        todo!();
                    }
                }
            }
            Var::OperatorNew { new_type } => {
                let vt = new_type.as_type(&self.type_table);
                let fields = match &vt {
                    Type::Struct { name: _, fields } => fields.clone(),
                    _ => todo!(),
                };
                let sz = vt.get_size(&self.type_table);
                let ptr = self.heap.allocate(sz, new_type.index as u32).unwrap();
                for i in 1..sz + 1 {
                    *self.heap.get_mut(ptr as usize + i) = fields[i - 1]
                        .1
                        .as_type(&self.type_table)
                        .as_default(&self.type_table)
                        .unwrap();
                }
                Ok(Value::Object { ptr: ptr as u64 })
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
    pub fn get_string(&self, var: Var) -> Result<Rc<str>, String> {
        match self.get_value(var)? {
            Value::String { v } => Ok(v),
            _ => Err("accessed non string as string".into()),
        }
    }
    pub fn update(&mut self) -> Result<(), Box<dyn Error>> {
        let ins = self.cmds[self.ip as usize].clone();
        self.ip += 1;
        // println!("{:#?}", ins);
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
                            *output = Value::String {
                                v: (lv.to_string() + &rv).into(),
                            };
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
                            *output = Value::String { v: out.into() };
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
            Cmd::Jmp { to: _, to_idx } => {
                self.ip = (to_idx) as u64;
            }
            Cmd::JmpCond {
                cond,
                to: _,
                to_idx,
            } => {
                let b = self.get_bool(cond)?;
                let loc = to_idx;
                if b {
                    self.ip = (loc) as u64;
                }
            }
            Cmd::DeclareVariables { values } => {
                for i in values.iter() {
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
                    Var::FunctionLiteral { name: _, idx } => idx,
                    _ => {
                        todo!()
                    }
                };
                let old = self.v_end;
                for i in args.iter() {
                    if self.stack.len() as u64 > self.v_end + 1 {
                        self.stack[self.v_end as usize] = self.get_value(i.clone())?;
                    } else {
                        self.stack.push(self.get_value(i.clone())?);
                    }
                    self.v_end += 1;
                }
                self.ip = loc as u64;
                self.v_start = old;
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
