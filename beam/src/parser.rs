use std::{
    collections::{HashMap, HashSet},
    error::Error,
    rc::Rc,
};

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
                if c == ' ' || c == '\t' {
                } else if c == '\n' {
                    line += 1;
                } else if c == ':'
                    || c == '+'
                    || c == '-'
                    || c == '*'
                    || c == '/'
                    || c == '('
                    || c == ')'
                    || c == '<'
                    || c == '>'
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
                        || c == ')'
                        || c == '>'
                        || c == '<')
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
                        || c == '>'
                        || c == '<'
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
                    buf = "\"".to_string() + &buf + "\"";
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
    //println!("{:#?}", out);
    out
}

pub fn default_types() -> Vec<(Rc<str>, Type)> {
    vec![
        ("int".into(), Type::Integer),
        ("float".into(), Type::Float),
        ("bool".into(), Type::Bool),
        ("string".into(), Type::String),
        ("void".into(), Type::Void),
    ]
}
pub fn skip_fn(tokens: &mut TokenStream) -> Result<(), Box<dyn Error>> {
    while let Some(n) = tokens.next() {
        if n.as_ref() == "end" {
            break;
        }
        if n.as_ref() == "fn" {
            skip_fn(tokens)?;
        }
    }
    Ok(())
}
pub fn skip_struct(tokens: &mut TokenStream) -> Result<(), Box<dyn Error>> {
    while let Some(n) = tokens.next() {
        if n.as_ref() == "end" {
            break;
        }
        if n.as_ref() == "struct" {
            skip_struct(tokens)?;
        }
        if n.as_ref() == "fn" {
            skip_fn(tokens)?;
        }
    }
    Ok(())
}

pub fn import_file(
    name: String,
    imports: &mut HashSet<String>,
    prog: &mut Program,
) -> Result<(), Box<dyn Error>> {
    if imports.contains(&name) {
        return Ok(());
    }
    imports.insert(name.clone());
    let f = std::fs::read_to_string(&name)?;
    let np = preprocess_file(f, name, imports)?;
    for i in np.types {
        if !prog.types.contains(&i) {
            prog.types.push(i);
        }
    }
    for i in np.functions {
        prog.functions.insert(i.0, i.1);
    }
    Ok(())
}
pub fn preprocess_file(
    string: String,
    file: String,
    imports: &mut HashSet<String>,
) -> Result<Program, Box<dyn Error>> {
    let mut toks = TokenStream::from_string(string, file);
    let old = toks.clone();
    let mut out = Program {
        types: default_types(),
        functions: HashMap::new(),
    };
    while let Some(n) = toks.next() {
        match n.as_ref() {
            "fn" => {
                skip_fn(&mut toks)?;
            }
            "import" => {
                let name = toks.next().unwrap().text;
                import_file(name, imports, &mut out)?;
            }
            "struct" => {
                skip_struct(&mut toks)?;
            }
            _ => {
                println!("{:#?}", n);
                todo!();
            }
        }
    }
    toks = old.clone();
    while let Some(n) = toks.next() {
        match n.as_ref() {
            "fn" => {
                skip_fn(&mut toks)?;
            }
            "import" => {
                let _ = toks.next();
            }
            "struct" => {
                let strct = parse_struct(&mut toks, &out.types)?;
                out.types.push(strct);
            }
            _ => {
                todo!();
            }
        }
    }
    toks = old.clone();
    while let Some(n) = toks.next() {
        match n.as_ref() {
            "fn" => {
                let f = parse_fn_header(&mut toks, &out.types)?;
                skip_fn(&mut toks)?;
                out.functions.insert(f.0.to_string(), f.1);
            }
            "import" => {
                let _ = toks.next();
            }
            "struct" => {
                skip_struct(&mut toks)?;
            }
            _ => {
                todo!();
            }
        }
    }
    Ok(out)
}

pub fn parse_to_program(string: String, file: String) -> Result<Program, Box<dyn Error>> {
    let mut imports = HashSet::new();
    let mut out = preprocess_file(string.clone(), file.clone(), &mut imports)?;
    let mut tnew = out.types.clone();
    for i in &mut tnew {
        match &mut i.1 {
            Type::Struct { name: _, fields } => {
                let mut f = fields.to_vec();
                for j in &mut f {
                    let mut strm = TokenStream {
                        tokens: vec![Token {
                            text: j.1.name.to_string(),
                            file: file.clone(),
                            line: 0,
                        }],
                        index: 0,
                    };
                    j.1 = parse_type(&mut strm, &out.types)?;
                }
                *fields = f.into();
            }
            _ => {
                continue;
            }
        }
    }
    out.types = tnew;
    let prefix = "".to_string();
    let mut tokens = TokenStream::from_string(string, file.clone());
    while let Some(n) = tokens.next() {
        match n.as_ref() {
            "fn" => {
                let (name, func) = parse_fn(&mut tokens, &out.types, file.clone())?;
                out.functions.insert(prefix.clone() + &name, func);
            }
            "import" => {
                let _ = tokens.next();
            }
            "struct" => {
                skip_struct(&mut tokens)?;
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
    println!("{:#?}", out);
    Ok(fixups(out)?)
}

pub fn parse_type(
    tokens: &mut TokenStream,
    type_table: &[(Rc<str>, Type)],
) -> Result<ShallowType, Box<dyn Error>> {
    let Some(t) = tokens.next() else {
        todo!();
    };
    let mut s = t.text;
    let mut array_count = 0;
    while let Some(n) = s.strip_prefix("[]") {
        s = n.to_string();
        array_count += 1;
    }
    let mut idx = 0;
    for i in type_table {
        if i.0.as_ref() == s {
            return Ok(ShallowType {
                name: s.into(),
                index: idx,
                array_count,
                is_ptr: !i.1.is_primitive(),
            });
        }
        idx += 1;
    }
    Err(format!("unkown type:{:#?}", s).into())
}

pub fn parse_fn(
    tokens: &mut TokenStream,
    type_table: &[(Rc<str>, Type)],
    file: String,
) -> Result<(Rc<str>, Function), Box<dyn Error>> {
    let header = parse_fn_header(tokens, type_table)?;
    let mut cmds = Vec::new();
    cmds.push(Cmd::DeclareVariables {
        values: Rc::new([]),
    });
    let mut labels = HashMap::new();
    let mut variables = HashMap::new();
    let mut vt_stack = Vec::new();
    for i in header.1.arguments.iter() {
        let idx = variables.len();
        variables.insert(i.0.to_string(), (idx, i.1.clone()));
    }
    loop {
        let n = parse_command(
            tokens,
            &mut variables,
            type_table,
            file.clone(),
            header.0.to_string(),
        )?;
        match n {
            ParseCommandOutput::Label { name } => {
                labels.insert(name, cmds.len());
            }
            ParseCommandOutput::Command { cmd } => {
                cmds.push(cmd);
            }
            ParseCommandOutput::Done => break,
            ParseCommandOutput::Declared { name, vt } => {
                vt_stack.push(vt.as_type(&type_table));
                variables.insert(name, (variables.len(), vt));
            }
        }
    }
    cmds[0] = Cmd::DeclareVariables {
        values: vt_stack.into(),
    };
    Ok((
        header.0.clone(),
        Function {
            arguments: header.1.arguments,
            return_type: header.1.return_type,
            cmds,
            labels,
            display_name: header.0.to_string(),
        },
    ))
}

pub fn parse_struct(
    tokens: &mut TokenStream,
    _type_table: &[(Rc<str>, Type)],
) -> Result<(Rc<str>, Type), Box<dyn Error>> {
    let name = tokens.next().unwrap().text;
    let mut v = Vec::new();
    loop {
        let name1 = tokens.next().unwrap().text;
        if name1 == "end" {
            break;
        }
        let typ1 = tokens.next().unwrap().text;
        let mut array_count = 0;
        let mut tmp = typ1.as_str();
        while let Some(p) = tmp.strip_prefix("[]") {
            tmp = p;
            array_count += 1;
        }
        let defaults = default_types();
        let mut is_ptr = true;
        for i in defaults {
            if i.0.as_ref() == tmp {
                is_ptr = false;
                break;
            }
        }
        v.push((
            name1.into(),
            ShallowType {
                index: 0,
                name: tmp.into(),
                array_count,
                is_ptr,
            },
        ));
    }
    let t: Rc<str> = name.into();
    Ok((
        t.clone(),
        Type::Struct {
            name: t,
            fields: v.into(),
        },
    ))
}

pub fn parse_fn_header(
    tokens: &mut TokenStream,
    type_table: &[(Rc<str>, Type)],
) -> Result<(Rc<str>, Function), Box<dyn Error>> {
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
        args.push((name.into(), tp));
    }
    let t: Rc<str> = name.into();
    Ok((
        t.clone(),
        Function {
            arguments: args.into(),
            return_type: tp,
            cmds: Vec::new(),
            labels: HashMap::new(),
            display_name: t.to_string(),
        },
    ))
}

pub fn parse_var(
    v: String,
    variables: &HashMap<String, (usize, ShallowType)>,
    type_table: &[(Rc<str>, Type)],
) -> Result<Var, Box<dyn Error>> {
    if v == "unit" {
        return Ok(Var::Unit);
    } else if v.starts_with('"') {
        return Ok(Var::ConstString {
            value: v.trim_matches('"').into(),
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
            name: v.into(),
        });
    } else {
        if let Some(_p) = v.find('.') {
            let mut st = v.clone();
            let bst = st.split_terminator(".").next().unwrap().to_string();
            let base = parse_var(bst, variables, type_table)?;
            while let Some(p) = v.find('.') {
                let bst = st.split_off(p + 1);
                let t = base.get_type(type_table);
                match t {
                    Type::Ptr { to } => {
                        let typ = to.as_type(type_table);
                        match typ {
                            Type::Struct { name: _, fields } => {
                                let mut idx = 0;
                                for i in fields.iter() {
                                    if i.0.as_ref() == bst {
                                        return Ok(Var::FieldAccess {
                                            of: Rc::new(base),
                                            index: idx,
                                            return_type: i.1.clone(),
                                        });
                                    }
                                    idx += 1;
                                }
                            }
                            _ => {
                                println!("{:#?}", typ);
                            }
                        }
                        todo!();
                    }
                    Type::Struct { name, fields } => {
                        let mut idx = 0;
                        for i in fields.iter() {
                            if i.0.as_ref() == bst {
                                return Ok(Var::FieldAccess {
                                    of: Rc::new(base),
                                    index: idx,
                                    return_type: i.1.clone(),
                                });
                            }
                            idx += 1;
                        }
                        println!("type:{:#?} does not have field:{:#?}", name, bst);
                        return Err(
                            format!("type:{:#?} does not have field:{:#?}", name, bst).into()
                        );
                    }
                    _ => {
                        todo!()
                    }
                }
            }
            todo!()
        }
    }
    // println!("unknown var:{}", v);
    Err(format!("unknown var:{}", v).into())
}

pub fn parse_command(
    tokens: &mut TokenStream,
    variables: &mut HashMap<String, (usize, ShallowType)>,
    type_table: &[(Rc<str>, Type)],
    file: String,
    function_name: String,
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
            name: function_name + &tokens.next().unwrap().text,
        });
    }
    if s == "goto" {
        return Ok(ParseCommandOutput::Command {
            cmd: Cmd::Jmp {
                to: (function_name + &tokens.next().unwrap().text).into(),
                to_idx: 0,
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
                to: (function_name + &to.text).into(),
                to_idx: 0,
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
                    args: args.into(),
                },
            })
        } else if n.text == "=" {
            let Some(ln) = tokens.next() else { todo!() };
            if ln.text == "new" {
                let mut t = parse_type(tokens, type_table)?;
                t.is_ptr = false;
                return Ok(ParseCommandOutput::Command {
                    cmd: Cmd::Assign {
                        l: v,
                        r: Var::OperatorNew { new_type: t },
                    },
                });
            }
            let Ok(l) = parse_var(ln.text.clone(), variables, type_table) else {
                let paren = tokens.next().unwrap();
                if paren.text != "(" {
                    println!("{:#?}, {:#?}", ln, paren);
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
                            name: func_mangle(ln.text.clone(), file.clone()).into(),
                            idx: 0,
                        },
                        returned: v,
                        args: args.into(),
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
                    "*" => Binop::Mul,
                    "/" => Binop::Div,
                    "==" => Binop::Equal,
                    "!=" => Binop::NotEqual,
                    ">" => Binop::Greater,
                    "<" => Binop::Less,
                    "or" => Binop::Or,
                    "and" => Binop::And,
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
                        args: args.into(),
                    },
                })
            } else if op.text == "new" {
                let t = parse_type(tokens, type_table)?;
                Ok(ParseCommandOutput::Command {
                    cmd: Cmd::Assign {
                        l,
                        r: Var::OperatorNew { new_type: t },
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

pub fn func_mangle(function_name: String, _file: String) -> String {
    function_name.to_string()
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
                    println!(
                        "incompatable types:{:#?}, {:#?}",
                        l.get_type(&p.types),
                        r.get_type(&p.types)
                    );
                    todo!()
                }
            }
            Cmd::Jmp { to, to_idx: _ } => {
                if !f.labels.contains_key(to.as_ref()) {
                    todo!()
                }
            }
            Cmd::JmpCond {
                cond,
                to,
                to_idx: _,
            } => {
                if cond.get_type(&p.types) != Type::Bool {
                    todo!()
                }
                if !f.labels.contains_key(to.as_ref()) {
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
                        let f = &p.functions[name.as_ref()];
                        let from: Vec<Type> =
                            f.arguments.iter().map(|i| i.1.as_type(&p.types)).collect();
                        let to = f.return_type.as_type(&p.types);
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
                if to_return.get_type(&p.types) != f.return_type.as_type(&p.types) {
                    println!(
                        "expected type:{:#?}, found type:{:#?}",
                        f.return_type.as_type(&p.types),
                        to_return.get_type(&p.types),
                    );
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
    for _ in 0..8 {
        out.cmds.push(Cmd::Jmp {
            to: "__failed".into(),
            to_idx: 0,
        })
    }
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
            out.type_table.push((j.0.clone(), j.1.clone()));
        }
    }
    for i in &mut out.cmds {
        match i {
            Cmd::Call {
                to_call,
                returned: _,
                args: _,
            } => {
                if let Var::FunctionLiteral { name, idx } = to_call {
                    if let Some(s) = out.symbol_table.get(name.as_ref()) {
                        *idx = *s;
                    }
                }
            }
            Cmd::Jmp { to, to_idx } => {
                if let Some(s) = out.symbol_table.get(to.as_ref()) {
                    *to_idx = *s;
                }
            }
            Cmd::JmpCond {
                to,
                to_idx,
                cond: _,
            } => {
                if let Some(s) = out.symbol_table.get(to.as_ref()) {
                    *to_idx = *s;
                }
            }
            _ => continue,
        }
    }
    out
}
