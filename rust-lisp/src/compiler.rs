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
        captures: Vec<Var>,
        to_call: Box<Callable>,
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
        }
    }
    pub fn is_defined(&self, s: &str) -> bool {
        if self.variables.contains_key(s) {
            return true;
        }
        if self.is_function_base {
            return false;
        }
        if let Some(k) = self.next.as_ref() {
            return k.is_defined(s);
        }
        false
    }
    pub fn get_var(&self, s: &str) -> Option<Var> {
        if self.variables.contains_key(s) {
            return Some(self.variables[s].clone());
        }
        if self.is_function_base {
            return None;
        }
        if let Some(k) = self.next.as_ref() {
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
}
#[derive(Clone, Debug)]
pub struct Compiler {
    pub types: HashMap<String, Type>,
    pub current_scope: Box<Scope>,
    pub global_functions: HashMap<String, Vec<Function>>,
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
    pub fn parse_type(&self, s: &str) -> Option<Type> {
        if &s[0..1] == "*" {
            Some(Type::Box {
                vtype: Box::new(self.parse_type(&s[2..])?),
            })
        } else if &s[0..2] == "[]" {
            Some(Type::List {
                vtype: Box::new(self.parse_type(&s[2..])?),
            })
        } else if s == "int" {
            Some(Type::Integer)
        } else if s == "real" {
            Some(Type::Double)
        } else if s == "String" {
            Some(Type::String)
        } else if s == "byte" {
            Some(Type::Char)
        } else if s == "void" {
            Some(Type::Void)
        } else {
            self.types.get(s).cloned()
        }
    }
    pub fn is_valid_type(&self, s: &str) -> bool {
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
                } else if s == "branch" {
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
                    let cond = self.compile(ls[1].clone()).unwrap();
                    self.push_scope();
                    let _ = self.compile(ls[2].clone());
                    let ins = self.current_scope.instructions.clone();
                    self.pop_scope();
                    self.current_scope.instructions = Vec::new();
                    let p = Instruction::Loop {
                        condition: cond,
                        to_do: ins,
                    };
                    println!("{p:#?}");
                    self.current_scope.instructions.push(p);
                } else if s == "defun" {
                    let name = ls[1].assume_value().unwrap();
                    let ret = if let Some(r) = ls[2].assume_value() {
                        self.parse_type(r.as_ref())?
                    } else {
                        Type::Integer
                    };
                    self.push_scope();
                    self.current_scope.is_function_base = true;
                    let arg_list = ls[3].assume_list()?;
                    let mut args = Vec::new();
                    let mut i = 0;
                    while i < arg_list.len() {
                        let s = arg_list[i].assume_value().unwrap();
                        let t = arg_list[i + 1].assume_value().unwrap();
                        let typ = self.parse_type(t.as_ref())?;
                        let v = self.current_scope.decl(s, &typ);
                        args.push(v);
                        i += 2;
                    }
                    let scope_node = ls[4].clone();
                    let t = self.compile(scope_node)?;
                    let ins = self.current_scope.instructions.clone();
                    self.current_scope.instructions = Vec::new();
                    let func = Function {
                        return_type: ret,
                        arguments: args.iter().map(|i| i.get_type()).collect(),
                        ins,
                        external: false,
                    };
                    self.declare_function(name, func);
                    self.pop_scope();
                } else if s == "let" {
                    let name = ls[1].assume_value().unwrap();
                    let type_name = ls[2].assume_value().unwrap();
                    let t = self.parse_type(&type_name).unwrap();
                    let v = self.current_scope.decl(name.clone(), &t);
                    self.current_scope.instructions.push(Instruction::Declare {
                        to_declare: v.clone(),
                    });
                    return Some(v);
                } else if s == "extern" {
                    let name = ls[1].assume_value().unwrap();
                    let return_type_name = ls[2].assume_value().unwrap();
                    let return_type = self.parse_type(&return_type_name).unwrap();
                    let arg_list = ls[3].assume_list().unwrap();
                    let mut args = Vec::new();
                    let mut i = 0;
                    while i < arg_list.len() {
                        let s = arg_list[i].assume_value().unwrap();
                        let t = arg_list[i + 1].assume_value().unwrap();
                        let typ = self.parse_type(t.as_ref()).unwrap();
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
                } else {
                    self.current_scope.get_var(s.as_ref())
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
