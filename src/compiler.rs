use crate::parser::*;
use crate::types::Type;
use std::fs;
use std::io::Write;

pub fn compile_function_header(func:&Function, filename:&str)->Result<String,()>{
    let mut out= String::new();
    out += &name_mangle_type(&func.return_type);
    out += " ";
    out += &name_mangle_function(func, filename);
    out += "(";
    for i in 0..func.args.len(){
        out += &name_mangle_type(&func.args[i]);
        out += " ";
        out += &func.arg_names[i];
        if i <func.args.len()-1{
            out += ",";
        }
    }
    out += ");\n";
    return Ok(out);
}

pub fn compile_function_table_header(_name:&String, data:&FunctionTable,filename:&str)->Result<String, ()>{
    let mut out = String::new(); 
    for i in &data.functions{
        out += &compile_function_header(i,filename)?;
    }
    return Ok(out);
}

pub fn compile_type(_name:String, data:Type)->Result<String, ()>{
    match &data{
        Type::SliceT { ptr_type:_}=>{

        }
        Type::StructT { name:_, components:_ }=>{

        }
        _=>{
            return Ok(String::new());
        }
    }
    let mut out =String::from("");
    let name = format!("typedef struct {{\n");
    let end = format!("}}{};\n", name_mangle_type(&data));
    let mut vars = String::new();
    match &data{
        Type::SliceT { ptr_type }=>{
            vars = format!("    {} * start; {}* end;\n", name_mangle_type(&ptr_type), name_mangle_type(&ptr_type));
        }
        Type::StructT { name:_, components }=>{
            for i in components{
                vars += &format!("    {} {};\n",&name_mangle_type(&i.1), &i.0);
            }
        }
        _=>{
            unreachable!();
        }
    }
    out += &name;
    out += &vars;
    out += &end;
    Ok(out)
} 

pub fn compile_static(name:&String,vtype:&Type, index:usize)->Result<String,()>{
    let mut out = name_mangle_type(vtype)+" "+&name;
    out += match vtype{
        Type::BoolT=>{
            "=false"
        }
        Type::FloatT=>{
            "= 0.0"
        }
        Type::IntegerT=>{
            "= 0"
        }
        Type::PointerT { ptr_type:_ }=>{
            "= 0"
        }
        _ =>{
            "= {0}"
        }
    };
    out += ";\n";
    return Ok(out);
}
pub fn compiler_function(func:&Function)->Result<String,()>{
    todo!();
}

pub fn compile(prog:Program, base_filename:&str)->Result<(),()>{
    let filename = &base_filename[0..base_filename.len()-5];
    let mut out = String::new();
    let mut typedecs = "".to_owned();
    typedecs += "typedef struct {char * start; char * end;}String;\n";
    for i in &prog.types{
        typedecs += &compile_type(i.0.clone(), i.1.clone())?;
    };
    let mut func_decs = String::new();
    for i in &prog.functions{
        func_decs += &compile_function_table_header(i.0, i.1,filename)?;
    };
    let mut statics = String::new();
    for i in &prog.static_variables{
        statics += &compile_static(&i.0, &i.1.0, i.1.1)?;
    }
    let mut fout = fs::File::create("main.c").expect("testing expect");
    out += &typedecs;
    out += &func_decs;
    out += &statics;
    fout.write(out.as_bytes()).expect("tesing expect");
    return Ok(());
}