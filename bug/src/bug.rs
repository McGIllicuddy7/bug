use crate::parser::Expr;
use crate::tokens::Token;
use crate::tokens::TokenType;
use crate::utils::Stream;

use std::error::Error;
use std::sync::Arc;
pub struct Var {
    pub vtype: Type,
    pub name: Arc<str>,
}
pub enum Type {
    TStr,
    TInt,
    TChar,
    TFloat,
    TStruct { name: Arc<str>, fields: Arc<[Var]> },
}

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
    },
    Basic {
        list: Expr,
    },
}
pub struct Function {
    name: String,
    args: Vec<Var>,
    return_type: Type,
    list: Vec<Statement>,
}
pub struct Context {
    pub types: Vec<Type>,
    pub functions: Vec<Function>,
    pub scopes: Vec<Scope>,
}
pub struct Scope {
    vars: Vec<Var>,
}
pub fn parse_statement(
    context: &mut Context,
    tokens: Stream<Token>,
) -> Result<Statement, Box<dyn Error>> {
    todo!();
}
pub fn parse_scope(
    context: &mut Context,
    tokens: Stream<Token>,
) -> Result<Vec<Statement>, Box<dyn Error>> {
    todo!();
}
pub fn get_paren_expr<'a>(strem: &mut Stream<'a, Token>) -> Stream<'a, Token> {
    let mut out = strem.collect_until(&mut |tokens: &[Token]| {
        let mut count = 0;
        for i in tokens {
            if i.tt == TokenType::TokenOpenParen {
                count += 1;
            }
            if i.tt == TokenType::TokenCloseParen {
                count -= 1;
            }
            if count == 0 {
                return true;
            }
        }
        false
    });
    out.values = &out.values[1..out.values.len() - 1];
    out
}
pub fn parse_program(tokens: Vec<Token>) -> Context {
    let mut out = Context {
        types: Vec::new(),
        functions: Vec::new(),
        scopes: Vec::new(),
    };
    let mut strem = Stream::new(&tokens);
    while let Some(n) = strem.next() {
        if n.equals("defun") {
            let name = strem.next().unwrap();
            let rvt = strem.next().unwrap();
        } else if n.equals("deftype") {
        } else {
            continue;
        }
    }
    out
}
