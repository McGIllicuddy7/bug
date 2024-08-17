use crate::parser::*;
use crate::types::Type;
use std::fs;
use std::env;
use std::io::Write;
pub fn name_mangle_type(var:&Type,filename:&str)->String{
    match var{
        Type::BoolT=>{
            return String::from("bool");
        }
        Type::FloatT=>{
            return String::from("double");
        }
        Type::IntegerT=>{
            return String::from("long");
        }
        Type::StringT=>{
            return String::from("String");
        }
        Type::VoidT=>{
            return String::from("void");
        }
        Type::PointerT { ptr_type }=>{
            return name_mangle_type(ptr_type,filename)+"*";
        }
        Type::ArrayT { size, array_type }=>{
            return name_mangle_type(array_type,filename)+&format!("[{size}]");
        }
        Type::SliceT { ptr_type }=>{
            return name_mangle_type(ptr_type,filename)+"Slice_t";
        }
        Type::StructT { name, components:_ }=>{
            return name.clone()+filename;
        }
    }
}
pub fn compile_type_header(name:&String, data:&Type,filename:&str)->Result<String, ()>{
    match data{
        Type::SliceT { ptr_type:_ }=>{
            Ok(format!("typedef struct {};", name_mangle_type(data, filename)))
        }
        Type::StructT { name:_, components:_ }=>{
            Ok(format!("typedef struct {};", name_mangle_type(data, filename)))
        }
        _=>{
            Ok("".to_owned())
        }
    }
}
pub fn compile_function_header(name:&String, data:&Type,filename:&str)->Result<String, ()>{
    match data{
        Type::SliceT { ptr_type:_ }=>{
            Ok(format!("typedef struct {};", name_mangle_type(data, filename)))
        }
        Type::StructT { name:_, components:_ }=>{
            Ok(format!("typedef struct {};", name_mangle_type(data, filename)))
        }
        _=>{
            Ok("".to_owned())
        }
    }
}
pub fn compile_type(name:String, data:Type)->Result<String, ()>{
    let mut out =String::from("");
    Ok(out)
} 
pub fn compile(prog:Program, base_filename:&str)->Result<(),()>{
    let filename = &base_filename[0..base_filename.len()-5];
    let mut out = String::new();
    let mut typedecs = "".to_owned();
    for i in &prog.types{
        typedecs += &compile_type_header(i.0, i.1,filename)?;
    };
    for i in &prog.types{
        typedecs += &compile_type(i.0.clone(), i.1.clone())?;
    };
    let mut func_decs = String::new();
    for i in &prog.functions{
        func_decs += &compile_function_header(i.0, i.1,filename)?;
    };
    let mut fout = fs::File::create("main.c").expect("testing expect");
    out += &typedecs;
    fout.write(out.as_bytes()).expect("tesing expect");
    return Ok(());
}