pub use crate::lisp;
pub use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum CompilerResult {
    NoReturnedVariable,
}
#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Void,
    Integer,
    Double,
    Char,
    String,
    Bool,
    List {
        vtype: Box<Type>,
    },
    Box {
        vtype: Box<Type>,
    },
    FunctionPointer {
        return_type: Box<Type>,
        args: Vec<Type>,
    },
}
#[derive(Clone, Debug, PartialEq)]
pub enum Var {
    Basic {
        idx: usize,
        vtype: Type,
        byte_offset: usize,
    },
    FieldAccess {
        field_idx: usize,
        base: Box<Var>,
    },
    DeRef {
        base: Box<Var>,
    },
    TakeRef {
        base: Box<Var>,
    },
    ListLiteral {
        list: Vec<Var>,
    },
    StringLiteral {
        v: String,
    },
    IntegerLiteral {
        v: i64,
    },
    DoubleLiteral {
        v: f64,
    },
    CharacterLiteral {
        v: char,
    },
    BoolLiteral {
        v: bool,
    },
    FunctionPointerLiteral {
        name: String,
        args: Vec<Type>,
        return_type: Type,
    },
    LambdaLiteral {
        name: String,
        args: Vec<Type>,
        return_type: Type,
        captures: Vec<Var>,
    },
    Capture {
        idx: usize,
        vtype: Type,
    },
}
impl Var {
    pub fn get_type(&self) -> Type {
        match self {
            Self::Basic {
                idx: _,
                vtype,
                byte_offset: _,
            } => vtype.clone(),
            Self::StringLiteral { v: _ } => Type::String,
            Self::IntegerLiteral { v: _ } => Type::Integer,
            Self::DoubleLiteral { v: _ } => Type::Double,
            Self::Capture { idx: _, vtype } => vtype.clone(),
            Self::FunctionPointerLiteral {
                name: _,
                args,
                return_type,
            } => Type::FunctionPointer {
                args: args.clone(),
                return_type: Box::new(return_type.clone()),
            },
            _ => {
                todo!();
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub enum Callable {
    Variable { v: Var },
    Function { v: String },
}
#[derive(Clone, Debug)]
pub enum Instruction {
    FunctionCall {
        to_call: Callable,
        arguments: Vec<Var>,
        output: Option<Var>,
    },
    Return {
        to_return: Option<Var>,
    },
    Branch {
        condition: Var,
        if_true: Vec<Instruction>,
        if_false: Vec<Instruction>,
    },
    Loop {
        condition: Var,
        to_do: Vec<Instruction>,
    },
    Declare {
        to_declare: Var,
    },
    Assignment {
        left: Var,
        right: Var,
    },
}
#[derive(Clone, Debug)]
pub struct Function {
    pub return_type: Type,
    pub arguments: Vec<Type>,
    pub ins: Vec<Instruction>,
    pub external: bool,
}
#[derive(Clone, Debug)]
pub struct Scope {
    pub variables: HashMap<String, Var>,
    pub next: Option<Box<Scope>>,
    pub instructions: Vec<Instruction>,
    pub is_function_base: bool,
    pub can_capture: bool,
    pub captures: Vec<Var>,
}
impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}

impl Scope {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            next: None,
            instructions: Vec::new(),
            is_function_base: false,
            can_capture: false,
            captures: Vec::new(),
        }
    }
    pub fn get_capture(&self, s: &str) -> Option<Var> {
        if let Some(var) = self.variables.get(s) {
            return Some(var.clone());
        }
        if self.is_function_base {
            return None;
        }
        if let Some(k) = self.next.as_ref() {
            return k.get_capture(s);
        }
        None
    }
    pub fn is_defined(&self, s: &str) -> bool {
        if self.variables.contains_key(s) {
            return true;
        }
        if self.is_function_base {
            if self.can_capture {
                if let Some(k) = self.next.as_ref() {
                    return k.get_capture(s).is_some();
                }
            }
            return false;
        }
        if let Some(k) = self.next.as_ref() {
            return k.is_defined(s);
        }
        false
    }
    pub fn get_var(&mut self, s: &str) -> Option<Var> {
        if self.variables.contains_key(s) {
            return Some(self.variables[s].clone());
        }
        if self.is_function_base {
            if self.can_capture {
                if let Some(k) = self.next.as_ref() {
                    if let Some(p) = k.get_capture(s) {
                        let mut idx = 0;
                        let mut hit = false;
                        for i in 0..self.captures.len() {
                            if self.captures[i] == p {
                                hit = true;
                                idx = i;
                            }
                        }
                        if (!hit) {
                            idx = self.captures.len();
                            self.captures.push(p.clone());
                        }
                        return Some(Var::Capture {
                            idx,
                            vtype: p.get_type(),
                        });
                    } else {
                        return None;
                    }
                }
            }
            return None;
        }
        if let Some(k) = self.next.as_mut() {
            return k.get_var(s);
        }
        None
    }
    pub fn nv(&self) -> usize {
        let base = self.variables.len();
        if self.is_function_base {
            base
        } else {
            base + if let Some(p) = &self.next { p.nv() } else { 0 }
        }
    }
    pub fn decl_tmp(&mut self, vtype: &Type) -> Var {
        let id = self.nv();
        let v = Var::Basic {
            idx: id,
            vtype: vtype.clone(),
            byte_offset: 0,
        };

        self.variables.insert(format!("tmp_x{id}"), v.clone());
        v
    }
    pub fn decl(&mut self, name: String, vtype: &Type) -> Var {
        let id = self.nv();
        let v = Var::Basic {
            idx: id,
            vtype: vtype.clone(),
            byte_offset: 0,
        };
        self.variables.insert(name, v.clone());
        v
    }
    pub fn in_global_scope(&self) -> bool {
        self.next.is_none()
    }
    pub fn get_function_pointer(&mut self, name: &str) -> Option<Var> {
        let p = self.get_var(name)?;
        let v = p.get_type();
        match v {
            Type::FunctionPointer {
                return_type: _,
                args: _,
            } => Some(p),
            _ => None,
        }
    }
}
#[derive(Clone, Debug)]
pub struct Compiler {
    pub types: HashMap<String, Type>,
    pub current_scope: Box<Scope>,
    pub global_functions: HashMap<String, Vec<Function>>,
    pub lambda_count: usize,
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

impl Compiler {
    pub fn declare_function(&mut self, name: String, func: Function) {
        if !self.global_functions.contains_key(&name) {
            self.global_functions.insert(name.clone(), vec![]);
        }
        let ps = self.global_functions.get_mut(&name).unwrap();
        let mut inserted = false;
        for i in ps {
            let mut hit = false;
            if func.arguments.len() != i.arguments.len() {
                continue;
            }
            if i.arguments.is_empty() {
                hit = true;
            }
            for j in 0..i.arguments.len() {
                if func.arguments[j] != i.arguments[j] {
                    hit = true;
                    break;
                }
            }
            if !hit {
                if i.external {
                    *i = func.clone();
                    inserted = true;
                    break;
                } else {
                    println!(
                        "error:functions {name} and {name} have the same signature {i:#?}, {func:#?}"
                    );
                    todo!()
                }
            }
        }
        if !inserted {
            self.global_functions.get_mut(&name).unwrap().push(func);
        }
    }
    pub fn new() -> Self {
        let mut out = Self {
            types: HashMap::new(),
            current_scope: Box::new(Scope::new()),
            global_functions: HashMap::new(),
            lambda_count: 0,
        };
        out.declare_function(
            "print".to_owned(),
            Function {
                return_type: Type::Integer,
                arguments: vec![Type::String],
                ins: vec![],
                external: true,
            },
        );
        out.declare_function(
            "println".to_owned(),
            Function {
                return_type: Type::Integer,
                arguments: vec![Type::String],
                ins: vec![],
                external: true,
            },
        );
        out.declare_function(
            "+".to_owned(),
            Function {
                return_type: Type::Integer,
                arguments: vec![Type::Integer, Type::Integer],
                ins: vec![],
                external: true,
            },
        );
        out.declare_function(
            "-".to_owned(),
            Function {
                return_type: Type::Integer,
                arguments: vec![Type::Integer, Type::Integer],
                ins: vec![],
                external: true,
            },
        );
        out.declare_function(
            "*".to_owned(),
            Function {
                return_type: Type::Integer,
                arguments: vec![Type::Integer, Type::Integer],
                ins: vec![],
                external: true,
            },
        );
        out.declare_function(
            "/".to_owned(),
            Function {
                return_type: Type::Integer,
                arguments: vec![Type::Integer, Type::Integer],
                ins: vec![],
                external: true,
            },
        );
        out.declare_function(
            "==".to_owned(),
            Function {
                return_type: Type::Bool,
                arguments: vec![Type::Integer, Type::Integer],
                ins: vec![],
                external: true,
            },
        );
        out.declare_function(
            ">=".to_owned(),
            Function {
                return_type: Type::Bool,
                arguments: vec![Type::Integer, Type::Integer],
                ins: vec![],
                external: true,
            },
        );
        out.declare_function(
            "<=".to_owned(),
            Function {
                return_type: Type::Bool,
                arguments: vec![Type::Integer, Type::Integer],
                ins: vec![],
                external: true,
            },
        );
        out.declare_function(
            "to_string".to_owned(),
            Function {
                return_type: Type::String,
                arguments: vec![Type::Integer],
                ins: vec![],
                external: true,
            },
        );
        out.declare_function(
            "to_string".to_owned(),
            Function {
                return_type: Type::String,
                arguments: vec![Type::Double],
                ins: vec![],
                external: true,
            },
        );
        out
    }
    pub fn pop_scope(&mut self) {
        if self.current_scope.next.is_none() {
            return;
        }
        let mut p = self.current_scope.next.take().unwrap();
        for i in &self.current_scope.instructions {
            p.instructions.push(i.clone())
        }
        self.current_scope = p;
    }
    pub fn push_scope(&mut self) {
        let mut p = Scope::new();
        std::mem::swap(self.current_scope.as_mut(), &mut p);
        self.current_scope.next = Some(Box::new(p));
    }
    pub fn parse_type_string(&self, s: &str) -> Option<Type> {
        if s == "int" {
            Some(Type::Integer)
        } else if s == "real" {
            Some(Type::Double)
        } else if s == "string" {
            Some(Type::String)
        } else if s == "byte" {
            Some(Type::Char)
        } else {
            self.types.get(s).cloned()
        }
    }
    pub fn parse_type(&self, s: &lisp::Node) -> Option<Type> {
        if let Some(st) = s.assume_value() {
            self.parse_type_string(&st)
        } else if let Some(vec) = s.assume_list() {
            if vec.is_empty() {
                return Some(Type::Void);
            } else {
                let base = vec[0].assume_value().unwrap();
                if base == "[]" {
                    return Some(Type::List {
                        vtype: Box::new(self.parse_type(&vec[1])?),
                    });
                } else if base == "box" {
                    return Some(Type::Box {
                        vtype: Box::new(self.parse_type(&vec[1])?),
                    });
                } else if base == "fun" {
                    let ret = self.parse_type(&vec[1])?;
                    let arg_nodes = vec[2].assume_list()?;
                    let mut args = Vec::new();
                    for i in &arg_nodes {
                        let t = self.parse_type(&i)?;
                        args.push(t);
                    }
                    return Some(Type::FunctionPointer {
                        return_type: Box::new(ret),
                        args,
                    });
                }
                None
            }
        } else {
            None
        }
    }
    pub fn is_valid_type(&self, s: &lisp::Node) -> bool {
        self.parse_type(s).is_some()
    }

    pub fn compile_list(&mut self, ls: Vec<lisp::Node>) -> Option<Var> {
        let p = ls.first()?;
        match p {
            lisp::Node::Value { s } => {
                let u = "_".to_string() + s.as_ref();
                if self.current_scope.is_defined(u.as_ref()) {
                    let mut list = Vec::new();
                    for i in ls {
                        list.push(self.compile(i)?);
                    }
                    return Some(Var::ListLiteral { list });
                } else if s == "=" {
                    let l = self.compile(ls[1].clone())?;
                    let r = self.compile(ls[2].clone())?;
                    self.current_scope
                        .instructions
                        .push(Instruction::Assignment {
                            left: l.clone(),
                            right: r,
                        });
                    return Some(l);
                } else if self.global_functions.contains_key(s.as_str()) {
                    let funcs = self.global_functions[s.as_str()].clone();
                    let mut args = Vec::new();
                    for i in 1..ls.len() {
                        args.push(self.compile(ls[i].clone())?);
                    }
                    let mut func_opt = None;
                    for i in funcs {
                        if i.arguments.len() == args.len() {
                            if args.is_empty() {
                                func_opt = Some(i);
                                break;
                            }
                            let mut hit = false;
                            for j in 0..i.arguments.len() {
                                if i.arguments[j] != args[j].get_type() {
                                    hit = true;
                                    break;
                                }
                            }
                            if !hit {
                                func_opt = Some(i);
                            }
                        }
                    }
                    if func_opt.is_none() {
                        println!(
                            "error could not find function {} with arguments {:#?}",
                            s.as_str(),
                            args.iter().map(|i| i.get_type()).collect::<Vec<Type>>()
                        );
                        panic!();
                    }
                    let f = func_opt.unwrap();
                    let outv = self.current_scope.decl_tmp(&f.return_type);
                    self.current_scope.instructions.push(Instruction::Declare {
                        to_declare: outv.clone(),
                    });
                    let ir = Instruction::FunctionCall {
                        to_call: Callable::Function { v: s.clone() },
                        arguments: args,
                        output: Some(outv.clone()),
                    };
                    self.current_scope.instructions.push(ir);
                    return Some(outv);
                } else if let Some(func) = self.current_scope.get_function_pointer(&s) {
                    let mut iargs = Vec::new();
                    for i in 1..ls.len() {
                        iargs.push(self.compile(ls[i].clone())?);
                    }
                    let mut types = Vec::new();
                    for i in &iargs {
                        types.push(i.get_type());
                    }
                    match func.get_type() {
                        Type::FunctionPointer { return_type, args } => {
                            if args.len() != types.len() {
                                todo!()
                            }
                            for i in 0..args.len() {
                                if args[i] != types[i] {
                                    todo!()
                                }
                            }
                            let out = self.current_scope.decl_tmp(&return_type);
                            let ins = Instruction::FunctionCall {
                                to_call: Callable::Variable { v: func },
                                output: Some(out.clone()),
                                arguments: iargs,
                            };
                            self.current_scope.instructions.push(ins);
                            return Some(out);
                        }
                        _ => unreachable!(),
                    }
                } else if s == "ref" {
                    let name = ls[1].assume_value().unwrap();
                    if let Some(t) = self.current_scope.get_function_pointer(&name) {
                        return Some(t);
                    }
                    let ret_type = self.parse_type(&ls[2]).unwrap();
                    let l = ls[3].assume_list().unwrap();
                    let mut types = Vec::new();
                    for i in &l {
                        let p = self.parse_type(i).unwrap();
                        types.push(p);
                    }
                    return Some(Var::FunctionPointerLiteral {
                        name,
                        args: types,
                        return_type: ret_type,
                    });
                } else if s == "if" {
                    let cond = self.compile(ls[1].clone())?;
                    self.push_scope();
                    let _ = self.compile(ls[2].clone());
                    let ins = self.current_scope.instructions.clone();
                    self.current_scope.instructions = Vec::new();
                    self.pop_scope();
                    let else_ins = if ls.len() > 2 {
                        self.push_scope();
                        let _ = self.compile(ls[3].clone());
                        let ins = self.current_scope.instructions.clone();
                        self.current_scope.instructions = Vec::new();
                        self.pop_scope();
                        ins
                    } else {
                        Vec::new()
                    };
                    let p = Instruction::Branch {
                        condition: cond,
                        if_true: ins,
                        if_false: else_ins,
                    };
                    self.current_scope.instructions.push(p);
                } else if s == "loop" {
                    self.push_scope();
                    let cond = self.compile(ls[1].clone()).unwrap();
                    let _ = self.compile(ls[2].clone());
                    let ins = self.current_scope.instructions.clone();
                    self.pop_scope();
                    self.current_scope.instructions = Vec::new();
                    let p = Instruction::Loop {
                        condition: cond,
                        to_do: ins,
                    };
                    self.current_scope.instructions.push(p);
                } else if s == "defun" {
                    let name = ls[1].assume_value().unwrap();
                    if name == "lambda" {
                        todo!()
                    }
                    let ret = self.parse_type(&ls[2])?;
                    self.push_scope();
                    self.current_scope.is_function_base = true;
                    let arg_list = ls[3].assume_list()?;
                    let mut args = Vec::new();
                    let mut i = 0;
                    while i < arg_list.len() {
                        let s = arg_list[i].assume_value().unwrap();
                        let t = arg_list[i + 1].clone();
                        let typ = self.parse_type(&t)?;
                        let v = self.current_scope.decl(s, &typ);
                        args.push(v);
                        i += 2;
                    }
                    self.declare_function(
                        name.clone(),
                        Function {
                            return_type: ret.clone(),
                            arguments: args.clone().iter().map(|i| i.get_type()).collect(),
                            ins: vec![],
                            external: true,
                        },
                    );
                    let scope_node = ls[4].clone();
                    let _ = self.compile(scope_node)?;
                    let ins = self.current_scope.instructions.clone();
                    self.current_scope.instructions = Vec::new();
                    let func = Function {
                        return_type: ret,
                        arguments: args.iter().map(|i| i.get_type()).collect(),
                        ins,
                        external: false,
                    };
                    self.declare_function(name, func.clone());
                    self.pop_scope();
                } else if s == "lambda" {
                    let return_type = self.parse_type(&ls[1]).unwrap();
                    let arg_list = ls[2].assume_list().unwrap();
                    let mut args = Vec::new();
                    let mut i = 0;
                    self.push_scope();
                    self.current_scope.is_function_base = true;
                    self.current_scope.can_capture = true;
                    while i < arg_list.len() {
                        let s = arg_list[i].assume_value().unwrap();
                        let t = arg_list[i + 1].clone();
                        let typ = self.parse_type(&t)?;
                        let v = self.current_scope.decl(s, &typ);
                        args.push(v);
                        i += 2;
                    }
                    let name = format!("lamdba {}", self.lambda_count);
                    self.lambda_count += 1;
                    self.declare_function(
                        name.clone(),
                        Function {
                            return_type: return_type.clone(),
                            arguments: args.clone().iter().map(|i| i.get_type()).collect(),
                            ins: vec![],
                            external: true,
                        },
                    );
                    let scope_node = ls[3].clone();
                    let _ = self.compile(scope_node)?;
                    let ins = self.current_scope.instructions.clone();
                    self.current_scope.instructions = Vec::new();
                    let captures = self.current_scope.captures.clone();
                    let func = Function {
                        return_type: return_type.clone(),
                        arguments: args.iter().map(|i| i.get_type()).collect(),
                        ins,
                        external: false,
                    };
                    self.declare_function(name.clone(), func.clone());
                    self.pop_scope();
                    let out = Var::LambdaLiteral {
                        name,
                        args: args.iter().map(|i| i.get_type()).collect(),
                        return_type,
                        captures,
                    };
                    return Some(out);
                } else if s == "let" {
                    assert!(!self.current_scope.in_global_scope());
                    let name = ls[1].assume_value().unwrap();
                    let type_name = ls[2].clone();
                    let t = self.parse_type(&type_name).unwrap();
                    let v = self.current_scope.decl(name.clone(), &t);
                    self.current_scope.instructions.push(Instruction::Declare {
                        to_declare: v.clone(),
                    });
                    return Some(v);
                } else if s == "extern" {
                    let name = ls[1].assume_value().unwrap();
                    let return_type_name = ls[2].clone();
                    let return_type = self.parse_type(&return_type_name).unwrap();
                    let arg_list = ls[3].assume_list().unwrap();
                    let mut args = Vec::new();
                    let mut i = 0;
                    while i < arg_list.len() {
                        let s = arg_list[i].assume_value().unwrap();
                        let t = arg_list[i + 1].clone();
                        let typ = self.parse_type(&t).unwrap();
                        args.push(typ);
                        i += 2;
                    }
                    let func = Function {
                        return_type,
                        arguments: args,
                        ins: vec![],
                        external: true,
                    };
                    self.declare_function(name, func);
                } else if s == "return" {
                    let var = self.compile(ls[1].clone());
                    let ins = Instruction::Return { to_return: var };
                    self.current_scope.instructions.push(ins)
                } else if s == "import" {
                    let p = ls[1].assume_value().unwrap();
                    todo!()
                } else {
                    println!("{s}");
                    todo!()
                }
            }
            lisp::Node::List { s: _ } => {
                let mut list = Vec::new();
                for i in ls {
                    if let Some(j) = self.compile(i.clone()) {
                        list.push(j);
                    } else {
                        //todo!();
                    }
                }
                return Some(Var::ListLiteral { list });
            }
        }
        None
    }
    pub fn compile(&mut self, node: lisp::Node) -> Option<Var> {
        match node {
            lisp::Node::List { s } => self.compile_list(s),
            lisp::Node::Value { s } => {
                if s.starts_with("\"") {
                    Some(Var::StringLiteral { v: s })
                } else if let Ok(i) = s.parse::<i64>() {
                    Some(Var::IntegerLiteral { v: i })
                } else if let Ok(d) = s.parse::<f64>() {
                    Some(Var::DoubleLiteral { v: d })
                } else if let Some(p) = self.current_scope.get_var(&s) {
                    self.current_scope.get_var(s.as_ref())
                } else {
                    None
                }
            }
        }
    }
}
pub fn compile(nodes: Vec<lisp::Node>) -> Option<Compiler> {
    let mut c = Compiler::new();
    for i in nodes {
        let _ = c.compile(i);
    }
    Some(c)
}
