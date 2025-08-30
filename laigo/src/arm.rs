use crate::laigo::*;
pub fn compile_ins(ins_list: &[LaigoIns], idx: usize) -> String {
    let n = &ins_list[idx];
    match n {
        LaigoIns::Declare { count: _ } => {
            return "".into();
        }
        LaigoIns::Binop {
            output,
            left,
            right,
            binop_type,
        } => {
            assert!(output.is_reg() && left.is_num_or_reg() && right.is_num_or_reg());
            let op = if left.is_fp() {
                match binop_type {
                    BinopType::None => todo!(),
                    BinopType::Add => "addv",
                    BinopType::Sub => "subv",
                    BinopType::Mul => "mulv",
                    BinopType::Div => "divv",
                    BinopType::Equal => "cmpv",
                    BinopType::Greater => "cmpv",
                    BinopType::Less => "cmpv",
                    BinopType::And => todo!(),
                    BinopType::Or => todo!(),
                    BinopType::Xor => todo!(),
                }
            } else {
                match binop_type {
                    BinopType::None => todo!(),
                    BinopType::Add => "add",
                    BinopType::Sub => "sub",
                    BinopType::Mul => "mul",
                    BinopType::Div => "div",
                    BinopType::Equal => "cmp",
                    BinopType::Greater => "cmp",
                    BinopType::Less => "cmp",
                    BinopType::And => "and",
                    BinopType::Or => "or",
                    BinopType::Xor => "xor",
                }
            };
            let l1 = left.get_imm_arm();
            let l2 = right.get_imm_arm();
            let out = output.get_imm_arm();
            match binop_type {
                BinopType::Greater => {
                    format!("    subs {}, {}, {}\n    cset {}, gt\n", l1, out, l2, out)
                }
                BinopType::Equal => {
                    format!("    subs {}, {}, {}\n    cset {}, eq\n", l1, out, l2, out)
                }
                BinopType::Less => {
                    format!("    subs {}, {}, {}\n    cset {}, lt\n", l1, out, l2, out)
                }
                _ => {
                    format!(
                        "   {} {}, {}, {}\n",
                        op,
                        output.get_imm_arm(),
                        left.get_imm_arm(),
                        right.get_imm_arm()
                    )
                }
            }
        }
        LaigoIns::Not { left, right } => {
            format!("    not {}, {}\n", right.get_imm_arm(), left.get_imm_arm())
        }
        LaigoIns::Assign { left, right } => {
            if left.is_reg() {
                if right.is_num_or_reg() {
                    format!("    mov {}, {}\n", left.get_imm_arm(), right.get_imm_arm())
                } else {
                    match right {
                        LaigoOp::TakeRef { to_take_ref } => {
                            format!(
                                "    mov {}, {}\n    sub {}, {}, {}\n",
                                left.get_imm_arm(),
                                to_take_ref.get_as_ref_arm().0,
                                left.get_imm_arm(),
                                left.get_imm_arm(),
                                to_take_ref.get_as_ref_arm().1,
                            )
                        }
                        LaigoOp::Symbol { v } => {
                            format!(
                                "   adrp {}, _{}@PAGE\n    add {}, {}, _{}@PAGEOFF\n",
                                left.get_imm_arm(),
                                v,
                                left.get_imm_arm(),
                                left.get_imm_arm(),
                                v
                            )
                        }
                        _ => {
                            format!(
                                "   ldr {}, {}\n",
                                left.get_imm_arm(),
                                right.get_mem_op_arm()
                            )
                        }
                    }
                }
            } else {
                assert!(right.is_reg());
                format!(
                    "    str {}, {}\n",
                    right.get_imm_arm(),
                    left.get_mem_op_arm()
                )
            }
        }
        LaigoIns::Jmp { target } => {
            format!("   b {}\n", target.get_imm_arm())
        }
        LaigoIns::If {
            condition,
            left,
            right,
        } => {
            format!(
                "    cbz {},{}\n    b {}\n",
                condition.get_imm_arm(),
                left.get_imm_arm(),
                right.get_imm_arm()
            )
        }
        LaigoIns::Call { to_call } => {
            if to_call.is_reg() {
                format!("    blr {}\n", to_call.get_imm_arm())
            } else {
                format!("    bl {}\n", to_call.get_imm_arm())
            }
        }
        LaigoIns::Syscall { call } => {
            format!("    bl _interupt\n")
        }
        LaigoIns::Ret => {
            format!(
                "    mov sp, fp\n    ldr fp, [sp, #8]\n   ldr lr, [sp, #16]\n    add sp, sp, #16\n    ret\n"
            )
        }
        LaigoIns::Noop => "    nop\n".into(),
        LaigoIns::FnBegin => {
            let mut depth = 0;
            for i in idx + 1..ins_list.len() {
                match &ins_list[i] {
                    LaigoIns::Declare { count } => {
                        depth += count;
                    }
                    LaigoIns::FnEnd => {
                        break;
                    }
                    _ => continue,
                }
            }
            if depth % 2 != 0 {
                depth += 1;
            }
            format!(
                "    sub sp, sp, #16\n    str fp, [sp, #16]\n    str lr, [sp, #16]\n    mov fp, sp\n    sub sp, sp, {}\n",
                depth * 8
            )
        }
        LaigoIns::FnEnd => {
            format!(
                "    mov sp, fp\n    ldr fp, [sp, #8]\n   ldr lr, [sp, #16]\n    add sp, sp, #16\n    ret\n"
            )
        }
    }
}
pub fn compile(prog: LaigoUnit, name: &str) {
    let mut out = String::from(".extern _interupt\n");
    for i in prog.globals {
        out += &format!(".global _{}\n", i);
    }
    for i in prog.externs {
        out += &format!(".extern _{}\n", i);
    }
    for i in 0..prog.instructions.len() {
        if let Some(p) = prog.label_indexs.get(&i) {
            out += &format!("_{p}:\n");
        }
        out += &compile_ins(&prog.instructions, i);
    }
    out += ".data\n";
    for i in prog.data_table {
        out += &format!("_{}:\n", i.0);
        match &i.1 {
            LaigoValue::Bytes { v } => {
                out += ".byte ";
                for j in 0..v.len() {
                    out += &format!("{}", v[j]);
                    if j != v.len() - 1 {
                        out += ",";
                    }
                }
                out += "\n";
            }

            LaigoValue::Float { f } => {
                out += &format!(".word {}\n", unsafe {
                    std::mem::transmute_copy::<f64, u64>(f)
                });
            }
            LaigoValue::Integer { v } => {
                out += &format!(".word {}\n", v);
            }
            LaigoValue::String { v } => {
                let b: Vec<u8> = v.bytes().collect();
                out += ".byte ";
                for j in 0..b.len() {
                    out += &format!("{}", b[j]);
                    out += ",";
                }
                out += "0\n";
            }
            LaigoValue::Unsigned { u } => {
                out += &format!(".word {}\n", u);
            }
        }
    }

    std::fs::write(name, out).unwrap();
}
