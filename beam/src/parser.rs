use std::{collections::HashMap, error::Error};

use crate::mach::{self, Binop, Cmd, Function, Heap, Machine, Program, ShallowType, Type, Var};

#[derive(Clone, Debug)]
pub struct Token {
    pub text: String,
    pub file: String,
    pub line: usize,
}
#[derive(Clone, Debug)]
pub struct TokenStream {
    pub tokens: Vec<Token>,
    pub index: usize,
}
#[derive(Clone, Debug)]
pub enum ParseCommandOutput {
    Label { name: String },
    Command { cmd: Cmd },
    Declared { name: String, vt: ShallowType },
    Done,
}
impl AsRef<str> for Token {
    fn as_ref(&self) -> &str {
        &self.text
    }
}
impl TokenStream {
    pub fn from_string(s: String, file: String) -> Self {
        let tokens = tokenize(s, file);
        Self { tokens, index: 0 }
    }
    pub fn peek(&self) -> Option<Token> {
        let mut t = self.clone();
        t.next()
    }
    pub fn insert_next(&mut self, t: Token) {
        self.tokens.insert(self.index, t);
    }
}
impl Iterator for TokenStream {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.tokens.len() {
            let out = self.tokens[self.index].clone();
            self.index += 1;
            Some(out)
        } else {
            None
        }
    }
}

pub fn tokenize(s: String, file: String) -> Vec<Token> {
    enum State {
        Whitespace,
        Ident,
        String,
        Comment,
        StringEscaped,
    }
    let mut out = Vec::new();
    let mut buf = String::new();
    let mut line = 1;
    let mut state = State::Whitespace;
    for c in s.chars() {
        match state {
            State::Whitespace => {
                if c == ' ' {
                } else if c == '\n' {
                    line += 1;
                } else if c == ':'
                    || c == '+'
                    || c == '-'
                    || c == '*'
                    || c == '/'
                    || c == '('
                    || c == ')'
                {
                    out.push(Token {
                        text: c.to_string(),
                        file: file.clone(),
                        line,
                    });
                } else if c == '"' {
                    buf = String::new();
                    state = State::String;
                } else if c == ';' {
                    buf = String::new();
                    state = State::Comment;
                } else {
                    buf = String::new();
                    buf.push(c);
                    state = State::Ident;
                }
            }
            State::Ident => {
                if !c.is_whitespace()
                    && !(c == ':'
                        || c == '+'
                        || c == '-'
                        || c == '*'
                        || c == '/'
                        || c == ';'
                        || c == '('
                        || c == ')')
                {
                    buf.push(c);
                } else {
                    out.push(Token {
                        text: buf,
                        file: file.clone(),
                        line,
                    });
                    buf = String::new();
                    if c == '\n' {
                        line += 1;
                    } else if c == ':'
                        || c == '+'
                        || c == '-'
                        || c == '*'
                        || c == '/'
                        || c == '('
                        || c == ')'
                    {
                        out.push(Token {
                            text: c.to_string(),
                            file: file.clone(),
                            line,
                        });
                    }
                    state = if c == ';' {
                        State::Comment
                    } else {
                        State::Whitespace
                    };
                }
            }
            State::String => {
                if c == '"' {
                    out.push(Token {
                        text: buf,
                        file: file.clone(),
                        line,
                    });
                    buf = String::new();
                    state = State::Whitespace;
                } else if c == '\\' {
                    state = State::StringEscaped;
                } else if c == '\n' {
                    line += 1;
                } else {
                    buf.push(c);
                }
            }
            State::StringEscaped => {
                buf.push(c);
                state = State::String;
            }
            State::Comment => {
                if c == '\n' {
                    line += 1;
                    state = State::Whitespace
                }
            }
        }
    }
    if !buf.is_empty() {
        out.push(Token {
            text: buf,
            file,
            line,
        });
    }
    println!("{:#?}", out);
    out
}
pub fn default_types() -> Vec<(String, Type)> {
    vec![
        ("int".to_string(), Type::Integer),
        ("float".to_string(), Type::Float),
        ("bool".to_string(), Type::Bool),
        ("string".to_string(), Type::String),
        ("void".to_string(), Type::Void),
    ]
}
pub fn parse_to_program(string: String, file: String) -> Result<Program, Box<dyn Error>> {
    let mut out = Program {
        types: default_types(),
        functions: HashMap::new(),
    };
    let prefix = "fn_".to_string() + &file;
    let mut tokens = TokenStream::from_string(string, file.clone());
    while let Some(n) = tokens.next() {
        match n.as_ref() {
            "fn" => {
                let (name, func) = parse_fn(&mut tokens, &out.types, file.clone())?;
                out.functions.insert(prefix.clone() + "_" + &name, func);
            }
            "import" => {
                todo!()
            }
            "struct" => {
                let (name, t) = parse_struct(&mut tokens, &out.types)?;
                out.types.push((name, t));
            }
            "extern" => {
                let Some(n) = tokens.next() else { todo!() };
                if n.as_ref() == "fn" {
                    todo!()
                } else if n.as_ref() == "struct" {
                    todo!()
                } else {
                    todo!()
                }
            }
            _ => {
                return Err(format!(
                    "unexepected token:{:#?} file:{:#?} line:{:#?}",
                    n.as_ref(),
                    n.file,
                    n.line
                )
                .into());
            }
        }
    }
    Ok(fixups(out)?)
}
pub fn parse_type(
    tokens: &mut TokenStream,
    type_table: &[(String, Type)],
) -> Result<ShallowType, Box<dyn Error>> {
    let Some(t) = tokens.next() else {
        todo!();
    };
    let s = t.text;
    if let Some(_n) = s.strip_prefix("[]") {
        todo!()
    } else {
        let mut idx = 0;
        for i in type_table {
            if i.0 == s {
                return Ok(ShallowType {
                    name: s,
                    index: idx,
                });
            }
            idx += 1;
        }
    }
    Err(format!("unkown type:{:#?}", s).into())
}
pub fn parse_fn(
    tokens: &mut TokenStream,
    type_table: &[(String, Type)],
    file: String,
) -> Result<(String, Function), Box<dyn Error>> {
    let header = parse_fn_header(tokens, type_table)?;
    let mut cmds = Vec::new();
    cmds.push(Cmd::DeclareVariables { values: Vec::new() });
    let mut labels = HashMap::new();
    let mut variables = HashMap::new();
    for i in &header.1.arguments {
        let idx = variables.len();
        variables.insert(i.0.clone(), (idx, i.1.clone()));
    }
    loop {
        let n = parse_command(tokens, &mut variables, type_table, file.clone())?;
        match n {
            ParseCommandOutput::Label { name } => {
                labels.insert(name, cmds.len());
            }
            ParseCommandOutput::Command { cmd } => {
                cmds.push(cmd);
            }
            ParseCommandOutput::Done => break,
            ParseCommandOutput::Declared { name, vt } => {
                variables.insert(name, (variables.len(), vt));
            }
        }
    }
    let mut arg_types = Vec::new();
    for i in variables {
        arg_types.push(type_table[i.1.1.index as usize].1.clone());
    }
    if !header.1.arguments.is_empty() {
        arg_types = arg_types[header.1.arguments.len() - 1..].to_vec();
    }
    cmds[0] = Cmd::DeclareVariables { values: arg_types };
    Ok((
        header.0.clone(),
        Function {
            arguments: header.1.arguments,
            return_type: header.1.return_type,
            cmds,
            labels,
            display_name: header.0.clone(),
        },
    ))
}
pub fn parse_struct(
    _tokens: &mut TokenStream,
    _type_table: &[(String, Type)],
) -> Result<(String, Type), Box<dyn Error>> {
    todo!()
}
pub fn parse_fn_header(
    tokens: &mut TokenStream,
    type_table: &[(String, Type)],
) -> Result<(String, Function), Box<dyn Error>> {
    let tp = parse_type(tokens, type_table)?;
    let Some(name) = tokens.next().map(|i| i.text) else {
        todo!()
    };
    let mut args = Vec::new();
    while let Some(t) = tokens.next() {
        if t.text == ":" {
            break;
        }
        let name = t.text;
        let tp = parse_type(tokens, type_table)?;
        args.push((name, tp));
    }
    Ok((
        name.clone(),
        Function {
            arguments: args,
            return_type: tp,
            cmds: Vec::new(),
            labels: HashMap::new(),
            display_name: name,
        },
    ))
}
pub fn parse_var(
    v: String,
    variables: &HashMap<String, (usize, ShallowType)>,
    type_table: &[(String, Type)],
) -> Result<Var, Box<dyn Error>> {
    if v == "unit" {
        return Ok(Var::Unit);
    } else if v.starts_with('"') {
        return Ok(Var::ConstString {
            value: v.trim_matches('"').to_string(),
        });
    } else if let Ok(s) = v.parse::<i64>() {
        return Ok(Var::ConstInt { value: s });
    } else if let Ok(s) = v.parse::<f64>() {
        return Ok(Var::ConstFloat { value: s });
    } else if let Ok(s) = v.parse::<bool>() {
        return Ok(Var::ConstBool { value: s });
    } else if let Some(s) = variables.get(&v) {
        return Ok(Var::Stack {
            vtype: s.1.clone(),
            index: s.0,
            name: v,
        });
    } else {
        if let Some(p) = v.find('.') {
            let mut st = v.clone();
            let bst = st.split_off(p);
            let base = parse_var(bst, variables, type_table)?;
            while let Some(p) = v.find('.') {
                let _bst = st.split_off(p);
                let t = base.get_type(type_table);
                match t {
                    Type::Ptr { to: _ } => {
                        todo!()
                    }
                    Type::Struct { name: _, fields: _ } => {
                        todo!()
                    }
                    _ => {
                        todo!()
                    }
                }
            }
            todo!()
        }
    }
    Err(format!("unknown var:{}", v).into())
}
pub fn parse_command(
    tokens: &mut TokenStream,
    variables: &mut HashMap<String, (usize, ShallowType)>,
    type_table: &[(String, Type)],
    file: String,
) -> Result<ParseCommandOutput, Box<dyn Error>> {
    let Some(base) = tokens.next() else { todo!() };
    let s = base.text.clone();
    let base_s = s.clone();
    if s == "end" {
        return Ok(ParseCommandOutput::Done);
    }
    if s == "return" {
        return Ok(ParseCommandOutput::Command {
            cmd: Cmd::Return {
                to_return: parse_var(tokens.next().unwrap().text, variables, type_table)?,
            },
        });
    }
    if s == "label" {
        return Ok(ParseCommandOutput::Label {
            name: tokens.next().unwrap().text,
        });
    }
    if s == "goto" {
        return Ok(ParseCommandOutput::Command {
            cmd: Cmd::Jmp {
                to: tokens.next().unwrap().text,
            },
        });
    }
    if s == "if" {
        let v = parse_var(tokens.next().unwrap().text, variables, type_table)?;
        let t = tokens.next().unwrap();
        if t.text != "goto" {
            todo!();
        }
        let to = tokens.next().unwrap();
        return Ok(ParseCommandOutput::Command {
            cmd: Cmd::JmpCond {
                cond: v,
                to: to.text,
            },
        });
    }
    if let Ok(v) = parse_var(s, variables, type_table) {
        let Some(n) = tokens.next() else { todo!() };
        if n.text == "(" {
            let mut args = Vec::new();
            loop {
                let Some(n) = tokens.next() else { todo!() };
                if n.text == ")" {
                    break;
                } else {
                    args.push(parse_var(n.text, variables, type_table)?);
                }
            }
            Ok(ParseCommandOutput::Command {
                cmd: Cmd::Call {
                    to_call: v,
                    returned: Var::Unit,
                    args,
                },
            })
        } else if n.text == "=" {
            let Some(ln) = tokens.next() else { todo!() };
            let Ok(l) = parse_var(ln.text.clone(), variables, type_table) else {
                let paren = tokens.next().unwrap();
                if paren.text != "(" {
                    println!("{:#?}", paren);
                    todo!();
                }
                let mut args = Vec::new();
                loop {
                    let Some(n) = tokens.next() else { todo!() };
                    if n.text == ")" {
                        break;
                    } else {
                        args.push(parse_var(n.text, variables, type_table)?);
                    }
                }
                return Ok(ParseCommandOutput::Command {
                    cmd: Cmd::Call {
                        to_call: Var::FunctionLiteral {
                            name: func_mangle(ln.text.clone(), file.clone()),
                        },
                        returned: v,
                        args,
                    },
                });
            };
            let Some(op) = tokens.peek() else { todo!() };
            if op.text == "+"
                || op.text == "-"
                || op.text == "*"
                || op.text == "/"
                || op.text == "=="
                || op.text == "!="
                || op.text == ">"
                || op.text == "<"
                || op.text == "or"
                || op.text == "and"
            {
                let _ = tokens.next();
                let Some(rn) = tokens.next() else { todo!() };
                let r = parse_var(rn.text, variables, type_table)?;
                let opr = match op.text.as_str() {
                    "+" => Binop::Add,
                    "-" => Binop::Sub,
                    "*" => Binop::Add,
                    "/" => Binop::Sub,
                    "==" => Binop::Equal,
                    "!=" => Binop::NotEqual,
                    ">" => Binop::Greater,
                    "<" => Binop::Less,
                    "or" => Binop::Or,
                    "and" => Binop::Add,
                    _ => {
                        todo!()
                    }
                };
                Ok(ParseCommandOutput::Command {
                    cmd: Cmd::Binop {
                        l,
                        r,
                        out: v,
                        op: opr,
                    },
                })
            } else if op.text == "(" {
                let _ = tokens.next();
                let mut args = Vec::new();
                loop {
                    let Some(n) = tokens.next() else { todo!() };
                    if n.text == ")" {
                        break;
                    } else {
                        args.push(parse_var(n.text, variables, type_table)?);
                    }
                }
                Ok(ParseCommandOutput::Command {
                    cmd: Cmd::Call {
                        to_call: l,
                        returned: v,
                        args,
                    },
                })
            } else {
                Ok(ParseCommandOutput::Command {
                    cmd: Cmd::Assign { l: v, r: l },
                })
            }
        } else {
            println!("{:#?}", base);
            todo!()
        }
    } else {
        let Some(n) = tokens.next() else { todo!() };
        if n.text != "(" {
            if n.text == ":" {
                let vt = parse_type(tokens, type_table)?;
                if let Some(t) = tokens.peek()
                    && t.text == "="
                {
                    tokens.insert_next(base.clone());
                }
                return Ok(ParseCommandOutput::Declared { name: base_s, vt });
            } else {
                println!("{:#?}", base);
                todo!()
            }
        }
        let mut args = Vec::new();
        loop {
            let Some(n) = tokens.next() else { todo!() };
            if n.text == ")" {
                break;
            } else {
                args.push(parse_var(n.text, variables, type_table)?);
            }
        }
        todo!()
    }
}
pub fn func_mangle(function_name: String, file: String) -> String {
    format!("fn_{}_{}", file, function_name)
}
pub fn fixups(p: Program) -> Result<Program, String> {
    let mut out = Program {
        types: p.types.clone(),
        functions: HashMap::new(),
    };
    for i in &p.functions {
        let f = function_fixups(&p, i.1)?;
        out.functions.insert(i.0.clone(), f);
    }
    Ok(out)
}
pub fn function_fixups(p: &Program, f: &Function) -> Result<Function, String> {
    let mut out = f.clone();
    out.cmds.clear();
    for i in &f.cmds {
        match i {
            Cmd::Binop {
                l,
                r,
                out: _,
                op: _,
            } => {
                if l.get_type(&p.types) != r.get_type(&p.types) {
                    todo!()
                }
            }
            Cmd::Assign { l, r } => {
                if l.get_type(&p.types) != r.get_type(&p.types) {
                    todo!()
                }
            }
            Cmd::Jmp { to } => {
                if !f.labels.contains_key(to) {
                    todo!()
                }
            }
            Cmd::JmpCond { cond, to } => {
                if cond.get_type(&p.types) != Type::Bool {
                    todo!()
                }
                if !f.labels.contains_key(to) {
                    todo!()
                }
            }
            Cmd::DeclareVariables { values: _ } => {}
            Cmd::Call {
                to_call,
                returned,
                args,
            } => {
                let t = to_call.get_type(&p.types);
                match t {
                    Type::Function {
                        from: _,
                        to: _,
                        name,
                    } => {
                        println!("{:#?}", name);
                        let f = &p.functions[&name];
                        let from: Vec<Type> = f
                            .arguments
                            .iter()
                            .map(|i| p.types[i.1.index as usize].1.clone())
                            .collect();
                        let to = p.types[f.return_type.index as usize].1.clone();
                        if returned.get_type(&p.types) != to {
                            todo!()
                        }
                        if args.len() != from.len() {
                            todo!()
                        }
                        for i in 0..args.len() {
                            if args[i].get_type(&p.types) != from[i] {
                                todo!()
                            }
                        }
                    }
                    _ => {
                        todo!()
                    }
                }
            }
            Cmd::Return { to_return } => {
                if to_return.get_type(&p.types) != p.types[f.return_type.index as usize].1 {
                    todo!()
                }
            }
        }
        out.cmds.push(i.clone());
    }
    Ok(out)
}
pub fn link(progs: &[Program]) -> mach::Machine {
    let mut out = Machine {
        cmds: Vec::new(),
        ip: 0,
        v_start: 0,
        v_end: 0,
        frames: Vec::new(),
        to_return: None,
        heap: Heap::new(),
        type_table: Vec::new(),
        symbol_table: HashMap::new(),
        stack: Vec::new(),
        done: false,
    };
    for i in progs {
        for j in &i.functions {
            if j.1.display_name == "main" {
                out.ip = out.cmds.len() as u64;
            }
            let base = out.cmds.len();
            out.symbol_table.insert(j.0.clone(), out.cmds.len());
            for k in &j.1.cmds {
                out.cmds.push(k.clone());
            }
            for k in &j.1.labels {
                out.symbol_table.insert(k.0.clone(), *k.1 + base);
            }
        }
        for j in &i.types {
            out.type_table.push(j.clone());
        }
    }
    out
}
