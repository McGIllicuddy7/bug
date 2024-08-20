
use crate::AstNode;
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
        AstNode::BoundFunctionCall { variable, function_name, args }=>{
            alide_parens(variable);
            for i in args{
                alide_parens(i);
            }
        }
        _=>{

        }
    }
}