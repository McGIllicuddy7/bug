use std::{collections::{HashMap, HashSet}, rc::Rc};
use std::fs;
use std::io::Write;
mod ir_to_as;
use crate::{ir::{compile_function_to_ir, compile_ir_instr_to_c}, name_mangle_function, name_mangle_type, name_mangle_type_for_names, Function, FunctionTable, Program, Type};
pub fn compile_function_header(func:&Function, filename:&str)->Result<String,String>{
    if func.forward_declared {
       return Ok(format!("extern {}\n",name_mangle_function(func, filename)));
    }
    return Ok(format!("global {}\n",name_mangle_function(func, filename)));
}

pub fn compile_function_table_header(_name:&String, data:&FunctionTable,filename:&str)->Result<String, String>{
    let mut out = String::new(); 
    for i in &data.functions{
        out += &compile_function_header(i,filename)?;
    }
    return Ok(out);
} 

pub fn compile_static(_name:&String,_vtype:&Type, _index:usize)->Result<String,String>{
    todo!();
}
pub fn compile_function(func:&mut Function, filename:&str, functions:&HashMap<String,FunctionTable>, types:&HashMap<String, Type>,used_types:&mut HashSet<Type>, statics_count:&mut usize, static_section:&mut String)->Result<String,String>{
    let mut out = String::new();
    out += &name_mangle_function(func, filename);
    out += ":\n";
    out += "    push rbp\n";
    out += "    mov rbp,rsp\n";
    out += "    push rbx\n";
    out += "    push rcx\n";
    out += "    push rdx\n";
    let ir = compile_function_to_ir(func, functions, types);
    println!("ir representation:{:#?}", ir);
    let mut depth = 1;
    for i in &ir{
        let tmp = ir_to_as::compile_ir_instr_to_x86(i, &mut depth, used_types,statics_count, static_section);
        out += &tmp;
        out += "\n";
    }
    out += "    pop rdx\n";
    out += "    pop rcx\n";
    out += "    pop rbx\n";
    out += "    mov rsp, rbp\n";
    out += "    pop rbp\n";
    out += "\n";
    out += "    ret\n";
    return Ok(out);
}
pub fn gc_function_name(t:&Type)->String{
    return "gc_".to_owned()+&name_mangle_type_for_names(t);
}
fn compile_gc_functions(types:HashSet<Type>)->String{
    let mut out = String::new();
    for i in &types{
        match i{
            Type::StringT=>{
                continue;
            }
            _=>{
                out += "void ";
                out += &(gc_function_name(i)+"(void*);\n");
            }
        }
  
    }
    for i in &types{
        match i{
            Type::StringT{}=>{
                continue;
            }
            Type::IntegerT{}=>{
                continue;
            }
            Type::BoolT=>{
                continue;
            }
            Type::CharT=>{
                continue;
            }
            Type::FloatT=>{
                continue;
            }
            _=>{}
        }
        if i.is_partially_defined(){
            continue;
        }
        out += "void ";
        out += &(gc_function_name(i)+"(void* ptr){\n");
        out += &("  ".to_owned()+&(name_mangle_type(i)+"* var = ptr;\n"));
        match i{
            Type::PointerT { ptr_type }=>{
                out += "   if(!(*var)){return;}\n";
                out += "   bool hit =gc_any_ptr(*var);\n   if(hit){return;}\n";
                out += "    ";

                out += &(gc_function_name(ptr_type)+"(*var);\n");
            }
            Type::SliceT { ptr_type}=>{
                out += "   bool hit = gc_any_ptr(var->start);\n";
                out += "   if(hit){return;}\n";
                out += "    for(int i =0; i<var->len; i++){";
                out += "    "; 
                out += &(gc_function_name(ptr_type)+"(&var->start[i]);}\n");
            }
            Type::StructT { name:_, components }=>{
                for i in components{
                    out += "    ";
                    out += &gc_function_name(&i.1);
                    out += "(";
                    out += "&var->";
                    out += &i.0;
                    out += ");\n";

                }
            }
            _=>{
                out += "return;\n";
            }
        }
        out += "}\n";
    }
    return out;
}
fn get_all_types_contained(t:&Type, types:&HashMap<String, Type>)->Vec<Type>{
    let mut out = vec![];
    match t{
        Type::ArrayT { size, array_type }=>{
            out.push(get_all_types_contained(array_type,types));
            match array_type.as_ref(){
                Type::PartiallyDefined { name }=>{
                    out.push(vec![Type::PointerT { ptr_type: Rc::new(types.get(name.as_ref()).expect("name exists").clone())}]);
                }
                _=>{
                    out.push(vec![Type::ArrayT { size:*size,array_type:array_type.clone() }]);
                }
            }
            return out.into_iter().flatten().collect();
        }
        Type::PointerT { ptr_type }=>{
            out.push(get_all_types_contained(ptr_type,types));
            match ptr_type.as_ref(){
                Type::PartiallyDefined { name }=>{
                    out.push(vec![Type::PointerT { ptr_type: Rc::new(types.get(name.as_ref()).expect("name exists").clone())}]);
                }
                _=>{
                    out.push(vec![Type::PointerT { ptr_type:ptr_type.clone() }]);
                }
            }
            return out.into_iter().flatten().collect();
        }
        Type::SliceT { ptr_type }=>{
            out.push(get_all_types_contained(ptr_type,types));
            match ptr_type.as_ref(){
                Type::PartiallyDefined { name }=>{
                    out.push(vec![Type::SliceT { ptr_type: Rc::new(types.get(name.as_ref()).expect("name exists").clone())}]);
                }
                _=>{
                    out.push(vec![Type::SliceT { ptr_type:ptr_type.clone() }]);
                }
            }
            return out.into_iter().flatten().collect();
        }
        Type::StructT { name:_, components }=>{
            for i in components{
                out.push(get_all_types_contained(&i.1, types));
            }
        }
        Type::PartiallyDefined { name}=>{
            return vec![types.get(name.as_ref()).expect("type must exist").clone()];
        }
        _=>{
            
        }
    }
    out.push(vec![t.clone()]);
    return out.into_iter().flatten().collect();
}
fn recurse_used_types(types:&HashSet<Type>, type_table:&HashMap<String,Type>)->HashSet<Type>{
    let mut out = HashSet::new();
    for i in types{
        let j = get_all_types_contained(i, type_table);
        for k in j{
            match k{
                Type::PartiallyDefined { name:_}=>{
                    continue;
                }
                _=>{

                }
            }
            out.insert(k);
        }
    }
    return out;
}
pub fn compile_to_asm_x86(prog:Program,base_filename:&String)->Result<(),String>{
    println!("compiling file: {}", base_filename);
    let fname = "output/".to_owned()+&base_filename[0..base_filename.len()-4];
    let filename = &fname;
    let mut out = String::new();
    let mut typedecs = "".to_owned();
    let mut used_types = HashSet::new();
    let mut func_decs = String::new();
    for i in &prog.functions{
        func_decs += &compile_function_table_header(i.0, i.1,filename)?;
    };
    let mut statics = "section .text\n".to_owned();
    let mut functions = String::new();
    let mut statics_count = 0;
    for i in &prog.functions{
        for func in &i.1.functions{
            if func.forward_declared{
                continue;
            }
            let mut f =  func.clone();
            functions+= &compile_function(&mut f,filename, &prog.functions, &prog.types, &mut used_types,&mut statics_count,&mut statics)?;
        }
    }
    let out_file_name = filename.to_owned()+".s";
    let mut fout = fs::File::create(&out_file_name).expect("testing expect");
    used_types = recurse_used_types(&used_types, &prog.types);
    typedecs += &compile_gc_functions(used_types);
    out += &typedecs;
    out += &func_decs;
    out += &statics;
    out += &functions;
    fout.write(out.as_bytes()).expect("testing expect");
    drop(fout);
    let mut cmd=std::process::Command::new("nasm");
    cmd.arg(&out_file_name);
    println!("{}", std::env::consts::OS);
    if std::env::consts::OS == "linux"{ 
        let _ = cmd.arg("-f elf64");
    } else{
        let _ = cmd.arg("-f macho64").output();
    }

    return Ok(());
}