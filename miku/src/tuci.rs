use crate::miku;
use miku::MikuInstr;
use miku::OpType;
use std::collections::HashSet;
pub fn tuci(prog: miku::MikuObject) -> String {
    let mut used_types = HashSet::new();
    let mut out = "#include \"miku_prelude.h\"\n".to_string();
    for i in &prog.extern_vars {
        used_types.insert(i.1.clone());
        out += &format!("extern {} {};\n", i.0, i.1.as_c_type())
    }
    out += "\n";
    for i in &prog.global_vars {
        used_types.insert(i.1.var_type.clone());
        out += &format!("{} {};\n", i.0, i.1.var_type.as_c_type());
    }
    out += "\n";
    for i in &prog.extern_functions {
        used_types.insert(i.return_type.clone());
        for j in &i.args {
            used_types.insert(j.var_type.clone());
        }
        out += &i.as_c_dec();
        out += ";\n";
    }
    out += "\n";
    for i in &prog.functions {
        used_types.insert(i.return_type.clone());
        for j in &i.args {
            used_types.insert(j.var_type.clone());
        }
        out += &i.as_c_dec();
        out += ";\n";
    }
    out += "\n";
    for i in prog.instructions {
        match i {
            MikuInstr::BeginFunction {
                name,
                label_index: _,
            } => {
                for j in &prog.functions {
                    if j.name == name {
                        out += &j.as_c_dec();
                        out += "{\n";
                        break;
                    }
                }
            }
            MikuInstr::EndFunction { name: _ } => {
                out += "}\n";
            }
            MikuInstr::Label {
                name,
                label_index: _,
            } => {
                out += &name;
                out += ":\n";
            }
            MikuInstr::Jmp { to, to_index: _ } => {
                out += &format!("    goto {};\n", to);
            }
            MikuInstr::Branch {
                to_true,
                to_true_index: _,
                to_false,
                to_false_index: _,
                var,
            } => {
                out += &format!(
                    "    if ({}){{goto {};}} else{{ goto {};}};\n",
                    var.as_c(),
                    to_true,
                    to_false
                );
            }
            MikuInstr::Call {
                to,
                to_index: _,
                return_var,
                args,
            } => {
                let mut tmp = format!("miku_{}(", to);
                for i in 0..args.len() {
                    used_types.insert(args[i].get_type());
                    tmp += &args[i].as_c();
                    if i != args.len() - 1 {
                        tmp += ",";
                    }
                }
                tmp += ");\n";
                if let Some(ret) = return_var {
                    tmp = format!("    {} = {}", ret.as_c(), tmp);
                } else {
                    tmp = format!("    {}", tmp);
                }
                out += &tmp;
            }
            MikuInstr::DeclVar {
                vtype,
                var_name,
                count: _,
            } => {
                let tmp = format!("    {} {};\n", vtype.as_c_type(), var_name);
                used_types.insert(vtype);
                out += &tmp;
            }
            MikuInstr::DeclStatic {
                vtype: _,
                var_name: _,
                count: _,
                base_value: _,
            } => {}
            MikuInstr::DeclExtern {
                vtype: _,
                var_name: _,
            } => {}
            MikuInstr::Assign { left, right } => {
                out += &format!("    {} = {};\n", left.as_c(), right.as_c());
            }
            MikuInstr::DerefAssign { left, right } => {
                out += &format!("    *{} = {};\n", left.as_c(), right.as_c());
            }
            MikuInstr::GetRef { assigned_to, of } => {
                out += &format!("    {} = &{};\n", assigned_to.as_c(), of.as_c());
            }
            MikuInstr::BinOp {
                left,
                right,
                output,
                operation,
            } => {
                let op = match operation {
                    OpType::Add => "+",
                    OpType::Sub => "-",
                    OpType::Div => "/",
                    OpType::Mul => "*",
                    OpType::Or => "||",
                    OpType::And => "&&",
                    OpType::CmpG => ">",
                    OpType::CmpE => "==",
                    OpType::CmpL => "<",
                };
                let tmp = format!(
                    "    {} = {} {} {};\n",
                    output.as_c(),
                    left.as_c(),
                    op,
                    right.as_c()
                );
                out += &tmp;
            }
            MikuInstr::Return { value } => {
                if let Some(v) = value {
                    out += &format!("    return {};\n", v.as_c());
                } else {
                    out += &format!("    return;\n");
                }
            }
            _ => {}
        }
    }
    out
}
