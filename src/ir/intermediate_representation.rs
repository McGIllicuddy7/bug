use crate::{
    get_function_by_args, Function, FunctionTable
};
use crate::{AstNode, Type};
use std::collections::HashMap;
#[allow(unused)]
#[derive(Clone, Debug)]
pub enum IrOperand {
    StacKOperand {
        var_idx: usize,
        name: String,
        stack_offset: usize,
        vtype: Type,
    },
    Name {
        name: String,
        vtype: Type,
    },
    Deref {
        to_deref: Box<IrOperand>,
    },
    TakeRef {
        to_ref: Box<IrOperand>,
    },
    ArrayLiteral {
        vtype: Type,
        values: Vec<IrOperand>,
    },
    StructLiteral {
        vtype: Type,
        values: Vec<IrOperand>,
    },
    StringLiteral {
        value: String,
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
        name: String,
    },
    ArrayAccess {
        base: Box<IrOperand>,
        value: Box<IrOperand>,
    },
    FunctionArg {
        name: String,
        vtype: Type,
    },
}

#[allow(unused)]
#[derive(Clone, Debug)]
pub enum IrInstr {
    VariableDeclaration {
        name: String,
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
    },
    CallWithRet {
        target: IrOperand,
        func_name: String,
        args: Vec<IrOperand>,
        vtype: Type,
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
        name: format!("tmp{}", *variable_counter),
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
            );
            val_stack.push(IrInstr::Mov {
                left: lv?,
                right: rv?,
                vtype: left.get_type(functions, types).expect("bruh"),
            });
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
            );
            let target = stack_push(
                left.get_type(functions, types).expect("please work"),
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
            );
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
            );
            let target = stack_push(
                left.get_type(functions, types).expect("please work"),
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
            );
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
            );
            let target = stack_push(
                left.get_type(functions, types).expect("please work"),
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
            );
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
            );
            let target = stack_push(
                left.get_type(functions, types).expect("please work"),
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
            );
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
            );
            let target = stack_push(
                left.get_type(functions, types).expect("please work"),
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
            );
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
            );
            let target = stack_push(
                left.get_type(functions, types).expect("please work"),
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
            );
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
            );
            let target = stack_push(
                left.get_type(functions, types).expect("please work"),
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
            );
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
            );
            let target = stack_push(
                left.get_type(functions, types).expect("please work"),
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
            );
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
            );
            let target = stack_push(
                left.get_type(functions, types).expect("please work"),
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
            );
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
            );
            let target = stack_push(
                left.get_type(functions, types).expect("please work"),
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
            );
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
            );
            let target = stack_push(
                left.get_type(functions, types).expect("please work"),
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
            );
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
            )?;
            let target = stack_push(
                value.get_type(functions, types).expect("please work"),
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
            );
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
            );
            let target = stack_push(
                left.get_type(functions, types).expect("please work"),
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
            );
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
            let op = IrOperand::StacKOperand { var_idx:*variable_counter, name: "user_".to_owned()+&name, stack_offset: *stack_ptr, vtype: var_type.clone() };
            val_stack.push(IrInstr::VariableDeclaration {
                name: "user_".to_owned()+&name,
                vtype: var_type.clone(),
            });
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
                    )
                    .expect("should return")
                })
                .collect();
            let fname = func.name;
            if func.return_type == Type::VoidT {
                let tmp = IrInstr::Call {
                    func_name: fname,
                    args: f_args,
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
            )
            .expect("should return");
            return Some(IrOperand::FieldAccess {
                base: Box::new(base_ir),
                name: field_name.clone(),
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
                value: value.clone(),
            };
            let ln = IrOperand::IntLiteral {
                value: value.len() as i64 - 2,
            };
            val_stack.push(IrInstr::CallWithRet {
                target: out.clone(),
                func_name: "make_string_from".to_owned(),
                args: vec![tmp, ln],
                vtype: Type::StringT,
            });
            return Some(out);
        }
        AstNode::ArrayLiteral { nodes } => {
            let vtype = Type::ArrayT {
                size: nodes.len(),
                array_type: Box::new(nodes[0].get_type(functions, types).expect("must have type")),
            };
            return Some(IrOperand::ArrayLiteral {
                vtype,
                values: nodes
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
                        )
                        .expect("must return")
                    })
                    .collect(),
            });
        }
        AstNode::StructLiteral { vtype, nodes } => {
            return Some(IrOperand::StructLiteral {
                vtype: vtype.clone(),
                values: nodes
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
                        )
                        .expect("must return")
                    })
                    .collect(),
            });
        }
        AstNode::If {
            condition,
            thing_to_do,
            r#else,
        } => {
            todo!();
        }
        AstNode::ForLoop {
            variable,
            condition,
            post_op,
            body,
        } => {
            todo!();
        }
        AstNode::Loop { condition, body } => {
            let base = *label_counter;
            let end = *label_counter+1;
            let lbody = *label_counter + 2;
            *label_counter += 3;
            val_stack.push(IrInstr::Label { name: format!("L{}", base )}); 
            val_stack.push(IrInstr::BeginScope);   
            let tmp = compile_ast_node_to_ir(condition, val_stack, variable_counter, stack_ptr, pop_table, name_table, functions, types, label_counter)?; 
            val_stack.push(IrInstr::CondGoto { cond: tmp, target: format!("L{}",lbody)});
            val_stack.push(IrInstr::Goto { target: format!("L{}",end) });
            val_stack.push(IrInstr::Label { name: format!("L{}",lbody) });
            for i in body{
                compile_ast_node_to_ir(i, val_stack, variable_counter, stack_ptr, pop_table, name_table, functions, types, label_counter);
            }
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
            )
            .expect("should return");
            val_stack.push(IrInstr::Ret { to_return });
        }
        AstNode::OperatorMake { vtype, size } => {
            let out = stack_push(
                Type::SliceT {
                    ptr_type: Box::new(vtype.clone()),
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
            );
            val_stack.push(IrInstr::CallWithRet {
                target: out.clone(),
                func_name: "gc_alloc".to_owned(),
                args: vec![ln?],
                vtype: Type::SliceT {
                    ptr_type: Box::new(vtype.clone()),
                },
            });
            return Some(out);
        }
        AstNode::OperatorNew { vtype } => {
            let out = stack_push(
                Type::PointerT {
                    ptr_type: Box::new(vtype.clone()),
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
                    ptr_type: Box::new(vtype.clone()),
                },
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
    let mut stack_ptr = 0;
    let mut pop_table = vec![];
    let mut name_table = vec![HashMap::new()];
    let mut label_counter = 0;
    for i in 0..func.args.len() {
        let op = IrOperand::FunctionArg {
            name: "user_".to_owned()+&func.arg_names[i],
            vtype: func.args[i].clone(),
        };
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
        );
    }
    return out;
}
