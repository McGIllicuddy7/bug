pub use serde_derive::{Deserialize, Serialize};
pub use std::collections::HashMap;
pub use std::sync::Arc;
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub enum MikuType {
    Integer,
    Double,
    String,
    Char,
    Bool,
    Undefined {
        name: Arc<str>,
    },
    PointerTo {
        to: Arc<MikuType>,
    },
    SliceOf {
        of: Arc<MikuType>,
    },
    ArrayOf {
        count: usize,
        of: Arc<MikuType>,
    },
    Struct {
        name: Arc<str>,
        fields: Arc<[MikuType]>,
    },
}
impl MikuType {
    pub fn as_c_type(&self) -> String {
        match self {
            Self::Integer => "long".to_string(),
            Self::Double => "double".to_string(),
            Self::Char => "char".to_string(),
            Self::Undefined { name } => name.to_string(),
            Self::Bool => "bool".to_string(),
            _ => {
                todo!()
            }
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MikuLiteral {
    Integer { v: i64 },
    Double { v: f64 },
    String { v: Arc<str> },
    Char { v: char },
    Bool { v: bool },
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VarDec {
    pub name: Arc<str>,
    pub index: usize,
    pub var_type: MikuType,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FuncDec {
    pub name: Arc<str>,
    pub index: Option<usize>,
    pub args: Vec<VarDec>,
    pub return_type: MikuType,
}
impl FuncDec {
    pub fn as_c_dec(&self) -> String {
        let mut base = format!("{} miku_{}(", self.return_type.as_c_type(), self.name);
        for i in 0..self.args.len() {
            base += &self.args[i].var_type.as_c_type();
            base += " ";
            base += &self.args[i].name;
            if i != self.args.len() - 1 {
                base += ",";
            }
        }
        base += ")";
        base
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum VarUse {
    Declared {
        index: Option<usize>,
        name: Arc<str>,
        vtype: MikuType,
    },
    Literal {
        v: MikuLiteral,
    },
}
impl VarUse {
    pub fn as_c(&self) -> String {
        match self {
            Self::Declared {
                index: _,
                name,
                vtype: _,
            } => name.to_string(),
            Self::Literal { v } => match v {
                MikuLiteral::Char { v } => {
                    format!("{}", v)
                }
                MikuLiteral::Bool { v } => {
                    format!("{}", v)
                }
                MikuLiteral::Integer { v } => {
                    format!("{}", v)
                }
                MikuLiteral::Double { v } => {
                    format!("{}", v)
                }
                _ => {
                    todo!()
                }
            },
        }
    }
    pub fn get_type(&self) -> MikuType {
        match self {
            Self::Declared {
                index: _,
                name: _,
                vtype,
            } => vtype.clone(),
            Self::Literal { v } => match v {
                MikuLiteral::Integer { v: _ } => MikuType::Integer,
                MikuLiteral::Double { v: _ } => MikuType::Double,
                MikuLiteral::Char { v: _ } => MikuType::Char,
                MikuLiteral::Bool { v: _ } => MikuType::Bool,
                MikuLiteral::String { v: _ } => MikuType::String,
            },
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OpType {
    Add,
    Sub,
    Mul,
    Div,
    CmpG,
    CmpE,
    CmpL,
    Or,
    And,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MikuInstr {
    BeginFunction {
        name: Arc<str>,
        label_index: usize,
    },
    EndFunction {
        name: Arc<str>,
    },
    Label {
        name: Arc<str>,
        label_index: usize,
    },
    Jmp {
        to: Arc<str>,
        to_index: Option<usize>,
    },
    Branch {
        to_true: Arc<str>,
        to_true_index: Option<usize>,
        to_false: Arc<str>,
        to_false_index: Option<usize>,
        var: VarUse,
    },
    Call {
        to: Arc<str>,
        to_index: Option<usize>,
        return_var: Option<VarUse>,
        args: Vec<VarUse>,
    },
    DeclVar {
        vtype: MikuType,
        var_name: Arc<str>,
        count: u32,
    },
    DeclStatic {
        vtype: MikuType,
        var_name: Arc<str>,
        count: u32,
        base_value: MikuLiteral,
    },
    DeclExtern {
        vtype: MikuType,
        var_name: Arc<str>,
    },
    Assign {
        left: VarUse,
        right: VarUse,
    },
    DerefAssign {
        left: VarUse,
        right: VarUse,
    },
    GetRef {
        assigned_to: VarUse,
        of: VarUse,
    },
    BinOp {
        left: VarUse,
        right: VarUse,
        output: VarUse,
        operation: OpType,
    },
    Return {
        value: Option<VarUse>,
    },
}
#[derive(Clone, Debug)]
pub struct ParserState {
    pub instructions: Vec<MikuInstr>,
    pub labels: HashMap<String, usize>,
    pub functions: Vec<FuncDec>,
    pub extern_functions: Vec<FuncDec>,
    pub local_vars: HashMap<String, VarDec>,
    pub global_vars: HashMap<String, VarDec>,
    pub extern_vars: HashMap<String, MikuType>,
    pub types: HashMap<String, MikuType>,
    pub current_function: Option<Arc<str>>,
}
impl ParserState {
    pub fn expect_literal<'a>(&self, line: &[&'a str], index: usize) -> Result<&'a str, String> {
        if index >= line.len() {
            return Err("expected literal found nothing".into());
        }
        return Ok(line[index]);
    }
    pub fn expect_variable<'a>(&self, line: &[&'a str], index: usize) -> Result<VarUse, String> {
        let s = self.expect_literal(line, index)?;
        if s == "_" {
            return Err("_".into());
        }
        if s.starts_with('"') {
            return Ok(VarUse::Literal {
                v: MikuLiteral::String { v: s.into() },
            });
        }
        if s.starts_with("'") {
            if let Ok(c) = &s[1..s.len() - 1].parse::<char>() {
                return Ok(VarUse::Literal {
                    v: MikuLiteral::Char { v: *c },
                });
            }
        }
        if s.contains('.') {
            if let Ok(f) = s.parse::<f64>() {
                return Ok(VarUse::Literal {
                    v: MikuLiteral::Double { v: f },
                });
            } else {
                return Err("'.' not allowed in literals other than doubles".into());
            }
        }
        if let Ok(v) = s.parse::<i64>() {
            return Ok(VarUse::Literal {
                v: MikuLiteral::Integer { v },
            });
        }
        if s == "true" {
            return Ok(VarUse::Literal {
                v: MikuLiteral::Bool { v: true },
            });
        }
        if s == "false" {
            return Ok(VarUse::Literal {
                v: MikuLiteral::Bool { v: false },
            });
        }
        if self.local_vars.contains_key(s) {
            let p = &self.local_vars[s];
            return Ok(VarUse::Declared {
                index: Some(p.index),
                name: s.into(),
                vtype: p.var_type.clone(),
            });
        }
        if self.global_vars.contains_key(s) {
            let p = &self.global_vars[s];
            return Ok(VarUse::Declared {
                index: Some(p.index),
                name: s.into(),
                vtype: p.var_type.clone(),
            });
        }
        if self.extern_vars.contains_key(s) {
            let p = self.extern_vars[s].clone();
            return Ok(VarUse::Declared {
                index: None,
                name: s.into(),
                vtype: p,
            });
        }
        return Err(format!("undeclared variable {:#?}", s));
    }
    pub fn expect_type<'a>(&self, line: &'a [&'a str], index: usize) -> Result<MikuType, String> {
        let s = self.expect_literal(line, index)?;
        match s {
            "int" => return Ok(MikuType::Integer),
            "float" => return Ok(MikuType::Double),
            "char" => return Ok(MikuType::Char),
            "bool" => return Ok(MikuType::Bool),
            "string" => return Ok(MikuType::String),
            _ => {}
        }
        return Ok(MikuType::Undefined { name: s.into() });
    }
    pub fn expect_function_header<'a>(&self, line: &[&'a str]) -> Result<FuncDec, String> {
        let return_type = self.expect_type(line, 0)?;
        let name = self.expect_literal(line, 1)?;
        let mut index = 2;
        let mut arg_count = 0;
        let mut args: Vec<VarDec> = Vec::new();
        while index < line.len() {
            let vtype = self.expect_type(line, index + 1)?;
            let vname = self.expect_literal(line, index)?;
            let dec = VarDec {
                name: vname.into(),
                var_type: vtype,
                index: arg_count,
            };
            index += 2;
            arg_count += 1;
            args.push(dec);
        }
        let out = FuncDec {
            name: name.into(),
            index: None,
            args,
            return_type,
        };
        return Ok(out);
    }
    pub fn parse_instruction<'a>(&mut self, line: Vec<&'a str>) -> Result<(), String> {
        if line.len() < 1 {
            return Ok(());
        }
        let mut line = line;
        if line[0].ends_with(":") {
            let name = line[0].strip_suffix(":").unwrap();
            let index = self.instructions.len();
            let ins = MikuInstr::Label {
                name: name.into(),
                label_index: index,
            };
            self.labels
                .insert(name.to_string(), self.instructions.len());
            self.instructions.push(ins);
            line.remove(0);
            if line.is_empty() {
                return Ok(());
            };
        }
        match line[0] {
            "=" => {
                let l = self.expect_variable(&line, 1)?;
                let r = self.expect_variable(&line, 2)?;
                let v = MikuInstr::Assign { left: l, right: r };
                self.instructions.push(v);
            }
            "^=" => {
                let l = self.expect_variable(&line, 1)?;
                let r = self.expect_variable(&line, 2)?;
                let v = MikuInstr::DerefAssign { left: l, right: r };
                self.instructions.push(v)
            }
            "&" => {
                let to = self.expect_variable(&line, 1)?;
                let of = self.expect_variable(&line, 2)?;
                let v = MikuInstr::GetRef {
                    assigned_to: to,
                    of,
                };
                self.instructions.push(v)
            }
            "defun" => {
                if line.len() < 2 {
                    return Err("function requires information".into());
                }
                if self.current_function.is_some() {
                    return Err("cannot declare a function inside another function".into());
                }
                let mut func = self.expect_function_header(&line[1..])?;
                func.index = Some(self.instructions.len());
                self.current_function = Some(func.name.clone());
                let v = MikuInstr::BeginFunction {
                    name: line[2].into(),
                    label_index: self.instructions.len(),
                };
                for i in &func.args {
                    self.local_vars.insert(i.name.to_string(), i.clone());
                }
                self.instructions.push(v);
                self.functions.push(func);
            }
            "extern" => {
                if line.len() < 2 {
                    return Err("function requires information".into());
                }
                let func = self.expect_function_header(&line[1..])?;
                self.extern_functions.push(func);
            }
            "end" => {
                let p = if let Some(p) = &self.current_function {
                    p.clone()
                } else {
                    return Err("end not inside of function".into());
                };
                self.current_function = None;
                self.instructions.push(MikuInstr::EndFunction { name: p });
                self.local_vars.clear();
            }
            "+" | "-" | "*" | "/" | "<" | ">" | "==" | "&&" | "||" => {
                let op = match line[0] {
                    "+" => OpType::Add,
                    "-" => OpType::Sub,
                    "*" => OpType::Mul,
                    "/" => OpType::Div,
                    "<" => OpType::CmpL,
                    ">" => OpType::CmpG,
                    "==" => OpType::CmpE,
                    "&&" => OpType::And,
                    "||" => OpType::Or,
                    _ => unreachable!(),
                };
                let output = self.expect_variable(&line, 1)?;
                let left = self.expect_variable(&line, 2)?;
                let right = self.expect_variable(&line, 3)?;
                let v = MikuInstr::BinOp {
                    operation: op,
                    left,
                    right,
                    output,
                };
                self.instructions.push(v)
            }
            "ret" => {
                let ret = if let Ok(rt) = self.expect_variable(&line, 1) {
                    Some(rt)
                } else {
                    None
                };
                let v = MikuInstr::Return { value: ret };
                self.instructions.push(v);
            }
            "jmp" => {
                let target = self.expect_literal(&line, 1)?;
                let v = MikuInstr::Jmp {
                    to: target.into(),
                    to_index: None,
                };
                self.instructions.push(v);
            }
            "br" => {
                let cond = self.expect_variable(&line, 1)?;
                let if_true = self.expect_literal(&line, 2)?;
                let if_false = self.expect_literal(&line, 3)?;
                let v = MikuInstr::Branch {
                    var: cond.clone(),
                    to_true: if_true.into(),
                    to_false: if_false.into(),
                    to_true_index: None,
                    to_false_index: None,
                };
                if cond.get_type() != MikuType::Bool {
                    return Err(format!("conditional variable {:#?} is not a boolean", cond));
                }
                self.instructions.push(v);
            }
            "decl" => {
                let name = self.expect_literal(&line, 1)?;
                let vtype = self.expect_type(&line, 2)?;
                let count = self.local_vars.len() as u32;
                let dc = VarDec {
                    name: name.into(),
                    var_type: vtype.clone(),
                    index: count as usize,
                };
                let v = MikuInstr::DeclVar {
                    var_name: name.into(),
                    vtype,
                    count,
                };
                self.instructions.push(v);
                self.local_vars.insert(name.into(), dc);
            }
            "static" => {
                let name = self.expect_literal(&line, 1)?;
                let vtype = self.expect_type(&line, 2)?;
                let lit = self.expect_literal(&line, 3)?;
                let vlit = match vtype {
                    MikuType::String => MikuLiteral::String { v: lit.into() },
                    MikuType::Char => {
                        todo!()
                    }
                    MikuType::Bool => {
                        todo!()
                    }
                    MikuType::Double => {
                        if let Ok(p) = lit.parse::<f64>() {
                            MikuLiteral::Double { v: p }
                        } else {
                            return Err(format!("cannot parse {:#?} into float", lit));
                        }
                    }
                    MikuType::Integer => {
                        if let Ok(p) = lit.parse::<i64>() {
                            MikuLiteral::Integer { v: p }
                        } else {
                            return Err(format!("cannot parse {:#?} into int", lit));
                        }
                    }
                    _ => {
                        return Err(format!(
                            "statics of type {:#?} are currently unsupported",
                            vtype
                        ));
                    }
                };
                let count = self.global_vars.len();
                let v = MikuInstr::DeclStatic {
                    base_value: vlit,
                    count: count as u32,
                    var_name: name.into(),
                    vtype: vtype.clone(),
                };
                self.global_vars.insert(
                    name.into(),
                    VarDec {
                        name: name.into(),
                        index: count,
                        var_type: vtype,
                    },
                );
                self.instructions.push(v);
            }
            "decl_extern" => {
                todo!();
            }
            _ => {
                let to_call = self.expect_literal(&line, 0)?;
                let ret = if let Ok(rt) = self.expect_variable(&line, 1) {
                    Some(rt)
                } else {
                    None
                };
                let mut args = Vec::new();
                let mut index = 2;
                while index < line.len() {
                    let ag = self.expect_variable(&line, index)?;
                    args.push(ag);
                    index += 1;
                }
                let v = MikuInstr::Call {
                    to: to_call.into(),
                    return_var: ret,
                    args,
                    to_index: None,
                };
                self.validate_call(&v)?;
                self.instructions.push(v);
            }
        }
        Ok(())
    }
    pub fn validate_call(&self, call: &MikuInstr) -> Result<(), String> {
        match call {
            MikuInstr::Call {
                to,
                return_var,
                args,
                to_index: _,
            } => {
                let func: Option<&FuncDec> = {
                    let mut output = None;
                    for i in &self.functions {
                        if i.name.as_ref() == to.as_ref() {
                            output = Some(i);
                            break;
                        }
                    }
                    for i in &self.extern_functions {
                        if i.name.as_ref() == to.as_ref() {
                            output = Some(i);
                            break;
                        }
                    }
                    output
                };
                if let Some(func) = func {
                    let out = Err(format!(
                        "calls args don't match signature, expected {:#?} found {:#?}",
                        func.args, args
                    ));
                    if func.args.len() != args.len() {
                        return out;
                    }
                    for i in 0..func.args.len() {
                        if func.args[i].var_type != args[i].get_type() {
                            return out;
                        }
                    }
                    if let Some(v) = return_var {
                        let k = func.return_type.clone();
                        if v.get_type() != k {
                            return Err(format!(
                                "expected return type {:#?}, found {:#?}",
                                k,
                                v.get_type()
                            ));
                        }
                    }
                    Ok(())
                } else {
                    Err(format!("{:#?} is not a declared function", to))
                }
            }
            _ => Err(format!("{:#?} was not a call instruction", call)),
        }
    }
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            labels: HashMap::new(),
            functions: Vec::new(),
            extern_functions: Vec::new(),
            local_vars: HashMap::new(),
            global_vars: HashMap::new(),
            extern_vars: HashMap::new(),
            types: HashMap::new(),
            current_function: None,
        }
    }
    pub fn fix_ups(&mut self) -> Result<(), String> {
        for i in &mut self.instructions {
            match i {
                MikuInstr::Jmp { to, to_index } => {
                    if let Some(idx) = self.labels.get(to.as_ref()) {
                        *to_index = Some(*idx);
                    } else {
                        return Err(format!("could not find label {:#?}", to));
                    }
                }
                MikuInstr::Branch {
                    to_true,
                    to_false,
                    to_true_index,
                    to_false_index,
                    var,
                } => {
                    if let Some(tr) = self.labels.get(to_true.as_ref()) {
                        *to_true_index = Some(*tr);
                    } else {
                        return Err(format!("could not find label {:#?}", to_true));
                    }
                    if let Some(fs) = self.labels.get(to_false.as_ref()) {
                        *to_false_index = Some(*fs);
                    } else {
                        return Err(format!("could not find label {:#?}", to_false));
                    }
                    if var.get_type() != MikuType::Bool {
                        todo!()
                    }
                } /*
                MikuInstr::Call {
                to,
                to_index,
                args: _,
                return_var: _,
                } => {
                let mut indx = None;
                for i in &self.functions {
                if i.name.as_ref() == to.as_ref() {
                indx = i.index;
                break;
                }
                }
                if let Some(idx) = indx {
                 *to_index = Some(idx);
                } else {
                return Err(format!("could not find function {:#?}", to));
                }
                }*/
                _ => {}
            }
        }

        Ok(())
    }

    pub fn parse_to_program(str: &str) -> Result<MikuObject, String> {
        let lines: Vec<&str> = str.lines().collect();
        let mut out = Self::new();
        let mut line_count = 1;
        for i in lines {
            let p = split_line(i);
            let t = out.parse_instruction(p);
            if let Err(e) = t {
                let e = Err(format!("Error line:{}, {}", line_count, e));
                return e;
            }
            line_count += 1;
        }
        out.fix_ups()?;
        Ok(out.into_obj())
    }
    pub fn into_obj(self) -> MikuObject {
        MikuObject {
            instructions: self.instructions,
            labels: self.labels,
            functions: self.functions,
            extern_functions: self.extern_functions,
            global_vars: self.global_vars,
            extern_vars: self.extern_vars,
            types: self.types,
        }
    }
}

pub fn split_line(v: &str) -> Vec<&str> {
    v.split_whitespace().collect()
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MikuObject {
    pub instructions: Vec<MikuInstr>,
    pub labels: HashMap<String, usize>,
    pub functions: Vec<FuncDec>,
    pub extern_functions: Vec<FuncDec>,
    pub global_vars: HashMap<String, VarDec>,
    pub extern_vars: HashMap<String, MikuType>,
    pub types: HashMap<String, MikuType>,
}

impl MikuObject {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            labels: HashMap::new(),
            functions: Vec::new(),
            extern_functions: Vec::new(),
            global_vars: HashMap::new(),
            extern_vars: HashMap::new(),
            types: HashMap::new(),
        }
    }
    pub fn link(objects: &[MikuObject]) -> Result<Self, String> {
        let mut out = MikuObject::new();
        let mut instruction_base_count = 0;
        let mut global_var_count = 0;
        for i in objects {
            let mut actual_count = 0;
            for mut j in i.instructions.clone() {
                match &mut j {
                    MikuInstr::Jmp { to: _, to_index } => {
                        if let Some(k) = to_index {
                            *k += instruction_base_count;
                        }
                    }
                    MikuInstr::Branch {
                        to_true: _,
                        to_false: _,
                        to_false_index,
                        to_true_index,
                        var: _,
                    } => {
                        if let Some(k) = to_true_index {
                            *k += instruction_base_count;
                        } else {
                            todo!()
                        }
                        if let Some(k) = to_false_index {
                            *k += instruction_base_count;
                        } else {
                            todo!()
                        }
                    }
                    MikuInstr::Call {
                        to: _,
                        to_index: _,
                        return_var: _,
                        args: _,
                    } => {}
                    MikuInstr::DeclStatic {
                        vtype: _,
                        var_name: _,
                        count: _,
                        base_value: _,
                    } => {
                        continue;
                    }
                    MikuInstr::DeclExtern {
                        vtype: _,
                        var_name: _,
                    } => {
                        continue;
                    }
                    _ => {}
                }
                actual_count += 1;
                out.instructions.push(j);
            }
            for j in &i.global_vars {
                let mut k = j.1.clone();
                k.index += global_var_count;
                out.global_vars.insert(j.0.clone(), k);
            }
            for j in &i.functions {
                if let Some(indx) = j.index {
                    let mut k = j.clone();
                    k.index = Some(indx + instruction_base_count);
                    out.functions.push(k);
                }
            }
            instruction_base_count += i.instructions.len();
            global_var_count += actual_count;
        }
        fn update_var(var: &mut VarUse, globals: &HashMap<String, VarDec>) {
            match var {
                VarUse::Declared {
                    index,
                    name,
                    vtype: _,
                } => {
                    if globals.contains_key(name.as_ref()) {
                        *index = Some(globals[name.as_ref()].index);
                    }
                }
                _ => {}
            }
        }
        let mut ins_count = 0;
        for i in &mut out.instructions {
            match i {
                MikuInstr::BeginFunction {
                    name: _,
                    label_index,
                } => {
                    *label_index = ins_count;
                }
                MikuInstr::EndFunction { name: _ } => {}
                MikuInstr::Label {
                    name: _,
                    label_index,
                } => {
                    *label_index = ins_count;
                }
                MikuInstr::Jmp { to: _, to_index: _ } => {}
                MikuInstr::Branch {
                    to_true: _,
                    to_true_index: _,
                    to_false: _,
                    to_false_index: _,
                    var,
                } => {
                    update_var(var, &out.global_vars);
                }
                MikuInstr::Call {
                    to: _,
                    to_index: _,
                    return_var,
                    args,
                } => {
                    if let Some(v) = return_var {
                        update_var(v, &out.global_vars);
                    }
                    for k in args {
                        update_var(k, &out.global_vars);
                    }
                }
                MikuInstr::DeclVar {
                    vtype: _,
                    var_name: _,
                    count: _,
                } => {}
                MikuInstr::DeclStatic {
                    vtype: _,
                    var_name,
                    count,
                    base_value: _,
                } => {
                    *count = (out.global_vars[var_name.as_ref()].index) as u32;
                }
                MikuInstr::Assign { left, right } => {
                    update_var(left, &out.global_vars);
                    update_var(right, &out.global_vars);
                }
                MikuInstr::DerefAssign { left, right } => {
                    update_var(left, &out.global_vars);
                    update_var(right, &out.global_vars);
                }
                MikuInstr::GetRef { assigned_to, of } => {
                    update_var(assigned_to, &out.global_vars);
                    update_var(of, &out.global_vars);
                }
                MikuInstr::BinOp {
                    left,
                    right,
                    output,
                    operation: _,
                } => {
                    update_var(left, &out.global_vars);
                    update_var(right, &out.global_vars);
                    update_var(output, &out.global_vars);
                }
                MikuInstr::Return { value } => {
                    if let Some(v) = value {
                        update_var(v, &out.global_vars);
                    }
                }
                MikuInstr::DeclExtern {
                    vtype: _,
                    var_name: _,
                } => {}
            }
            ins_count += 1;
        }
        for i in objects {
            for j in &i.extern_functions {
                let mut hit = false;
                for k in &out.functions {
                    if k.name.as_ref() == j.name.as_ref() {
                        hit = true;
                        break;
                    }
                }
                if !hit {
                    out.extern_functions.push(j.clone());
                }
            }
            for j in &i.extern_vars {
                if !out.global_vars.contains_key(j.0) {
                    out.extern_vars.insert(j.0.clone(), j.1.clone());
                }
            }
        }
        out.fix_ups()?;
        Ok(out)
    }
    pub fn fix_ups(&mut self) -> Result<(), String> {
        for i in &mut self.instructions {
            match i {
                MikuInstr::Call {
                    to,
                    to_index,
                    args: _,
                    return_var: _,
                } => {
                    let mut indx = None;
                    for i in &self.functions {
                        if i.name.as_ref() == to.as_ref() {
                            indx = i.index;
                            break;
                        }
                    }
                    if let Some(idx) = indx {
                        *to_index = Some(idx);
                    } else {
                        return Err(format!("could not find function {:#?}", to));
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}
