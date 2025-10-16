pub use crate::tokens;
use std::sync::Arc;
pub use tokens::TokenType;
#[derive(Debug, Clone)]
pub struct Expr {
    pub ops: Vec<Opr>,
}
#[derive(Debug, Clone)]
pub struct Var {
    pub t: tokens::Token,
    pub sv: i32,
}
#[derive(Debug, Clone)]
pub struct Eval {
    pub oprs: Vec<Opr>,
    pub ops: Vec<Op>,
    pub vars: Vec<Var>,
    pub sp: i32,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum OpType {
    o_ad,   //add
    o_sb,   //subtract
    o_ml,   //multiply
    o_dv,   //divide
    o_as,   //assign
    o_gt,   //goto
    o_cgt,  //conditional_goto
    o_dec,  //declare
    o_type, //type operator
    o_num,  //number operator
    o_flt,  //float operator
    o_str,  //string operator
    o_idnt, //indentifer operator
    o_fld,  //field access
    o_call, //call
    o_lable, //label
    o_return,//return
    o_clear,//clear stack/cleanup
    o_function,
    o_func_begin,//function prelude
    o_auto_dec, //auto_declare
}
#[derive(Debug, Clone)]
pub struct Opr {
    pub t: OpType,
    pub s: Arc<str>,
    pub v: i64,
    pub f: f64,
    pub token: tokens::Token,
}
impl Default for Opr {
    fn default() -> Self {
        Self::new()
    }
}

impl Opr {
    pub fn new() -> Self {
        Self {
            t: OpType::o_ad,
            s: "".into(),
            v: -1,
            f: -1.,
            token: tokens::Token {
                st: "".into(),
                file: "".into(),
                line: 0,
                tt: tokens::TokenType::TokenNone,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Er,
    OpenParen,
    CloseParen,
    Add,
    Sub,
    Mul,
    Div,
    Call,
    Dot,
    Colon,
    ColonEquals,
    Assign,
}
pub fn op_prior(o: Op) -> i32 {
    if o == Op::Mul {
        2
    } else if o == Op::Div {
        return 2;
    } else if o == Op::Sub {
        return 1;
    } else if o == Op::Add {
        return 1;
    } else if o == Op::Assign {
        return 0;
    } else if o == Op::Dot {
        return 3;
    } else if o == Op::Colon {
        return 4;
    } else if o == Op::ColonEquals {
        return 4;
    } else if o == Op::OpenParen {
        return -1;
    } else {
        todo!();
    }
}
pub fn eval_op(o: Op, ev: &mut Eval) {
    if !ev.vars.is_empty() {
        let v0 = ev.vars[ev.vars.len() - 1].clone();
        ev.vars.pop();
        write_var(ev, v0);
    }
    if !ev.vars.is_empty() {
        let v1 = ev.vars[ev.vars.len() - 1].clone();
        ev.vars.pop();
        write_var(ev, v1);
    }
    let mut op = Opr::new();
    if o == Op::Add {
        op.t = OpType::o_ad;
    } else if o == Op::Sub {
        op.t = OpType::o_sb;
    } else if o == Op::Div {
        op.t = OpType::o_dv;
    } else if o == Op::Mul {
        op.t = OpType::o_ml;
    } else if o == Op::Assign {
        op.t = OpType::o_as;
    } else if o == Op::Dot {
        op.t = OpType::o_fld;
    } else if o == Op::Colon {
        op.t = OpType::o_dec;
    } else if o == Op::ColonEquals {
        op.t = OpType::o_auto_dec;
    } else {
        todo!();
    }
    ev.oprs.push(op);
}
pub fn write_var(ev: &mut Eval, v: Var) {
    let mut o = Opr::new();
    o.token = v.t.clone();
    if v.t.tt == TokenType::TokenInt {
        o.t = OpType::o_num;
        o.v = v.t.st.parse::<i64>().unwrap();
    } else if v.t.tt == TokenType::TokenFloat {
        o.t = OpType::o_flt;
        o.f = v.t.st.parse::<f64>().unwrap();
    } else if v.t.tt == TokenType::TokenStr {
        o.t = OpType::o_str;
        o.s = v.t.st.clone();
    } else if v.t.tt == TokenType::TokenIdent {
        o.t = OpType::o_idnt;
        o.s = v.t.st.clone();
    } else {
        todo!();
    }
    ev.oprs.push(o);
}

pub fn get_next_outside_of_expr(tokens: &[tokens::Token], start: usize, t: TokenType) -> i64 {
    let mut paren_count: i64 = 0;
    let mut curly_count: i64 = 0;
    for i in start..tokens.len() {
        if paren_count == 0 && curly_count == 0 && tokens[i].tt == t {
            return i as i64;
        }
        if tokens[i].tt == TokenType::TokenOpenParen {
            paren_count += 1;
        } else if tokens[i].tt == TokenType::TokenCloseParen {
            paren_count -= 1;
        }
        if tokens[i].tt == TokenType::TokenOpenCurl {
            curly_count += 1;
        } else if tokens[i].tt == TokenType::TokenCloseCurl {
            curly_count -= 1;
        }
    }
    tokens[tokens.len() - 1].print();
    -1
}
pub fn parse_expression(tokens: &[tokens::Token]) -> Result<Expr, Box<dyn std::error::Error>> {
    let mut ev: Eval = Eval {
        ops: Vec::new(),
        oprs: Vec::new(),
        vars: Vec::new(),
        sp: 0,
    };
    let mut i: usize = 0;
    let mut last_was_v = false;
    while i < tokens.len() {
        if tokens[i].tt == TokenType::TokenInt
            || tokens[i].tt == TokenType::TokenFloat
            || tokens[i].tt == TokenType::TokenStr
            || tokens[i].tt == TokenType::TokenIdent
        {
            let v = Var {
                t: tokens[i].clone(),
                sv: -1,
            };
            ev.vars.push(v);
            last_was_v = true;
        } else if tokens[i].tt == TokenType::TokenOpenParen {
            if last_was_v {
                i += 1;
                let v = ev.vars[ev.vars.len() - 1].clone();
                ev.vars.pop();
                let end = get_next_outside_of_expr(tokens, i, TokenType::TokenCloseParen);
                if end == -1 {
                    return Err("expected close paren".into());
                }
                let mut arg_count: i64 = 0;
                while i < end as usize {
                    let mut e = get_next_outside_of_expr(tokens, i, TokenType::TokenComma);
                    if e == -1 || e > end {
                        e = end;
                    }
                    let ep = parse_expression(&tokens[i..e as usize])?;
                    arg_count += 1;
                    for j in 0..ep.ops.len() {
                        let k = ep.ops[j].clone();
                        ev.oprs.push(k);
                    }
                    i = e as usize+1;
                }
                if arg_count>0{
                     i -=1;
                }
                last_was_v = true;
                let mut op = Opr {
                    t: OpType::o_idnt,
                    s: v.t.st.clone(),
                    v: 0,
                    f: 0.0,
                    token: tokens[i].clone(),
                };
                ev.oprs.push(op);
                last_was_v = true;
                op = Opr {
                    t: OpType::o_call,
                    v: arg_count,
                    s: "".into(),
                    f: -1.0,
                    token: tokens[i].clone(),
                };
                ev.oprs.push(op);
            } else {
                last_was_v = false;
                ev.ops.push(Op::OpenParen);
            }
        } else if tokens[i].tt == TokenType::TokenCloseParen {
            while ev.ops[ev.ops.len() - 1] != Op::OpenParen {
                let o = ev.ops[ev.ops.len() - 1];
                ev.ops.pop();
                eval_op(o, &mut ev);
            }
            ev.ops.pop();
            last_was_v = true;
        } else if tokens[i].tt == TokenType::TokenDot
            || tokens[i].tt == TokenType::TokenOperator
            || tokens[i].tt == TokenType::TokenColon
        {
            last_was_v = false;
            let mut o: Op;
            if tokens[i].tt == TokenType::TokenDot {
                o = Op::Dot;
            } else if tokens[i].equals("+") {
                o = Op::Add;
            } else if tokens[i].equals("-") {
                o = Op::Sub;
            } else if tokens[i].equals("*") {
                o = Op::Mul;
            } else if tokens[i].equals("/") {
                o = Op::Div;
            } else if tokens[i].equals(":") {
                o = Op::Colon;
            } else if tokens[i].equals(":=") {
                o = Op::ColonEquals;
            } else if tokens[i].equals("="){
                o = Op::Assign;
            }else {
                tokens[i].print();
                return Err("invalid expression".into());
            }
            while !ev.ops.is_empty() {
                if op_prior(ev.ops[ev.ops.len() - 1]) < op_prior(o) {
                    break;
                }
                let t = ev.ops[ev.ops.len() - 1];
                ev.ops.pop();
                if t == Op::OpenParen {
                    break;
                }
                eval_op(t, &mut ev);
            }
            ev.ops.push(o);
            last_was_v = false;
        } else {
            tokens[i].print();
            return Err("unsupported_token".into());
        }
        i += 1;
    }
    if ev.ops.is_empty() && ev.vars.len() == 1 {
        let l = ev.vars[0].clone();
        write_var(&mut ev, l);
        ev.vars.pop();
    }
    while !ev.ops.is_empty() {
        //std.debug.print("{any}", .{ev});
        let o = ev.ops[ev.ops.len() - 1];
        if o == Op::OpenParen || o == Op::CloseParen {
            ev.ops.pop();
            continue;
        }
        ev.ops.pop();
        eval_op(o, &mut ev);
    }
    Ok(Expr { ops: ev.oprs })
}
