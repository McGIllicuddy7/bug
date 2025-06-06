use crate::tokenizer::*;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Scope {
    pub variables: HashMap<String, (Type, usize)>,
}
impl Scope {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
}
#[derive(Clone, Debug)]
pub struct Function {
    pub ret_type: Type,
    pub arg_types: Vec<Type>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Undef,
    Void,
    Char,
    Integer,
    Float,
    Pointer { to: Box<Type> },
    Array { of: Box<Type>, size: usize },
    Slice { of: Box<Type> },
    Struct { of: Vec<Type> },
    Type,
    Function,
}
impl Type {
    pub fn size_of(&self) -> usize {
        match self {
            Self::Void => 0,
            Self::Char => 1,
            Self::Integer => 8,
            Self::Float => 8,
            Self::Pointer { to: _ } => 8,
            Self::Array { of, size } => of.size_of() * size,
            Self::Struct { of } => {
                let mut out = 0;
                for a in of {
                    if out % a.align_of() == 0 {
                        out += a.size_of()
                    } else {
                        out += a.align_of() - out % a.align_of();
                        assert!(out % a.align_of() == 0);
                        out += a.size_of();
                    }
                }
                out
            }
            Self::Undef => 0,
            Self::Type => 0,
            Self::Function => 0,
            Self::Slice { of: _ } => 16,
        }
    }
    pub fn align_of(&self) -> usize {
        match self {
            Self::Void => 0,
            Self::Char => 1,
            Self::Integer => 8,
            Self::Float => 8,
            Self::Undef => 0,
            Self::Pointer { to: _ } => 8,
            Self::Array { of, size: _ } => of.align_of(),
            Self::Struct { of } => {
                let mut out = 0;
                for a in of {
                    if a.align_of() > out {
                        out = a.align_of()
                    }
                }
                out
            }
            Self::Type => 0,
            Self::Function => 0,
            Self::Slice { of: _ } => 8,
        }
    }
}
#[derive(Clone, Debug)]
pub enum TTNode {
    Empty,
    VariableUsage {
        variable_name: Arc<str>,
        stack_offset: usize,
        variable_type: Type,
    },
    Assignment {
        target: Box<TTNode>,
        target_type: Type,
        source: Box<TTNode>,
    },
    Call {
        function_name: Arc<str>,
        return_type: Type,
        args: Vec<TTNode>,
    },
    Return {
        to_return: Option<Box<TTNode>>,
        return_type: Type,
    },
    DeclareVariable {
        name: Arc<str>,
        offset: usize,
        variable_type: usize,
        value: Option<Box<TTNode>>,
    },
    Scope {
        expressions: Vec<TTNode>,
    },
    Loop {
        condition: Box<TTNode>,
        to_run: Box<TTNode>,
    },
    If {
        condition: Box<TTNode>,
        to_run: Box<TTNode>,
        elseblck: Box<TTNode>,
    },
    StringLiteral {
        value: Arc<str>,
    },
    ArrayLiteral {
        values: Vec<TTNode>,
    },
    IntegerLiteral {
        value: i64,
    },
    FloatLiteral {
        value: f64,
    },
    DeclareFunction {
        name: Arc<str>,
        return_type: Type,
        args: Vec<Type>,
        list: Option<Box<TTNode>>,
    },
    DeclareStruct {
        name: Arc<str>,
        fields: (Type, Arc<str>),
    },
    TakeRef {
        of: Box<TTNode>,
    },
    Deref {
        v: Box<TTNode>,
    },
    Dot {
        field_name: Arc<str>,
        field_type: Type,
    },
}
impl TTNode {
    pub fn get_type(&self) -> Type {
        match self {
            Self::Empty => Type::Void,
            Self::VariableUsage {
                variable_name: _,
                stack_offset: _,
                variable_type,
            } => variable_type.clone(),
            Self::Assignment {
                target: _,
                target_type: _,
                source: _,
            } => Type::Void,
            Self::Call {
                function_name: _,
                return_type,
                args: _,
            } => return_type.clone(),
            Self::DeclareVariable {
                name: _,
                offset: _,
                variable_type: _,
                value: _,
            } => Type::Void,
            Self::Scope { expressions: _ } => Type::Void,
            Self::Loop {
                condition: _,
                to_run: _,
            } => Type::Void,
            Self::If {
                condition: _,
                to_run,
                elseblck: _,
            } => to_run.get_type(),
            Self::StringLiteral { value } => Type::Array {
                of: Box::new(Type::Char),
                size: value.len(),
            },
            Self::ArrayLiteral { values } => match values.first() {
                None => Type::Array {
                    of: Box::new(Type::Undef),
                    size: 0,
                },
                Some(k) => Type::Array {
                    of: Box::new(k.get_type()),
                    size: values.len(),
                },
            },
            Self::IntegerLiteral { value: _ } => Type::Integer,
            Self::FloatLiteral { value: _ } => Type::Float,
            Self::DeclareFunction {
                name: _,
                return_type: _,
                args: _,
                list: _,
            } => Type::Function,
            Self::DeclareStruct { name: _, fields: _ } => Type::Type,
            Self::TakeRef { of } => Type::Pointer {
                to: Box::new(of.get_type()),
            },
            Self::Deref { v } => match v.get_type() {
                Type::Pointer { to } => to.as_ref().clone(),
                _ => {
                    unreachable!()
                }
            },
            Self::Return {
                to_return: _,
                return_type,
            } => return_type.clone(),
            Self::Dot {
                field_name: _,
                field_type,
            } => field_type.clone(),
        }
    }
}

pub struct Parser<'a> {
    pub tokens: crate::tokenizer::Tokenizer<'a>,
    pub scope_stack: Vec<Scope>,
    pub global_var_offset: usize,
    pub local_var_offset: usize,
    pub declared_types: HashMap<String, Type>,
    pub declared_functions: HashMap<String, Function>,
}

impl<'a> Parser<'a> {
    pub fn new(tokenizer: crate::tokenizer::Tokenizer<'a>) -> Self {
        Self {
            tokens: tokenizer,
            scope_stack: vec![Scope::new()],
            global_var_offset: 0,
            local_var_offset: 0,
            declared_types: HashMap::new(),
            declared_functions: HashMap::new(),
        }
    }
    pub fn get_as_type(&self, t: Token) -> Result<Type, String> {
        if !self.declared_types.contains_key(t.st.as_ref()) {
            return Err("undeclared type".into());
        } else {
            return Ok(self.declared_types[t.st.as_ref()].clone());
        }
    }

    pub fn next_token(&mut self) -> crate::tokenizer::Token {
        self.tokens.next_token()
    }
    pub fn expect_next_token_is(&mut self, tt: TokenType) -> bool {
        let s = self.next_token();
        s.tt == tt
    }
    pub fn expect_type(&mut self) -> Result<Type, String> {
        let p = self.next_token();
        match p.tt {
            TokenType::Literal => match p.st.as_ref() {
                "void" => Ok(Type::Void),
                "char" => Ok(Type::Char),
                "int" => Ok(Type::Integer),
                "float" => Ok(Type::Float),
                _ => Err(format!("error unknown type: {:#?}", p.st.as_ref())),
            },
            TokenType::OpenBracket => {
                let s = self.next_token();
                match s.tt {
                    TokenType::Number => {
                        if self.expect_next_token_is(TokenType::CloseBracket) {
                            let ty = self.expect_type()?;
                            Ok(Type::Array {
                                of: Box::new(ty),
                                size: s.st.parse::<usize>().unwrap(),
                            })
                        } else {
                            Err(format!("error unknown type: {:#?}", s.st.as_ref()))
                        }
                    }
                    _ => {
                        if self.expect_next_token_is(TokenType::CloseBracket) {
                            Ok(Type::Slice {
                                of: Box::new(self.expect_type()?),
                            })
                        } else {
                            Err(format!("error unknown type: {:#?}", s.st.as_ref()))
                        }
                    }
                }
            }
            TokenType::Operator => match p.st.as_ref() {
                "*" => {
                    let tpe = self.expect_type()?;
                    Ok(Type::Pointer { to: Box::new(tpe) })
                }
                _ => Err(format!("error unknown type: {:#?}", p.st.as_ref())),
            },
            _ => Err(format!("error unknown type: {:#?}", p.st.as_ref())),
        }
    }
    pub fn parse_fn(&mut self) -> Result<TTNode, String> {
        let t_name = self.next_token();
        if t_name.tt != TokenType::Literal {
            return Err("".to_string());
        }
        let return_type = self.expect_type()?;
        let s = self.expect_next_token_is(TokenType::OpenParen);
        if !s {
            return Err("".to_string());
        }
        let mut arg_types = Vec::new();
        let mut paren_count = 1;
        loop {
            let s = self.tokens.peek_next();
            if s.tt == TokenType::TokenNone {
                return Err("".to_string());
            }
            if s.tt == TokenType::OpenParen {
                paren_count += 1;
            } else if s.tt == TokenType::CloseParen {
                paren_count -= 1;
            }
            if paren_count == 0 {
                let _ = self.next_token();
                break;
            }
            arg_types.push(self.expect_type()?);
        }
        let f: Function = Function {
            arg_types: arg_types.clone(),
            ret_type: return_type.clone(),
        };
        println!("{:#?}", f);
        if self.tokens.peek_next().tt == TokenType::CloseParen {
            let _ = self.tokens.next_token();
            let out = TTNode::DeclareFunction {
                name: t_name.st.clone(),
                return_type: return_type.clone(),
                args: arg_types.clone(),
                list: None,
            };
            Ok(out)
        } else {
            let p = self.parse_statement()?;
            println!("365 peak:{:#?}", self.tokens.peek_next());
            let out = TTNode::DeclareFunction {
                name: t_name.st.clone(),
                return_type: return_type.clone(),
                args: arg_types.clone(),
                list: Some(Box::new(p)),
            };
            let _ = self.next_token();
            let _ = self.next_token();
            Ok(out)
        }
    }
    pub fn parse_global(&mut self) -> Result<TTNode, String> {
        let p = self.tokens.next_token();
        match p.tt {
            TokenType::DeFN => self.parse_fn(),
            TokenType::DeStruct => {
                todo!()
            }
            TokenType::Literal => {
                todo!()
            }
            _ => Err(format!("unexpected token type:{:#?}", p)),
        }
    }
    pub fn parse_fn_call(&mut self) -> Result<TTNode, String> {
        let p = self.next_token();
        let name = p.st.clone();
        println!("fn name:{:#?}", name.as_ref());
        let mut args = Vec::new();
        loop {
            if self.tokens.peek_next().tt == TokenType::CloseParen {
                let _ = self.next_token();
                break;
            } else {
                args.push(self.parse_expr()?);
            }
        }
        let out = TTNode::Call {
            function_name: name,
            return_type: Type::Void,
            args,
        };
        println!("fn call parse return: {:#?}", out);
        return Ok(out);
    }
    pub fn parse_assignment(&mut self) -> Result<TTNode, String> {
        let left = self.parse_expr()?;
        match left.clone() {
            TTNode::Deref { v: _ } => {}
            TTNode::VariableUsage {
                variable_name: _,
                variable_type: _,
                stack_offset: _,
            } => {}
            _ => {
                return Err(format!("{:#?} is not assignable", left));
            }
        }
        let right = self.parse_expr()?;
        let _ = self.next_token();
        Ok(TTNode::Assignment {
            target: Box::new(left.clone()),
            target_type: left.get_type(),
            source: Box::new(right),
        })
    }
    pub fn get_variable_info(&self, name: &Token) -> Result<(Type, usize), String> {
        let mut idx = self.scope_stack.len();
        while idx > 0 {
            idx -= 1;
            if self.scope_stack[idx]
                .variables
                .contains_key(name.st.as_ref())
            {
                return Ok(self.scope_stack[idx].variables[name.st.as_ref()].clone());
            }
        }
        return Err("could not find".to_string());
    }
    pub fn parse_statement(&mut self) -> Result<TTNode, String> {
        let s = self.tokens.peek_next();
        println!("parsing statement starting at:{:#?}", s);
        match s.tt {
            TokenType::Literal => {
                let out = self.parse_fn_call();
                println!("out:{:#?}", out);
                return out;
            }
            TokenType::Operator => match s.st.as_ref() {
                "=" => self.parse_assignment(),
                _ => Err(format!(
                    "error: operator {:#?} is not allowed as a statement",
                    s
                )),
            },
            TokenType::OpenParen => {
                println!("hit scope");
                let mut statements = Vec::new();
                let _ = self.next_token();
                loop {
                    let s = self.tokens.peek_next();
                    println!("running scope parse, next:{:#?}", s);
                    if s.tt == TokenType::CloseParen {
                        println!("broke");
                        break;
                    }
                    let _ = self.next_token();
                    let v = self.parse_statement()?;
                    statements.push(v);
                }
                println!("returned with {:#?} as next_token", self.tokens.peek_next());
                return Ok(TTNode::Scope {
                    expressions: statements,
                });
            }
            _ => return Err(format!("unsupported statement expression: {:#?}", s)),
        }
    }
    pub fn parse_expr(&mut self) -> Result<TTNode, String> {
        let p = self.next_token();
        println!("{:#?}", p);
        match p.tt {
            TokenType::OpenParen => self.parse_fn_call(),
            TokenType::String => Ok(TTNode::StringLiteral {
                value: p.st.clone(),
            }),
            TokenType::Number => {
                if p.st.contains('.') {
                    Ok(TTNode::FloatLiteral {
                        value: p.st.parse::<f64>().unwrap(),
                    })
                } else {
                    Ok(TTNode::IntegerLiteral {
                        value: p.st.parse::<i64>().unwrap(),
                    })
                }
            }
            TokenType::Literal => {
                let info = self.get_variable_info(&p)?;
                let v = TTNode::VariableUsage {
                    variable_name: p.st.clone(),
                    variable_type: info.0,
                    stack_offset: info.1,
                };
                Ok(v)
            }
            _ => Err(format!("expr:unexected token type:{:#?}", p)),
        }
    }
    pub fn parse(&mut self) -> Result<Vec<TTNode>, String> {
        let mut out = Vec::new();
        loop {
            let p = self.next_token();
            println!("{:#?}", p);
            if p.tt == TokenType::TokenNone {
                break;
            } else if p.tt == TokenType::OpenParen {
                out.push(self.parse_global()?);
            } else {
                return Err(format!("error unexpected token:{:#?}", p));
            }
        }
        Ok(out)
    }
    pub fn parse_tokens(tokens: crate::tokenizer::Tokenizer<'a>) -> Result<Vec<TTNode>, String> {
        Parser::new(tokens).parse()
    }
}
