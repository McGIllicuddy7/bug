use crate::parser::*;
use crate::types::Type;
use std::fmt::format;
use std::fs;
use std::io::Write;
pub fn compile_function_header(func:&Function, filename:&str)->Result<String,String>{
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

pub fn compile_function_table_header(_name:&String, data:&FunctionTable,filename:&str)->Result<String, String>{
    let mut out = String::new(); 
    for i in &data.functions{
        out += &compile_function_header(i,filename)?;
    }
    return Ok(out);
}

pub fn compile_type(_name:String, data:Type)->Result<String, String>{
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

pub fn compile_static(name:&String,vtype:&Type, _index:usize)->Result<String,String>{
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
pub fn compile_expression(tmp_counter:&mut usize,expr:&mut AstNode,expect_return:bool)->Result<String,String>{
    match expr{
        AstNode::VoidLiteral=>{
            return Err("found void literal".to_owned());
        }
        AstNode::BoolLiteral { value }=>{
            if *value{
                return Ok("true".to_owned());
            } else{
                return Ok("false".to_owned());
            }
        }
        AstNode::StringLiteral {value,}=>{
            return Ok(value.clone());
        }
        AstNode::IntLiteral { value }=>{
            return Ok(format!("{value}"));
        }
        AstNode::FloatLiteral { value }=>{
            return Ok(format!("{value}"));
        }
        AstNode::StructLiteral { nodes }=>{
            let mut out = String::from("{");
            for i in nodes{
                out += &compile_expression(tmp_counter, i, expect_return)?;
                out += ",";
            }
            out += "}";
            return Ok(out);
        }
        AstNode::ArrayLiteral { nodes }=>{
            let mut out = String::from("{");
            for i in nodes{
                out += &compile_expression(tmp_counter, i, expect_return)?;
                out += ",";
            }
            out += "}";
            return Ok(out);
        }
        AstNode::VariableUse { name, index:_, vtype:_, is_arg:_, data:_ }=>{
            return Ok(name.clone());
        }
        AstNode::FunctionCall { function_name, args, data:_ }=>{
            let mut base = function_name.clone()+"(";
            for i in args{
                base += &compile_expression(tmp_counter, expr, true)?;
            }
            base += ");";
            if expect_return{

            } else{

            }
        }
        _=>{
            unreachable!();
        }
    }
    todo!()
}
pub fn compile_function(func:&mut Function, filename:&str)->Result<String,String>{
    let mut out = String::new();
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
    out += "){\n";
    let mut temp_counter = 0;
    for i in &mut func.program{
        out += &compile_expression(&mut temp_counter,i,false)?;
    }
    out += "}\n";
    return Ok(out);
}

pub fn compile(prog:Program, base_filename:&str)->Result<(),String>{
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
    let mut functions = String::new();
    for i in &prog.functions{
        for func in &i.1.functions{
            let mut f =  func.clone();
            functions+= &compile_function(&mut f,filename)?;
        }
    }
    let mut fout = fs::File::create("main.c").expect("testing expect");
    out += &typedecs;
    out += &func_decs;
    out += &statics;
    out += &functions;
    fout.write(out.as_bytes()).expect("tesing expect");
    return Ok(());
}