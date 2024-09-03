use crate::ir::{IrInstr, IrOperand};
use crate::Type;
use std::collections::HashSet;
/*
left operand rax
right operand rbx
 */
#[allow(unused)]
pub fn compile_ir_operand_to_x86(op:&IrOperand, depth:&mut usize, used_types:&mut HashSet<Type>, statics_count:&mut usize, statics:&mut String)->String{
    match op{
    _=>{
        todo!();
    }
    }
    todo!();
}
pub fn compile_ir_instr_to_x86(instr: &IrInstr, depth :&mut usize, used_types:&mut HashSet<Type>, statics_count:&mut usize, statics:&mut String)->String{
   match instr{
        IrInstr::Add { target, left, right, vtype}=>{
            todo!();
        }
        IrInstr::Sub { target, left, right, vtype}=>{
            todo!();
        }
        IrInstr::Div { target, left, right, vtype}=>{
            todo!();
        }
        IrInstr::Mul { target, left, right, vtype}=>{
            todo!();
        }        
        IrInstr::And { target, left, right, vtype}=>{
            todo!();
        }        
        IrInstr::Or { target, left, right, vtype}=>{
            todo!();
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

        }
        IrInstr::EndScope{}=>{

        }
        IrInstr::Call { func_name, args }=>{

        }
        IrInstr::CallWithRet { target, func_name, args, vtype }=>{

        }
        IrInstr::Mov { left, right, vtype }=>{

        }
        IrInstr::Goto { target }=>{

        }
        IrInstr::Label { name }=>{

        }
        IrInstr::VariableDeclaration { name, vtype }=>{

        }
        IrInstr::CondGoto { cond, target }=>{

        }
        IrInstr::Equals { target, left, right, vtype }=>{

        }
        IrInstr::NotEquals { target, left, right, vtype }=>{

        }
        IrInstr::Ret { to_return }=>{

        }
        IrInstr::Not { target, value, vtype }=>{

        }
        IrInstr::Push { vtype, val_idx }=>{

        }
        IrInstr::Pop { vtype }=>{

        }
   } 
   return todo!();
}