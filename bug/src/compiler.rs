use crate::bug::{Context, Function, Statement};
pub use crate::bug::Type;
use crate::parser::OpType;
pub use crate::parser::{Opr, Op};
use std::{error::Error, sync::Arc};
#[derive(Clone, Debug)]
pub struct Var{
    pub name:String, 
    pub vt:Type,
}
#[derive(Clone, Debug)]
pub struct IrFunc{
    pub rv:Type, 
    pub args:Vec<Var>,
    pub ins:Vec<Opr>, 
}
#[derive(Clone, Debug)]
pub struct Ir{
    pub functions:Vec<IrFunc>, 
    pub externs:Vec<IrFunc>,
    pub types:Vec<Type>,
}

#[derive(Clone,Debug)]
pub struct Compiler{
    pub ir:Ir,
    pub lable_count:usize,
    pub scopes:Vec<Vec<Var>>,
}
impl Compiler{
    pub fn dec_var(&mut self,v:Var){
        let l = self.scopes.len();
        self.scopes[l-1].push(v);
    }
    pub fn check_var(&self, name:&str)->Option<Var>{
        for i in &self.scopes{
            for j in i{
                if j.name == name{
                    return Some(j.clone());
                }
            }
        }
        None
    }
    pub fn push_scope(&mut self){
        self.scopes.push(Vec::new());
    }
    pub fn pop_scope(&mut self){
        self.scopes.pop();
    }
}
pub fn compile_statement( cmp:&mut Compiler, func:&mut IrFunc, st:&Statement)->Result<(), Box<dyn Error>>{
    match st{
        Statement::While { cond, list } =>{
                todo!()
        }
        Statement::If { cond, list, else_list } => {
            for i in &cond.ops{
                func.ins.push(i.clone());
            }
            let mut elb = cmp.lable_count;
            cmp.lable_count +=1;
            let mut op = Opr::new();
            op.v = elb.into();
            let mut done = cmp.lable_count;
            cmp.lable_count += 1;
            
        }
        Statement::Declare { v, list } => {
            if cmp.check_var(&v.name).is_some(){
                todo!();
            }
            cmp.dec_var(Var{name:v.name.to_string(), vt:v.vtype.clone()});
            let mut op = Opr::new();
            op.t = OpType::o_dec;
            op.s = v.name.clone();
            func.ins.push(op);
            for i in &list.ops{
                func.ins.push(i.clone());
            }
        }
        Statement::Basic { list } => {
            for i in &list.ops{
                func.ins.push(i.clone());
            }
        }
        Statement::Return { list } => {
            for i in &list.ops{
                func.ins.push(i.clone());
            }
            let mut op = Opr::new();
            op.t = OpType::o_return;
            func.ins.push(op);
        }
    }
    Ok(())
}
pub fn compile_function(cmp:&mut Compiler, func:&Function)->Result<(), Box<dyn Error>>{
    let mut funct = IrFunc{rv:func.return_type.clone(), args:func.args.iter().map(|i|{Var{name:i.name.to_string(), vt:i.vtype.clone()}}).collect(), ins:Vec::new()};
    for  i in &func.list{
        compile_statement(cmp, &mut funct,i)?;
    }
    Ok(())
}
pub fn compile_to_ir(context:&Context)->Result<Ir, Box<dyn Error>>{
    let mut out = Ir{functions:Vec::new(), types:Vec::new(), externs:Vec::new()};
    out.types = context.types.clone();
    for i in &context.functions{
        let funct = IrFunc{rv:i.return_type.clone(), args:i.args.iter().map(|i|{Var{name:i.name.to_string(), vt:i.vtype.clone()}}).collect(), ins:Vec::new()};
        out.externs.push(funct);
    }
    let mut cmp = Compiler{ir:out, lable_count:0};
    for i in &context.functions{
        compile_function(&mut cmp, i)?;
    }
    Ok(cmp.ir)
}