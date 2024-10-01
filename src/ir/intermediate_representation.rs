//know hannah and bruce
use crate::{get_function_by_args, Function, FunctionTable};
use crate::{AstNode, Type};
use std::collections::{HashMap, HashSet};
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
impl IrOperand {
    pub fn get_type(&self) -> Type {
        match self {
            Self::IntLiteral { value: _ } => {
                return Type::IntegerT;
            }
            Self::FloatLiteral { value: _ } => {
                return Type::FloatT;
            }
            Self::ArrayAccess { base, value: _ } => {
                return base.get_type().get_array_type().expect("");
            }
            Self::CharLiteral { value: _ } => {
                return Type::CharT;
            }
            Self::Deref { to_deref } => match to_deref.get_type() {
                Type::PointerT { ptr_type } => {
                    return ptr_type.as_ref().clone();
                }
                _ => {
                    unreachable!();
                }
            },
            Self::TakeRef { to_ref } => {
                return Type::PointerT {
                    ptr_type: Rc::new(to_ref.get_type().clone()),
                };
            }
            Self::Name { name: _, vtype } => {
                return vtype.clone();
            }
            Self::StacKOperand {
                var_idx: _,
                name: _,
                stack_offset: _,
                vtype,
            } => {
                return vtype.clone();
            }
            Self::FieldAccess { base, name } => {
                let fname = name;
                let bs = base.get_type();
                match &bs {
                    Type::StructT {
                        name: _,
                        components,
                    } => {
                        for i in components {
                            if &i.0 == fname.as_ref() {
                                return i.1.clone();
                            }
                        }
                        println!("struct:{:#?}, name:{}", bs, fname.as_ref());
                        unreachable!();
                    }
                    Type::SliceT { ptr_type }=>{
                        return Type::PointerT { ptr_type: ptr_type.clone() };
                    }
                    _ => {
                        unreachable!();
                    }
                }
            }
            Self::StringLiteral { value: _ } => {
                return Type::PointerT {
                    ptr_type: Rc::new(Type::CharT),
                };
            }
        }
        //todo!();
    }
}
#[allow(unused)]
#[derive(Clone, Debug)]
pub enum IrInstr {
    VariableDeclaration {
        name: Rc<str>,
        vtype: Type,
        stack_offset: usize,
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
        stack_ptr_when_called: usize,
    },
    CallWithRet {
        target: IrOperand,
        func_name: String,
        args: Vec<IrOperand>,
        vtype: Type,
        stack_ptr_when_called: usize,
    },
    Ret {
        to_return: IrOperand,
        stack_ptr: usize,
    },
    Push {
        vtype: Type,
        val_idx: usize,
        stack_offset_of_value: usize,
    },
    Pop {
        vtype: Type,
    },
    BeginScope {
        stack_ptr: usize,
    },
    EndScope {
        stack_ptr: usize,
    },
    BeginGcFrame,
    EndGcFrame,
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
        stack_offset_of_value: stack_ptr.clone(),
    });
    pop_table.push(vtype.clone());
    *variable_counter += 1;
    *stack_ptr += vtype.get_size_bytes();
    return out;
}
pub struct IrCompState<'a> {
    val_stack: &'a mut Vec<IrInstr>,
    variable_counter: &'a mut usize,
    stack_ptr: &'a mut usize,
    pop_table: &'a mut Vec<Type>,
    name_table: &'a mut Vec<HashMap<String, IrOperand>>,
    label_counter: &'a mut usize,
}

pub fn compile_ast_node_to_ir(
    node: &AstNode,
    state: &mut IrCompState,
    functions: &HashMap<String, FunctionTable>,
    types: &HashMap<String, Type>,
    target: Option<IrOperand>,
) -> Option<IrOperand> {
    //println!("{:#?}", node);
    match node {
        AstNode::Assignment {
            left,
            right,
            data: _,
        } => {
            let lv = compile_ast_node_to_ir(left, state, functions, types, None);
            let rv = compile_ast_node_to_ir(right, state, functions, types, lv.clone());
            match right.as_ref() {
                AstNode::Add {
                    left: _,
                    right: _,
                    data: _,
                }
                | AstNode::Sub {
                    left: _,
                    right: _,
                    data: _,
                }
                | AstNode::Mult {
                    left: _,
                    right: _,
                    data: _,
                }
                | AstNode::Div {
                    left: _,
                    right: _,
                    data: _,
                }
                | AstNode::And {
                    left: _,
                    right: _,
                    data: _,
                }
                | AstNode::Or {
                    left: _,
                    right: _,
                    data: _,
                }
                | AstNode::GreaterOrEq {
                    left: _,
                    right: _,
                    data: _,
                }
                | AstNode::LessOrEq {
                    left: _,
                    right: _,
                    data: _,
                }
                | AstNode::GreaterThan {
                    left: _,
                    right: _,
                    data: _,
                }
                | AstNode::LessThan {
                    left: _,
                    right: _,
                    data: _,
                }
                | AstNode::Not { value: _, data: _ } => {}
                AstNode::OperatorMake { vtype, size }=>{
                    state.val_stack.push(IrInstr::Mov { left:IrOperand::FieldAccess { base: lv.clone().expect("msg").into(), name: "start".into() }, right: rv.expect("").clone(), vtype:vtype.clone() });
                    let sv = compile_ast_node_to_ir(size, state, functions, types, target).expect("");
                    state.val_stack.push(IrInstr::Mov { left:IrOperand::FieldAccess { base: lv.clone().expect("msg").into(), name: "len".into() }, right: sv.clone(), vtype:vtype.clone() });
                }
                _ => {
                    state.val_stack.push(IrInstr::Mov {
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
            let lv = compile_ast_node_to_ir(left, state, functions, types, None);
            let rv = compile_ast_node_to_ir(right, state, functions, types, None);
            let target = if target.is_some() {
                target.unwrap()
            } else {
                stack_push(
                    left.get_type(functions, types).expect("please work"),
                    state.val_stack,
                    state.variable_counter,
                    state.stack_ptr,
                    state.pop_table,
                )
            };
            state.val_stack.push(IrInstr::Add {
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
            let lv = compile_ast_node_to_ir(left, state, functions, types, None);
            let rv = compile_ast_node_to_ir(right, state, functions, types, None);
            let target = if target.is_some() {
                target.unwrap()
            } else {
                stack_push(
                    left.get_type(functions, types).expect("please work"),
                    state.val_stack,
                    state.variable_counter,
                    state.stack_ptr,
                    state.pop_table,
                )
            };
            state.val_stack.push(IrInstr::Sub {
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
            let lv = compile_ast_node_to_ir(left, state, functions, types, None);
            let rv = compile_ast_node_to_ir(right, state, functions, types, None);
            let target = if target.is_some() {
                target.unwrap()
            } else {
                stack_push(
                    left.get_type(functions, types).expect("please work"),
                    state.val_stack,
                    state.variable_counter,
                    state.stack_ptr,
                    state.pop_table,
                )
            };
            state.val_stack.push(IrInstr::Mul {
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
            let lv = compile_ast_node_to_ir(left, state, functions, types, None);
            let rv = compile_ast_node_to_ir(right, state, functions, types, None);
            let target = if target.is_some() {
                target.unwrap()
            } else {
                stack_push(
                    left.get_type(functions, types).expect("please work"),
                    state.val_stack,
                    state.variable_counter,
                    state.stack_ptr,
                    state.pop_table,
                )
            };
            state.val_stack.push(IrInstr::Div {
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
            let lv = compile_ast_node_to_ir(left, state, functions, types, None);
            let rv = compile_ast_node_to_ir(right, state, functions, types, None);
            let target = if target.is_some() {
                target.unwrap()
            } else {
                stack_push(
                    left.get_type(functions, types).expect("please work"),
                    state.val_stack,
                    state.variable_counter,
                    state.stack_ptr,
                    state.pop_table,
                )
            };
            state.val_stack.push(IrInstr::And {
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
            let lv = compile_ast_node_to_ir(left, state, functions, types, None);
            let rv = compile_ast_node_to_ir(right, state, functions, types, None);
            let target = if target.is_some() {
                target.unwrap()
            } else {
                stack_push(
                    left.get_type(functions, types).expect("please work"),
                    state.val_stack,
                    state.variable_counter,
                    state.stack_ptr,
                    state.pop_table,
                )
            };
            state.val_stack.push(IrInstr::Add {
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
            let lv = compile_ast_node_to_ir(left, state, functions, types, None);
            let rv = compile_ast_node_to_ir(right, state, functions, types, None);
            let target = if target.is_some() {
                target.unwrap()
            } else {
                stack_push(
                    left.get_type(functions, types).expect("please work"),
                    state.val_stack,
                    state.variable_counter,
                    state.stack_ptr,
                    state.pop_table,
                )
            };
            state.val_stack.push(IrInstr::Equals {
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
            let lv = compile_ast_node_to_ir(left, state, functions, types, None);
            let rv = compile_ast_node_to_ir(right, state, functions, types, None);
            let target = if target.is_some() {
                target.unwrap()
            } else {
                stack_push(
                    left.get_type(functions, types).expect("please work"),
                    state.val_stack,
                    state.variable_counter,
                    state.stack_ptr,
                    state.pop_table,
                )
            };
            state.val_stack.push(IrInstr::GreaterThan {
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
            let lv = compile_ast_node_to_ir(left, state, functions, types, None);
            let rv = compile_ast_node_to_ir(right, state, functions, types, None);
            let target = if target.is_some() {
                target.unwrap()
            } else {
                stack_push(
                    left.get_type(functions, types).expect("please work"),
                    state.val_stack,
                    state.variable_counter,
                    state.stack_ptr,
                    state.pop_table,
                )
            };
            state.val_stack.push(IrInstr::LessThan {
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
            let lv = compile_ast_node_to_ir(left, state, functions, types, None);
            let rv = compile_ast_node_to_ir(right, state, functions, types, None);
            let target = if target.is_some() {
                target.unwrap()
            } else {
                stack_push(
                    left.get_type(functions, types).expect("please work"),
                    state.val_stack,
                    state.variable_counter,
                    state.stack_ptr,
                    state.pop_table,
                )
            };
            state.val_stack.push(IrInstr::GreaterThanOrEq {
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
            let lv = compile_ast_node_to_ir(left, state, functions, types, None);
            let rv = compile_ast_node_to_ir(right, state, functions, types, None);
            let target = if target.is_some() {
                target.unwrap()
            } else {
                stack_push(
                    left.get_type(functions, types).expect("please work"),
                    state.val_stack,
                    state.variable_counter,
                    state.stack_ptr,
                    state.pop_table,
                )
            };
            state.val_stack.push(IrInstr::LessThanOrEq {
                target: target.clone(),
                left: lv?,
                right: rv?,
                vtype: left.get_type(functions, types).expect("bruh"),
            });
            return Some(target);
        }
        AstNode::Not { value, data: _ } => {
            let v = compile_ast_node_to_ir(value, state, functions, types, None)?;
            let target = if target.is_some() {
                target.unwrap()
            } else {
                stack_push(
                    value.get_type(functions, types).expect("please work"),
                    state.val_stack,
                    state.variable_counter,
                    state.stack_ptr,
                    state.pop_table,
                )
            };
            state.val_stack.push(IrInstr::Not {
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
            let lv = compile_ast_node_to_ir(left, state, functions, types, None);
            let rv = compile_ast_node_to_ir(right, state, functions, types, None);
            let target = if target.is_some() {
                target.unwrap()
            } else {
                stack_push(
                    left.get_type(functions, types).expect("please work"),
                    state.val_stack,
                    state.variable_counter,
                    state.stack_ptr,
                    state.pop_table,
                )
            };
            state.val_stack.push(IrInstr::NotEquals {
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
            let op = IrOperand::StacKOperand {
                var_idx: *state.variable_counter,
                name: ("user_".to_owned() + &name).into(),
                stack_offset: *state.stack_ptr,
                vtype: var_type.clone(),
            };
            state.val_stack.push(IrInstr::VariableDeclaration {
                name: ("user_".to_owned() + &name).into(),
                vtype: var_type.clone(),
                stack_offset: state.stack_ptr.clone(),
            });
            state.pop_table.push(var_type.clone());
            *state.variable_counter += 1;
            *state.stack_ptr += var_type.get_size_bytes();
            state
                .name_table
                .last_mut()
                .expect("must exist")
                .insert(name.clone(), op);
            if value_assigned.is_some() {
                let _ = compile_ast_node_to_ir(
                    value_assigned.as_ref().expect("exists"),
                    state,
                    functions,
                    types,
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
                    compile_ast_node_to_ir(i, state, functions, types, None).expect("should return")
                })
                .collect();
            let fname = func.name;
            if func.return_type == Type::VoidT {
                let tmp = IrInstr::Call {
                    func_name: fname,
                    args: f_args,
                    stack_ptr_when_called: *state.stack_ptr,
                };
                state.val_stack.push(tmp);
            } else {
                let return_v = stack_push(
                    func.return_type.clone(),
                    state.val_stack,
                    state.variable_counter,
                    state.stack_ptr,
                    state.pop_table,
                );
                let tmp = IrInstr::CallWithRet {
                    target: return_v.clone(),
                    func_name: fname,
                    args: f_args,
                    vtype: func.return_type,
                    stack_ptr_when_called: *state.stack_ptr,
                };
                state.val_stack.push(tmp);
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
            for i in state.name_table.clone() {
                if i.contains_key(name) {
                    return Some(i[name].clone());
                }
            }
        }
        AstNode::FieldUsage { base, field_name } => {
            let base_ir =
                compile_ast_node_to_ir(base, state, functions, types, None).expect("should return");
            return Some(IrOperand::FieldAccess {
                base: Box::new(base_ir),
                name: field_name.clone().into(),
            });
        }
        AstNode::ArrayAccess { variable, index } => {
            let var_ir = compile_ast_node_to_ir(variable, state, functions, types, None)
                .expect("should return");
            let idx_ir = compile_ast_node_to_ir(index, state, functions, types, None)
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
                    state,
                    functions,
                    types,
                    None,
                )?),
            });
        }
        AstNode::Deref { thing_to_deref } => {
            return Some(IrOperand::Deref {
                to_deref: Box::new(compile_ast_node_to_ir(
                    thing_to_deref,
                    state,
                    functions,
                    types,
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
                state.val_stack,
                state.variable_counter,
                state.stack_ptr,
                state.pop_table,
            );
            let tmp = IrOperand::StringLiteral {
                value: value.clone().into(),
            };
            let ln = IrOperand::IntLiteral {
                value: value.len() as i64 - 2,
            };
            state.val_stack.push(IrInstr::CallWithRet {
                target: out.clone(),
                func_name: "make_string_from".to_owned(),
                args: vec![tmp, ln],
                vtype: Type::StringT,
                stack_ptr_when_called: *state.stack_ptr,
            });
            return Some(out);
        }
        AstNode::ArrayLiteral { nodes } => {
            let vtype = Type::ArrayT {
                size: nodes.len(),
                array_type: Rc::new(nodes[0].get_type(functions, types).expect("must have type")),
            };
            let ops: Vec<IrOperand> = nodes
                .iter()
                .map(|i| compile_ast_node_to_ir(i, state, functions, types, None).expect(""))
                .collect();
            let out = stack_push(
                vtype.clone(),
                state.val_stack,
                state.variable_counter,
                state.stack_ptr,
                state.pop_table,
            );
            for i in 0..nodes.len() {
                state.val_stack.push(IrInstr::Mov {
                    left: IrOperand::ArrayAccess {
                        base: Box::new(out.clone()),
                        value: Box::new(IrOperand::IntLiteral { value: i as i64 }),
                    },
                    right: ops[i].clone(),
                    vtype: vtype.clone(),
                });
            }
            return Some(out);
        }
        AstNode::StructLiteral { vtype, nodes } => {
            let ops: Vec<IrOperand> = nodes
                .iter()
                .map(|i| compile_ast_node_to_ir(i, state, functions, types, None).expect(""))
                .collect();
            let out = stack_push(
                vtype.clone(),
                state.val_stack,
                state.variable_counter,
                state.stack_ptr,
                state.pop_table,
            );
            let comps = match vtype {
                Type::StructT {
                    name: _,
                    components,
                } => components.clone(),
                _ => {
                    unreachable!();
                }
            };
            for i in 0..nodes.len() {
                let tmp = IrOperand::FieldAccess {
                    base: Box::new(out.clone()),
                    name: comps[i].0.clone().into(),
                };
                state.val_stack.push(IrInstr::Mov {
                    left: tmp,
                    right: ops[i].clone(),
                    vtype: comps[i].1.clone(),
                })
            }
            return Some(out);
        }
        AstNode::If {
            condition,
            thing_to_do,
            r#else,
        } => {
            let cond = compile_ast_node_to_ir(&condition, state, functions, types, None)?;
            let base_label = *state.label_counter;
            let else_label = *state.label_counter + 1;
            let end_label = *state.label_counter + 2;
            *state.label_counter += 3;
            state.val_stack.push(IrInstr::CondGoto {
                cond: cond,
                target: format!("L{}", base_label),
            });
            state.val_stack.push(IrInstr::Goto {
                target: format!("L{}", else_label),
            });
            state.val_stack.push(IrInstr::Label {
                name: format!("L{}", base_label),
            });
            state.val_stack.push(IrInstr::BeginScope {
                stack_ptr: *state.stack_ptr,
            });
            state.val_stack.push(IrInstr::BeginGcFrame);
            let mut body_pop_table = vec![];
            for i in thing_to_do {
                compile_ast_node_to_ir(i, state, functions, types, None);
            }
            body_pop_table.reverse();
            for i in body_pop_table {
                state.val_stack.push(IrInstr::Pop { vtype: i });
            }
            state.val_stack.push(IrInstr::EndGcFrame);
            state.val_stack.push(IrInstr::EndScope {
                stack_ptr: *state.stack_ptr,
            });
            state.val_stack.push(IrInstr::Goto {
                target: format!("L{}", end_label),
            });
            state.val_stack.push(IrInstr::Label {
                name: format!("L{}", else_label),
            });
            if r#else.is_some() {
                state.val_stack.push(IrInstr::BeginScope {
                    stack_ptr: *state.stack_ptr,
                });
                state.val_stack.push(IrInstr::BeginGcFrame);
                let elblk = r#else.as_ref().expect("is some");
                let mut pop_stack = vec![];
                for i in elblk {
                    compile_ast_node_to_ir(i, state, functions, types, None);
                }
                pop_stack.reverse();
                for i in pop_stack {
                    state.val_stack.push(IrInstr::Pop { vtype: i });
                }
                state.val_stack.push(IrInstr::EndGcFrame);
                state.val_stack.push(IrInstr::EndScope {
                    stack_ptr: *state.stack_ptr,
                });
            }
            state.val_stack.push(IrInstr::Label {
                name: format!("L{}", end_label),
            });
        }
        AstNode::ForLoop {
            variable,
            condition,
            post_op,
            body,
        } => {
            let base = *state.label_counter;
            let end = *state.label_counter + 1;
            let lbody = *state.label_counter + 2;
            *state.label_counter += 3;
            state.val_stack.push(IrInstr::BeginScope {
                stack_ptr: *state.stack_ptr,
            });
            let _ = compile_ast_node_to_ir(variable, state, functions, types, None);
            state.val_stack.push(IrInstr::Label {
                name: format!("L{}", base),
            });
            state.val_stack.push(IrInstr::BeginScope {
                stack_ptr: *state.stack_ptr,
            });
            let mut loop_pop_table = vec![];
            let tmp = compile_ast_node_to_ir(condition, state, functions, types, None)?;
            state.val_stack.push(IrInstr::CondGoto {
                cond: tmp,
                target: format!("L{}", lbody),
            });
            state.val_stack.push(IrInstr::Goto {
                target: format!("L{}", end),
            });
            state.val_stack.push(IrInstr::Label {
                name: format!("L{}", lbody),
            });
            state.val_stack.push(IrInstr::BeginScope {
                stack_ptr: *state.stack_ptr,
            });
            state.val_stack.push(IrInstr::BeginGcFrame);
            for i in body {
                compile_ast_node_to_ir(i, state, functions, types, None);
            }
            compile_ast_node_to_ir(post_op, state, functions, types, None);
            loop_pop_table.reverse();
            for i in loop_pop_table {
                state.val_stack.push(IrInstr::Pop { vtype: i });
            }
            state.val_stack.push(IrInstr::EndGcFrame);
            state.val_stack.push(IrInstr::EndScope {
                stack_ptr: *state.stack_ptr,
            });
            state.val_stack.push(IrInstr::EndScope {
                stack_ptr: *state.stack_ptr,
            });
            state.val_stack.push(IrInstr::Goto {
                target: format!("L{}", base),
            });
            state.val_stack.push(IrInstr::EndScope {
                stack_ptr: *state.stack_ptr,
            });
            state.val_stack.push(IrInstr::Label {
                name: format!("L{}", end),
            });
        }
        AstNode::Loop { condition, body } => {
            let base = *state.label_counter;
            let end = *state.label_counter + 1;
            let lbody = *state.label_counter + 2;
            *state.label_counter += 3;
            state.val_stack.push(IrInstr::Label {
                name: format!("L{}", base),
            });
            state.val_stack.push(IrInstr::BeginScope {
                stack_ptr: *state.stack_ptr,
            });
            let tmp = compile_ast_node_to_ir(condition, state, functions, types, None)?;
            state.val_stack.push(IrInstr::CondGoto {
                cond: tmp,
                target: format!("L{}", lbody),
            });
            state.val_stack.push(IrInstr::Goto {
                target: format!("L{}", end),
            });
            state.val_stack.push(IrInstr::Label {
                name: format!("L{}", lbody),
            });
            state.val_stack.push(IrInstr::BeginScope {
                stack_ptr: *state.stack_ptr,
            });
            state.val_stack.push(IrInstr::BeginGcFrame);
            let mut loop_pop_table = vec![];
            for i in body {
                compile_ast_node_to_ir(i, state, functions, types, None);
            }
            loop_pop_table.reverse();
            for i in loop_pop_table {
                state.val_stack.push(IrInstr::Pop { vtype: i });
            }
            state.val_stack.push(IrInstr::EndGcFrame);
            state.val_stack.push(IrInstr::EndScope {
                stack_ptr: *state.stack_ptr,
            });
            state.val_stack.push(IrInstr::EndScope {
                stack_ptr: *state.stack_ptr,
            });
            state.val_stack.push(IrInstr::Goto {
                target: format!("L{}", base),
            });
            state.val_stack.push(IrInstr::Label {
                name: format!("L{}", end),
            });
        }
        AstNode::Return { body } => {
            let to_return =
                compile_ast_node_to_ir(body, state, functions, types, None).expect("should return");
            state.val_stack.push(IrInstr::Ret {
                to_return,
                stack_ptr: *state.stack_ptr,
            });
        }
        AstNode::OperatorMake { vtype, size } => {
            let out = IrOperand::FieldAccess{base:stack_push(
                Type::SliceT {
                    ptr_type: Rc::new(vtype.clone()),
                },
                state.val_stack,
                state.variable_counter,
                state.stack_ptr,
                state.pop_table,
            ).into(),name:"start".into()
        };
            let mult = AstNode::Mult {
                left: size.clone(),
                right: Box::new(AstNode::IntLiteral {
                    value: vtype.get_size_bytes() as i64,
                }),
                data: None,
            };
            let ln = compile_ast_node_to_ir(&mult, state, functions, types, None);
            state.val_stack.push(IrInstr::CallWithRet {
                target: out.clone(),
                func_name: "gc_alloc".to_owned(),
                args: vec![ln?],
                vtype: Type::SliceT {
                    ptr_type: Rc::new(vtype.clone()),
                },
                stack_ptr_when_called: *state.stack_ptr,
            });
            return Some(out);
        }
        AstNode::OperatorNew { vtype } => {
            let out = stack_push(
                Type::PointerT {
                    ptr_type: Rc::new(vtype.clone()),
                },
                state.val_stack,
                state.variable_counter,
                state.stack_ptr,
                state.pop_table,
            );
            let ln = IrOperand::IntLiteral {
                value: vtype.get_size_bytes() as i64,
            };
            state.val_stack.push(IrInstr::CallWithRet {
                target: out.clone(),
                func_name: "gc_alloc".to_owned(),
                args: vec![ln],
                vtype: Type::SliceT {
                    ptr_type: Rc::new(vtype.clone()),
                },
                stack_ptr_when_called: *state.stack_ptr,
            });
            return Some(out);
        }
        _ => {
            println!("{:#?}", node);
            unreachable!();
        }
    }
    None
}
pub fn compile_function_to_ir(
    func: &Function,
    functions: &HashMap<String, FunctionTable>,
    types: &HashMap<String, Type>,
    stack_ptr: &mut usize,
) -> Vec<IrInstr> {
    let mut out = vec![IrInstr::BeginScope {
        stack_ptr: *stack_ptr,
    }];
    let mut variable_counter = 0;
    if func.return_type.get_size_bytes() > 16 {
        *stack_ptr += 8;
    }
    let mut pop_table = vec![];
    let mut name_table = vec![HashMap::new()];
    let mut label_counter = 0;
    for i in 0..func.args.len() {
        let op = IrOperand::StacKOperand {
            var_idx: variable_counter,
            name: ("user_".to_owned() + &func.arg_names[i]).into(),
            stack_offset: *stack_ptr,
            vtype: func.args[i].clone(),
        };
        *stack_ptr += func.args[i].get_size_bytes();
        variable_counter += 1;
        let name = func.arg_names[i].clone();
        name_table[0].insert(name, op);
        pop_table.push(func.args[i].clone());
    }
    let mut state = IrCompState {
        variable_counter: &mut variable_counter,
        val_stack: &mut out,
        stack_ptr,
        pop_table: &mut pop_table,
        name_table: &mut name_table,
        label_counter: &mut label_counter,
    };
    for i in &func.program {
        let _ = compile_ast_node_to_ir(i, &mut state, functions, types, None);
    }
    pop_table.reverse();
    for i in pop_table {
        out.push(IrInstr::Pop { vtype: i });
    }
    out.push(IrInstr::EndScope {
        stack_ptr: *stack_ptr,
    });
    return out;
}
fn get_types_in_operand(op: &IrOperand, types: &mut HashSet<Type>) {
    match op {
        IrOperand::StacKOperand {
            var_idx: _,
            name: _,
            stack_offset: _,
            vtype,
        } => {
            types.insert(vtype.clone());
        }
        IrOperand::Name { name: _, vtype } => {
            types.insert(vtype.clone());
        }
        IrOperand::Deref { to_deref } => {
            get_types_in_operand(&to_deref, types);
        }
        IrOperand::TakeRef { to_ref } => {
            get_types_in_operand(&to_ref, types);
            types.insert(Type::PointerT {
                ptr_type: to_ref.get_type().clone().into(),
            });
        }
        IrOperand::StringLiteral { value: _ } => {
            types.insert(Type::StringT);
        }
        IrOperand::IntLiteral { value: _ } => {
            types.insert(Type::IntegerT);
        }
        IrOperand::FloatLiteral { value: _ } => {
            types.insert(Type::FloatT);
        }
        IrOperand::CharLiteral { value: _ } => {
            types.insert(Type::CharT);
        }
        IrOperand::FieldAccess { base, name: _ } => {
            types.insert(base.get_type());
        }
        IrOperand::ArrayAccess { base, value } => {
            types.insert(base.get_type());
            types.insert(value.get_type());
        }
    }
}

pub fn get_types_used_in_ir(instructions: &[IrInstr], types: &mut HashSet<Type>) {
    for i in instructions {
        match i {
            IrInstr::VariableDeclaration {
                name: _,
                vtype,
                stack_offset: _,
            } => {
                types.insert(vtype.clone());
            }
            IrInstr::Mov { left, right, vtype } => {
                types.insert(vtype.clone());
                get_types_in_operand(left, types);
                get_types_in_operand(right, types);
            }
            IrInstr::CondGoto { cond, target: _ } => {
                get_types_in_operand(cond, types);
            }
            IrInstr::Add {
                target,
                left,
                right,
                vtype,
            } => {
                types.insert(vtype.clone());
                get_types_in_operand(target, types);
                get_types_in_operand(left, types);
                get_types_in_operand(right, types);
            }
            IrInstr::Sub {
                target,
                left,
                right,
                vtype,
            } => {
                types.insert(vtype.clone());
                get_types_in_operand(target, types);
                get_types_in_operand(left, types);
                get_types_in_operand(right, types);
            }
            IrInstr::Mul {
                target,
                left,
                right,
                vtype,
            } => {
                types.insert(vtype.clone());
                get_types_in_operand(target, types);
                get_types_in_operand(left, types);
                get_types_in_operand(right, types);
            }
            IrInstr::Div {
                target,
                left,
                right,
                vtype,
            } => {
                types.insert(vtype.clone());
                get_types_in_operand(target, types);
                get_types_in_operand(left, types);
                get_types_in_operand(right, types);
            }
            IrInstr::And {
                target,
                left,
                right,
                vtype,
            } => {
                types.insert(vtype.clone());
                get_types_in_operand(target, types);
                get_types_in_operand(left, types);
                get_types_in_operand(right, types);
            }
            IrInstr::Or {
                target,
                left,
                right,
                vtype,
            } => {
                types.insert(vtype.clone());
                get_types_in_operand(target, types);
                get_types_in_operand(left, types);
                get_types_in_operand(right, types);
            }
            IrInstr::Equals {
                target,
                left,
                right,
                vtype,
            } => {
                types.insert(vtype.clone());
                get_types_in_operand(target, types);
                get_types_in_operand(left, types);
                get_types_in_operand(right, types);
            }
            IrInstr::NotEquals {
                target,
                left,
                right,
                vtype,
            } => {
                types.insert(vtype.clone());
                get_types_in_operand(target, types);
                get_types_in_operand(left, types);
                get_types_in_operand(right, types);
            }
            IrInstr::GreaterThan {
                target,
                left,
                right,
                vtype,
            } => {
                types.insert(vtype.clone());
                get_types_in_operand(target, types);
                get_types_in_operand(left, types);
                get_types_in_operand(right, types);
            }
            IrInstr::GreaterThanOrEq {
                target,
                left,
                right,
                vtype,
            } => {
                types.insert(vtype.clone());
                get_types_in_operand(target, types);
                get_types_in_operand(left, types);
                get_types_in_operand(right, types);
            }
            IrInstr::LessThan {
                target,
                left,
                right,
                vtype,
            } => {
                types.insert(vtype.clone());
                get_types_in_operand(target, types);
                get_types_in_operand(left, types);
                get_types_in_operand(right, types);
            }
            IrInstr::LessThanOrEq {
                target,
                left,
                right,
                vtype,
            } => {
                types.insert(vtype.clone());
                get_types_in_operand(target, types);
                get_types_in_operand(left, types);
                get_types_in_operand(right, types);
            }
            IrInstr::Not {
                target,
                value,
                vtype,
            } => {
                types.insert(vtype.clone());
                get_types_in_operand(target, types);
                get_types_in_operand(value, types);
            }
            IrInstr::Call {
                func_name: _,
                args,
                stack_ptr_when_called: _,
            } => {
                for i in args {
                    get_types_in_operand(&i, types);
                }
            }
            IrInstr::CallWithRet {
                target,
                func_name: _,
                args,
                vtype: _,
                stack_ptr_when_called: _,
            } => {
                for i in args {
                    get_types_in_operand(&i, types);
                }
                get_types_in_operand(target, types);
            }
            IrInstr::Ret {
                to_return,
                stack_ptr: _,
            } => {
                get_types_in_operand(to_return, types);
            }
            IrInstr::Push {
                vtype,
                val_idx: _,
                stack_offset_of_value: _,
            } => {
                types.insert(vtype.clone());
            }
            IrInstr::Pop { vtype } => {
                types.insert(vtype.clone());
            }
            _ => {}
        }
    }
}
