pub use crate::bug::Type;
use crate::bug::{Context, Function, Statement};
use crate::parser::OpType;
pub use crate::parser::{Op, Opr};
use core::error;
use std::{error::Error, sync::Arc};
#[derive(Clone, Debug)]
pub struct Var {
    pub name: String,
    pub vt: Type,
}
#[derive(Clone, Debug)]
pub struct IrFunc {
    pub rv: Type,
    pub args: Vec<Var>,
    pub ins: Vec<Opr>,
    pub name:String,
}
impl IrFunc{
    pub fn type_of(&self)->Type{
        Type::TFunction { return_type: Box::new(self.rv.clone()), args: self.args.iter().map(|i| i.vt.clone()).collect::<Vec<Type>>() }
    }
}
#[derive(Clone, Debug)]
pub struct Ir {
    pub functions: Vec<IrFunc>,
    pub externs: Vec<IrFunc>,
    pub types: Vec<Type>,
}

#[derive(Clone, Debug)]
pub struct Compiler {
    pub ir: Ir,
    pub label_count: usize,
    pub scopes: Vec<Vec<Var>>,
}
impl Compiler {
    pub fn dec_var(&mut self, v: Var) {
        let l = self.scopes.len();
        self.scopes[l - 1].push(v);
    }
    pub fn check_var(&self, name: &str) -> Option<Var> {
        for i in &self.scopes {
            for j in i {
                if j.name == name {
                    return Some(j.clone());
                }
            }
        }
        None
    }
    pub fn push_scope(&mut self) {
        self.scopes.push(Vec::new());
    }
    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }
}
pub fn compile_statement(
    cmp: &mut Compiler,
    func: &mut IrFunc,
    st: &Statement,
) -> Result<(), Box<dyn Error>> {
    match st {
        Statement::While { cond, list } => {
            let start = cmp.label_count;
            cmp.label_count+=1;
            let blck = cmp.label_count;
            cmp.label_count+=1;
            let done = cmp.label_count;
            cmp.label_count += 1;
            let mut op = Opr::new();
            op.t = OpType::o_lable;
            op.v = start as i64;
            func.ins.push(op.clone());
            for i in &cond.ops{
                func.ins.push(i.clone());
            }
            op.t = OpType::o_cgt;
            op.v = blck as i64;
            func.ins.push(op.clone());
            op.t = OpType::o_gt;
            op.v = done as i64;
            func.ins.push(op.clone());
            op.t = OpType::o_lable;
            op.v = blck as i64;
            func.ins.push(op.clone());
            for i in list{
                compile_statement(cmp, func, i)?;
            }
            op.t = OpType::o_gt;
            op.v = start as i64;
            func.ins.push(op.clone());
            op.t = OpType::o_lable;
            op.v = done as i64;
            func.ins.push(op.clone());
        }
        Statement::If {
            cond,
            list,
            else_list,
        } => {
            for i in &cond.ops {
                func.ins.push(i.clone());
            }
            let ifb = cmp.label_count;
            cmp.label_count+=1;
            let elfb = cmp.label_count;
            cmp.label_count+=1;
            let donb = cmp.label_count;
            cmp.label_count += 1;
            let mut op = Opr::new();
            op.t = OpType::o_cgt;
            op.v = ifb as i64;
            func.ins.push(op.clone());
            let mut op = Opr::new();
            op.t = OpType::o_cgt;
            op.v = ifb as i64;
            func.ins.push(op.clone());
            op.t= OpType::o_gt;
            op.v = elfb as i64;
            func.ins.push(op.clone());
            for i in list{
                compile_statement(cmp, func, i)?
            }
            op.v = donb as i64;
            func.ins.push(op.clone());
            op.t = OpType::o_lable;
            op.v = elfb as i64;
            func.ins.push(op.clone());
             for i in else_list{
                compile_statement(cmp, func, i)?
            }
            op.v = donb as i64;
            func.ins.push(op);
        }
        Statement::Declare { v, list } => {
            if cmp.check_var(&v.name).is_some() {
                todo!();
            }
            cmp.dec_var(Var {
                name: v.name.to_string(),
                vt: v.vtype.clone(),
            });
            let mut op = Opr::new();
            op.t = OpType::o_dec;
            op.s = v.name.clone();
            op.vt = v.vtype.clone();
            func.ins.push(op);
            for i in &list.ops {
               func.ins.push(i.clone());
            }
        }
        Statement::Basic { list } => {
            for i in &list.ops {
                func.ins.push(i.clone());
            }
        }
        Statement::Return { list } => {
            if !list.ops.is_empty() && func.rv == Type::TVoid{
                return Err("attempted to return from a void function".into());
            }
            for i in &list.ops {
                func.ins.push(i.clone());
            }
            let mut op = Opr::new();
            op.t = OpType::o_return;
            func.ins.push(op);
        }
    }
    /* 
    let mut op = Opr::new();
    op.t = OpType::o_clear;
    func.ins.push(op);*/
    Ok(())
}
pub fn validate_function(cmp:&mut Compiler, func:&mut IrFunc)->Result<(), Box<dyn Error>>{
    let mut vars:Vec<Vec<Var>> = Vec::new();
    vars.push(Vec::new());
    for i in &func.args{
        vars.last_mut().unwrap().push(i.clone());
    }
    let mut stack:Vec<Type> = Vec::new();
    let mut line = 0;
    let mut last_ident = None;
    for i in &mut func.ins{
        if i.token.line>line{
            line = i.token.line;
        }
        match i.t{
            OpType::o_ad => {
                last_ident = None;
                if stack.len()<2 {
                    return Err(format!("not enough stack operands:line{}", line).into());
                }
                let p = stack.pop().unwrap();
                let p2 = stack.pop().unwrap();
                if p != p2{
                    return Err(format!("mismatched types:{:#?},{:#?}, line:{}", p, p2, line).into())
                }
                stack.push(p);
            }
            OpType::o_sb => {
                last_ident = None;
                if stack.len()<2 {
                    return Err(format!("not enough stack operands:line{}", line).into());
                }
                let p = stack.pop().unwrap();
                let p2 = stack.pop().unwrap();
                if p != p2{
                    return Err(format!("mismatched types:{:#?},{:#?}, line:{}", p, p2, line).into())
                }
                stack.push(p);
            }
            OpType::o_ml => {
                last_ident = None;
                if stack.len()<2 {
                    return Err(format!("not enough stack operands:line{}", line).into());
                }
                let p = stack.pop().unwrap();
                let p2 = stack.pop().unwrap();
                if p != p2{
                    return Err(format!("mismatched types:{:#?},{:#?}, line:{}", p, p2, line).into())
                }
                stack.push(p);
            },
            OpType::o_dv => {
                last_ident = None;
                if stack.len()<2 {
                    return Err(format!("not enough stack operands:line{}", line).into());
                }
                let p = stack.pop().unwrap();
                let p2 = stack.pop().unwrap();
                if p != p2{
                    return Err(format!("mismatched types:{:#?},{:#?}, line:{}", p, p2, line).into())
                }
                stack.push(p);
            }
            OpType::o_as => {
                last_ident = None;
                if stack.len()<2 {
                    return Err(format!("not enough stack operands:line{}", line).into());
                }
                let p = stack.pop().unwrap();
                let p2 = stack.pop().unwrap();
                if p != p2{
                    return Err(format!("mismatched types:{:#?},{:#?}, line:{}", p, p2, line).into())
                }
                stack.push(p);
            }
            OpType::o_gr => {
                last_ident = None;
                if stack.len()<2 {
                    return Err(format!("not enough stack operands:line{}", line).into());
                }
                let p = stack.pop().unwrap();
                let p2 = stack.pop().unwrap();
                if p != p2{
                    return Err(format!("mismatched types:{:#?},{:#?}, line:{}", p, p2, line).into())
                }
                stack.push(Type::TInt);
            },
            OpType::o_ls => {
                last_ident = None;
                if stack.len()<2 {
                    return Err(format!("not enough stack operands:line{}", line).into());
                }
                let p = stack.pop().unwrap();
                let p2 = stack.pop().unwrap();
                if p != p2{
                    return Err(format!("mismatched types:{:#?},{:#?}, line:{}", p, p2, line).into())
                }
                stack.push(Type::TInt);
            },
            OpType::o_gt => {
                last_ident = None;
                continue;
            }
            OpType::o_cgt => {
                last_ident = None;
                if stack.len()<1 {
                    return Err(format!("not enough stack operands:line{}", line).into());
                } 
                let t = stack.pop().unwrap();
                if t != Type::TInt{
                    return Err(format!("jump on non integral type,{}", line).into());
                }
            }
            OpType::o_dec => {
                last_ident = None; 
                let v = Var{name:i.s.to_string(), vt:i.vt.clone()};
                vars.last_mut().unwrap().push(v);
            }
            OpType::o_type => {
                todo!();
            }
            OpType::o_num => {
                last_ident = Some(i.token.st.clone());
                stack.push(Type::TInt);
            }
            OpType::o_flt => {
                last_ident = Some(i.token.st.clone());
                stack.push(Type::TFloat);
            }
            OpType::o_str => {
                last_ident = Some(i.token.st.clone());
                stack.push(Type::TStr);
            }
            OpType::o_idnt => {
               last_ident = Some(i.token.st.clone());
                let s = i.s.as_ref();
                let mut hit = false;
                for j in &vars{
                    for k in j{
                        if k.name == s{
                            hit = true;
                            stack.push(k.vt.clone());
                            break;
                        }
                    }
                }
                if !hit{
                    for k in &cmp.ir.externs{
                        i.t = OpType::o_function;
                        if k.name == s{
                            hit = true;
                            stack.push(k.type_of());
                            break;
                        }
                    }
                }
                if !hit{
                    return Err(format!("undeclared indentifier:{}, line:{}", s, line).into());
                }
            }
            OpType::o_fld => {
                if stack.len()<2 {
                    return Err(format!("not enough stack operands:line{}", line).into());
                }
                let fld = stack.pop().unwrap();
                let v = stack.pop().unwrap();
                if let Some(n) = last_ident{
                    match &v{
                        Type::TStruct { name:_, fields } => {
                            let mut hit = false;
                            for j in fields.as_ref(){
                                if j.name.as_ref() == n.as_ref(){
                                    stack.push(j.vtype.clone());
                                    hit = true;
                                }
                            }
                            if !hit{
                                return Err(format!("type :{:#?} has no field{} line:{}", v, n, line).into());
                            }
                        }
                        _=>{
                            return Err(format!("type :{:#?} has no field:{} line:{}", v, n, line).into());
                        }
                    }
                }else{
                    todo!()
                }
                last_ident = None;
            }
            OpType::o_call => {
                last_ident = None;
                let count = i.v as usize;
                if stack.len()<count+1{
                    return Err(format!("not enough stack operands:line{}", line).into()); 
                }
                let to_call = stack.pop().unwrap();
                let ag;
                let rv ;
                match to_call{
                    Type::TFunction { return_type, args } =>{
                        ag = args;
                        rv = *return_type;
                    }
                    _=>{
                        return Err(format!("{:#?} is not a function, line:{}", to_call,line ).into());
                    }
                }
                for i in 0..count{
                    let tmp = stack.pop().unwrap();
                    if tmp != ag[count-i-1]{
                        return Err(format!("expected type:{:#?}, found type:{:#?}, line:{}", tmp, ag[count-i-1], line).into());
                    }
                }
                if rv != Type::TVoid{
                    stack.push(rv);
                }
            }
            OpType::o_lable =>{
                last_ident = None;
                continue;
            }
            OpType::o_return => {
                last_ident = None;
                if func.rv == Type::TVoid {
                    continue;
                }
                if stack.len()<1 {
                    return Err(format!("not enough stack operands:line{}", line).into());
                }
                let t = stack.pop().unwrap();
                if t != func.rv{
                    return Err(format!("returned type:{:#?}, expected:{:#?}, line:{}", t, func.rv,line ).into()); 
                }
            }   
            OpType::o_clear =>{
                last_ident = None;
                stack.clear();
            }
            OpType::o_function =>{
                last_ident = None;
                let t = i.s.as_ref();
                let mut hit = false;
                for i in &cmp.ir.externs{
                   if i.name == t{
                        stack.push(i.type_of());
                   }
                   hit = true;
                   break;
                }
                if !hit{
                    return Err(format!("undeclared function{t}, line:{line}").into());
                }
            }
            OpType::o_func_begin => {
                last_ident = None;
                continue;
            }
            OpType::o_auto_dec => {
                last_ident = None;
                continue;
            }
            OpType::o_begin_scope=>{
                last_ident = None;
                vars.push(Vec::new());
            }
            OpType::o_end_scope=>{
                 last_ident = None;
                _ = vars.pop();
            }
        }

    }
    Ok(())
}
pub fn compile_function(cmp: &mut Compiler, func: &Function) -> Result<(), Box<dyn Error>> {
    cmp.push_scope();
    let mut funct = IrFunc {
        rv: func.return_type.clone(),
        args: func
            .args
            .iter()
            .map(|i| Var {
                name: i.name.to_string(),
                vt: i.vtype.clone(),
            })
            .collect(),
        ins: Vec::new(),
        name:func.name.clone()
    };
    for i in &func.list {
        compile_statement(cmp, &mut funct, i)?;
    }
    validate_function(cmp,&mut funct)?;
    cmp.ir.functions.push(funct);
    cmp.pop_scope();
    Ok(())
}
pub fn compile_to_ir(context: &Context) -> Result<Ir, Box<dyn Error>> {
    let mut out = Ir {
        functions: Vec::new(),
        types: Vec::new(),
        externs: Vec::new(),
    };
    for i in &context.externs{
        out.externs.push(IrFunc { rv: i.return_type.clone(), args: i.args.iter().map(|i| Var{name:i.name.to_string(), vt: i.vtype.clone()}).collect(), ins: Vec::new(), name: i.name.clone() });
    }
    out.types = context.types.clone();
    for i in &context.functions {
        let funct = IrFunc {
            rv: i.return_type.clone(),
            args: i
                .args
                .iter()
                .map(|i| Var {
                    name: i.name.to_string(),
                    vt: i.vtype.clone(),
                })
                .collect(),
            ins: Vec::new(),
            name: i.name.clone()
        };
        out.externs.push(funct);
    }
    let mut cmp = Compiler {
        ir: out,
        label_count: 0,
        scopes:Vec::new()
    };
    for i in &context.functions {
        compile_function(&mut cmp, i)?;
    }
    Ok(cmp.ir)
}
