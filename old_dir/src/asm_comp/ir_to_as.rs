use crate::asm_comp::{gc_function_name, x86};
use crate::ir::{IrInstr, IrOperand};
use crate::{Target, Type};
use std::collections::HashSet;
#[derive(Debug)]
pub struct AsmOperand {
    pub value: String,
    pub is_address: bool,
}
impl AsmOperand {
    pub fn new(a: String, is_address: bool) -> Self {
        Self {
            value: a,
            is_address,
        }
    }
}
impl AsRef<str> for AsmOperand {
    fn as_ref(&self) -> &str {
        return self.value.as_ref();
    }
}
fn get_asmx86_type_name(vtype: &Type) -> &'static str {
    match vtype {
        Type::BoolT | Type::CharT => return "BYTE",
        _ => return "QWORD",
    }
}

fn get_sreg(left: bool) -> String {
    if left {
        return "r11".to_owned();
    } else {
        return "r10".to_owned();
    }
}
fn compile_binary_op(
    ir_instr: &str,
    fp_instr: &str,
    left: &IrOperand,
    right: &IrOperand,
    target: &IrOperand,
    vtype: &Type,
    statics: &mut String,
    statics_count: &mut usize,
) -> String {
    match vtype {
        Type::FloatT => {
            let mut stack = "".to_owned();
            let l = compile_ir_op_to_x86(left, true, &mut stack, statics, statics_count);
            let r = compile_ir_op_to_x86(right, false, &mut stack, statics, statics_count);
            if l.is_address {
                stack += &format!("    movsd xmm0, [{}]\n", l.as_ref());
            } else {
                stack += &format!("    movsd xmm0, {}\n", l.as_ref());
            }
            if r.is_address {
                stack += &format!("    movsd xmm1,[{}]\n", r.as_ref());
            } else {
                stack += &format!("    movsd xmm1, {}\n", r.as_ref());
            }
            stack += &format!("    {}\n", fp_instr);
            let v = compile_ir_op_to_x86(target, true, &mut stack, statics, statics_count);
            stack += &format!("    movsd [{}], xmm0\n", v.as_ref());
            return stack;
        }
        _ => {
            let mut stack = "".to_owned();
            let l = compile_ir_op_to_x86(left, true, &mut stack, statics, statics_count);
            let r = compile_ir_op_to_x86(right, false, &mut stack, statics, statics_count);
            if l.is_address {
                stack += &format!("    mov rax, QWORD [{}]\n", l.as_ref());
            } else {
                stack += &format!("    mov rax, {}\n", l.as_ref());
            }
            if r.is_address {
                stack += &format!("    mov rbx, QWORD [{}]\n", r.as_ref());
            } else {
                stack += &format!("    mov rbx, {}\n", r.as_ref());
            }
            stack += &format!("    {}\n", ir_instr);
            let v = compile_ir_op_to_x86(target, true, &mut stack, statics, statics_count);
            stack += &format!(
                "    mov {} [{}], rax\n",
                get_asmx86_type_name(vtype),
                v.as_ref()
            );
            return stack;
        }
    }
}
fn compile_binary_comp_op(
    ir_instr: &str,
    left: &IrOperand,
    right: &IrOperand,
    target: &IrOperand,
    vtype: &Type,
    statics: &mut String,
    statics_count: &mut usize,
) -> String {
    match vtype {
        Type::FloatT => {
            let mut stack = "".to_owned();
            let l = compile_ir_op_to_x86(left, true, &mut stack, statics, statics_count);
            let r = compile_ir_op_to_x86(right, false, &mut stack, statics, statics_count);
            if l.is_address {
                stack += &format!("    movsd xmm1, [{}]\n", l.as_ref());
            } else {
                stack += &format!("    movsd xmm1, {}\n", l.as_ref());
            }
            if r.is_address {
                stack += &format!("    movsd xmm0, QWORD [{}]\n", r.as_ref());
            } else {
                stack += &format!("    movsd xmm0, {}\n", r.as_ref());
            }
            stack += &format!("    fcom\n");
            stack += &format!("    mov rbx, 0\n");
            stack += &format!("    mov rax, 1\n");
            stack += &format!("    {}\n", ir_instr);
            let v = compile_ir_op_to_x86(target, true, &mut stack, statics, statics_count);
            stack += &format!(
                "    mov {} [{}], rax\n",
                get_asmx86_type_name(vtype),
                v.as_ref()
            );
            return stack;
        }
        _ => {
            let mut stack = "".to_owned();
            let l = compile_ir_op_to_x86(left, true, &mut stack, statics, statics_count);
            let r = compile_ir_op_to_x86(right, false, &mut stack, statics, statics_count);
            if l.is_address {
                stack += &format!("    mov rax, QWORD [{}]\n", l.as_ref());
            } else {
                stack += &format!("    mov rax, {}\n", l.as_ref());
            }
            if r.is_address {
                stack += &format!("    mov rbx, QWORD [{}]\n", r.as_ref());
            } else {
                stack += &format!("    mov rbx, {}\n", r.as_ref());
            }
            stack += &format!("    cmp rbx, rax\n");
            stack += &format!("    mov rbx, 0\n");
            stack += &format!("    mov rax, 1\n");
            stack += &format!("    {}\n", ir_instr);
            let v = compile_ir_op_to_x86(target, true, &mut stack, statics, statics_count);
            stack += &format!(
                "    mov {} [{}], rax\n",
                get_asmx86_type_name(vtype),
                v.as_ref()
            );
            return stack;
        }
    }
}
#[allow(unused)]
pub fn compile_ir_op_to_x86(
    op: &IrOperand,
    left: bool,
    stack: &mut String,
    statics: &mut String,
    statics_count: &mut usize,
) -> AsmOperand {
    match op {
        IrOperand::ArrayAccess { base, value } => {
            let b_ =base;
            let base = compile_ir_op_to_x86(base, left, stack, statics, statics_count);
            *stack += &format!("    mov rbx, qword [{}]\n", base.as_ref());
            let value = compile_ir_op_to_x86(value, left, stack, statics, statics_count);
            if value.is_address {
                *stack += &format!("    mov rax, QWORD[{}]\n", value.as_ref());
            } else {
                *stack += &format!("    mov rax, {}\n", value.as_ref());
            }
            *stack += &format!("    add rbx, rax\n");
            *stack += &format!("    mov {}, rbx\n", get_sreg(left));
            return AsmOperand::new(get_sreg(left), true);
        }
        IrOperand::CharLiteral { value } => {
            return AsmOperand::new(format!("{value}"), false);
        }
        IrOperand::IntLiteral { value } => {
            return AsmOperand::new(format!("{value}"), false);
        }
        IrOperand::FloatLiteral { value } => {
            let mut ifloat = unsafe { core::mem::transmute::<f64, [u8; 8]>(*value) };
            let name_str = {
                let mut tmp = String::new();
                for i in 0..8 {
                    tmp += &format!("{}", ifloat[i]);
                    if i != 7 {
                        tmp += ",";
                    }
                }
                tmp
            };
            *statics += &format!("   static{} : db {}\n", statics_count, name_str);
            *statics_count += 1;
            *stack += &format!(
                "    lea {}, [rel static{}]\n",
                get_sreg(left),
                *statics_count - 1
            );
            return AsmOperand::new(get_sreg(left), true);
        }
        IrOperand::Deref { to_deref } => {
            let base = compile_ir_op_to_x86(&to_deref, left, stack, statics, statics_count);
            *stack += &format!("    mov {},[{}]\n", get_sreg(left), base.as_ref());
            return AsmOperand::new(get_sreg(left), true);
        }
        IrOperand::StacKOperand {
            var_idx: _,
            name: _,
            stack_offset,
            vtype,
        } => {
            *stack += &format!("    lea {}, [rbp-{}]\n", get_sreg(left), stack_offset + 8);
            return AsmOperand::new(get_sreg(left), true);
        }
        IrOperand::Name { name, vtype } => {
            *stack += &format!("    lea {}, [rel {}]\n", name, get_sreg(left));
            return AsmOperand::new(get_sreg(left), true);
        }
        IrOperand::TakeRef { to_ref } => {
            let base = compile_ir_op_to_x86(&to_ref, left, stack, statics, statics_count);
            //*stack += &format!("   mov {}, {}\n", get_sreg(left), base.as_ref());
            return AsmOperand::new(get_sreg(left), false);
        }
        IrOperand::StringLiteral { value } => {
            *statics += &format!("   static{}: db {},0x0\n", statics_count, value);
            *statics_count += 1;
            *stack += &format!(
                "    lea {}, [rel static{}]\n",
                get_sreg(left),
                *statics_count - 1
            );
            return AsmOperand::new(get_sreg(left), true);
        }
        IrOperand::FieldAccess { base, name } => {
            let offset = base.get_type().get_variable_offset(name).expect("contains");
            let btype = base.get_type().get_variable_type(name);
            match base.as_ref() {
                IrOperand::StacKOperand {
                    var_idx: _,
                    name: _,
                    stack_offset,
                    vtype,
                } => {
                    *stack += &format!(
                        "    lea {}, [rbp-{}]\n",
                        get_sreg(left),
                        stack_offset + 8 + offset
                    );
                    return AsmOperand::new(get_sreg(left), true);
                }
                _ => {
                    let base = compile_ir_op_to_x86(base, left, stack, statics, statics_count);
                    *stack += &format!("    add {}, {}\n", base.as_ref(), offset);
                    return AsmOperand::new(get_sreg(left), true);
                }
            }
        }
    }
    todo!();
}
pub fn compile_ir_instr_to_x86(
    instr: &IrInstr,
    depth: &mut usize,
    _used_types: &mut HashSet<Type>,
    statics_count: &mut usize,
    statics: &mut String,
    cmp_target: &Target,
) -> String {
    match instr {
        IrInstr::Add {
            target,
            left,
            right,
            vtype,
        } => {
            return compile_binary_op(
                "add rax,rbx",
                "addsd xmm0, xmm1",
                left,
                right,
                target,
                vtype,
                statics,
                statics_count,
            );
        }
        IrInstr::Sub {
            target,
            left,
            right,
            vtype,
        } => {
            return compile_binary_op(
                "sub rax,rbx",
                "subsd xmm0, xmm1",
                left,
                right,
                target,
                vtype,
                statics,
                statics_count,
            );
        }
        IrInstr::Div {
            target,
            left,
            right,
            vtype,
        } => {
            return compile_binary_op(
                "cdq\n    idiv ebx",
                "divsd xmm0, xmm1",
                left,
                right,
                target,
                vtype,
                statics,
                statics_count,
            );
        }
        IrInstr::Mul {
            target,
            left,
            right,
            vtype,
        } => {
            return compile_binary_op(
                "imul rax,rbx",
                "mulsd xmm0, xmm1",
                left,
                right,
                target,
                vtype,
                statics,
                statics_count,
            );
        }
        IrInstr::And {
            target,
            left,
            right,
            vtype,
        } => {
            return compile_binary_op(
                "and rax,rbx",
                "",
                left,
                right,
                target,
                vtype,
                statics,
                statics_count,
            );
        }
        IrInstr::Or {
            target,
            left,
            right,
            vtype,
        } => {
            return compile_binary_op(
                "or rax,rbx",
                "",
                left,
                right,
                target,
                vtype,
                statics,
                statics_count,
            );
        }
        IrInstr::GreaterThan {
            target,
            left,
            right,
            vtype,
        } => {
            return compile_binary_comp_op(
                "cmovg rax,rbx",
                left,
                right,
                target,
                vtype,
                statics,
                statics_count,
            );
        }
        IrInstr::GreaterThanOrEq {
            target,
            left,
            right,
            vtype,
        } => {
            return compile_binary_comp_op(
                "cmovge rax,rbx",
                left,
                right,
                target,
                vtype,
                statics,
                statics_count,
            );
        }
        IrInstr::LessThan {
            target,
            left,
            right,
            vtype,
        } => {
            return compile_binary_comp_op(
                "cmovl rax,rbx",
                left,
                right,
                target,
                vtype,
                statics,
                statics_count,
            );
        }
        IrInstr::LessThanOrEq {
            target,
            left,
            right,
            vtype,
        } => {
            return compile_binary_comp_op(
                "cmovle rax,rbx",
                left,
                right,
                target,
                vtype,
                statics,
                statics_count,
            );
        }
        IrInstr::BeginScope { stack_ptr: _ } => {
            return "".to_string();
        }
        IrInstr::EndScope { stack_ptr: _ } => {
            return "".to_string();
        }
        IrInstr::Call {
            func_name,
            args,
            stack_ptr_when_called: _,
        } => {
            let mut st = String::new();
            let mut ag = x86::ArgCPU::new();
            let mut pop_count = 0;
            let mut vs = vec![];
            for i in args {
                let mut tmp_st = String::new();
                let s = compile_ir_op_to_x86(i, true, &mut tmp_st, statics, statics_count);
                vs.push(tmp_st + &ag.generate_arg(s.as_ref(), &i.get_type(), &mut pop_count));
            }
            if pop_count % 2 != 0 {
                st += "    push r10\n";
            }
            st += "  push r10\n";
            st += "  push r11\n";
            vs.reverse();
            for i in &vs {
                st += i;
            }
            match cmp_target {
                Target::MacOs { arm: _ } => {
                    st += &format!("    call _{}\n", func_name);
                }
                _ => {
                    st += &format!("    call {}\n", func_name);
                }
            }
            st += "  pop r11\n";
            st += "  pop r10\n";
            if pop_count % 2 != 0 {
                st += "    pop r10\n";
            }
            for _ in 0..pop_count {
                st += "    pop r10\n";
            }
            return st;
        }
        IrInstr::CallWithRet {
            target,
            func_name,
            args,
            vtype,
            stack_ptr_when_called: _,
        } => {
            let mut st = String::new();
            let mut ag = x86::ArgCPU::new();
            let tstr = compile_ir_op_to_x86(target, true, &mut st, statics, statics_count);
            let mut pop_count = 0;
            let mut vs = vec![];
            if target.get_type().get_size_bytes() > 16 {
                ag.int_registers[0] = 8;
                st += &format!("  mov rdi, {}\n", tstr.as_ref());
            }
            for i in args {
                let mut tmp_st = String::new();
                let s = compile_ir_op_to_x86(i, false, &mut tmp_st, statics, statics_count);
                vs.push(tmp_st + &ag.generate_arg(s.as_ref(), &i.get_type(), &mut pop_count));
            }
            vs.reverse();
            if pop_count % 2 != 0 {
                st += "    push r10\n";
            }
            st += "    push r10\n";
            st += "    push r11\n";
            for i in &vs {
                st += i;
            }
            match cmp_target {
                Target::MacOs { arm: _ } => {
                    st += &format!("    call _{}\n", func_name);
                }
                _ => {
                    st += &format!("    call {}\n", func_name);
                }
            }
            if pop_count % 2 != 0 {
                st += "    pop r10\n";
            }
            st += "    pop r11\n";
            st += "    pop r10\n";
            if vtype.get_size_bytes() <= 16 {
                let typs = vtype.flatten_to_basic_types();
                let mut hit_float = false;
                let mut hit_int = false;
                match typs[0] {
                    Type::FloatT => {
                        hit_float = true;
                        st += &format!("    movsd [{}], xmm0\n", tstr.as_ref());
                    }
                    _ => {
                        hit_int = true;
                        st += &format!("    mov QWORD[{}], rax\n", tstr.as_ref());
                    }
                }

                if vtype.get_size_bytes() > 8 {
                    match typs[0] {
                        Type::FloatT => {
                            if hit_float {
                                st += &format!("    movsd [{}-8], xmm1\n", tstr.as_ref());
                            } else {
                                st += &format!("    movsd [{}-8], xmm0\n", tstr.as_ref());
                            }
                        }
                        _ => {
                            if hit_int {
                                st += &format!("    mov QWORD[{}-8], rdx\n", tstr.as_ref());
                            } else {
                                st += &format!("    mov QWORD[{}-8], rax\n", tstr.as_ref());
                            }
                        }
                    }
                }
            }
            for _ in 0..pop_count {
                st += "    pop r10\n";
            }
            return st;
        }
        IrInstr::Mov { left, right, vtype } => {
            let mut stack = "".to_owned();
            let l = compile_ir_op_to_x86(left, true, &mut stack, statics, statics_count);
            let r = compile_ir_op_to_x86(right, false, &mut stack, statics, statics_count);
            let total = vtype.get_size_bytes();
            match right {
                IrOperand::IntLiteral { value } => {
                    stack += &format!("   mov rax, {value}\n    mov QWORD[{}], rax\n", l.as_ref());
                    return stack;
                }
                IrOperand::CharLiteral { value } => {
                    stack += &format!("   mov rax, {value}\n    mov BYTE [{}], rax\n", l.as_ref());
                    return stack;
                }
                IrOperand::FloatLiteral { value } => {
                    let ifloat = unsafe { core::mem::transmute::<f64, [u8; 8]>(*value) };
                    let name_str = {
                        let mut tmp = String::new();
                        for i in 0..8 {
                            tmp += &format!("{}", ifloat[i]);
                            if i != 7 {
                                tmp += ",";
                            }
                        }
                        tmp
                    };
                    *statics += &format!("   static{} : db {}\n", statics_count, name_str);
                    *statics_count += 1;
                    stack += &format!("    movsd xmm0, [rel static{}]\n", *statics_count - 1);
                    stack += &format!(" movsd [r11], xmm0\n");
                    return stack;
                }
                _ => {}
            }
            let mut count = 0;
            stack += &format!("    mov rax, {}\n", l.as_ref());
            stack += &format!("    mov rbx, {}\n", r.as_ref());
            while count < total {
                if r.is_address {
                    stack += &format!("    mov rcx,QWORD [rbx]\n");
                } else {
                    stack += &format!("    mov rcx,rbx\n");
                }
                stack += &format!("    mov QWORD [rax], rcx\n");
                stack += &format!("    sub rax,8\n");
                stack += &format!("    sub rbx, 8\n");
                count += 8;
            }
            stack += &format!("");
            return stack;
        }
        IrInstr::Goto { target } => {
            return format!("     jmp {target}");
        }
        IrInstr::Label { name } => {
            return format!("{name}:");
        }
        IrInstr::VariableDeclaration {
            name: _,
            vtype,
            stack_offset,
        } => match cmp_target {
            Target::MacOs { arm: _ } => {
                return format!("    lea rdi, [rbp-{stack_offset}]\n    lea rsi, [rel _{}]\n    call _gc_register_ptr", gc_function_name(vtype));
            }
            _ => {
                return format!("    lea rdi, [rbp-{stack_offset}]\n    lea rsi, [rel {}]\n    call gc_register_ptr", gc_function_name(vtype));
            }
        },
        IrInstr::CondGoto { cond, target } => {
            let mut stack = "".to_owned();
            let cond = compile_ir_op_to_x86(cond, true, &mut stack, statics, statics_count);
            stack += &format!("    mov rax, QWORD[{}]\n", cond.as_ref());
            stack += &format!("    cmp rax, 0\n");
            stack += &format!("    jne {}", target);
            return stack;
        }
        IrInstr::Equals {
            target,
            left,
            right,
            vtype,
        } => {
            let mut stack = "".to_owned();
            let l = compile_ir_op_to_x86(left, true, &mut stack, statics, statics_count);
            let r = compile_ir_op_to_x86(right, false, &mut stack, statics, statics_count);
            if l.is_address {
                stack += &format!("    mov rax, QWORD [{}]\n", l.as_ref());
            } else {
                stack += &format!("    mov rax, {}\n", l.as_ref());
            }
            if r.is_address {
                stack += &format!("    mov rbx, QWORD [{}]\n", r.as_ref());
            } else {
                stack += &format!("    mov rbx, {}\n", r.as_ref());
            }
            stack += &format!("    cmp rbx, rax\n");
            stack += &format!("    mov rbx, 0\n");
            stack += &format!("    mov rax, 1\n");
            stack += &format!("    cmovne rax, rbx\n");
            let v = compile_ir_op_to_x86(target, true, &mut stack, statics, statics_count);
            stack += &format!(
                "    mov {} [{}], rax\n",
                get_asmx86_type_name(vtype),
                v.as_ref()
            );
            return stack;
        }
        IrInstr::NotEquals {
            target,
            left,
            right,
            vtype,
        } => {
            let mut stack = "".to_owned();
            let l = compile_ir_op_to_x86(left, true, &mut stack, statics, statics_count);
            let r = compile_ir_op_to_x86(right, false, &mut stack, statics, statics_count);
            if l.is_address {
                stack += &format!("    mov rax, QWORD [{}]\n", l.as_ref());
            } else {
                stack += &format!("    mov rax, {}\n", l.as_ref());
            }
            if r.is_address {
                stack += &format!("    mov rbx, QWORD [{}]\n", r.as_ref());
            } else {
                stack += &format!("    mov rbx, {}\n", r.as_ref());
            }
            stack += &format!("    cmp rbx, rax\n");
            stack += &format!("    mov rbx, 0\n");
            stack += &format!("    mov rax, 1\n");
            stack += &format!("    cmove rax, rbx\n");
            let v = compile_ir_op_to_x86(target, true, &mut stack, statics, statics_count);
            stack += &format!(
                "    mov {} [{}], rax\n",
                get_asmx86_type_name(vtype),
                v.as_ref()
            );
            return stack;
        }
        IrInstr::Ret {
            to_return,
            stack_ptr: _,
        } => {
            let t = to_return.get_type();
            let mut out = "".to_owned();
            for _ in 0..*depth + 1 {
                out += match cmp_target {
                    Target::MacOs { arm: _ } => "    call _gc_pop_frame\n",
                    _ => "    call gc_pop_frame\n",
                };
            }
            let a = compile_ir_op_to_x86(to_return, true, &mut out, statics, statics_count);
            let types = t.flatten_to_basic_types();
            if t.get_size_bytes() == 0 {
            } else if t.get_size_bytes() <= 8 {
                if a.is_address {
                    out += &format!("    mov rax, QWORD [{}]\n", a.as_ref());
                } else {
                    out += &format!("    mov rax, {}\n", a.as_ref());
                }
            } else if t.get_size_bytes() <= 16 {
                let mut hit_int = false;
                let mut hit_float = false;
                if a.is_address {
                    match types[0] {
                        Type::FloatT => {
                            hit_float = true;
                            out += &format!("    movsd xmm0, [{}]\n", a.as_ref());
                        }
                        _ => {
                            hit_int = true;
                            out += &format!("    mov rax, QWORD [{}]\n", a.as_ref());
                        }
                    }
                    match types[1] {
                        Type::FloatT => {
                            if hit_float {
                                out += &format!("    movsd xmm1, [{}-8]\n", a.as_ref());
                            } else {
                                out += &format!("    movsd xmm0, [{}-8]\n", a.as_ref());
                            }
                        }
                        _ => {
                            if hit_int {
                                out += &format!("    mov rdx, QWORD [{}-8]\n", a.as_ref())
                            } else {
                                out += &format!("    mov rax, QWORD [{}-8]\n", a.as_ref());
                            }
                        }
                    }
                } else {
                    out += &format!("    mov rax, {}\n", a.as_ref());
                    out += &format!("    mov rdx, {}\n", a.as_ref());
                }
            } else {
                let max = to_return.get_type().get_size_bytes();
                let mut count = 0;
                out += "    mov rdi, QWORD [rbp -32]\n";
                while count < max {
                    out += &format!(
                        "    mov rax, QWORD [r11]\n    mov [rdi-{}], rax\n    sub r11, 8\n",
                        count
                    );
                    count += 8;
                }
            }
            out += "    mov rsp, rbp\n";
            out += "    sub rsp, 32\n";
            out += "    pop r10\n";
            out += "    pop rdx\n";
            out += "    pop rcx\n";
            out += "    pop rbx\n";
            out += "    mov rsp, rbp\n";
            out += "    pop rbp\n";
            out += "    ret\n";
            return out;
        }
        IrInstr::Not {
            target: _,
            value: _,
            vtype: _,
        } => {
            todo!();
        }
        IrInstr::Push {
            vtype,
            val_idx: _,
            stack_offset_of_value,
        } => match cmp_target {
            Target::MacOs { arm: _ } => {
                return format!("    lea rdi, [rbp-{stack_offset_of_value}]\n    lea rsi, [rel _{}]\n    call _gc_register_ptr", gc_function_name(vtype));
            }
            _ => {
                return format!("    lea rdi, [rbp-{stack_offset_of_value}]\n    lea rsi, [rel {}]\n    call gc_register_ptr", gc_function_name(vtype));
            }
        },
        IrInstr::Pop { vtype: _ } => {
            return "".to_owned();
        }
        IrInstr::BeginGcFrame => {
            *depth += 1;
            match cmp_target {
                Target::MacOs { arm: _ } => {
                    return format!("    call _gc_push_frame\n");
                }
                _ => {
                    return format!("    call gc_push_frame\n");
                }
            }
        }
        IrInstr::EndGcFrame => {
            *depth -= 1;
            match cmp_target {
                Target::MacOs { arm: _ } => {
                    return format!("    call _gc_pop_frame\n");
                }
                _ => {
                    return format!("    call gc_pop_frame\n");
                }
            }
        }
    }
    //todo!();
}
