use std::fs;
use std::io::Write;
use std::collections::{HashMap, HashSet};
use crate::{
    name_mangle_type, name_mangle_type_for_names,
    Target, Type,
};
fn gc_function_name(t: &Type) -> String {
    return "gc_".to_owned() + &name_mangle_type_for_names(t);
}
pub fn compile_gc_functions(types: &HashSet<Type>, _target: &Target){
    let mut out = String::from("#include \"../builtins.h\"\n");
    let mut stypes = HashMap::new();
    for i in types{
        stypes.insert(i.get_name(), i.clone());
    }
    let types = crate::c_comp::handle_dependencies(&stypes);
    for i in &types{
        out += &crate::c_comp::compile_type(i.0.clone(), i.1.clone()).expect("184");
    }
    for i in &types {
        match i.1 {
            Type::StringT => {
                continue;
            }
            _ => {
                out += "void ";
                out += &(gc_function_name(&i.1) + "(void * ptr);\n");
            }
        }
    }
    for i in &types {
        match i.1 {
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
        if i.1.is_partially_defined() {
            continue;
        }
        out += &("void ".to_string()+&(gc_function_name(&i.1) + "(void * ptr){\n"));
        out += &("  ".to_owned() + &(name_mangle_type(&i.1) + "* var = ptr;\n"));
        match &i.1 {
            Type::PointerT { ptr_type } => {
                out += "   if(!(*var)){return;}\n";
                out += "   bool hit =gc_any_ptr(*var);\n   if(hit){return;}\n";
                out += "    ";

                out += &(gc_function_name(ptr_type.as_ref()) + "(*var);\n");
            }
            Type::SliceT { ptr_type } => {
                out += "   bool hit = gc_any_ptr(var->start);\n";
                out += "   if(hit){return;}\n";
                out += "    for(int i =0; i<var->len; i++){";
                out += "    ";
                out += &(gc_function_name(ptr_type.as_ref()) + "(&var->start[i]);}\n");
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
    let mut fout = fs::File::create("output/gc_functions.c").expect("testing expect");
    fout.write(out.as_bytes()).expect("testing expect");
    drop(fout);
    
}