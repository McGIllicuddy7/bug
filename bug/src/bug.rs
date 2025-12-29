use crate::parser::Expr;
use crate::tokens::Token;
use crate::tokens::TokenStream;
use crate::tokens::TokenType;
use crate::utils::Stream;

use std::error::Error;
use std::sync::Arc;
#[derive(Clone, Debug, PartialEq)]
pub struct Var {
    pub vtype: Type,
    pub name: Arc<str>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    TStr,
    TInt,
    TChar,
    TFloat,
    TVoid,
    TStruct { name: Arc<str>, fields: Arc<[Var]> }, //ptr to struct,
    TFunction{return_type:Box<Type>, args:Vec<Type>},
    TPtr{tname:Arc<str>},
}

#[derive(Clone, Debug)]
pub enum Statement {
    While {
        cond: Expr,
        list: Vec<Statement>,
    },
    If {
        cond: Expr,
        list: Vec<Statement>,
        else_list: Vec<Statement>,
    },
    Declare {
        v: Var,
        list: Expr,
    },
    Basic {
        list: Expr,
    },
    Return {
        list: Expr,
    },
}
#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    pub args: Vec<Var>,
    pub return_type: Type,
    pub list: Vec<Statement>,
}

#[derive(Clone, Debug)]
pub struct Context {
    pub types: Vec<Type>,
    pub functions: Vec<Function>,
    pub externs:Vec<Function>,
}
pub struct Scope {
    pub vars: Vec<Var>,
}
impl Context {}
pub fn parse_statement(
    context: &mut Context,
    tokens: &mut Stream<Token>,
) -> Result<Statement, Box<dyn Error>> {
    let base = tokens.peek().unwrap();
    if base.equals("if") {
        let _ = tokens.next();
        let exp = get_paren_expr(tokens);
        let expr = crate::parser::parse_expression(exp.values)?;
        println!("{:#?}", tokens);
        let scpe = get_scope(tokens);
        let scope = parse_scope(context, scpe)?;
        let mut elscp = Vec::new();
        if let Some(n) = tokens.peek() {
            if n.equals("else") {
                let _ = tokens.next();
                let nscpe = get_scope(tokens);
                elscp = parse_scope(context, nscpe)?;
            }
        }
        let out = Statement::If {
            cond: expr,
            list: scope,
            else_list: elscp,
        };
        return Ok(out);
    } else if base.equals("while") {
        let _ = tokens.next();
        let exp = get_paren_expr(tokens);
        let expr = crate::parser::parse_expression(exp.values)?;
        let scpe = get_scope(tokens);
        let scope = parse_scope(context, scpe)?;
        let out = Statement::While {
            cond: expr,
            list: scope,
        };
        return Ok(out);
    } else if base.equals("let") {
        _ = tokens.next().unwrap();
        let tt = tokens.next().unwrap();
        let vt = parse_type(&context, &tt.st)?;
        let namet = tokens.peek().unwrap();
        let name = namet.st;
        let mut ts = Vec::new();
        while let Some(k) = tokens.next() {
            if k.tt == TokenType::TokenSemi {
                break;
            }
            ts.push(k);
        }
        let expr = crate::parser::parse_expression(&ts)?;
        let out = Statement::Declare {
            v: Var { vtype: vt, name },
            list: expr,
        };
        return Ok(out);
    } else if base.equals("return") {
        _ = tokens.next().unwrap();

        let mut ts = Vec::new();
        while let Some(k) = tokens.next() {
            if k.tt == TokenType::TokenSemi {
                break;
            }
            ts.push(k);
        }
        let expr = crate::parser::parse_expression(&ts)?;
        return Ok(Statement::Return { list: expr });
    }
    let mut ts = Vec::new();
    while let Some(k) = tokens.next() {
        if k.tt == TokenType::TokenSemi {
            break;
        }
        ts.push(k);
    }
    let expr = crate::parser::parse_expression(&ts)?;
    let out = Statement::Basic { list: expr };
    Ok(out)
}
pub fn parse_scope(
    context: &mut Context,
    tokens: Stream<Token>,
) -> Result<Vec<Statement>, Box<dyn Error>> {
    let mut toks = tokens;
    let mut out = Vec::new();
    loop {
        if toks.peek().is_none() {
            break;
        }
        out.push(parse_statement(context, &mut toks)?);
    }
    Ok(out)
}
pub fn get_paren_expr<'a>(stream: &mut Stream<'a, Token>) -> Stream<'a, Token> {
    let mut count = 0;
    let tmps = stream.values;
    let mut i = 0;
    while let Some(s) = stream.next() {
        i += 1;
        if s.tt == TokenType::TokenOpenParen {
            count += 1;
        } else if s.tt == TokenType::TokenCloseParen {
            count -= 1;
        }
        if count == 0 {
            break;
        }
    }
    return Stream::new(&tmps[1..i - 1]);
}
pub fn parse_type(context: &Context, v: &str) -> Result<Type, Box<dyn Error>> {
    if v == "int" {
        return Ok(Type::TInt);
    } else if v == "float" {
        return Ok(Type::TFloat);
    } else if v == "string" {
        return Ok(Type::TStr);
    } else if v == "char" {
        return Ok(Type::TChar);
    }
    Err(format!("undefined type:{v}").into())
}
pub fn get_scope<'a>(stream: &mut Stream<'a, Token>) -> Stream<'a, Token> {
    let mut count = 0;
    let tmps = stream.values;
    let mut i = 0;
    while let Some(s) = stream.next() {
        i += 1;
        if s.tt == TokenType::TokenOpenCurl {
            count += 1;
        } else if s.tt == TokenType::TokenCloseCurl {
            count -= 1;
        }
        if count == 0 {
            break;
        }
    }
    return Stream::new(&tmps[1..i - 1]);
}
pub fn parse_program(tokens: Vec<Token>) -> Result<Context, Box<dyn Error>> {
    let mut context = Context {
        types: Vec::new(),
        functions: Vec::new(),
        externs:Vec::new()
    };
    let mut strem = Stream::new(&tokens);
    let old = strem.clone();
    while let Some(n) = strem.next() {
        if n.equals("deftype") {
            todo!();
        } else {
            continue;
        }
    }
    strem = old.clone();
    while let Some(n) = strem.next() {
        if n.equals("pub"){
            continue;
        }
        if n.equals("#include"){
            let to_import = strem.next().unwrap();
            let s= to_import.st.strip_prefix('"').unwrap().strip_suffix('"').unwrap();
            include_file(&mut context, s)?;

        }
        if n.equals("extern") {
            let name = strem.next().unwrap();
            let rvt = strem.next().unwrap();
            let rv = parse_type(&context, &rvt.st)?;
            let mut args: Vec<Var> = Vec::new();
            let mut s = strem.next().unwrap();
            if s.tt != TokenType::TokenOpenParen {
                todo!();
            }
            while let Some(p) = strem.next() {
                if p.tt == TokenType::TokenCloseParen {
                    break;
                }
                let name = p.st.clone();
                let t = strem.next().unwrap();
                let vt = parse_type(&context, &t.st)?;
                let v = Var {
                    name: name,
                    vtype: vt,
                };
                args.push(v);
            }
            s = strem.next().unwrap();
            if s.tt != TokenType::TokenSemi {
                todo!();
            }   
            context.externs.push(Function { name:name.st.to_string(), args, return_type: rv, list:Vec::new() });
        } else {
            continue;
        }
    }
    strem = old;
    while let Some(n) = strem.next() {
        if n.equals("defun") {
            let name = strem.next().unwrap();
            let rvt = strem.next().unwrap();
            let rv = parse_type(&context, &rvt.st)?;
            let mut args: Vec<Var> = Vec::new();
            let s = strem.next().unwrap();
            if s.tt != TokenType::TokenOpenParen {
                todo!();
            }
            while let Some(p) = strem.next() {
                if p.tt == TokenType::TokenCloseParen {
                    break;
                }
                let name = p.st.clone();
                let t = strem.next().unwrap();
                let vt = parse_type(&context, &t.st)?;
                let v = Var {
                    name: name,
                    vtype: vt,
                };
                args.push(v);
            }
            let s = get_scope(&mut strem);
            let scope = parse_scope(&mut context, s)?;
            let f = Function {
                name: name.st.to_string(),
                args,
                return_type: rv,
                list: scope,
            };
            context.functions.push(f);
        }
    }
    Ok(context)
}


pub fn include_file(context:&mut Context, s:&str)->Result<(), Box<dyn Error>>{
    let f = std::fs::read_to_string(s)?;
    let mut tokens = TokenStream::new(&f, s);
    while let Some(n) = tokens.next(){
        if n.equals("pub"){
            let mut p = tokens.next().unwrap();
            if p.equals("extern"){
                p = tokens.next().unwrap();
            }
            if !p.equals("defun"){
                todo!();
            }
            let name = tokens.next().unwrap();
            let rvt = tokens.next().unwrap();
            let rv = parse_type(&context, &rvt.st)?;
            let mut args: Vec<Var> = Vec::new();
            let s = tokens.next().unwrap();
            if s.tt != TokenType::TokenOpenParen {
                todo!();
            }
            while let Some(p) = tokens.next() {
                if p.tt == TokenType::TokenCloseParen {
                    break;
                }
                let name = p.st.clone();
                let t = tokens.next().unwrap();
                let vt = parse_type(&context, &t.st)?;
                let v = Var {
                    name: name,
                    vtype: vt,
                };
                args.push(v);
            }
            let f = Function{name:name.st.to_string(),args, return_type:rv,list:Vec::new()};
            context.externs.push(f);
        }
    }
    Ok(())

}
const BUITLINS:&[(&str, usize)] = &[
    ("int_to_string", 0x10000000), 
    ("float_to_string", 0x10000001), 
    ("print", 0x10000002), 
    ("println", 0x10000003), 
    ("strcat", 0x10000004), 
    ("or", 0x10000005), 
    ("and", 0x10000006), 
];
pub fn builtin(s:&str)->Option<usize>{
    for i in BUITLINS{
        if i.0 == s{
            return Some(i.1);
        }
    }
    None
}