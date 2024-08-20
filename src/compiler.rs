use crate::parser::*;
use crate::types::Type;
use std::fs;
use std::io::Write;
use std::collections::HashSet;
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
            vars = format!("    {} * start; size_t len;\n", name_mangle_type(&ptr_type));
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
pub fn compile_expression(tmp_counter:&mut usize,expr:&mut AstNode,expect_return:bool, stack:&mut String,functions:&HashMap<String, FunctionTable>,types:&HashMap<String,Type>,indent:usize)->Result<String,String>{
    fn calc_indent(indent: usize)->String{
        let mut out = String::new();
        for _ in 0..indent*4{
            out += " ";
        }
        return out;
    }
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
        AstNode::StructLiteral { vtype:_,nodes }=>{
            let mut out = String::from("{");
            for i in nodes{
                out += &compile_expression(tmp_counter, i, true,stack,functions,types,indent+1)?;
                out += ",";
            }
            out += "}";
            return Ok(out);
        }
        AstNode::ArrayLiteral { nodes }=>{
            let type_name = name_mangle_type(&nodes[0].get_type(functions, types).expect("should_return_type"))+"[]";
            let mut out = format!("({})",&type_name)+"{";
            for i in nodes{
                out += &compile_expression(tmp_counter, i, true,stack,functions,types,indent)?;
                out += ",";
            }
            out += "}";
            return Ok(out);
        }
        AstNode::VariableUse { name, index:_, vtype:_, is_arg:_, data:_ }=>{
            return Ok(String::from("user_")+name);
        }
        AstNode::FunctionCall { function_name, args, data:_ }=>{
            let bargs = args.clone();
            let mut fn_args = vec![];
            for i in bargs{
                fn_args.push(i.get_type(functions, types).expect("should_have_type"));
            }
            let retv = get_function_by_args(function_name,fn_args.as_slice(),functions).expect("should find function");
            let mut base =retv.name.clone()+"(";
            for i in args{
                base += &compile_expression(tmp_counter,i, true,stack,functions,types,indent)?;
            }
            base += ");\n";
            if expect_return{
                *stack+= &(calc_indent(indent)+&format!("{} tmp{} = {}", &name_mangle_type(&retv.return_type),*tmp_counter,&base ));
                let fmt = calc_indent(indent)+&format!("tmp{}",*tmp_counter);
                *tmp_counter+=1;
                return Ok(fmt);
            } else{
               return Ok(base);
            }
        }
        AstNode::Assignment { left, right, data:_ }=>{
            let left_s = compile_expression(tmp_counter, left, false, stack, functions, types,1)?;
            let right_s = compile_expression(tmp_counter, right, true, stack, functions, types,1)?;
            if left.get_type(functions, types).expect("should have type").is_array(){
                let s = right.get_type(functions, types).expect("right should have type");
                match s{
                    Type::SliceT { ptr_type:_ }=>{
                        return Ok(left_s+ "=" +&right_s+";")
                    }
                    Type::ArrayT { size, array_type:_ }=>{
                        return Ok(left_s.clone()+ ".start=" +&right_s+";\n"+&calc_indent(indent)+&left_s+&format!(".len = {};\n", size));
                    }
                    _=>{
                        unreachable!()
                    }
                }  

            }
            match left.get_type(functions, types).expect("should have type"){
                Type::StringT=>{
                    match right.as_ref(){
                        AstNode::StringLiteral { value}=>{
                            return Ok(left_s.clone()+ ".start=" +&right_s+";\n"+&calc_indent(indent)+&left_s+&format!(".len = {};\n", value.len()));
                        }
                        _=>{

                        }
                    }
                }
                _=>{

                }
            }
            return Ok(calc_indent(indent)+&left_s+" = "+&right_s+";\n");
        }
        AstNode::Add { left, right, data:_ }=>{
            let left_s = compile_expression(tmp_counter, left, true, stack, functions, types,indent)?;
            let right_s = compile_expression(tmp_counter, right, true, stack, functions, types,indent)?;
            let pushv = calc_indent(indent)+&format!("{} tmp{} = {}+{};\n",&name_mangle_type(&left.get_type(functions,types).expect("")),*tmp_counter,left_s,right_s);
            let stack_var_name = format!("tmp{}", tmp_counter);
            *tmp_counter +=1;
            *stack +=&pushv;
            return Ok(stack_var_name);
        } 
        AstNode::Sub { left, right, data:_ }=>{
            let left_s = compile_expression(tmp_counter, left, true, stack, functions, types,indent)?;
            let right_s = compile_expression(tmp_counter, right, true, stack, functions, types,indent)?;
            let pushv = calc_indent(indent)+&format!("{} tmp{} = {}-{};\n",&name_mangle_type(&left.get_type(functions,types).expect("")),*tmp_counter,left_s,right_s);
            let stack_var_name = format!("tmp{}", tmp_counter);
            *tmp_counter +=1;
            *stack +=&pushv;
            return Ok(stack_var_name);
        } 
        AstNode::Mult { left, right, data:_ }=>{
            let left_s = compile_expression(tmp_counter, left, true, stack, functions, types,indent)?;
            let right_s = compile_expression(tmp_counter, right, true, stack, functions, types,indent)?;
            let pushv = calc_indent(indent)+&format!("{} tmp{} = {}*{};\n",&name_mangle_type(&left.get_type(functions,types).expect("")),*tmp_counter,left_s,right_s);
            let stack_var_name = format!("tmp{}", tmp_counter);
            *tmp_counter +=1;
            *stack +=&pushv;
            return Ok(stack_var_name);
        } 
        AstNode::Div{ left, right, data:_ }=>{
            let left_s = compile_expression(tmp_counter, left, true, stack, functions, types,indent)?;
            let right_s = compile_expression(tmp_counter, right, true, stack, functions, types,indent)?;
            let pushv = calc_indent(indent)+&format!("{} tmp{} = {}/{};\n",&name_mangle_type(&left.get_type(functions,types).expect("")),*tmp_counter,left_s,right_s);
            let stack_var_name = format!("tmp{}", tmp_counter);
            *tmp_counter +=1;
            *stack +=&pushv;
            return Ok(stack_var_name);
        } 
        AstNode::Equals{ left, right, data:_ }=>{
            let left_s = compile_expression(tmp_counter, left, true, stack, functions, types,indent)?;
            let right_s = compile_expression(tmp_counter, right, true, stack, functions, types,indent)?;
            let pushv = format!("({}=={})",left_s,right_s);
            return Ok(pushv);
        } 
        AstNode::GreaterThan{ left, right, data:_ }=>{
            let left_s = compile_expression(tmp_counter, left, true, stack, functions, types,indent)?;
            let right_s = compile_expression(tmp_counter, right, true, stack, functions, types,indent)?;
            let pushv = format!("({}>{})",left_s,right_s);
            return Ok(pushv);
        } 
        AstNode::LessThan{ left, right, data:_ }=>{
            let left_s = compile_expression(tmp_counter, left, true, stack, functions, types,indent)?;
            let right_s = compile_expression(tmp_counter, right, true, stack, functions, types,indent)?;
            let pushv = format!("({}<{})",left_s,right_s);
            return Ok(pushv);
        } 
        AstNode::GreaterOrEq{ left, right, data:_ }=>{
            let left_s = compile_expression(tmp_counter, left, true, stack, functions, types,indent)?;
            let right_s = compile_expression(tmp_counter, right, true, stack, functions, types,indent)?;
            let pushv = format!("({}>={})",left_s,right_s);
            return Ok(pushv);
        } 
        AstNode::LessOrEq{ left, right, data:_ }=>{
            let left_s = compile_expression(tmp_counter, left, true, stack, functions, types,indent)?;
            let right_s = compile_expression(tmp_counter, right, true, stack, functions, types,indent)?;
            let pushv = format!("({}<={})",left_s,right_s);
            return Ok(pushv);
        } 
        AstNode::Not { value, data:_ }=>{
            let right_s = compile_expression(tmp_counter, value, true, stack, functions, types,indent)?;
            let pushv = format!("!({})",right_s);
            return Ok(pushv);
        } 
        AstNode::And{ left, right, data:_ }=>{
            let left_s = compile_expression(tmp_counter, left, true, stack, functions, types,indent)?;
            let right_s = compile_expression(tmp_counter, right, true, stack, functions, types,indent)?;
            let pushv = format!("({}&&{})",left_s,right_s);
            return Ok(pushv);
        } 
        AstNode::Or{ left, right, data:_ }=>{
            let left_s = compile_expression(tmp_counter, left, true, stack, functions, types,indent)?;
            let right_s = compile_expression(tmp_counter, right, true, stack, functions, types,indent)?;
            let pushv = format!("({}||{})",left_s,right_s);
            return Ok(pushv);
        } 
        AstNode::VariableDeclaration { name, var_type, value_assigned }=>{
            let mut pushv = calc_indent(indent)+&format!("{} user_{} =",name_mangle_type(var_type), name);
            let next = 
            match var_type{
                Type::IntegerT =>{
                    "0;\n"
                }
                Type::FloatT=>{
                    "0.0;\n"
                }
                Type::PointerT { ptr_type:_ }=>{
                    "0;\n"
                }
                Type::BoolT{}=>{
                   "false;\n"
                }
                _=>{
                    "{0};\n"
                }
                
            };
            pushv+=next;
            if let Some(assigned) = value_assigned{
                let l = compile_expression(tmp_counter, assigned, true, stack, functions, types,indent)?;
                pushv +=&(calc_indent(indent)+&l);
            }
            pushv += "\n";
            *stack +=&pushv;
            return Ok("".to_owned());
        }
        AstNode::If { condition, thing_to_do, r#else }=>{
            let cond = "if ".to_owned()+&compile_expression(tmp_counter,  condition, true, stack, functions, types,indent)?;
            let mut to_do = String::from("{\n");
            for i in thing_to_do{
                let mut stack = String::new();
                let base = &compile_expression(tmp_counter,i,false,&mut stack, functions,types,indent)?;
                to_do += &stack;
                to_do+= base;
            }
            to_do += "}\n";
            let mut thing_else = String::new();
            if let Some(els) = r#else{
                thing_else += "{\n";
                for i in els{
                    let mut stack = String::new();
                    let base = &compile_expression(tmp_counter,i,false,&mut stack, functions,types,indent+1)?;
                    to_do += &stack;
                    to_do+= base;
                }
                thing_else += "\n}";
            }
            *stack += &cond;
            *stack += &to_do;
            *stack += &thing_else;
            return Ok("".to_owned());
        } 
        AstNode::Return { body }=>{
            return Ok(calc_indent(indent)+"return "+&compile_expression(tmp_counter, body, expect_return, stack, functions, types,indent)?+";");
        }
        AstNode::Loop { condition, body }=>{
            let cond = "while".to_owned()+&compile_expression(tmp_counter,  condition, true, stack, functions, types,indent)?;
            let mut to_do = String::from("{\n");
            for i in body{
                let mut stack = String::new();
                let base = &compile_expression(tmp_counter,i,false,&mut stack, functions,types,indent+1)?;
                to_do += &stack;
                to_do+= base;
            }
            to_do += "}\n";
            *stack += &cond;
            *stack += &to_do;
            return Ok("".to_owned());
        }
        AstNode::ForLoop{ variable,condition, body ,post_op}=>{
            *stack += "{\n";
            let var = compile_expression(tmp_counter, variable, expect_return, stack, functions, types,indent)?;
            let cond= calc_indent(indent)+ "while"+&compile_expression(tmp_counter,  condition, true, stack, functions, types,indent)?+"{\n";
            let mut to_do = String::new();
            *stack += &var;
            *stack += &cond;
            for i in body{
                let base = &compile_expression(tmp_counter,i,false, stack, functions,types,indent+1)?;
                to_do+= base;
            }
            let post_op = calc_indent(indent)+&compile_expression(tmp_counter, post_op, expect_return, stack, functions, types,indent)?;
            to_do += &post_op;
            to_do += "}\n}\n";
            *stack += &to_do;
            return Ok("".to_owned());
        }
        AstNode::ArrayAccess { variable, index }=>{
            let left = compile_expression(tmp_counter, variable, false, stack, functions, types, indent)?;
            let idx = compile_expression(tmp_counter, index, true, stack, functions, types, indent)?;
            return Ok(left+".start["+&idx+"]");
        }
        AstNode::FieldUsage { base, field_name }=>{
            let left = compile_expression(tmp_counter, base, false, stack, functions, types, indent)?;
            return Ok(left+"."+&field_name);
        }
        AstNode::Deref { thing_to_deref }=>{
            let right = compile_expression(tmp_counter, thing_to_deref.as_mut(),false, stack, functions, types, indent)?;
            return Ok("*".to_owned()+&right);
        }
        AstNode::TakeRef { thing_to_ref }=>{
            let right = compile_expression(tmp_counter, thing_to_ref.as_mut(),false, stack, functions, types, indent)?;
            return Ok("&".to_owned()+&right);
        }
        _=>{
            unreachable!();
        }
    }
}
pub fn compile_function(func:&mut Function, filename:&str, functions:&HashMap<String,FunctionTable>, types:&HashMap<String, Type>)->Result<String,String>{
    let mut out = String::new();
    out += &name_mangle_type(&func.return_type);
    out += " ";
    out += &name_mangle_function(func, filename);
    out += "(";
    for i in 0..func.args.len(){
        out += &name_mangle_type(&func.args[i]);
        out += " ";
        out += "user_";
        out += &func.arg_names[i];
        if i <func.args.len()-1{
            out += ",";
        }
    }
    out += "){\n";
    let mut temp_counter = 0;
    for i in &mut func.program{
        let mut stack = String::new();
        let base = &compile_expression(&mut temp_counter,i,false,&mut stack, functions,types,1)?;
        out += &stack;
        out += base;
    }
    out += "\n}\n";
    return Ok(out);
}
pub fn handle_dependencies(map:&HashMap<String,Type>)->Vec<(String,Type)>{
    fn contains_undeclared_type(t:&Type, map: &HashSet<String>, recursed:bool)->bool{
        match t{
            Type::ArrayT { size:_, array_type }=>{
                if contains_undeclared_type(&array_type, map,true){
                    return true;
                }
                false
            }
            Type::PointerT { ptr_type }=>{
                if contains_undeclared_type(&ptr_type, map,true){
                    return true;
                }
                false
            }
            Type::SliceT { ptr_type }=>{
                if contains_undeclared_type(&ptr_type, map,true){
                    return true;
                }
                false
            }
            Type::StructT { name, components }=>{
                if recursed{
                    if !map.contains(name){
                        return true;
                    }
                }
                for i in components{
                    if contains_undeclared_type(&i.1, map,true){
                        return true;
                    }
                }
                false
            }
            _=>{
                false
            }
        }
    }
    let mut declared_types: HashSet<String> = HashSet::new();
    let mut que:Vec<(&String,&Type)> = vec![];
    let mut out = vec![];
    for i in map{
        let t = i.1;
        if contains_undeclared_type(t, &declared_types,false){
            que.push(i);
        } else{
            declared_types.insert(i.0.clone());
            out.push((i.0.clone(), i.1.clone()));
        }
        let mut pushed = false;
        loop{
            for k in 0..que.len(){
                let j = &que[k];
                if !contains_undeclared_type((*j).1, &declared_types,false){
                    declared_types.insert(j.0.clone());
                    out.push((j.0.clone(), j.1.clone()));
                    pushed = true;
                }
                que.remove(k);
                break;
            }
            if !pushed{
                break;
            }
            pushed = false;
        }
    }
    return out;
}
pub fn compile(prog:Program, base_filename:&str)->Result<(),String>{
    println!("compiling file: {}", base_filename);
    let filename = &base_filename[0..base_filename.len()-5];
    let mut out = String::new();
    let mut typedecs = "".to_owned();
    typedecs += "typedef struct {char * start; size_t length;}String;\n";
    let progtypes = handle_dependencies(&prog.types);
    for i in &progtypes{
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
            if func.forward_declared{
                continue;
            }
            let mut f =  func.clone();
            functions+= &compile_function(&mut f,filename, &prog.functions, &prog.types)?;
        }
    }
    let out_file_name = filename.to_owned()+".c";
    let mut fout = fs::File::create(&out_file_name).expect("testing expect");
    out += "#include <stdio.h>\n";
    out += &typedecs;
    out += &func_decs;
    out += &statics;
    out += &functions;
    if prog.functions.contains_key("main"){
    out += "int main(int argc,const char ** argv){\n    long result = user_main();\n    printf(\"exited with %ld\\n\",result);\n}";
    }
    fout.write(out.as_bytes()).expect("tesing expect");
    drop(fout);
    let _=std::process::Command::new("gcc").arg(&out_file_name).arg("-std=c2x").arg("-c").output();
    return Ok(());
}
