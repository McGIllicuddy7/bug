use crate::ir::{IrInstr, IrOperand};
use crate::Type;
use std::collections::HashSet;
fn get_asmx86_type_name(vtype:&Type)->&'static str{
    match vtype{
        Type::BoolT| Type::CharT=>{
            return "BYTE"
        }
        _=>{
            return "QWORD"
        }
    }
}
/*
left operand rcx
right operand rdx
 */
fn get_sreg(left:bool)->String{
    if left{
        return "rcx".to_owned();
    } else{
        return "rdx".to_owned()
    }
}
#[allow(unused)]
fn compile_ir_op_to_x86(op:&IrOperand, left:bool,stack:&mut String, statics:&mut String, statics_count:&mut usize)->String{
    match op{
        IrOperand::ArrayAccess { base, value }=>{
            let base = compile_ir_op_to_x86(base, left, stack, statics, statics_count);
            *stack += &format!("    lea rax, {}\n", base);
            let value = compile_ir_op_to_x86(value, left, stack, statics, statics_count);
            *stack += &format!("    add rax, {}", value);
            *stack += &format!("    mov {}, rax", get_sreg(left));
            return get_sreg(left);
        }
        IrOperand::CharLiteral { value }=>{
            return format!("{value}");
        }
        IrOperand::IntLiteral { value }=>{
            return format!("{value}");
        }
        IrOperand::FloatLiteral { value }=>{
            return format!("{value}");
        }
        IrOperand::Deref { to_deref }=>{
            let base = compile_ir_op_to_x86(&to_deref,left, stack, statics, statics_count);
            *stack += &format!("    mov {},[{}]\n", get_sreg(left), base);
            return get_sreg(left);
        }
        IrOperand::StacKOperand { var_idx:_, name:_, stack_offset, vtype }=>{
            *stack += &format!("    lea {}, [rbp-{}]\n",get_sreg(left), stack_offset+8 );
            return get_sreg(left);
        }
        IrOperand::Name { name, vtype }=>{
            *stack += &format!("    lea {}, [rel {}]\n",name, get_sreg(left));
            return get_sreg(left);
        }
        IrOperand::TakeRef { to_ref }=>{
            let base =compile_ir_op_to_x86(&to_ref, left, stack, statics, statics_count);
            *stack += &format!("lea {}, {}\n", get_sreg(left),base);
            return get_sreg(left);
        } 
        IrOperand::StringLiteral { value }=>{
            todo!();
        }
        IrOperand::FieldAccess { base, name }=>{
            let offset = base.get_type().get_variable_offset(name).expect("contains");
            let btype = base.get_type().get_variable_type(name);
            match base.as_ref(){
                IrOperand::StacKOperand { var_idx:_, name:_, stack_offset, vtype }=>{
                    *stack += &format!("    lea {}, [rbp-{}]\n",get_sreg(left), stack_offset+8+offset );
                    return get_sreg(left);
                }
                _=>{
                    let base = compile_ir_op_to_x86(base, left, stack, statics, statics_count);
                    *stack += &format!("add {}, {}", base, offset);
                    return get_sreg(left);
                }
            }
        }
    }
    todo!();
}
#[allow(unused)]
fn generate_mov_instr(left:&IrOperand, right:&IrOperand)->String{
    todo!();
}
pub fn compile_ir_instr_to_x86(instr: &IrInstr, _depth :&mut usize, _used_types:&mut HashSet<Type>, statics_count:&mut usize, statics:&mut String)->String{
   match instr{
        IrInstr::Add { target, left, right, vtype}=>{
            let mut stack = "".to_owned();
            let l = compile_ir_op_to_x86(left, true, &mut stack, statics, statics_count);
            let r = compile_ir_op_to_x86(right, false, &mut stack, statics, statics_count);
              if l.as_bytes()[0]== b'r'{
                stack += &format!("    mov rax, QWORD [rcx]\n");
            } else{
                stack += &format!("    mov rax, {}\n",l);
            }
            if r.as_bytes()[0]== b'r'{
                stack += &format!("    mov rbx, QWORD [rdx]\n");
            } else{
                stack += &format!("    mov rbx, {}\n",r);
            }
            stack += &format!("    add rax, rbx\n");
            let v = compile_ir_op_to_x86(target, true, &mut stack, statics, statics_count);
            stack += &format!("    mov {} [{}], rax\n", get_asmx86_type_name(vtype), v);
            return stack;
        }
        IrInstr::Sub { target, left, right, vtype}=>{
            let mut stack = "".to_owned();
            let l = compile_ir_op_to_x86(left, true, &mut stack, statics, statics_count);
            let r = compile_ir_op_to_x86(right, false, &mut stack, statics, statics_count);
            if l.as_bytes()[0]== b'r'{
                stack += &format!("    mov rax, QWORD [rcx]\n");
            } else{
                stack += &format!("    mov rax, {}\n",l);
            }
            if r.as_bytes()[0]== b'r'{
                stack += &format!("    mov rbx, QWORD [rdx]\n");
            } else{
                stack += &format!("    mov rbx, {}\n",r);
            }
            stack += &format!("    sub rax, rbx\n");
            let v = compile_ir_op_to_x86(target, true, &mut stack, statics, statics_count);
            stack += &format!("    mov, {} [{}], rax\n", get_asmx86_type_name(vtype), v);
            return stack;
        }
        IrInstr::Div { target, left, right, vtype}=>{
            let mut stack = "".to_owned();
            let l = compile_ir_op_to_x86(left, true, &mut stack, statics, statics_count);
            let r = compile_ir_op_to_x86(right, false, &mut stack, statics, statics_count);
            if l.as_bytes()[0]== b'r'{
                stack += &format!("    mov rax, QWORD [rcx]\n");
            } else{
                stack += &format!("    mov rax, {}\n",l);
            }
            if r.as_bytes()[0]== b'r'{
                stack += &format!("    mov rdx, QWORD [rdx]\n");
            } else{
                stack += &format!("    mov rdx, {}\n",r);
            }
            stack += &format!("    idiv rax, rdx\n");
            let v = compile_ir_op_to_x86(target, true, &mut stack, statics, statics_count);
            stack += &format!("    mov, {} [{}], rax\n", get_asmx86_type_name(vtype), v);
            return stack;
        }
        IrInstr::Mul { target, left, right, vtype}=>{
            let mut stack = "".to_owned();
            let l = compile_ir_op_to_x86(left, true, &mut stack, statics, statics_count);
            let r = compile_ir_op_to_x86(right, false, &mut stack, statics, statics_count);
            if l.as_bytes()[0]== b'r'{
                stack += &format!("    mov rax, QWORD [rcx]\n");
            } else{
                stack += &format!("    mov rax, {}\n",l);
            }
            if r.as_bytes()[0]== b'r'{
                stack += &format!("    mov rbx, QWORD [rdx]\n");
            } else{
                stack += &format!("    mov rbx, {}\n",r);
            }
            stack += &format!("    imul rax, rbx\n");
            let v = compile_ir_op_to_x86(target, true, &mut stack, statics, statics_count);
            stack += &format!("    mov {} [{}], rax\n", get_asmx86_type_name(vtype), v);
            return stack;
        }        
        IrInstr::And { target, left, right, vtype}=>{
            let mut stack = "".to_owned();
            let l = compile_ir_op_to_x86(left, true, &mut stack, statics, statics_count);
            let r = compile_ir_op_to_x86(right, false, &mut stack, statics, statics_count);
            if l.as_bytes()[0]== b'r'{
                stack += &format!("    mov rax, QWORD [rcx]\n");
            } else{
                stack += &format!("    mov rax, rcx\n");
            }
            if r.as_bytes()[0]== b'r'{
                stack += &format!("    mov rbx, QWORD [rdx]\n");
            } else{
                stack += &format!("    mov rbx, rdx\n");
            }
            stack += &format!("    and rax, rbx\n");
            let v = compile_ir_op_to_x86(target, true, &mut stack, statics, statics_count);
            stack += &format!("    mov, {} [{}], rax\n", get_asmx86_type_name(vtype), v);
            return stack;
        }        
        IrInstr::Or { target, left, right, vtype}=>{
            let mut stack = "".to_owned();
            let l = compile_ir_op_to_x86(left, true, &mut stack, statics, statics_count);
            let r = compile_ir_op_to_x86(right, false, &mut stack, statics, statics_count);
            if l.as_bytes()[0]== b'r'{
                stack += &format!("    mov rax, QWORD [rcx]\n");
            } else{
                stack += &format!("    mov rax, rcx\n");
            }
            if r.as_bytes()[0]== b'r'{
                stack += &format!("    mov rbx, QWORD [rdx]\n");
            } else{
                stack += &format!("    mov rbx, rdx\n");
            }
            stack += &format!("    or rax, rbx\n");
            let v = compile_ir_op_to_x86(target, true, &mut stack, statics, statics_count);
            stack += &format!("    mov, {} [{}], rax\n", get_asmx86_type_name(vtype), v);
            return stack;
        }
        IrInstr::GreaterThan { target, left, right, vtype }=>{
            todo!();
        }
        IrInstr::GreaterThanOrEq { target, left, right, vtype }=>{
            todo!();
        }
        IrInstr::LessThan { target, left, right, vtype }=>{
            todo!();
        }
        IrInstr::LessThanOrEq { target, left, right, vtype }=>{
            todo!();
        }
        IrInstr::BeginScope{}=>{
            return format!("    call _push_frame()");
        }
        IrInstr::EndScope{}=>{
            return format!("    call _pop_frame()");
        }
        IrInstr::Call { func_name, args }=>{
            todo!();
        }
        IrInstr::CallWithRet { target, func_name, args, vtype }=>{
            todo!();
        }
        IrInstr::Mov { left, right, vtype }=>{
            let mut stack = "".to_owned();
            let l = compile_ir_op_to_x86(left, true,&mut stack, statics, statics_count);
            let r = compile_ir_op_to_x86(left, false,&mut stack, statics, statics_count);
            let total = vtype.get_size_bytes();
            let mut count = 0;
            stack += &format!("  mov rax, {}\n",l);
            stack += &format!("  mov rbx, {}\n", r);
            while count<total{
                stack += &format!("    mov rcx,QWORD [rbx]\n");
                stack += &format!("    mov QWORD [rax], rcx\n");
                stack += &format!("    add rax,8\n");
                stack += &format!("    add rbx, 8\n");
                count += 8;
            }
            stack += &format!("") ;
            return stack;
        }
        IrInstr::Goto { target }=>{
            return format!("    jmp {target}");
        }
        IrInstr::Label { name }=>{
            return format!("{name}:");
        }
        IrInstr::VariableDeclaration { name:_, vtype }=>{
            let mut total = 0;
            let sz = vtype.get_size_bytes();
            let mut out = format!("   xor r10,r10;\n");
            while total<sz{
                out += "    push r10";
                total += 8;
            }
            return out;
        }
        IrInstr::CondGoto { cond, target }=>{
            let mut stack = "".to_owned();
            let cond = compile_ir_op_to_x86(cond, true, &mut stack, statics, statics_count);
            stack += &format!("cmp {}, 0\n", cond);
            stack += &format!("jne {}", target);
            return stack;
        }
        IrInstr::Equals { target, left, right, vtype }=>{
            todo!();
        }
        IrInstr::NotEquals { target, left, right, vtype }=>{
            todo!();
        }
        IrInstr::Ret { to_return }=>{
            todo!();
        }
        IrInstr::Not { target, value, vtype }=>{
            todo!();
        }
        IrInstr::Push { vtype, val_idx:_ }=>{
            let mut total = 0;
            let sz = vtype.get_size_bytes();
            let mut out = format!("   xor r10, r10\n");
            while total<sz{
                out += "    push r10\n";
                total += 8;
            }
            return out;
        }
        IrInstr::Pop { vtype }=>{
            let mut total = 0;
            let sz = vtype.get_size_bytes();
            let mut out = format!("");
            while total<sz{
                out += "    pop r10\n";
                total += 8;
            }
            out += "    xor r10,r10\n";
            return out;
        }
   } 
   todo!();
}