
use crate::get_function_by_args;
use crate::is_compatible_type;
use crate::AstNode;
use std::collections::HashMap;
use crate::FunctionTable;
use crate::Type;
use crate::Program;
pub fn alide_parens(root:&mut AstNode){
    match root{
        AstNode::Paren { internals }=>{
            *root = *internals.clone();
        }
        AstNode::FunctionCall { function_name:_, args, data:_ }=>{
            for i in 0..args.len(){
                alide_parens(&mut args[i]);
            }
        }
        AstNode::Assignment { left, right, data:_ }=>{
            alide_parens(left);
            alide_parens(right);
        }
        AstNode::Add { left, right, data:_ }=>{
            alide_parens(left);
            alide_parens(right);
        }
        AstNode::Sub{ left, right, data:_ }=>{
            alide_parens(left);
            alide_parens(right);
        }
        AstNode::Mult{ left, right, data:_ }=>{
            alide_parens(left);
            alide_parens(right);
        }
        AstNode::Div { left, right, data:_ }=>{
            alide_parens(left);
            alide_parens(right);
        }
        AstNode::Equals { left, right, data:_ }=>{
            alide_parens(left);
            alide_parens(right);
        }
        AstNode::GreaterThan{ left, right, data:_ }=>{
            alide_parens(left);
            alide_parens(right);
        }
        AstNode::LessThan{ left, right, data:_ }=>{
            alide_parens(left);
            alide_parens(right);
        }
        AstNode::LessOrEq { left, right, data:_ }=>{
            alide_parens(left);
            alide_parens(right);
        }
        AstNode::GreaterOrEq { left, right, data:_ }=>{
            alide_parens(left);
            alide_parens(right);
        }
        AstNode::Not { value, data:_ }=>{
            alide_parens(value);
        }
        AstNode::And { left, right, data:_ }=>{
            alide_parens(left);
            alide_parens(right);
        }
        AstNode::Or { left, right, data:_ }=>{
            alide_parens(left);
            alide_parens(right);
        }
        AstNode::If { condition, thing_to_do, r#else }=>{
            alide_parens(condition);
            for i in thing_to_do{
                alide_parens(i);
            }
            if let Some(i) = r#else{
                for j in i{
                    alide_parens(j);
                }
            }
        }
        AstNode::Loop{ condition, body }=>{
            alide_parens(condition);
            for i in body{
                alide_parens(i);
            }
        }
        AstNode::ForLoop{ variable,condition, post_op, body
         }=>{
            alide_parens(variable);
            alide_parens(condition);
            for i in body{
                alide_parens(i);
            }
            alide_parens(post_op);
        }
        AstNode::Return { body }=>{
            alide_parens(body);
        }
        AstNode::Deref { thing_to_deref }=>{
            alide_parens(thing_to_deref);
        }
        AstNode::TakeRef { thing_to_ref }=>{
            alide_parens(thing_to_ref);
        }
        AstNode::FieldUsage { base, field_name:_ }=>{
            alide_parens(base);
        }
        AstNode::ArrayAccess { variable, index }=>{
            alide_parens(variable);
            alide_parens(index);
        }
        AstNode::BoundFunctionCall { variable, function_name:_, args ,data:_}=>{
            alide_parens(variable);
            for i in args{
                alide_parens(i);
            }
        }
        _=>{

        }
    }
}
fn validate_ast_node(node:&AstNode, types:&HashMap<String,Type>, functions:&mut HashMap<String,FunctionTable>, is_root:bool, inside_loop:bool, return_type:Option<Type>)->Result<AstNode,String>{
    match node{
        AstNode::Assignment { left, right, data }=>{
            let left = validate_ast_node(left, types, functions, false,inside_loop, return_type.clone())?;
            let right =  validate_ast_node(right, types, functions, false,inside_loop, return_type.clone())?;
            let lt = left.get_type(functions, types).expect("must have type");
            let rt = right.get_type(functions, types).expect("must have type");
            if !is_compatible_type(&lt, &rt){
                return Err(format!("imcompatable assignment types line:{} {:#?} and {:#?}",data.clone().expect("must have data").line, lt, rt));
            }
            let out= Ok(AstNode::Assignment{left:Box::new(left), right: Box::new(right), data: data.clone() });
            return out;
        }
        AstNode::FunctionCall { function_name, args, data }=>{
            let args:Vec<Result<AstNode,String>>= args.iter().map(|i| validate_ast_node(i, types, functions, false, inside_loop, return_type.clone())).collect();
            let args = {
                let mut tmp_args = vec![];
                for a in args{
                    tmp_args.push(a?);
                }
                tmp_args
            };
            let arg_types:Vec<Type> = args.clone().iter().map(|i| i.get_type(functions, types).expect("should work")).collect();
            if get_function_by_args(function_name, &arg_types, functions).is_some(){
                return Ok(AstNode::FunctionCall { function_name: function_name.to_string(), args: args, data: data.clone() });
            } else{
                return Err(format!("could not find viable implementation of {} for arguments {:#?} line {}", function_name, arg_types,data.clone().expect("").line));
            }
        }
        AstNode::VariableDeclaration { name, var_type, value_assigned ,data}=>{
            if !is_root{
                let out = Err(format!("variable declarations disallowed inside of other expressions line:{}",data.clone().expect("should_exist").line ));
                return out;
            }
            if let Some(v) = value_assigned{
                let tmp = validate_ast_node(v, types, functions, false, inside_loop, return_type)?;
                if !is_compatible_type(var_type, &tmp.get_type(functions,types).expect("must have type")){
                    let out = Err(format!("incompatable assignment types types line: {} {:#?} and {:#?}", data.clone().expect("must have data").line, var_type, tmp.get_type(functions,types).expect("must have type")));
                    return out;
                }
                return Ok(AstNode::VariableDeclaration{name:name.clone(), var_type:var_type.clone(), value_assigned:Some(Box::new(tmp)), data:data.clone()});
            } else{
                return Ok(AstNode::VariableDeclaration{name:name.clone(), var_type:var_type.clone(), value_assigned:None, data:data.clone()});
            }
        }
        AstNode::Add { left, right, data }=>{
            let left = validate_ast_node(left, types, functions, false,inside_loop, return_type.clone())?;
            let right =  validate_ast_node(right, types, functions, false,inside_loop, return_type.clone())?;
            let lt = left.get_type(functions, types).expect("must have type");
            let rt = right.get_type(functions, types).expect("must have type");
            if !lt.is_basic_number() || !rt.is_basic_number(){
                let out = AstNode::FunctionCall { function_name: "+".to_string(), args: vec![left,right], data: data.clone() };
                return validate_ast_node(&out, types, functions, is_root, inside_loop, return_type);
            } else{
                if !is_compatible_type(&lt, &rt){
                    return Err(format!("imcompatable addition types line:{} {:#?} and {:#?}",data.clone().expect("must have data").line, lt, rt));
                }
                let out= Ok(AstNode::Add{left:Box::new(left), right: Box::new(right), data: data.clone() });
                return out;
            }
  
        }
        AstNode::Sub { left, right, data }=>{
            let left = validate_ast_node(left, types, functions, false,inside_loop, return_type.clone())?;
            let right =  validate_ast_node(right, types, functions, false,inside_loop, return_type.clone())?;
            let lt = left.get_type(functions, types).expect("must have type");
            let rt = right.get_type(functions, types).expect("must have type");
            if !lt.is_basic_number() || !rt.is_basic_number(){
                let out = AstNode::FunctionCall { function_name: "-".to_string(), args: vec![left,right], data: data.clone() };
                return validate_ast_node(&out, types, functions, is_root, inside_loop, return_type);
            } else{
                if !is_compatible_type(&lt, &rt){
                    return Err(format!("imcompatable addition types line:{} {:#?} and {:#?}",data.clone().expect("must have data").line, lt, rt));
                }
                let out= Ok(AstNode::Sub{left:Box::new(left), right: Box::new(right), data: data.clone() });
                return out;
            }
        }
        AstNode::Mult { left, right, data }=>{
            let left = validate_ast_node(left, types, functions, false,inside_loop, return_type.clone())?;
            let right =  validate_ast_node(right, types, functions, false,inside_loop, return_type.clone())?;
            let lt = left.get_type(functions, types).expect("must have type");
            let rt = right.get_type(functions, types).expect("must have type");
            if !lt.is_basic_number() || !rt.is_basic_number(){
                let out = AstNode::FunctionCall { function_name: "*".to_string(), args: vec![left,right], data: data.clone() };
                return validate_ast_node(&out, types, functions, is_root, inside_loop, return_type);
            } else{
                if !is_compatible_type(&lt, &rt){
                    return Err(format!("imcompatable addition types line:{} {:#?} and {:#?}",data.clone().expect("must have data").line, lt, rt));
                }
                let out= Ok(AstNode::Mult{left:Box::new(left), right: Box::new(right), data: data.clone() });
                return out;
            }
        }
        AstNode::Div { left, right, data }=>{
            let left = validate_ast_node(left, types, functions, false,inside_loop, return_type.clone())?;
            let right =  validate_ast_node(right, types, functions, false,inside_loop, return_type.clone())?;
            let lt = left.get_type(functions, types).expect("must have type");
            let rt = right.get_type(functions, types).expect("must have type");
            if !lt.is_basic_number() || !rt.is_basic_number(){
                let out = AstNode::FunctionCall { function_name: "/".to_string(), args: vec![left,right], data: data.clone() };
                return validate_ast_node(&out, types, functions, is_root, inside_loop, return_type);
            } else{
                if !is_compatible_type(&lt, &rt){
                    return Err(format!("imcompatable addition types line:{} {:#?} and {:#?}",data.clone().expect("must have data").line, lt, rt));
                }
                let out= Ok(AstNode::Div{left:Box::new(left), right: Box::new(right), data: data.clone() });
                return out;
            }
        }
        AstNode::Equals { left, right, data }=>{
            let left = validate_ast_node(left, types, functions, false,inside_loop, return_type.clone())?;
            let right =  validate_ast_node(right, types, functions, false,inside_loop, return_type.clone())?;
            let lt = left.get_type(functions, types).expect("must have type");
            let rt = right.get_type(functions, types).expect("must have type");
            if !lt.is_basic_number() || !rt.is_basic_number(){
                let out = AstNode::FunctionCall { function_name: "==".to_string(), args: vec![left,right], data: data.clone() };
                return validate_ast_node(&out, types, functions, is_root, inside_loop, return_type);
            } else{
                if !is_compatible_type(&lt, &rt){
                    return Err(format!("imcompatable addition types line:{} {:#?} and {:#?}",data.clone().expect("must have data").line, lt, rt));
                }
                let out= Ok(AstNode::Equals{left:Box::new(left), right: Box::new(right), data: data.clone() });
                return out;
            }
        }        
        AstNode::NotEquals { left, right, data }=>{
            let left = validate_ast_node(left, types, functions, false,inside_loop, return_type.clone())?;
            let right =  validate_ast_node(right, types, functions, false,inside_loop, return_type.clone())?;
            let lt = left.get_type(functions, types).expect("must have type");
            let rt = right.get_type(functions, types).expect("must have type");
            if !lt.is_basic_number() || !rt.is_basic_number(){
                let out = AstNode::FunctionCall { function_name: "!=".to_string(), args: vec![left,right], data: data.clone() };
                return validate_ast_node(&out, types, functions, is_root, inside_loop, return_type);
            } else{
                if !is_compatible_type(&lt, &rt){
                    return Err(format!("imcompatable addition types line:{} {:#?} and {:#?}",data.clone().expect("must have data").line, lt, rt));
                }
                let out= Ok(AstNode::NotEquals{left:Box::new(left), right: Box::new(right), data: data.clone() });
                return out;
            }
        }
        AstNode::GreaterThan { left, right, data }=>{
            let left = validate_ast_node(left, types, functions, false,inside_loop, return_type.clone())?;
            let right =  validate_ast_node(right, types, functions, false,inside_loop, return_type.clone())?;
            let lt = left.get_type(functions, types).expect("must have type");
            let rt = right.get_type(functions, types).expect("must have type");
            if !lt.is_basic_number() || !rt.is_basic_number(){
                let out = AstNode::FunctionCall { function_name: ">".to_string(), args: vec![left,right], data: data.clone() };
                return validate_ast_node(&out, types, functions, is_root, inside_loop, return_type);
            } else{
                if !is_compatible_type(&lt, &rt){
                    return Err(format!("imcompatable addition types line:{} {:#?} and {:#?}",data.clone().expect("must have data").line, lt, rt));
                }
                let out= Ok(AstNode::GreaterThan{left:Box::new(left), right: Box::new(right), data: data.clone() });
                return out;
            }
        }
        AstNode::LessThan { left, right, data }=>{
            let left = validate_ast_node(left, types, functions, false,inside_loop, return_type.clone())?;
            let right =  validate_ast_node(right, types, functions, false,inside_loop, return_type.clone())?;
            let lt = left.get_type(functions, types).expect("must have type");
            let rt = right.get_type(functions, types).expect("must have type");
            if !lt.is_basic_number() || !rt.is_basic_number(){
                let out = AstNode::FunctionCall { function_name: "<".to_string(), args: vec![left,right], data: data.clone() };
                return validate_ast_node(&out, types, functions, is_root, inside_loop, return_type);
            } else{
                if !is_compatible_type(&lt, &rt){
                    return Err(format!("imcompatable addition types line:{} {:#?} and {:#?}",data.clone().expect("must have data").line, lt, rt));
                }
                let out= Ok(AstNode::LessThan{left:Box::new(left), right: Box::new(right), data: data.clone() });
                return out;
            }
        }
        AstNode::GreaterOrEq { left, right, data }=>{
            let left = validate_ast_node(left, types, functions, false,inside_loop, return_type.clone())?;
            let right =  validate_ast_node(right, types, functions, false,inside_loop, return_type.clone())?;
            let lt = left.get_type(functions, types).expect("must have type");
            let rt = right.get_type(functions, types).expect("must have type");
            if !lt.is_basic_number() || !rt.is_basic_number(){
                let out = AstNode::FunctionCall { function_name: ">=".to_string(), args: vec![left,right], data: data.clone() };
                return validate_ast_node(&out, types, functions, is_root, inside_loop, return_type);
            } else{
                if !is_compatible_type(&lt, &rt){
                    return Err(format!("imcompatable addition types line:{} {:#?} and {:#?}",data.clone().expect("must have data").line, lt, rt));
                }
                let out= Ok(AstNode::GreaterOrEq{left:Box::new(left), right: Box::new(right), data: data.clone() });
                return out;
            }
        }
        AstNode::LessOrEq { left, right, data }=>{
            let left = validate_ast_node(left, types, functions, false,inside_loop, return_type.clone())?;
            let right =  validate_ast_node(right, types, functions, false,inside_loop, return_type.clone())?;
            let lt = left.get_type(functions, types).expect("must have type");
            let rt = right.get_type(functions, types).expect("must have type");
            if !lt.is_basic_number() || !rt.is_basic_number(){
                let out = AstNode::FunctionCall { function_name: "<=".to_string(), args: vec![left,right], data: data.clone() };
                return validate_ast_node(&out, types, functions, is_root, inside_loop, return_type);
            } else{
                if !is_compatible_type(&lt, &rt){
                    return Err(format!("imcompatable addition types line:{} {:#?} and {:#?}",data.clone().expect("must have data").line, lt, rt));
                }
                let out= Ok(AstNode::LessOrEq{left:Box::new(left), right: Box::new(right), data: data.clone() });
                return out;
            }
        }
        AstNode::Not { value, data }=>{
            let tmp = validate_ast_node(value, types, functions, is_root, inside_loop, return_type)?;
            if !is_compatible_type(&tmp.get_type(functions, types).expect("should work"), &Type::BoolT){
                return Err(format!("value in not must be boolean line:{}", data.clone().expect("data should exist").line));
            }
            return Ok(AstNode::Not { value: Box::new(tmp), data: data.clone() });
        }
        AstNode::If { condition, thing_to_do, r#else }=>{
            if !is_root{
                return Err("cannot declare loop as non root".to_owned());
            }
            let cond = validate_ast_node(condition, types, functions, false, inside_loop, return_type.clone())?;
            if !is_compatible_type(&cond.get_type(functions, types).expect(""), &Type::BoolT){
                return Err(format!("condition must be turnable into bool line:{}", cond.get_data().expect("").line));
            }
            let mut new_body = vec![];
            for i in thing_to_do{
                let tmp = validate_ast_node(i, types, functions, true, true, return_type.clone())?;
                new_body.push(tmp);
            }
            let el = if r#else.is_some(){
                let mut tmp = vec![];
                let t = r#else.clone().unwrap();
                for i in &t{
                    let val = validate_ast_node(&i, types, functions, is_root, inside_loop, return_type.clone())?;
                    tmp.push(val)
                }
                Some(tmp)
            } else{None};
            return Ok(AstNode::If{ condition: Box::new(cond), thing_to_do: new_body , r#else:el});
        }
        AstNode::ForLoop { variable, condition, post_op, body }=>{
            let var = validate_ast_node(variable, types, functions, true, inside_loop,return_type.clone())?;
            let cond = validate_ast_node(condition,types, functions, false, inside_loop, return_type.clone())?;
            let op = validate_ast_node(post_op, types, functions, is_root, inside_loop, return_type.clone())?;
            let bd = {
                let mut bd_out = vec![];
                for i in body{
                    let tmp = validate_ast_node(i, types, functions, is_root, true, return_type.clone())?;
                    bd_out.push(tmp);
                }
                bd_out
            };
            return Ok(AstNode::ForLoop { variable: Box::new(var), condition: Box::new(cond), post_op: Box::new(op), body: bd });
        }
        AstNode::Loop { condition, body }=>{
            if !is_root{
                return Err("cannot declare loop as non root".to_owned());
            }
            let cond = validate_ast_node(condition, types, functions, false, inside_loop, return_type.clone())?;
            if !is_compatible_type(&cond.get_type(functions, types).expect(""), &Type::BoolT){
                return Err(format!("condition must be turnable into bool line:{}", cond.get_data().expect("").line));
            }
            let mut new_body = vec![];
            for i in body{
                let tmp = validate_ast_node(i, types, functions, true, true, return_type.clone())?;
                new_body.push(tmp);
            }
            return Ok(AstNode::Loop { condition: Box::new(cond), body: new_body });
        }
        AstNode::Return { body }=>{
            if return_type.is_some(){
                if is_compatible_type(&body.get_type(functions, types).expect(""),&return_type.clone().expect("")){
                    let tmp = validate_ast_node(body, types, functions, is_root, inside_loop, return_type)?;
                    return Ok(AstNode::Return { body: Box::new(tmp) });
                } else{
                    return Err(format!("incompable return types")); 
                }
            }
            else{
                return Err(format!("cannot return value from thing that cannot return"));
            }
        } 
        AstNode::OperatorMake { vtype, size }=>{
            let old = validate_ast_node(size, types, functions, false, inside_loop, return_type)?;
            if old.get_type(functions, types).expect("should return type") != Type::IntegerT{
                return Err(format!("make must have an integer for size"));
            }
            return Ok(AstNode::OperatorMake { vtype:vtype.clone(), size: Box::new(old) });
        }
        AstNode::BoundFunctionCall { variable, function_name, args ,data}=>{
            let mut new_args = vec![];
            new_args.push(variable.as_ref().clone()); 
            args.iter().for_each(|i| {new_args.push(i.clone())});
            let func = AstNode::FunctionCall { function_name: function_name.clone(),args:new_args, data:data.clone()};
            return validate_ast_node(&func, types, functions, is_root, inside_loop, return_type);
        }
        _=>{
            return Ok(node.clone());
        }
    }
}
#[allow(unused)]
pub fn validate_ast(prg:Program)->Result<Program, String>{
    let mut types = prg.types.clone();
    let mut functions  = prg.functions.clone();
    let mut static_variables = prg.static_variables.clone();
    let mut global_initializers = prg.global_initializers.clone();
    loop{
        let mut functions_new = functions.clone();
        let mut added_fn = false;
        for i in &prg.functions{
            let name = i.0;
            for j in 0..i.1.functions.len(){
                let mut ast:Vec<AstNode> =Vec::new();
                for k in &i.1.functions[j].program{
                    let t = i.1.functions[j].return_type.clone();
                    let node = validate_ast_node(k,&types,&mut functions,true, false,Some(t) )?;
                    ast.push(node);
                }
                functions.get_mut(i.0).expect("function should exist").functions[j].program = ast;
            }
        }
        for i in &prg.global_initializers{
            let node = {
                if let Some(n) = &i.1{
                    Some(validate_ast_node(n, &types, &mut functions,true, false, None)?)
                } else{
                    None
                }
            };
            global_initializers.push((i.0.to_owned(),i.1.to_owned() ));
        }
        if !added_fn{
            break;
        }
    }
    return Ok(Program{types,functions, static_variables, global_initializers});
}
