
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
        AstNode::BoundFunctionCall { variable, function_name:_, args }=>{
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
                return Err(format!("Error:imcompatable assignment types line:{} {:#?} and {:#?}",data.clone().expect("must have data").line, lt, rt));
            }
            let out= Ok(AstNode::Assignment{left:Box::new(left), right: Box::new(right), data: data.clone() });
            return out;
        }
        AstNode::FunctionCall { function_name, args, data }=>{
            return Ok(node.clone());
        }
        AstNode::VariableDeclaration { name, var_type, value_assigned ,data}=>{
            if !is_root{
                let out = Err(format!("Error: variable declarations disallowed inside of other expressions line:{}",data.clone().expect("should_exist").line ));
                return out;
            }
            if let Some(v) = value_assigned{
                let tmp = validate_ast_node(v, types, functions, false, inside_loop, return_type)?;
                if !is_compatible_type(var_type, &tmp.get_type(functions,types).expect("must have type")){
                    let out = Err(format!("Error: incompatable assignment types types line: {} {:#?} and {:#?}", data.clone().expect("must have data").line, var_type, tmp.get_type(functions,types).expect("must have type")));
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
            if !is_compatible_type(&lt, &rt){
                return Err(format!("Error:imcompatable addition types line:{} {:#?} and {:#?}",data.clone().expect("must have data").line, lt, rt));
            }
            let out= Ok(AstNode::Add{left:Box::new(left), right: Box::new(right), data: data.clone() });
            return out;
        }
        AstNode::Sub { left, right, data }=>{
           let left = validate_ast_node(left, types, functions, false,inside_loop, return_type.clone())?;
            let right =  validate_ast_node(right, types, functions, false,inside_loop, return_type.clone())?;
            let lt = left.get_type(functions, types).expect("must have type");
            let rt = right.get_type(functions, types).expect("must have type");
            if !is_compatible_type(&lt, &rt){
                return Err(format!("Error:imcompatable subtraction types line:{} {:#?} and {:#?}",data.clone().expect("must have data").line, lt, rt));
            }
            let out= Ok(AstNode::Sub{ left:Box::new(left), right: Box::new(right), data: data.clone() });
            return out;
        }
        AstNode::Mult { left, right, data }=>{
           let left = validate_ast_node(left, types, functions, false,inside_loop, return_type.clone())?;
            let right =  validate_ast_node(right, types, functions, false,inside_loop, return_type.clone())?;
            let lt = left.get_type(functions, types).expect("must have type");
            let rt = right.get_type(functions, types).expect("must have type");
            if !is_compatible_type(&lt, &rt){
                return Err(format!("Error:imcompatable multiplication types line:{} {:#?} and {:#?}",data.clone().expect("must have data").line, lt, rt));
            }
            let out= Ok(AstNode::Mult{ left:Box::new(left), right: Box::new(right), data: data.clone() });
            return out;
        }
        AstNode::Div { left, right, data }=>{
           let left = validate_ast_node(left, types, functions, false,inside_loop, return_type.clone())?;
            let right =  validate_ast_node(right, types, functions, false,inside_loop, return_type.clone())?;
            let lt = left.get_type(functions, types).expect("must have type");
            let rt = right.get_type(functions, types).expect("must have type");
            if !is_compatible_type(&lt, &rt){
                return Err(format!("Error:imcompatable division types line:{} {:#?} and {:#?}",data.clone().expect("must have data").line, lt, rt));
            }
            let out= Ok(AstNode::Div{ left:Box::new(left), right: Box::new(right), data: data.clone() });
            return out;
        }
        AstNode::Equals { left, right, data }=>{
           let left = validate_ast_node(left, types, functions, false,inside_loop, return_type.clone())?;
            let right =  validate_ast_node(right, types, functions, false,inside_loop, return_type.clone())?;
            let lt = left.get_type(functions, types).expect("must have type");
            let rt = right.get_type(functions, types).expect("must have type");
            if !is_compatible_type(&lt, &rt){
                return Err(format!("Error:imcompatable comparision types line:{} {:#?} and {:#?}",data.clone().expect("must have data").line, lt, rt));
            }
            let out= Ok(AstNode::Equals{ left:Box::new(left), right: Box::new(right), data: data.clone() });
            return out;
        }        
        AstNode::NotEquals { left, right, data }=>{
            let left = validate_ast_node(left, types, functions, false,inside_loop, return_type.clone())?;
            let right =  validate_ast_node(right, types, functions, false,inside_loop, return_type.clone())?;
            let lt = left.get_type(functions, types).expect("must have type");
            let rt = right.get_type(functions, types).expect("must have type");
            if !is_compatible_type(&lt, &rt){
                return Err(format!("Error:imcompatable comparision types line:{} {:#?} and {:#?}",data.clone().expect("must have data").line, lt, rt));
            }
            let out= Ok(AstNode::NotEquals{ left:Box::new(left), right: Box::new(right), data: data.clone() });
            return out;
        }
        AstNode::GreaterThan { left, right, data }=>{
            let left = validate_ast_node(left, types, functions, false,inside_loop, return_type.clone())?;
            let right =  validate_ast_node(right, types, functions, false,inside_loop, return_type.clone())?;
            let lt = left.get_type(functions, types).expect("must have type");
            let rt = right.get_type(functions, types).expect("must have type");
            if !is_compatible_type(&lt, &rt){
                return Err(format!("Error:imcompatable comparision types line:{} {:#?} and {:#?}",data.clone().expect("must have data").line, lt, rt));
            }
            let out= Ok(AstNode::GreaterThan{ left:Box::new(left), right: Box::new(right), data: data.clone() });
            return out;
        }
        AstNode::LessThan { left, right, data }=>{
            let left = validate_ast_node(left, types, functions, false,inside_loop, return_type.clone())?;
            let right =  validate_ast_node(right, types, functions, false,inside_loop, return_type.clone())?;
            let lt = left.get_type(functions, types).expect("must have type");
            let rt = right.get_type(functions, types).expect("must have type");
            if !is_compatible_type(&lt, &rt){
                return Err(format!("Error:imcompatable comparision types line:{} {:#?} and {:#?}",data.clone().expect("must have data").line, lt, rt));
            }
            let out= Ok(AstNode::LessThan{ left:Box::new(left), right: Box::new(right), data: data.clone() });
            return out;
        }
        AstNode::GreaterOrEq { left, right, data }=>{
            let left = validate_ast_node(left, types, functions, false,inside_loop, return_type.clone())?;
            let right =  validate_ast_node(right, types, functions, false,inside_loop, return_type.clone())?;
            let lt = left.get_type(functions, types).expect("must have type");
            let rt = right.get_type(functions, types).expect("must have type");
            if !is_compatible_type(&lt, &rt){
                return Err(format!("Error:imcompatable comparision types line:{} {:#?} and {:#?}",data.clone().expect("must have data").line, lt, rt));
            }
            let out= Ok(AstNode::GreaterOrEq{ left:Box::new(left), right: Box::new(right), data: data.clone() });
            return out;
        }
        AstNode::LessOrEq { left, right, data }=>{
            let left = validate_ast_node(left, types, functions, false,inside_loop, return_type.clone())?;
            let right =  validate_ast_node(right, types, functions, false,inside_loop, return_type.clone())?;
            let lt = left.get_type(functions, types).expect("must have type");
            let rt = right.get_type(functions, types).expect("must have type");
            if !is_compatible_type(&lt, &rt){
                return Err(format!("Error:imcompatable comparision types line:{} {:#?} and {:#?}",data.clone().expect("must have data").line, lt, rt));
            }
            let out= Ok(AstNode::LessOrEq{ left:Box::new(left), right: Box::new(right), data: data.clone() });
            return out;
        }
        AstNode::Not { value, data }=>{
            return Ok(node.clone());
        }
        AstNode::If { condition, thing_to_do, r#else }=>{
            return Ok(node.clone());
        }
        AstNode::ForLoop { variable, condition, post_op, body }=>{
            return Ok(node.clone());
        }
        AstNode::Loop { condition, body }=>{
            return Ok(node.clone());
        }
        AstNode::Return { body }=>{
            return Ok(node.clone());
        }
        _=>{
            return Ok(node.clone());
        }
    }
    todo!();
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
