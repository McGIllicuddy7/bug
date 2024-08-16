use crate::parser::*;
use std::fs;
use std::env;
pub fn compile_type(name:String, data:Type)->Result<String, ()>{
    return Err(());
}
pub fn compile(prog:Program)->Result<(),()>{
    let mut out = String::from("");
    for i in &prog.types{
        out += &compile_type(i.0.clone(), i.1.clone())?;
    };
    return Ok(());
}