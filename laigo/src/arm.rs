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
            let op = match binop_type {
                BinopType::None => todo!(),
                BinopType::Add => "add",
                BinopType::Sub => "sub",
                BinopType::Mul => "mul",
                BinopType::Div => "div",
                BinopType::Equal => todo!(),
                BinopType::Greater => todo!(),
                BinopType::Less => todo!(),
                BinopType::And => "and",
                BinopType::Or => "or",
                BinopType::Xor => "xor",
            };
            return format!(
                "   {} {}, {}, {}\n",
                op,
                left.get_imm_arm(),
                right.get_imm_arm(),
                output.get_imm_arm()
            );
        }
        LaigoIns::Not { left, right } => {
            todo!()
        }
        LaigoIns::Assign { left, right } => {
            if left.is_reg() {
                if right.is_num_or_reg() {
                    return format!("    mov {}, {}\n", left.get_imm_arm(), right.get_imm_arm());
                } else {
                    return format!("ldr {}, {}", left.get_imm_arm(), right.get_mem_op_arm());
                }
            } else {
                assert!(right.is_reg());
                return format!("str {}, {}", right.get_imm_arm(), left.get_mem_op_arm());
            }
        }
        LaigoIns::Jmp { target } => {
            todo!()
        }
        LaigoIns::If {
            condition,
            left,
            right,
        } => {
            todo!()
        }
        LaigoIns::Call { to_call } => {
            format!("bl {}\n", to_call.get_imm_arm())
        }
        LaigoIns::Syscall { call } => {
            format!("bl _interupt\n")
        }
        LaigoIns::Ret => {
            format!("mov sp, fp\n ldr lr, [sp, #8]\n ldr fp,[sp, #8]\n")
        }
        LaigoIns::Noop => {
            todo!()
        }
        LaigoIns::FnBegin => {
            format!(
                "sub sp, sp ,#16\n str fp, [sp, #-8]\n str lr ,[sp, #-16]\nmov fp, sp\n sub fp, fp, #16\n"
            )
        }
        LaigoIns::FnEnd => {
            format!("")
        }
    }
}
pub fn compile(prog: LaigoUnit, name: &str) {
    let mut out = String::from(".extern _interupt\n");
    for i in prog.globals {
        out += &format!(".global {}\n", i);
    }
    for i in 0..prog.instructions.len() {
        if let Some(p) = prog.label_indexs.get(&i) {
            out += &format!("{p}:\n");
        }
        out += &compile_ins(&prog.instructions, i);
    }
    std::fs::write(name, out).unwrap();
}
