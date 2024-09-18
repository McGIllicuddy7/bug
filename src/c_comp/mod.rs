use crate::frontend::*;
use crate::ir::{compile_function_to_ir, compile_ir_instr_to_c};
use crate::types::Type;
use std::collections::HashSet;
use std::fs;
use std::io::Write;
use std::rc::Rc;
#[allow(unused)]
pub fn compile_function_header(func: &Function, filename: &str) -> Result<String, String> {
    let mut out = String::new();
    out += &name_mangle_type(&func.return_type);
    out += " ";
    out += &name_mangle_function(func, filename);
    out += "(";
    for i in 0..func.args.len() {
        out += &name_mangle_type(&func.args[i]);
        out += " ";
        out += &func.arg_names[i];
        if i < func.args.len() - 1 {
            out += ",";
        }
    }
    out += ");\n";
    return Ok(out);
}

#[allow(unused)]
pub fn compile_function_table_header(
    _name: &String,
    data: &FunctionTable,
    filename: &str,
) -> Result<String, String> {
    let mut out = String::new();
    for i in &data.functions {
        out += &compile_function_header(i, filename)?;
    }
    return Ok(out);
}

#[allow(unused)]
pub fn compile_type(_aname: String, data: Type) -> Result<String, String> {
    match &data {
        Type::SliceT { ptr_type: _ } => {}
        Type::StructT {
            name: _,
            components: _,
        } => {}
        _ => {
            return Ok(String::new());
        }
    }
    let mut out = String::from("");
    let name = format!("typedef struct {}{{\n", name_mangle_type(&data));
    let end = format!("}}{};\n", name_mangle_type(&data));
    let mut vars = String::new();
    match &data {
        Type::SliceT { ptr_type } => {
            vars = format!(
                "    {} * start; size_t len;\n",
                name_mangle_type_for_struct(&ptr_type)
            );
        }
        Type::StructT {
            name: _,
            components,
        } => {
            for i in components {
                vars += &format!("    {} {};\n", &name_mangle_type_for_struct(&i.1), &i.0);
            }
        }
        _ => {
            unreachable!();
        }
    }
    out += &name;
    out += &vars;
    out += &end;
    Ok(out)
}

#[allow(unused)]
pub fn compile_static(name: &String, vtype: &Type, _index: usize) -> Result<String, String> {
    let mut out = name_mangle_type(vtype) + " " + &name;
    out += match vtype {
        Type::BoolT => "=false",
        Type::FloatT => "= 0.0",
        Type::IntegerT => "= 0",
        Type::PointerT { ptr_type: _ } => "= 0",
        _ => "= {0}",
    };
    out += ";\n";
    return Ok(out);
}

#[allow(unused)]
pub fn compile_function(
    func: &mut Function,
    filename: &str,
    functions: &HashMap<String, FunctionTable>,
    types: &HashMap<String, Type>,
    used_types: &mut HashSet<Type>,
) -> Result<String, String> {
    let mut out = String::new();
    out += &name_mangle_type(&func.return_type);
    out += " ";
    out += &name_mangle_function(func, filename);
    out += "(";
    for i in 0..func.args.len() {
        used_types.insert(func.args[i].clone());
        out += &name_mangle_type(&func.args[i]);
        out += " ";
        out += "user_";
        out += &func.arg_names[i];
        if i < func.args.len() - 1 {
            out += ",";
        }
    }
    out += "){\n";
    out += "    gc_push_frame();\n";
    let mut stack_ptr = 32;
    let ir = compile_function_to_ir(func, functions, types, &mut stack_ptr);
    println!("ir representation:{:#?}", ir);
    let mut depth = 1;
    for i in &ir {
        let tmp = compile_ir_instr_to_c(i, &mut depth, used_types);
        out += &tmp;
        out += "\n";
    }
    out += "    gc_pop_frame();\n";
    out += "\n}\n";
    return Ok(out);
}
#[allow(unused)]
pub fn handle_dependencies(map: &HashMap<String, Type>) -> Vec<(String, Type)> {
    fn contains_undeclared_type(t: &Type, map: &HashSet<String>, recursed: bool) -> bool {
        match t {
            Type::ArrayT {
                size: _,
                array_type,
            } => {
                if contains_undeclared_type(&array_type, map, true) {
                    return true;
                }
                false
            }
            Type::PointerT { ptr_type } => {
                if contains_undeclared_type(&ptr_type, map, true) {
                    return true;
                }
                false
            }
            Type::SliceT { ptr_type } => {
                if contains_undeclared_type(&ptr_type, map, true) {
                    return true;
                }
                false
            }
            Type::StructT { name, components } => {
                if recursed {
                    if !map.contains(name.as_ref()) {
                        return true;
                    }
                }
                for i in components {
                    if contains_undeclared_type(&i.1, map, true) {
                        return true;
                    }
                }
                false
            }
            _ => false,
        }
    }
    let mut declared_types: HashSet<String> = HashSet::new();
    let mut que: Vec<(&String, &Type)> = vec![];
    let mut out = vec![];
    for i in map {
        let t = i.1;
        if contains_undeclared_type(t, &declared_types, false) {
            que.push(i);
        } else {
            declared_types.insert(i.0.clone());
            out.push((i.0.clone(), i.1.clone()));
        }
        let mut pushed = false;
        loop {
            for k in 0..que.len() {
                let j = &que[k];
                if !contains_undeclared_type((*j).1, &declared_types, false) {
                    declared_types.insert(j.0.clone());
                    out.push((j.0.clone(), j.1.clone()));
                    pushed = true;
                }
                que.remove(k);
                break;
            }
            if !pushed {
                break;
            }
            pushed = false;
        }
    }
    return out;
}

#[allow(unused)]
pub fn gc_function_name(t: &Type) -> String {
    return "gc_".to_owned() + &name_mangle_type_for_names(t);
}

#[allow(unused)]
fn compile_gc_functions(types: HashSet<Type>) -> String {
    let mut out = String::new();
    for i in &types {
        match i {
            Type::StringT => {
                continue;
            }
            _ => {
                out += "void ";
                out += &(gc_function_name(i) + "(void*);\n");
            }
        }
    }
    for i in &types {
        match i {
            Type::StringT {} => {
                continue;
            }
            Type::IntegerT {} => {
                continue;
            }
            Type::BoolT => {
                continue;
            }
            Type::CharT => {
                continue;
            }
            Type::FloatT => {
                continue;
            }
            _ => {}
        }
        if i.is_partially_defined() {
            continue;
        }
        out += "void ";
        out += &(gc_function_name(i) + "(void* ptr){\n");
        out += &("  ".to_owned() + &(name_mangle_type(i) + "* var = ptr;\n"));
        match i {
            Type::PointerT { ptr_type } => {
                out += "   if(!(*var)){return;}\n";
                out += "   bool hit =gc_any_ptr(*var);\n   if(hit){return;}\n";
                out += "    ";

                out += &(gc_function_name(ptr_type) + "(*var);\n");
            }
            Type::SliceT { ptr_type } => {
                out += "   bool hit = gc_any_ptr(var->start);\n";
                out += "   if(hit){return;}\n";
                out += "    for(int i =0; i<var->len; i++){";
                out += "    ";
                out += &(gc_function_name(ptr_type) + "(&var->start[i]);}\n");
            }
            Type::StructT {
                name: _,
                components,
            } => {
                for i in components {
                    out += "    ";
                    out += &gc_function_name(&i.1);
                    out += "(";
                    out += "&var->";
                    out += &i.0;
                    out += ");\n";
                }
            }
            _ => {
                out += "return;\n";
            }
        }
        out += "}\n";
    }
    return out;
}

#[allow(unused)]
fn get_all_types_contained(t: &Type, types: &HashMap<String, Type>) -> Vec<Type> {
    let mut out = vec![];
    match t {
        Type::ArrayT { size, array_type } => {
            out.push(get_all_types_contained(array_type, types));
            match array_type.as_ref() {
                Type::PartiallyDefined { name } => {
                    out.push(vec![Type::PointerT {
                        ptr_type: Rc::new(types.get(name.as_ref()).expect("name exists").clone()),
                    }]);
                }
                _ => {
                    out.push(vec![Type::ArrayT {
                        size: *size,
                        array_type: array_type.clone(),
                    }]);
                }
            }
            return out.into_iter().flatten().collect();
        }
        Type::PointerT { ptr_type } => {
            out.push(get_all_types_contained(ptr_type, types));
            match ptr_type.as_ref() {
                Type::PartiallyDefined { name } => {
                    out.push(vec![Type::PointerT {
                        ptr_type: Rc::new(types.get(name.as_ref()).expect("name exists").clone()),
                    }]);
                }
                _ => {
                    out.push(vec![Type::PointerT {
                        ptr_type: ptr_type.clone(),
                    }]);
                }
            }
            return out.into_iter().flatten().collect();
        }
        Type::SliceT { ptr_type } => {
            out.push(get_all_types_contained(ptr_type, types));
            match ptr_type.as_ref() {
                Type::PartiallyDefined { name } => {
                    out.push(vec![Type::SliceT {
                        ptr_type: Rc::new(types.get(name.as_ref()).expect("name exists").clone()),
                    }]);
                }
                _ => {
                    out.push(vec![Type::SliceT {
                        ptr_type: ptr_type.clone(),
                    }]);
                }
            }
            return out.into_iter().flatten().collect();
        }
        Type::StructT {
            name: _,
            components,
        } => {
            for i in components {
                out.push(get_all_types_contained(&i.1, types));
            }
        }
        Type::PartiallyDefined { name } => {
            return vec![types.get(name.as_ref()).expect("type must exist").clone()];
        }
        _ => {}
    }
    out.push(vec![t.clone()]);
    return out.into_iter().flatten().collect();
}

#[allow(unused)]
fn recurse_used_types(types: &HashSet<Type>, type_table: &HashMap<String, Type>) -> HashSet<Type> {
    let mut out = HashSet::new();
    for i in types {
        let j = get_all_types_contained(i, type_table);
        for k in j {
            match k {
                Type::PartiallyDefined { name: _ } => {
                    continue;
                }
                _ => {}
            }
            out.insert(k);
        }
    }
    return out;
}

#[allow(unused)]
pub fn compile(prog: Program, base_filename: &str) -> Result<(), String> {
    println!("compiling file: {}", base_filename);
    let fname = "output/".to_owned() + &base_filename[0..base_filename.len() - 4];
    let filename = &fname;
    let mut out = String::new();
    let mut typedecs = "".to_owned();
    let mut used_types = HashSet::new();
    let progtypes = handle_dependencies(&prog.types);
    for i in &progtypes {
        typedecs += &compile_type(i.0.clone(), i.1.clone())?;
    }
    let mut func_decs = String::new();
    for i in &prog.functions {
        func_decs += &compile_function_table_header(i.0, i.1, filename)?;
    }
    let mut statics = String::new();
    for i in &prog.static_variables {
        statics += &compile_static(&i.0, &i.1 .0, i.1 .1)?;
    }
    let mut functions = String::new();
    for i in &prog.functions {
        for func in &i.1.functions {
            if func.forward_declared {
                continue;
            }
            let mut f = func.clone();
            functions += &compile_function(
                &mut f,
                filename,
                &prog.functions,
                &prog.types,
                &mut used_types,
            )?;
        }
    }
    let out_file_name = filename.to_owned() + ".c";
    let mut fout = fs::File::create(&out_file_name).expect("testing expect");
    used_types = recurse_used_types(&used_types, &prog.types);
    for i in &used_types {
        let mut hit = false;
        for j in &progtypes {
            if j.1 == *i {
                hit = true;
                break;
            }
        }
        if !hit {
            typedecs += &compile_type("".to_owned(), i.clone()).expect("must work");
        }
    }
    typedecs += &compile_gc_functions(used_types);
    out += "#include \"../builtins.h\"\n";
    out += &typedecs;
    out += &func_decs;
    out += &statics;
    out += &functions;
    fout.write(out.as_bytes()).expect("testing expect");
    drop(fout);
    let _ = std::process::Command::new("clang")
        .arg(&out_file_name)
        .arg("-std=c2x")
        .arg("-c")
        .arg(&format!("-o{}.o", fname))
        .output();
    return Ok(());
}
