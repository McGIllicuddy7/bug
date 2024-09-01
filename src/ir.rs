use crate::{
    get_function_by_args, name_mangle_function, name_mangle_type, Function, FunctionTable,
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
pub fn compile_ir_op_to_c(op: &IrOperand) -> String {
    match op {
        IrOperand::StacKOperand {
            var_idx: _,
            name,
            stack_offset: _,
            vtype: _,
        } => {
            return name.to_owned();
        }
        IrOperand::Name { name, vtype: _ } => {
            return name.to_owned();
        }
        IrOperand::Deref { to_deref } => {
            return "*".to_owned() + &compile_ir_op_to_c(to_deref);
        }
        IrOperand::TakeRef { to_ref } => {
            return "&".to_owned() + &compile_ir_op_to_c(to_ref);
        }
        IrOperand::FunctionArg { name, vtype: _ } => {
            return name.to_owned();
        }
        IrOperand::StringLiteral { value } => {
            return value.to_owned();
        }
        IrOperand::IntLiteral { value } => {
            return format!("{value}");
        }
        IrOperand::FloatLiteral { value } => {
            return format!("{value}");
        }
        IrOperand::FieldAccess { base, name } => {
            return compile_ir_op_to_c(base) + &format!(".{}", &name);
        }
        IrOperand::CharLiteral { value } => {
            todo!();
        }
        IrOperand::ArrayAccess { base, value } => {
            let base = compile_ir_op_to_c(base);
            return format!("{}.start[{}]", base, compile_ir_op_to_c(value));
        }
        IrOperand::ArrayLiteral { vtype, values } => {
            let base = name_mangle_type(vtype);
            let names: Vec<String> = values.iter().map(|i| compile_ir_op_to_c(i)).collect();
            let mut base = format!("({}[]){{", base);
            for i in &names {
                base += i;
            }
            base += "}";
            return base;
        }
        IrOperand::StructLiteral { vtype, values } => {
            let base = name_mangle_type(vtype);
            let names: Vec<String> = values.iter().map(|i| compile_ir_op_to_c(i)).collect();
            let mut base = format!("({}){{", base);
            for i in &names {
                base += i;
            }
            base += "}";
            return base;
        }
    }
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
pub fn compile_ir_instr_to_c(instr: &IrInstr) -> String {
    match instr {
        IrInstr::VariableDeclaration { name, vtype } => {
            todo!();
        }
        IrInstr::Mov { left, right, vtype } => {
            todo!();
        }
        IrInstr::Label { name } => {
            todo!();
        }
        IrInstr::Goto { target } => {
            todo!();
        }
        IrInstr::CondGoto { cond, target } => {
            todo!();
        }
        IrInstr::Add {
            target,
            left,
            right,
            vtype,
        } => {
            todo!();
        }
        IrInstr::Sub {
            target,
            left,
            right,
            vtype,
        } => {
            todo!();
        }
        IrInstr::Mul {
            target,
            left,
            right,
            vtype,
        } => {
            todo!();
        }
        IrInstr::Div {
            target,
            left,
            right,
            vtype,
        } => {
            todo!();
        }
        IrInstr::And {
            target,
            left,
            right,
            vtype,
        } => {
            todo!();
        }
        IrInstr::Or {
            target,
            left,
            right,
            vtype,
        } => {
            todo!();
        }
        IrInstr::Equals {
            target,
            left,
            right,
            vtype,
        } => {
            todo!();
        }
        IrInstr::NotEquals {
            target,
            left,
            right,
            vtype,
        } => {
            todo!();
        }
        IrInstr::GreaterThan {
            target,
            left,
            right,
            vtype,
        } => {
            todo!();
        }
        IrInstr::GreaterThanOrEq {
            target,
            left,
            right,
            vtype,
        } => {
            todo!();
        }
        IrInstr::LessThan {
            target,
            left,
            right,
            vtype,
        } => {
            todo!();
        }
        IrInstr::LessThanOrEq {
            target,
            left,
            right,
            vtype,
        } => {
            todo!();
        }
        IrInstr::Not {
            target,
            value,
            vtype,
        } => {
            todo!();
        }
        IrInstr::Call { func_name, args } => {
            todo!();
        }
        IrInstr::CallWithRet {
            target,
            func_name,
            args,
            vtype,
        } => {
            todo!();
        }
        IrInstr::Ret { to_return } => {
            todo!();
        }
        IrInstr::Push { vtype, val_idx } => {
            match vtype{
                Type::BoolT=>{

                }
                Type::FloatT=>{ }
            };
        }
        IrInstr::Pop { vtype } => {
            return "".to_owned();
        }
        IrInstr::BeginScope => {
            return "{\n".to_owned();
        }
        IrInstr::EndScope => {
            return "\n}\n".to_owned();
        }
    }
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
    println!("{:#?}", node);
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
        AstNode::Not { value, data } => {
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
            let op = stack_push(
                var_type.clone(),
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
            );
            val_stack.push(IrInstr::VariableDeclaration {
                name: name.clone(),
                vtype: var_type.clone(),
            });
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
            let fname = name_mangle_function(&func, "");
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
            return Some(IrOperand::TakeRef {
                to_ref: Box::new(compile_ast_node_to_ir(
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
            let cond = compile_ast_node_to_ir(
                condition,
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
                name_table,
                functions,
                types,
                label_counter,
            )
            .expect("must return");
            val_stack.push(IrInstr::CondGoto {
                cond,
                target: format!("L{}", label_counter),
            });
            val_stack.push(IrInstr::Goto {
                target: format!("L{}", *label_counter + 1),
            });
            val_stack.push(IrInstr::Label {
                name: format!("L{}", label_counter),
            });
            let else_block = *label_counter + 1;
            let end_block = *label_counter + 2;
            *label_counter += 3;
            let mut to_pop_stack = vec![];
            val_stack.push(IrInstr::BeginScope);
            for i in thing_to_do {
                compile_ast_node_to_ir(
                    i,
                    val_stack,
                    variable_counter,
                    stack_ptr,
                    &mut to_pop_stack,
                    name_table,
                    functions,
                    types,
                    label_counter,
                );
            }
            for i in to_pop_stack {
                *variable_counter -= 1;
                *stack_ptr -= i.get_size_bytes();
                val_stack.push(IrInstr::Pop { vtype: i });
            }
            val_stack.push(IrInstr::EndScope);
            if r#else.is_some() {
                val_stack.push(IrInstr::Goto {
                    target: format!("L{}", end_block),
                });
                let r#else = r#else.as_ref().expect("this is some");
                val_stack.push(IrInstr::Label {
                    name: format!("L{}", else_block),
                });
                val_stack.push(IrInstr::BeginScope);
                let mut to_pop_stack = vec![];
                for i in r#else {
                    compile_ast_node_to_ir(
                        i,
                        val_stack,
                        variable_counter,
                        stack_ptr,
                        &mut to_pop_stack,
                        name_table,
                        functions,
                        types,
                        label_counter,
                    );
                }
                to_pop_stack.reverse();
                for i in to_pop_stack {
                    *variable_counter -= 1;
                    *stack_ptr -= i.get_size_bytes();
                    val_stack.push(IrInstr::Pop { vtype: i });
                }
                val_stack.push(IrInstr::EndScope);
                val_stack.push(IrInstr::Label {
                    name: format!("L{}", end_block),
                })
            } else {
                val_stack.push(IrInstr::Label {
                    name: format!("L{}", else_block),
                });
            }
        }
        AstNode::ForLoop {
            variable,
            condition,
            post_op,
            body,
        } => {
            let mut var_pop_stack = vec![];
            val_stack.push(IrInstr::BeginScope);
            compile_ast_node_to_ir(
                variable,
                val_stack,
                variable_counter,
                stack_ptr,
                &mut var_pop_stack,
                name_table,
                functions,
                types,
                label_counter,
            );
            val_stack.push(IrInstr::BeginScope);
            let cond = compile_ast_node_to_ir(
                &AstNode::Not {
                    value: Box::new(*condition.clone()),
                    data: None,
                },
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
                name_table,
                functions,
                types,
                label_counter,
            );
            let start_label = *label_counter;
            val_stack.push(IrInstr::Label {
                name: format!("L{}", *label_counter),
            });
            *label_counter += 1;
            val_stack.push(IrInstr::CondGoto {
                cond: cond?,
                target: format!("L{}", *label_counter),
            });
            let end_label = *label_counter;
            *label_counter += 1;
            let mut to_pop_stack = vec![];
            for i in body {
                compile_ast_node_to_ir(
                    i,
                    val_stack,
                    variable_counter,
                    stack_ptr,
                    &mut to_pop_stack,
                    name_table,
                    functions,
                    types,
                    label_counter,
                );
            }
            compile_ast_node_to_ir(
                post_op,
                val_stack,
                variable_counter,
                stack_ptr,
                &mut to_pop_stack,
                name_table,
                functions,
                types,
                label_counter,
            );
            to_pop_stack.reverse();
            for i in to_pop_stack {
                *variable_counter -= 1;
                *stack_ptr -= i.get_size_bytes();
                val_stack.push(IrInstr::Pop { vtype: i });
            }
            val_stack.push(IrInstr::EndScope);
            val_stack.push(IrInstr::Goto {
                target: format!("L{}", start_label),
            });
            val_stack.push(IrInstr::Label {
                name: format!("L{}", end_label),
            });
            var_pop_stack.reverse();
            for i in var_pop_stack {
                *variable_counter -= 1;
                *stack_ptr -= i.get_size_bytes();
                val_stack.push(IrInstr::Pop { vtype: i });
            }
            val_stack.push(IrInstr::EndScope);
        }
        AstNode::Loop { condition, body } => {
            val_stack.push(IrInstr::BeginScope);
            let cond = compile_ast_node_to_ir(
                &AstNode::Not {
                    value: Box::new(*condition.clone()),
                    data: None,
                },
                val_stack,
                variable_counter,
                stack_ptr,
                pop_table,
                name_table,
                functions,
                types,
                label_counter,
            );
            let start_label = *label_counter;
            val_stack.push(IrInstr::Label {
                name: format!("L{}", *label_counter),
            });
            *label_counter += 1;
            val_stack.push(IrInstr::CondGoto {
                cond: cond?,
                target: format!("L{}", *label_counter),
            });
            let end_label = *label_counter;
            *label_counter += 1;
            let mut to_pop_stack = vec![];
            for i in body {
                compile_ast_node_to_ir(
                    i,
                    val_stack,
                    variable_counter,
                    stack_ptr,
                    &mut to_pop_stack,
                    name_table,
                    functions,
                    types,
                    label_counter,
                );
            }
            to_pop_stack.reverse();
            for i in to_pop_stack {
                *variable_counter -= 1;
                *stack_ptr -= i.get_size_bytes();
                val_stack.push(IrInstr::Pop { vtype: i });
            }
            val_stack.push(IrInstr::EndScope);
            val_stack.push(IrInstr::Goto {
                target: format!("L{}", start_label),
            });
            val_stack.push(IrInstr::Label {
                name: format!("L{}", end_label),
            });
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
                Type::SliceT {
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
            name: func.arg_names[i].clone(),
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
