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
                BinopType::Mul => "imul",
                BinopType::Div => "div",
                BinopType::Equal => "cmp",
                BinopType::Greater => "cmp",
                BinopType::Less => "cmp",
                BinopType::And => "and",
                BinopType::Or => "or",
                BinopType::Xor => "xor",
            }; 
            let l1 = left.get_imm_x86();
            let out = output.get_imm_x86();
            let l2 = right.get_imm_x86();
            match binop_type {
                BinopType::Greater=>{
                    if out != "rax"{
                        format!("   push rax\n   cmp {}, {}\n   setg al\n    movzx {},al\n   pop rax\n", l1, l2, out)
                    }else{
                        format!("    cmp {}, {}, setg al\n", l1, l2)
                    }
                   
                },
                BinopType::Less=>{
                     if out != "rax"{
                        format!("   push rax\n   cmp {}, {}\n   setg al\n    movzx {},al\n   pop rax\n", l1, l2, out)
                    }else{
                        format!("    cmp {}, {}, setg al\n", l1, l2)
                    }  
                },
                BinopType::Equal=>{
                     if out != "rax"{
                        format!("   push rax\n   cmp {}, {}\n   setg al\n    movzx {},al\n   pop rax\n", l1, l2, out)
                    }else{
                        format!("    cmp {}, {}, setg al\n", l1, l2)
                    }
                },
                _=>{
                    if l1 == out{
                        format!("    {} {}, {}\n", op, l1, l2)
                    }else if out != l2{
                        format!("    mov {}, {}\n    {} {}, {}\n",out ,l1, op,out, l2)
                    }else{
                        format!("    push {}\n    {} {}, {}\n    mov {}, {}\n pop {}\n", l1, op, l1, l2, l2, l1, l1)
                    }
                }
            }
            
            
        }
        LaigoIns::Not { left, right } => {
            format!("    mov {}, {}\n    not {}()",left.get_imm_x86(), right.get_imm_x86(), left.get_imm_x86())
        }
        LaigoIns::Assign { left, right } => {
            if left.is_reg() {
                if right.is_num_or_reg() {
                    return format!("    mov {}, {}\n", left.get_imm_x86(), right.get_imm_x86());
                } else {
                    return format!("    mov {}, {}\n", left.get_imm_x86(), right.get_mem_op_x86());
                }
            } else {
                assert!(right.is_reg());
                return format!("    mov {}, {}\n", right.get_imm_x86(), left.get_mem_op_x86());
            }
        }
        LaigoIns::Jmp { target } => {
            return format!("    jmp {}\n", target.get_imm_x86());
        }
        LaigoIns::If {
            condition,
            left,
            right,
        } => {
            return format!("    cmp {},0\n     je {}\n    jmp {}\n",condition.get_imm_x86(), left.get_imm_x86(), right.get_imm_x86());
        }
        LaigoIns::Call { to_call } => {
            format!("    call {}\n", to_call.get_imm_x86())
        }
        LaigoIns::Syscall { call } => {
            format!("    call interupt\n")
        }
        LaigoIns::Ret => {
            format!("    mov rsp, rbp\n    pop rbp\n    ret")
        }
        LaigoIns::Noop => {
            format!("    nop\n")
        }
        LaigoIns::FnBegin => {
            let mut depth =0;
            for i in idx+1..ins_list.len(){
                match &ins_list[i]{
                    LaigoIns::Declare{count}=>{
                        depth+=count;
                    }
                    LaigoIns::FnEnd=>{
                        break;
                    }
                    _=>continue,
                }
            }
            if depth%2 != 0{
                depth +=1;
            }
            format!("    push rbp\n    mov rbp, rsp\n    sub rsp, 8\n    sub rsp,{}\n",depth*8)
        }
        LaigoIns::FnEnd => {
            format!("mov rsp, rbp\n    pop rbp\n    ret\n") 
        }
    }
}
pub fn compile(prog: LaigoUnit, name: &str) {
    let mut out = String::from(".intel_syntax noprefix\n.extern interupt\n");
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
