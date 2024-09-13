use crate::{
    get_function_by_args, Function, FunctionTable
};
use crate::{AstNode, Type};
use std::collections::HashMap;
use std::rc::Rc;
#[allow(unused)]
#[derive(Clone, Debug)]
pub enum IrOperand {
    StacKOperand {
        var_idx: usize,
        name: Rc<str>,
        stack_offset: usize,
        vtype: Type,
    },
    Name {
        name: Rc<str>,
        vtype: Type,
    },
    Deref {
        to_deref: Box<IrOperand>,
    },
    TakeRef {
        to_ref: Box<IrOperand>,
    },
    StringLiteral {
        value: Rc<str>,
    },
    IntLiteral {
        value: i64,
    },
    FloatLiteral {
        value: f64,
    },
    CharLiteral {
        value: u8,
    },
    FieldAccess {
        base: Box<IrOperand>,
        name: Rc<str>,
    },
    ArrayAccess {
        base: Box<IrOperand>,
        value: Box<IrOperand>,
    },
}
impl IrOperand{
    pub fn get_type(&self)->Type{
        match self{
            Self::IntLiteral { value:_}=>{
                return Type::IntegerT;
            
            }
            Self::FloatLiteral { value:_ }=>{
                return Type::FloatT;
            }
            Self::ArrayAccess { base, value:_ }=>{
                return base.get_type().get_array_type().expect("");
            }
            Self::CharLiteral { value:_ }=>{
                return Type::CharT;
            }
            Self::Deref { to_deref }=>{
                match to_deref.get_type(){
                    Type::PointerT{ptr_type}=>{
                        return ptr_type.as_ref().clone();
                    }
                    _=>{ 
                        unreachable!();
                    }
                }
            }
            Self::TakeRef { to_ref } =>{
                return Type::PointerT { ptr_type: Rc::new(to_ref.get_type().clone())};
            }
            Self::Name { name:_, vtype }=>{
                return vtype.clone();
            }
            Self::StacKOperand { var_idx:_, name:_, stack_offset:_, vtype }=>{
                return vtype.clone();
            }
            Self::FieldAccess { base, name:_ }=>{
                let bs = base.get_type();
                match &bs{
                    Type::StructT { name, components }=>{
                        for i in components{
                            if &i.0 == name.as_ref(){
                               return i.1.clone(); 
                            }
                        }
                    }
                    _=>{
                        unreachable!();
                    }
                }
            }
            Self::StringLiteral { value:_ }=>{
                return Type::PointerT { ptr_type: Rc::new(Type::CharT) };
            }
        } 
        todo!();
    }
}
#[allow(unused)]
#[derive(Clone, Debug)]
pub enum IrInstr {
    VariableDeclaration {
        name: Rc<str>,
        vtype: Type,
    },
    Mov {
        left: IrOperand,
        right: IrOperand,
        vtype: Type,
    },
    Label {
        name: String,
    },
    Goto {
        target: String,
    },
    CondGoto {
        cond: IrOperand,
        target: String,
    },
    Add {
        target: IrOperand,
        left: IrOperand,
        right: IrOperand,
        vtype: Type,
    },
    Sub {
        target: IrOperand,
        left: IrOperand,
        right: IrOperand,
        vtype: Type,
    },
    Mul {
        target: IrOperand,
        left: IrOperand,
        right: IrOperand,
        vtype: Type,
    },
    Div {
        target: IrOperand,
        left: IrOperand,
        right: IrOperand,
        vtype: Type,
    },
    And {
        target: IrOperand,
        left: IrOperand,
        right: IrOperand,
        vtype: Type,
    },
    Or {
        target: IrOperand,
        left: IrOperand,
        right: IrOperand,
        vtype: Type,
    },
    Equals {
        target: IrOperand,
        left: IrOperand,
        right: IrOperand,
        vtype: Type,
    },
    NotEquals {
        target: IrOperand,
        left: IrOperand,
        right: IrOperand,
        vtype: Type,
    },
    GreaterThan {
        target: IrOperand,
        left: IrOperand,
        right: IrOperand,
        vtype: Type,
    },
    GreaterThanOrEq {
        target: IrOperand,
        left: IrOperand,
        right: IrOperand,
        vtype: Type,
    },
    LessThan {
        target: IrOperand,
        left: IrOperand,
        right: IrOperand,
        vtype: Type,
    },
    LessThanOrEq {
        target: IrOperand,
        left: IrOperand,
        right: IrOperand,
        vtype: Type,
    },
    Not {
        target: IrOperand,
        value: IrOperand,
        vtype: Type,
    },
    Call {
        func_name: String,
        args: Vec<IrOperand>,
        stack_ptr_when_called:usize,
    },
    CallWithRet {
        target: IrOperand,
        func_name: String,
        args: Vec<IrOperand>,
        vtype: Type,
        stack_ptr_when_called:usize,
    },
    Ret {
        to_return: IrOperand,
    },
    Push {
        vtype: Type,
        val_idx: usize,
    },
    Pop {
        vtype: Type,
    },
    BeginScope,
    EndScope,
}

fn stack_push(
    vtype: Type,
    val_stack: &mut Vec<IrInstr>,
    variable_counter: &mut usize,
    stack_ptr: &mut usize,
    pop_table: &mut Vec<Type>,
) -> IrOperand {
    let out = IrOperand::StacKOperand {
        var_idx: *variable_counter,
        name: format!("tmp{}", *variable_counter).into(),
        stack_offset: *stack_ptr,
        vtype: vtype.clone(),
    };
    val_stack.push(IrInstr::Push {
        vtype: vtype.clone(),
        val_idx: *variable_counter,
    });
    pop_table.push(vtype.clone());
    *variable_counter += 1;
    *stack_ptr += vtype.get_size_bytes();
    return out;
}

pub fn compile_ast_node_to_ir(
    node: &AstNode,
    val_stack: &mut Vec<IrInstr>,
    variable_counter: &mut usize,
    stack_ptr: &mut usize,
    pop_table: &mut Vec<Type>,
    name_table: &mut Vec<HashMap<String, IrOperand>>,
    functions: &HashMap<String, FunctionTable>,
    types: &HashMap<String, Type>,
    label_counter: &mut usize,
    target:Option<IrOperand>
) -> Option<IrOperand> {
    //println!("{:#?}", node);
    match node {
        AstNode::Assignment {
            left,
            right,
            data: _,
        } => {
            let lv = compile_ast_node_to_ir(
                left,
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
                name_table,
                functions,
                types,
                label_counter,
                None,
            );
            let rv = compile_ast_node_to_ir(
                right,
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
                name_table,
                functions,
                types,
                label_counter,
                lv.clone(),
            );
            match right.as_ref(){
                AstNode::Add { left:_, right:_, data:_ } | 
                AstNode::Sub { left:_, right:_, data:_ } |
                AstNode::Mult { left:_, right:_, data:_ } |
                AstNode::Div { left:_, right:_, data:_ } |
                AstNode::And { left:_, right:_, data:_ } |
                AstNode::Or{ left:_, right:_, data:_ } |
                AstNode::GreaterOrEq { left:_, right:_, data:_ } |  
                AstNode::LessOrEq { left:_, right:_, data:_ } |
                AstNode::GreaterThan { left:_, right:_, data:_ } | 
                AstNode::LessThan{ left:_, right:_, data:_ } | 
                AstNode::Not { value:_, data:_ } =>{
                    
                }
                _=>{
                val_stack.push(IrInstr::Mov {
                left: lv?,
                right: rv?,
                vtype: left.get_type(functions, types).expect("bruh"),
                });
                }
            }
        }
        AstNode::Add {
            left,
            right,
            data: _,
        } => {
            let lv = compile_ast_node_to_ir(
                left,
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
                name_table,
                functions,
                types,
                label_counter,
                None,
            );
            let rv = compile_ast_node_to_ir(
                right,
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
                name_table,
                functions,
                types,
                label_counter,
                None,
            );
            let target = if target.is_some(){ target.unwrap()} else{stack_push(
                left.get_type(functions, types).expect("please work"),
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
            )};
            val_stack.push(IrInstr::Add {
                target: target.clone(),
                left: lv?,
                right: rv?,
                vtype: left.get_type(functions, types).expect("bruh"),
            });
            return Some(target);
        }
        AstNode::Sub {
            left,
            right,
            data: _,
        } => {
            let lv = compile_ast_node_to_ir(
                left,
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
                name_table,
                functions,
                types,
                label_counter,
                None,
            );
            let rv = compile_ast_node_to_ir(
                right,
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
                name_table,
                functions,
                types,
                label_counter,
                None,
            );
            let target = if target.is_some(){ target.unwrap()} else{stack_push(
                left.get_type(functions, types).expect("please work"),
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
            )};
            val_stack.push(IrInstr::Sub {
                target: target.clone(),
                left: lv?,
                right: rv?,
                vtype: left.get_type(functions, types).expect("bruh"),
            });
            return Some(target);
        }
        AstNode::Mult {
            left,
            right,
            data: _,
        } => {
            let lv = compile_ast_node_to_ir(
                left,
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
                name_table,
                functions,
                types,
                label_counter,
                None,
            );
            let rv = compile_ast_node_to_ir(
                right,
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
                name_table,
                functions,
                types,
                label_counter,
                None,
            );
            let target = if target.is_some(){ target.unwrap()} else{stack_push(
                left.get_type(functions, types).expect("please work"),
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
            )};
            val_stack.push(IrInstr::Mul {
                target: target.clone(),
                left: lv?,
                right: rv?,
                vtype: left.get_type(functions, types).expect("bruh"),
            });
            return Some(target);
        }
        AstNode::Div {
            left,
            right,
            data: _,
        } => {
            let lv = compile_ast_node_to_ir(
                left,
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
                name_table,
                functions,
                types,
                label_counter,
                None,
            );
            let rv = compile_ast_node_to_ir(
                right,
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
                name_table,
                functions,
                types,
                label_counter,
                None,
            );
            let target = if target.is_some(){ target.unwrap()} else{stack_push(
                left.get_type(functions, types).expect("please work"),
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
            )};
            val_stack.push(IrInstr::Div {
                target: target.clone(),
                left: lv?,
                right: rv?,
                vtype: left.get_type(functions, types).expect("bruh"),
            });
            return Some(target);
        }
        AstNode::And {
            left,
            right,
            data: _,
        } => {
            let lv = compile_ast_node_to_ir(
                left,
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
                name_table,
                functions,
                types,
                label_counter,
                None,
            );
            let rv = compile_ast_node_to_ir(
                right,
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
                name_table,
                functions,
                types,
                label_counter,
                None,
            );
            let target = if target.is_some(){ target.unwrap()} else{stack_push(
                left.get_type(functions, types).expect("please work"),
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
            )};
            val_stack.push(IrInstr::And {
                target: target.clone(),
                left: lv?,
                right: rv?,
                vtype: left.get_type(functions, types).expect("bruh"),
            });
            return Some(target);
        }
        AstNode::Or {
            left,
            right,
            data: _,
        } => {
            let lv = compile_ast_node_to_ir(
                left,
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
                name_table,
                functions,
                types,
                label_counter,
                None,
            );
            let rv = compile_ast_node_to_ir(
                right,
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
                name_table,
                functions,
                types,
                label_counter,
                None,
            );
            let target = if target.is_some(){ target.unwrap()} else{stack_push(
                left.get_type(functions, types).expect("please work"),
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
            )};
            val_stack.push(IrInstr::Add {
                target: target.clone(),
                left: lv?,
                right: rv?,
                vtype: left.get_type(functions, types).expect("bruh"),
            });
            return Some(target);
        }
        AstNode::Equals {
            left,
            right,
            data: _,
        } => {
            let lv = compile_ast_node_to_ir(
                left,
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
                name_table,
                functions,
                types,
                label_counter,
                None,
            );
            let rv = compile_ast_node_to_ir(
                right,
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
                name_table,
                functions,
                types,
                label_counter,
                None,
            );
            let target = if target.is_some(){ target.unwrap()} else{stack_push(
                left.get_type(functions, types).expect("please work"),
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
            )};
            val_stack.push(IrInstr::Equals {
                target: target.clone(),
                left: lv?,
                right: rv?,
                vtype: left.get_type(functions, types).expect("bruh"),
            });
            return Some(target);
        }
        AstNode::GreaterThan {
            left,
            right,
            data: _,
        } => {
            let lv = compile_ast_node_to_ir(
                left,
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
                name_table,
                functions,
                types,
                label_counter,
                None,
            );
            let rv = compile_ast_node_to_ir(
                right,
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
                name_table,
                functions,
                types,
                label_counter,
                None,
            );
            let target = if target.is_some(){ target.unwrap()} else{stack_push(
                left.get_type(functions, types).expect("please work"),
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
            )};
            val_stack.push(IrInstr::GreaterThan {
                target: target.clone(),
                left: lv?,
                right: rv?,
                vtype: left.get_type(functions, types).expect("bruh"),
            });
            return Some(target);
        }
        AstNode::LessThan {
            left,
            right,
            data: _,
        } => {
            let lv = compile_ast_node_to_ir(
                left,
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
                name_table,
                functions,
                types,
                label_counter,
                None,
            );
            let rv = compile_ast_node_to_ir(
                right,
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
                name_table,
                functions,
                types,
                label_counter,
                None,
            );
            let target = if target.is_some(){ target.unwrap()} else{stack_push(
                left.get_type(functions, types).expect("please work"),
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
            )};
            val_stack.push(IrInstr::LessThan {
                target: target.clone(),
                left: lv?,
                right: rv?,
                vtype: left.get_type(functions, types).expect("bruh"),
            });
            return Some(target);
        }
        AstNode::GreaterOrEq {
            left,
            right,
            data: _,
        } => {
            let lv = compile_ast_node_to_ir(
                left,
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
                name_table,
                functions,
                types,
                label_counter,
                None,
            );
            let rv = compile_ast_node_to_ir(
                right,
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
                name_table,
                functions,
                types,
                label_counter,
                None,
            );
            let target = if target.is_some(){ target.unwrap()} else{stack_push(
                left.get_type(functions, types).expect("please work"),
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
            )};
            val_stack.push(IrInstr::GreaterThanOrEq {
                target: target.clone(),
                left: lv?,
                right: rv?,
                vtype: left.get_type(functions, types).expect("bruh"),
            });
            return Some(target);
        }
        AstNode::LessOrEq {
            left,
            right,
            data: _,
        } => {
            let lv = compile_ast_node_to_ir(
                left,
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
                name_table,
                functions,
                types,
                label_counter,
                None,
            );
            let rv = compile_ast_node_to_ir(
                right,
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
                name_table,
                functions,
                types,
                label_counter,
                None,
            );
            let target = if target.is_some(){ target.unwrap()} else{stack_push(
                left.get_type(functions, types).expect("please work"),
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
            )};
            val_stack.push(IrInstr::LessThanOrEq {
                target: target.clone(),
                left: lv?,
                right: rv?,
                vtype: left.get_type(functions, types).expect("bruh"),
            });
            return Some(target);
        }
        AstNode::Not { value, data :_} => {
            let v = compile_ast_node_to_ir(
                value,
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
                name_table,
                functions,
                types,
                label_counter,
                None,
            )?;
            let target = if target.is_some(){ target.unwrap()} else{stack_push(
                value.get_type(functions, types).expect("please work"),
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
            )};
            val_stack.push(IrInstr::Not {
                target: target.clone(),
                value: v.clone(),
                vtype: value.get_type(functions, types).expect("please work"),
            });
        }
        AstNode::NotEquals {
            left,
            right,
            data: _,
        } => {
            let lv = compile_ast_node_to_ir(
                left,
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
                name_table,
                functions,
                types,
                label_counter,
                None,
            );
            let rv = compile_ast_node_to_ir(
                right,
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
                name_table,
                functions,
                types,
                label_counter,
                None,
            );
            let target = if target.is_some(){ target.unwrap()} else{stack_push(
                left.get_type(functions, types).expect("please work"),
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
            )};
            val_stack.push(IrInstr::NotEquals {
                target: target.clone(),
                left: lv?,
                right: rv?,
                vtype: left.get_type(functions, types).expect("bruh"),
            });
            return Some(target);
        }
        AstNode::VariableDeclaration {
            name,
            var_type,
            value_assigned,
            data: _,
        } => {
            let op = IrOperand::StacKOperand { var_idx:*variable_counter, name: ("user_".to_owned()+&name).into(), stack_offset: *stack_ptr, vtype: var_type.clone() };
            val_stack.push(IrInstr::VariableDeclaration {
                name: ("user_".to_owned()+&name).into(),
                vtype: var_type.clone(),
            });
            pop_table.push(var_type.clone());
            *variable_counter += 1;
            *stack_ptr += var_type.get_size_bytes();
            name_table
                .last_mut()
                .expect("must exist")
                .insert(name.clone(), op);
            if value_assigned.is_some() {
                let _ = compile_ast_node_to_ir(
                    value_assigned.as_ref().expect("exists"),
                    val_stack,
                    variable_counter,
                    stack_ptr,
                    pop_table,
                    name_table,
                    functions,
                    types,
                    label_counter,
                    None,
                );
            }
        }
        AstNode::FunctionCall {
            function_name,
            args,
            data: _,
        } => {
            let t_args: Vec<Type> = args
                .iter()
                .map(|i| i.get_type(functions, types).expect("must exist"))
                .collect();
            let func = get_function_by_args(function_name, &t_args, functions).expect("must exist");
            let f_args: Vec<IrOperand> = args
                .iter()
                .map(|i| {
                    compile_ast_node_to_ir(
                        i,
                        val_stack,
                        variable_counter,
                        stack_ptr,
                        pop_table,
                        name_table,
                        functions,
                        types,
                        label_counter,
                        None,
                    )
                    .expect("should return")
                })
                .collect();
            let fname = func.name;
            if func.return_type == Type::VoidT {
                let tmp = IrInstr::Call {
                    func_name: fname,
                    args: f_args,
                    stack_ptr_when_called:*stack_ptr,
                };
                val_stack.push(tmp);
            } else {
                let return_v = stack_push(
                    func.return_type.clone(),
                    val_stack,
                    variable_counter,
                    stack_ptr,
                    pop_table,
                );
                let tmp = IrInstr::CallWithRet {
                    target: return_v.clone(),
                    func_name: fname,
                    args: f_args,
                    vtype: func.return_type,
                    stack_ptr_when_called:*stack_ptr,
                };
                val_stack.push(tmp);
                return Some(return_v);
            }
        }
        AstNode::VariableUse {
            name,
            index: _,
            vtype: _,
            is_arg: _,
            data: _,
        } => {
            for i in name_table {
                if i.contains_key(name) {
                    return Some(i[name].clone());
                }
            }
        }
        AstNode::FieldUsage { base, field_name } => {
            let base_ir = compile_ast_node_to_ir(
                base,
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
                name_table,
                functions,
                types,
                label_counter,
                None,
            )
            .expect("should return");
            return Some(IrOperand::FieldAccess {
                base: Box::new(base_ir),
                name: field_name.clone().into(),
            });
        }
        AstNode::ArrayAccess { variable, index } => {
            let var_ir = compile_ast_node_to_ir(
                variable,
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
                name_table,
                functions,
                types,
                label_counter,
                None,
            )
            .expect("should return");
            let idx_ir = compile_ast_node_to_ir(
                index,
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
                name_table,
                functions,
                types,
                label_counter,
                None,
            )
            .expect("should return");
            return Some(IrOperand::ArrayAccess {
                base: Box::new(var_ir),
                value: Box::new(idx_ir),
            });
        }
        AstNode::TakeRef { thing_to_ref } => {
            return Some(IrOperand::TakeRef {
                to_ref: Box::new(compile_ast_node_to_ir(
                    thing_to_ref,
                    val_stack,
                    variable_counter,
                    stack_ptr,
                    pop_table,
                    name_table,
                    functions,
                    types,
                    label_counter,
                    None,
                )?),
            });
        }
        AstNode::Deref { thing_to_deref } => {
            return Some(IrOperand::Deref{
                to_deref: Box::new(compile_ast_node_to_ir(
                    thing_to_deref,
                    val_stack,
                    variable_counter,
                    stack_ptr,
                    pop_table,
                    name_table,
                    functions,
                    types,
                    label_counter,
                    None,
                )?),
            });
        }
        AstNode::BoolLiteral { value } => {
            return Some(IrOperand::IntLiteral {
                value: if *value { 1 } else { 0 },
            });
        }
        AstNode::FloatLiteral { value } => {
            return Some(IrOperand::FloatLiteral { value: *value });
        }
        AstNode::IntLiteral { value } => {
            return Some(IrOperand::IntLiteral { value: *value });
        }
        AstNode::StringLiteral { value } => {
            let out = stack_push(
                Type::StringT,
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
            );
            let tmp = IrOperand::StringLiteral {
                value: value.clone().into(),
            };
            let ln = IrOperand::IntLiteral {
                value: value.len() as i64 - 2,
            };
            val_stack.push(IrInstr::CallWithRet {
                target: out.clone(),
                func_name: "make_string_from".to_owned(),
                args: vec![tmp, ln],
                vtype: Type::StringT,
                stack_ptr_when_called:*stack_ptr,
            });
            return Some(out);
        }
        AstNode::ArrayLiteral { nodes } => {
            let vtype = Type::ArrayT {
                size: nodes.len(),
                array_type: Rc::new(nodes[0].get_type(functions, types).expect("must have type")),
            };
            let ops:Vec<IrOperand> = nodes.iter().map(|i| compile_ast_node_to_ir(i, val_stack, variable_counter, stack_ptr, pop_table, name_table, functions, types, label_counter,None).expect("")).collect();
            let out = stack_push(vtype.clone(), val_stack, variable_counter, stack_ptr, pop_table);
            for i in 0..nodes.len(){
               val_stack.push(IrInstr::Mov { left: IrOperand::ArrayAccess { base: Box::new(out.clone()), value: Box::new(IrOperand::IntLiteral { value: i as i64 }) }, right: ops[i].clone(), vtype:vtype.clone()}); 
            }
            return Some(out); 
        }
        AstNode::StructLiteral { vtype, nodes } => {
            let ops:Vec<IrOperand> = nodes.iter().map(|i| compile_ast_node_to_ir(i, val_stack, variable_counter, stack_ptr, pop_table, name_table, functions, types, label_counter,None).expect("")).collect();
            let out = stack_push(vtype.clone(), val_stack, variable_counter, stack_ptr, pop_table);
            let comps = match vtype{
                Type::StructT { name, components }=>{
                    components.clone()
                }
                _=>{
                    unreachable!();
                }
            };
            for i in 0..nodes.len(){
                let tmp = IrOperand::FieldAccess { base: Box::new(out.clone()), name: comps[i].0.clone().into() };
                val_stack.push(IrInstr::Mov { left: tmp, right: ops[i].clone(), vtype: comps[i].1.clone() })
            }
            return Some(out); 
        }
        AstNode::If {
            condition,
            thing_to_do,
            r#else,
        } => {
            let cond = compile_ast_node_to_ir(&condition, val_stack, variable_counter, stack_ptr, pop_table, name_table, functions, types, label_counter,None)?;
            let base_label = *label_counter;
            let else_label = *label_counter+1;
            let end_label = *label_counter+2;
            *label_counter += 3;
            val_stack.push(IrInstr::CondGoto { cond: cond, target: format!("L{}",base_label) });
            val_stack.push(IrInstr::Goto { target: format!("L{}",else_label) });
            val_stack.push(IrInstr::Label { name: format!("L{}",base_label) });
            val_stack.push(IrInstr::BeginScope);
            let mut body_pop_table = vec![];
            for i in thing_to_do{
                compile_ast_node_to_ir(i, val_stack, variable_counter, stack_ptr, &mut body_pop_table, name_table, functions, types, label_counter,None);
            }
            body_pop_table.reverse();
            for i in body_pop_table{
                val_stack.push(IrInstr::Pop {vtype:i });
            }
            val_stack.push(IrInstr::EndScope);
            val_stack.push(IrInstr::Goto{target:format!("L{}",end_label)});
            val_stack.push(IrInstr::Label { name: format!("L{}",else_label )});
            if r#else.is_some(){
                val_stack.push(IrInstr::BeginScope);
                let elblk = r#else.as_ref().expect("is some");
                let mut pop_stack = vec![];
                for i in elblk{
                    compile_ast_node_to_ir(i, val_stack, variable_counter, stack_ptr, &mut pop_stack, name_table, functions, types, label_counter,None);
                }
                pop_stack.reverse();
                for i in pop_stack{
                    val_stack.push(IrInstr::Pop { vtype: i });
                }
                val_stack.push(IrInstr::EndScope);
            }
     
            val_stack.push(IrInstr::Label { name: format!("L{}",end_label)});
        }
        AstNode::ForLoop {
            variable,
            condition,
            post_op,
            body,
        } => {
            let base = *label_counter;
            let end = *label_counter+1;
            let lbody = *label_counter + 2;
            *label_counter += 3;
            val_stack.push(IrInstr::BeginScope); 
            let _ = compile_ast_node_to_ir(variable, val_stack, variable_counter, stack_ptr, pop_table, name_table, functions, types, label_counter,None);
            val_stack.push(IrInstr::Label { name: format!("L{}", base )}); 
            val_stack.push(IrInstr::BeginScope);    
            let mut loop_pop_table = vec![];
            let tmp = compile_ast_node_to_ir(condition, val_stack, variable_counter, stack_ptr, pop_table, name_table, functions, types, label_counter,None)?; 
            val_stack.push(IrInstr::CondGoto { cond: tmp, target: format!("L{}",lbody)});
            val_stack.push(IrInstr::Goto { target: format!("L{}",end) });
            val_stack.push(IrInstr::Label { name: format!("L{}",lbody) });
            val_stack.push(IrInstr::BeginScope);
            for i in body{
                compile_ast_node_to_ir(i, val_stack, variable_counter, stack_ptr,&mut loop_pop_table, name_table, functions, types, label_counter,None);
            } 
            compile_ast_node_to_ir(post_op, val_stack, variable_counter, stack_ptr, &mut loop_pop_table, name_table, functions, types, label_counter,None);
            loop_pop_table.reverse();
            for i in loop_pop_table{
                val_stack.push(IrInstr::Pop { vtype: i });
            }
            val_stack.push(IrInstr::EndScope);
            val_stack.push(IrInstr::EndScope);
            val_stack.push(IrInstr::Goto { target: format!("L{}",base) });
            val_stack.push(IrInstr::Label { name: format!("L{}",end) });
            val_stack.push(IrInstr::EndScope);
        }
        AstNode::Loop { condition, body } => {
            let base = *label_counter;
            let end = *label_counter+1;
            let lbody = *label_counter + 2;
            *label_counter += 3;
            val_stack.push(IrInstr::Label { name: format!("L{}", base )}); 
            val_stack.push(IrInstr::BeginScope);   
            let tmp = compile_ast_node_to_ir(condition, val_stack, variable_counter, stack_ptr, pop_table, name_table, functions, types, label_counter,None)?; 
            val_stack.push(IrInstr::CondGoto { cond: tmp, target: format!("L{}",lbody)});
            val_stack.push(IrInstr::Goto { target: format!("L{}",end) });
            val_stack.push(IrInstr::Label { name: format!("L{}",lbody) });
            val_stack.push(IrInstr::BeginScope);
            let mut loop_pop_table = vec![];
            for i in body{
                compile_ast_node_to_ir(i, val_stack, variable_counter, stack_ptr, &mut loop_pop_table, name_table, functions, types, label_counter,None);
            }
            loop_pop_table.reverse();
            for i in loop_pop_table{
                val_stack.push(IrInstr::Pop { vtype: i});
            }
            val_stack.push(IrInstr::EndScope);
            val_stack.push(IrInstr::EndScope);
            val_stack.push(IrInstr::Goto { target: format!("L{}",base) });
            val_stack.push(IrInstr::Label { name: format!("L{}",end) });
        }
        AstNode::Return { body } => {
            let to_return = compile_ast_node_to_ir(
                body,
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
                name_table,
                functions,
                types,
                label_counter,
                None,
            )
            .expect("should return"); 
            val_stack.push(IrInstr::Ret { to_return});
        }
        AstNode::OperatorMake { vtype, size } => {
            let out = stack_push(
                Type::SliceT {
                    ptr_type: Rc::new(vtype.clone()),
                },
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
            );
            let mult = AstNode::Mult {
                left: size.clone(),
                right: Box::new(AstNode::IntLiteral {
                    value: vtype.get_size_bytes() as i64,
                }),
                data: None,
            };
            let ln = compile_ast_node_to_ir(
                &mult,
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
                name_table,
                functions,
                types,
                label_counter,
                None,
            );
            val_stack.push(IrInstr::CallWithRet {
                target: out.clone(),
                func_name: "gc_alloc".to_owned(),
                args: vec![ln?],
                vtype: Type::SliceT {
                    ptr_type: Rc::new(vtype.clone()),
                },
                stack_ptr_when_called:*stack_ptr,
            });
            return Some(out);
        }
        AstNode::OperatorNew { vtype } => {
            let out = stack_push(
                Type::PointerT {
                    ptr_type: Rc::new(vtype.clone()),
                },
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
            );
            let ln = IrOperand::IntLiteral {
                value: vtype.get_size_bytes() as i64,
            };
            val_stack.push(IrInstr::CallWithRet {
                target: out.clone(),
                func_name: "gc_alloc".to_owned(),
                args: vec![ln],
                vtype: Type::SliceT {
                    ptr_type: Rc::new(vtype.clone()),
                },
                stack_ptr_when_called:*stack_ptr,
            });
            return Some(out);
        }
        _ => {
            unreachable!();
        }
    }
    None
}
pub fn compile_function_to_ir(
    func: &Function,
    functions: &HashMap<String, FunctionTable>,
    types: &HashMap<String, Type>,
) -> Vec<IrInstr> {
    let mut out = vec![];
    let mut variable_counter = 0;
    let mut stack_ptr = 8;
    let mut pop_table = vec![];
    let mut name_table = vec![HashMap::new()];
    let mut label_counter = 0;
    for i in 0..func.args.len() {
        let op = IrOperand::StacKOperand { var_idx: variable_counter, 
            name:("user_".to_owned()+&func.arg_names[i]).into(), 
            stack_offset: stack_ptr, vtype:func.args[i].clone() };
        stack_ptr += func.args[i].get_size_bytes();
        variable_counter += 1;
        let name = func.arg_names[i].clone();
        name_table[0].insert(name, op);
        pop_table.push(func.args[i].clone());
    }

    for i in &func.program {
        let _ = compile_ast_node_to_ir(
            i,
            &mut out,
            &mut variable_counter,
            &mut stack_ptr,
            &mut pop_table,
            &mut name_table,
            functions,
            types,
            &mut label_counter,
            None,
        );
    }
    pop_table.reverse();
    for i in pop_table{
        out.push(IrInstr::Pop { vtype:i });
    }
    //out.push(IrInstr::EndScope);
    return out;
}
