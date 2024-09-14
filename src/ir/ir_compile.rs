use std::collections::HashSet;

use super::intermediate_representation::*;
use crate::gc_function_name;
use crate::name_mangle_type;
use crate::Type;
pub fn compile_ir_op_to_c(op: &IrOperand) -> String {
    match op {
        IrOperand::StacKOperand {
            var_idx: _,
            name,
            stack_offset: _,
            vtype: _,
        } => {
            return name.as_ref().to_owned();
        }
        IrOperand::Name { name, vtype: _ } => {
            return name.as_ref().to_owned();
        }
        IrOperand::Deref { to_deref } => {
            return "(*".to_owned() + &(compile_ir_op_to_c(to_deref)+")");
        }
        IrOperand::TakeRef { to_ref } => {
              return "(&".to_owned() + &(compile_ir_op_to_c(to_ref)+")");
        }
        IrOperand::StringLiteral { value } => {
            return value.as_ref().to_owned();
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
            return format!("'{}'", *value as char)
        }
        IrOperand::ArrayAccess { base, value } => {
            let base = compile_ir_op_to_c(base);
            return format!("{}.start[{}]", base, compile_ir_op_to_c(value));
        }
    }
}
fn depth_calc(depth:&usize)->String{
    let mut out = "".to_owned();
    for _ in 0..*depth{
        out += "    ";
    }
    return out;
}
macro_rules!  depth_format {
    ($depth:expr, $lit:literal,$($arg:tt)*) => {
       depth_calc($depth)+&format!($lit,$($arg)*) 
    };    
    ($depth:expr, $lit:tt)=>{
        depth_calc($depth)+&$lit.to_owned()
    };
}
pub fn compile_ir_instr_to_c(instr: &IrInstr, depth :&mut usize, used_types:&mut HashSet<Type>) -> String {
    match instr {
        IrInstr::VariableDeclaration { name, vtype } => {
            used_types.insert(vtype.clone());
            let out = 
            match vtype{
                Type::IntegerT| Type::BoolT |Type::PointerT { ptr_type:_ }| Type::CharT | Type::FloatT=>{
                    depth_format!(depth, "{} {} = 0;\n",name_mangle_type(vtype),name)
                }
                _=>{
                   depth_format!(depth, "{} {} = {{0}};\n", name_mangle_type(vtype), name)
                }
            };
            return out + &depth_format!(depth, "gc_register_ptr(&{}, {});", name,gc_function_name(vtype));
        }
        IrInstr::Mov { left, right, vtype} => {
            used_types.insert(vtype.clone());
            depth_format!(depth, "{} = {};", compile_ir_op_to_c(left), compile_ir_op_to_c(right))
        }
        IrInstr::Label { name } => {
            name.to_owned()+":"
        }
        IrInstr::Goto { target } => {
            depth_format!(depth, "goto {};",target)
        }
        IrInstr::CondGoto { cond, target } => {
            depth_format!(depth, "if ({}) goto {};", compile_ir_op_to_c(cond), target)
        }
        IrInstr::Add {
            target,
            left,
            right,
            vtype,
        } => {
            used_types.insert(vtype.clone());
            depth_format!(depth, "{} = {}+{};", compile_ir_op_to_c(target), compile_ir_op_to_c(left), compile_ir_op_to_c(right))
        }
        IrInstr::Sub {
            target,
            left,
            right,
            vtype,
        } => {
            used_types.insert(vtype.clone());
            depth_format!(depth, "{} = {}-{};", compile_ir_op_to_c(target), compile_ir_op_to_c(left), compile_ir_op_to_c(right))
        }
        IrInstr::Mul {
            target,
            left,
            right,
            vtype,
        } => {
            used_types.insert(vtype.clone());
            depth_format!(depth, "{} = {}*{};", compile_ir_op_to_c(target), compile_ir_op_to_c(left), compile_ir_op_to_c(right))
        }
        IrInstr::Div {
            target,
            left,
            right,
            vtype,
        } => {
            used_types.insert(vtype.clone());
            depth_format!(depth, "{} = {}/{};", compile_ir_op_to_c(target), compile_ir_op_to_c(left), compile_ir_op_to_c(right))
        }
        IrInstr::And {
            target,
            left,
            right,
            vtype,
        } => {
            used_types.insert(vtype.clone());
            depth_format!(depth, "{} = {} && {};", compile_ir_op_to_c(target), compile_ir_op_to_c(left), compile_ir_op_to_c(right))
        }
        IrInstr::Or {
            target,
            left,
            right,
            vtype,
        } => {
            used_types.insert(vtype.clone());
            depth_format!(depth, "{} = {} || {};", compile_ir_op_to_c(target), compile_ir_op_to_c(left), compile_ir_op_to_c(right))
        }
        IrInstr::Equals {
            target,
            left,
            right,
            vtype,
        } => {
            used_types.insert(vtype.clone());
            depth_format!(depth, "{} = {} == {};", compile_ir_op_to_c(target), compile_ir_op_to_c(left), compile_ir_op_to_c(right))
        }
        IrInstr::NotEquals {
            target,
            left,
            right,
            vtype,
        } => {
            used_types.insert(vtype.clone());
            depth_format!(depth, "{} = {} != {};", compile_ir_op_to_c(target), compile_ir_op_to_c(left), compile_ir_op_to_c(right))
        }
        IrInstr::GreaterThan {
            target,
            left,
            right,
            vtype,
        } => {
            used_types.insert(vtype.clone());
            depth_format!(depth, "{} = {} > {};", compile_ir_op_to_c(target), compile_ir_op_to_c(left), compile_ir_op_to_c(right))
        }
        IrInstr::GreaterThanOrEq {
            target,
            left,
            right,
            vtype,
        } => {
            used_types.insert(vtype.clone());
            depth_format!(depth, "{} = {} >= {};", compile_ir_op_to_c(target), compile_ir_op_to_c(left), compile_ir_op_to_c(right))
        }
        IrInstr::LessThan {
            target,
            left,
            right,
            vtype,
        } => {
            used_types.insert(vtype.clone());
            depth_format!(depth, "{} = {} < {};", compile_ir_op_to_c(target), compile_ir_op_to_c(left), compile_ir_op_to_c(right))
        }
        IrInstr::LessThanOrEq {
            target,
            left,
            right,
            vtype,
        } => {
            used_types.insert(vtype.clone());
            depth_format!(depth, "{} = {} <= {};", compile_ir_op_to_c(target), compile_ir_op_to_c(left), compile_ir_op_to_c(right))
        }
        IrInstr::Not {
            target,
            value,
            vtype,
        } => {
            used_types.insert(vtype.clone());
            depth_format!(depth, "{} = !{};", compile_ir_op_to_c(target), compile_ir_op_to_c(value))
        }
        IrInstr::Call { func_name, args , stack_ptr_when_called:_} => {
            let mut base = depth_format!(depth, "{}(", func_name);
            let mut is_start = true;
            for i in args{
                if !is_start{
                    base += ",";
                }
                base += &compile_ir_op_to_c(i);
                is_start = false;
            }
            base += ");";
            return base;
        }
        IrInstr::CallWithRet {
            target,
            func_name,
            args,
            vtype,
            stack_ptr_when_called:_
        } => {
            used_types.insert(vtype.clone());
            let mut base = depth_format!(depth, "{} = {}(", compile_ir_op_to_c(target),func_name);
            let mut is_start = true;
            for i in args{
                if !is_start{
                    base += ",";
                }
                base += &compile_ir_op_to_c(i);
                is_start = false;
            }
            base += ");";
            return base;
        }
        IrInstr::Ret { to_return , stack_ptr:_} => {
            return depth_format!(depth,"gc_pop_frame();\n")+&depth_format!(depth, "return {};", compile_ir_op_to_c(to_return));
        }
        IrInstr::Push { vtype, val_idx } => {
            used_types.insert(vtype.clone());
            return depth_format!(depth, "{} tmp{};\n", name_mangle_type(vtype), val_idx)+&depth_format!(depth, "gc_register_ptr(&tmp{},{});",val_idx,gc_function_name(&vtype));
        }
        IrInstr::Pop { vtype} => {
            return depth_format!(depth, "//{}", name_mangle_type(vtype));
        }
        IrInstr::BeginScope => {
            let out = depth_format!(depth, "{\n");
            *depth += 1;
            return out;
        }
        IrInstr::EndScope {stack_ptr:_}=> {            
            *depth -= 1;
            let out = "\n".to_owned()+&depth_format!(depth, "}");
            return out;
        }
    }
}