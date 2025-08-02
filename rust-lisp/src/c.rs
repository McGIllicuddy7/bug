pub use crate::compiler::{Function, Instruction, Type, Var};
pub fn mangle_type(vtype: &Type) -> String {
    match vtype {
        Type::String => "bug_string".to_owned(),
        Type::Integer => "long".to_owned(),
        Type::Char => "char".to_owned(),
        Type::Double => "double".to_owned(),
        Type::Void => "void".to_owned(),
        _ => {
            todo!()
        }
    }
}
pub fn mangle_function(name: &str, func: &Function) -> String {
    let mut base = "bug_".to_owned() + name;
    for i in &func.arguments {
        let t = mangle_type(i);
        base += &t;
    }
    base
}
pub fn mangle_var(idx: usize) -> String {
    format!("x{idx}")
}
pub fn write_var(var: &Var) -> String {
    match var {
        Var::Basic {
            idx,
            vtype: _,
            byte_offset: _,
        } => mangle_var(*idx),
        Var::StringLiteral { v } => format!("to_bug_string({v})"),
        Var::IntegerLiteral { v } => format!("{v}"),
        Var::DoubleLiteral { v } => format!("{v}"),
        Var::BoolLiteral { v } => format!("{v}"),
        _ => {
            todo!();
        }
    }
}
pub fn indent(depth: usize) -> String {
    let mut out = String::new();
    for _ in 0..depth {
        out += " ";
    }
    out
}

pub fn compile_instruction(depth: usize, instruction: &Instruction) -> String {
    let mut out = indent(depth);
    match instruction {
        Instruction::FunctionCall {
            to_call,
            arguments,
            output,
        } => {
            if let Some(p) = output {
                if p.get_type() != Type::Void {
                    out += &format!("{} = ", write_var(p));
                }
            }
            let mut binop = false;
            match to_call {
                crate::compiler::Callable::Variable { v } => {
                    out += &format!("{}(", &write_var(v));
                }
                crate::compiler::Callable::Function { v } => match v.as_ref() {
                    "+" => {
                        binop = true;
                        out += &format!(
                            "{}+{};\n",
                            &write_var(&arguments[0]),
                            &write_var(&arguments[1])
                        );
                    }
                    "-" => {
                        binop = true;
                        out += &format!(
                            "{}-{};\n",
                            &write_var(&arguments[0]),
                            &write_var(&arguments[1])
                        );
                    }
                    "/" => {
                        binop = true;
                        out += &format!(
                            "{}/{};\n",
                            &write_var(&arguments[0]),
                            &write_var(&arguments[1])
                        );
                    }
                    "*" => {
                        binop = true;
                        out += &format!(
                            "{}*{};\n",
                            &write_var(&arguments[0]),
                            &write_var(&arguments[1])
                        );
                    }
                    "==" => {
                        binop = true;
                        out += &format!(
                            "{}=={};\n",
                            &write_var(&arguments[0]),
                            &write_var(&arguments[1])
                        );
                    }
                    "<=" => {
                        binop = true;
                        out += &format!(
                            "{}<={};\n",
                            &write_var(&arguments[0]),
                            &write_var(&arguments[1])
                        );
                    }
                    ">=" => {
                        binop = true;
                        out += &format!(
                            "{}>={};\n",
                            &write_var(&arguments[0]),
                            &write_var(&arguments[1])
                        );
                    }
                    _ => {
                        out += &format!(
                            "{}(",
                            mangle_function(
                                v,
                                &Function {
                                    return_type: output.as_ref().unwrap().get_type(),
                                    arguments: arguments.iter().map(|i| i.get_type()).collect(),
                                    ins: vec![],
                                    external: true
                                }
                            )
                        );
                    }
                },
            }
            if !binop {
                for i in 0..arguments.len() {
                    out += &write_var(&arguments[i]);
                    if i < arguments.len() - 1 {
                        out += ","
                    }
                }
                out += ");\n";
            }
        }
        Instruction::Loop { condition, to_do } => {
            out += &format!("while({}){{", write_var(condition));
            for i in to_do {
                out += &compile_instruction(depth + 4, i);
            }
            out += &indent(depth);
            out += "}\n";
        }
        Instruction::Branch {
            condition,
            if_true,
            if_false,
        } => {
            out += &format!("if({}){{", write_var(condition));
            for i in if_true {
                out += &compile_instruction(depth + 4, i);
            }
            out += &indent(depth);
            out += "} else {\n";
            for i in if_false {
                out += &compile_instruction(depth + 4, i);
            }
            out += &indent(depth);
            out += "}\n";
        }
        Instruction::Declare { to_declare } => match to_declare {
            Var::Basic {
                idx,
                vtype,
                byte_offset: _,
            } => {
                if *vtype == Type::Void {
                    return "".to_string();
                }
                out += &format!("{} {};\n", mangle_type(vtype), mangle_var(*idx));
            }
            _ => {
                todo!()
            }
        },
        Instruction::Assignment { left, right } => {
            out += &format!("{} = {};\n", write_var(left), write_var(right));
        }
        Instruction::Return { to_return } => {
            if let Some(v) = to_return {
                out += &format!("return {};\n", write_var(v));
            } else {
                out += "return;\n";
            }
        }
    }
    out
}
pub fn compile_function(name: &str, func: &Function) -> String {
    let mut out = format!(
        "{} {}(",
        mangle_type(&func.return_type),
        mangle_function(name, func)
    );
    for i in 0..func.arguments.len() {
        out += &mangle_type(&func.arguments[i]);
        out += " ";
        out += &mangle_var(i);
        if i < func.arguments.len() - 1 {
            out += ",";
        }
    }
    out += ")";
    if func.external {
        out = "extern ".to_owned() + &out;
        out += ";";
        out
    } else {
        out += "{\n";
        for i in &func.ins {
            out += &compile_instruction(4, i);
        }
        out += "}\n";
        out
    }
}
pub fn compile_to_c(comp: &crate::compiler::Compiler) -> String {
    let mut out = String::from("#include \"prelude.h\"\n");
    for i in &comp.global_functions {
        let name = i.0;
        if name == "+"
            || name == "-"
            || name == "*"
            || name == "/"
            || name == "=="
            || name == "<="
            || name == ">="
        {
            continue;
        }
        let funcs = i.1;
        for j in funcs {
            let mut tmp = j.clone();
            tmp.external = true;
            out += &compile_function(name.as_str(), &tmp);
            out += "\n";
        }
    }
    let mut has_main = false;
    for i in &comp.global_functions {
        let name = i.0;
        if name == "main" {
            has_main = true;
        }
        if name == "+"
            || name == "-"
            || name == "*"
            || name == "/"
            || name == "=="
            || name == "<="
            || name == ">="
        {
            continue;
        }
        let funcs = i.1;
        for j in funcs {
            if !j.external {
                out += &compile_function(name.as_str(), j);
                out += "\n";
            }
        }
    }
    if has_main {
        out += "int main(int argc ,const char ** argv){ return (int)(bug_main());}";
    }
    out
}
