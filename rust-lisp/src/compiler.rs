pub use crate::lisp;
pub use std::collections::HashMap;
#[derive(Clone)]
pub enum Type {
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
#[derive(Clone)]
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
    }
}
impl Var{
    pub fn get_type(&self)->Type{
    }
}
#[derive(Clone)]
pub enum Callable {
    Variable { v: Var },
    Function { v: String },
}
#[derive(Clone)]
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

pub struct Function {
    return_type: Type,
    arguments: Vec<Type>,
    ins: Vec<Instruction>,
}

pub struct Scope {
    variables: HashMap<String, Var>,
    next: Option<Box<Scope>>,
    instructions: Vec<Instruction>,
}
impl Scope {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            next: None,
            instructions: Vec::new(),
        }
    }
    pub fn is_defined(&self, s: &str) -> bool {
        if self.variables.contains_key(s) {
            return true;
        }
        if let Some(k) = self.next.as_ref() {
            return k.is_defined(s);
        }
        false
    }
    pub fn nv(&self) -> usize {
        let base = self.variables.len();
        return base + if let Some(p) = &self.next { p.nv() } else { 0 };
    }
    pub fn decl_tmp(&mut self, vtype: &Type) -> Var {
        let id = self.nv();
        let v = Var::Basic {
            idx: id,
            vtype: vtype.clone(),
            byte_offset: 0,
        };

        self.variables.insert(format!("tmp_x{}", id), v.clone());
        return v;
    }
    pub fn decl(&mut self, name: String, vtype: &Type) -> Var {
        let id = self.nv();
        let v = Var::Basic {
            idx: id,
            vtype: vtype.clone(),
            byte_offset: 0,
        };
        self.variables.insert(name, v.clone());
        return v;
    }
}
pub struct Compiler {
    types: HashMap<String, Type>,
    current_scope: Box<Scope>,
    global_functions: HashMap<String, Function>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            types: HashMap::new(),
            current_scope: Box::new(Scope::new()),
            global_functions: HashMap::new(),
        }
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
                    let f = &self.global_functions[s.as_str()];
                    let outv = self.current_scope.decl_tmp(&f.return_type);
                    let mut args = Vec::new();
                    for i in 1..ls.len() {
                        args.push(self.compile(ls[i].clone())?);
                    }
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
                    self.pop_scope();
                    let p = Instruction::Branch {
                        condition: cond,
                        if_true: ins,
                        if_false: else_ins,
                    };
                    self.current_scope.instructions.push(p);
                } else if s == "defun" {
                    let name = ls[1].assume_value()?;
                    let ret = if let Some(r) = ls[2].assume_value() {
                        self.parse_type(r.as_ref())?
                    } else {
                        Type::Integer
                    };
                    self.push_scope();
                    let arg_list = ls[3].assume_list()?;
                    let mut args = Vec::new();
                    let mut i = 0;
                    while i < arg_list.len() {
                        let s = arg_list[i].assume_value()?;
                        let t = arg_list[i + 1].assume_value()?;
                        let typ = self.parse_type(t.as_ref())?;
                        let v = self.current_scope.decl(s, &typ);
                        args.push(v);
                        i += 2;
                    }
                    let scope_node = ls[4].clone();
                    let t = self.compile(scope_node)?;
                    let ins = self.current_scope.instructions;
                    self.current_scope.instructions = Vec::new();
                    let func = Function{return_type:ret, arguments:args.iter().map(|i| i.get_type(), ins};
                    self.pop_scope();
                } else if s == "let" {
                }
            }
            lisp::Node::List { s: _ } => {
                let mut list = Vec::new();
                for i in ls {
                    if let Some(j) = self.compile(i.clone()) {
                        list.push(j);
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
            lisp::Node::Value { s } => None,
        }
    }
}
