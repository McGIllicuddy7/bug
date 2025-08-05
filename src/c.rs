pub use crate::compiler::{Function, Instruction, Type, Var};
pub struct CInfo {
    label_count: usize,
    var_count: usize,
}
impl CInfo {
    pub fn new() -> CInfo {
        CInfo {
            label_count: 0,
            var_count: 0,
        }
    }
}
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
    let p = base.split_whitespace();
    let mut base2 = String::new();
    for i in p {
        base2 += i;
    }
    base = base2;
    for i in &func.arguments {
        let t = mangle_type(i);
        base += &t;
    }
    base
}
pub fn mangle_var(idx: usize) -> String {
    format!("context.stack[{idx}]")
}
pub fn write_var(var: &Var) -> String {
    match var {
        Var::Basic {
            idx,
            vtype: _,
            byte_offset: _,
        } => mangle_var(*idx),
        Var::StringLiteral { v } => format!("to_bug_string(&context,{v})"),
        Var::IntegerLiteral { v } => {
            format!(
                "(bug_node_t){{.vtype = bug_integer, .car = (bug_value_t){{.integer = {}}}, .cdr= (bug_value_t){{.integer =0}}}}",
                v
            )
        }
        Var::DoubleLiteral { v } => {
            format!(
                "(bug_node_t){{.vtype = bug_double, .car = (bug_value_t){{.db = {}}}, .cdr= (bug_value_t){{.integer =0}}}}",
                v
            )
        }
        Var::BoolLiteral { v } => {
            format!(
                "(bug_node_t){{.vtype = bug_bool, .car = (bug_value_t){{.boolean= {}}}, .cdr= (bug_value_t){{.integer =0}}}}",
                if *v { "true" } else { "false" }
            )
        }
        Var::Capture { idx, vtype: _ } => {
            format!("context.captures[{}]", idx)
        }

        Var::FunctionPointerLiteral {
            name,
            args,
            return_type,
        } => {
            let st = mangle_function(
                name.as_ref(),
                &Function {
                    return_type: return_type.clone(),
                    arguments: args.clone(),
                    external: false,
                    ins: Vec::new(),
                },
            );
            let (tmp, tmp2) = if *return_type == Type::Void {
                ("void_fn", "bug_void_fn")
            } else {
                ("non_void_fn", "bug_non_void_fn")
            };

            format!(
                "(bug_node_t){{.vtype = {tmp2},.car= (bug_value_t){{.{} ={}}},.cdr = {{.ptr = 0}}}}",
                tmp, st
            )
        }
        Var::LambdaLiteral {
            name,
            args,
            return_type,
            captures,
        } => {
            let st = mangle_function(
                name.as_ref(),
                &Function {
                    return_type: return_type.clone(),
                    arguments: args.clone(),
                    external: false,
                    ins: Vec::new(),
                },
            );
            let mut capt_string = "bug_make_captures(&context,(int[]){".to_owned();
            for i in 0..captures.len() {
                match &captures[i] {
                    Var::Basic {
                        idx,
                        vtype: _,
                        byte_offset: _,
                    } => {
                        capt_string += &format!("{}", *idx);
                        if i < captures.len() - 1 {
                            capt_string += ", ";
                        }
                    }
                    _ => {
                        todo!()
                    }
                }
                capt_string += &format!("}},{})", captures.len());
            }
            if captures.is_empty() {
                capt_string = "0".to_string();
            }
            let (tmp, tmp2) = if *return_type == Type::Void {
                ("void_fn", "bug_void_fn")
            } else {
                ("non_void_fn", "bug_non_void_fn")
            };

            format!(
                "(bug_node_t){{.vtype = {tmp2},.car= (bug_value_t){{.{} ={}}},.cdr = {{.ptr = {}}}}}",
                tmp, st, capt_string
            )
        }
        _ => {
            todo!()
        }
    }
}
pub fn var_get(var: &Var) -> String {
    match var.get_type() {
        Type::Integer => "car.integer".to_owned(),
        Type::Bool => "car.boolean".to_owned(),
        Type::Char => "car.character".to_owned(),
        Type::Double => "car.db".to_owned(),
        Type::FunctionPointer {
            return_type,
            args: _,
        } => {
            if *return_type == Type::Void {
                "car.void_fn".to_owned()
            } else {
                "car._non_void_fn".to_owned()
            }
        }
        _ => todo!(),
    }
}
pub fn indent(depth: usize) -> String {
    let mut out = String::new();
    for _ in 0..depth {
        out += " ";
    }
    out
}

pub fn compile_instruction(depth: usize, instruction: &Instruction, cinfo: &mut CInfo) -> String {
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
                    let mut tmp = String::new();
                    out += &format!("({}.{})(&out_context)", &write_var(v), &var_get(v));
                    tmp += "out_context = context;\n";
                    tmp += &indent(depth);
                    tmp += "out_context.stack= out_context.stack_ptr;";
                    tmp += &format!("out_context.captures = {}.cdr.node;\n", &write_var(v));
                    for i in 0..arguments.len() {
                        tmp += &indent(depth);
                        tmp += &format!(
                            "*out_context.stack_ptr = {};out_context.stack_ptr++;\n",
                            &write_var(&arguments[i])
                        );
                    }
                    out = tmp + &out;
                    out += ";\n";
                    return out;
                }
                crate::compiler::Callable::Function { v } => match v.as_ref() {
                    "+" => {
                        binop = true;
                        //out = indent(depth);
                        out = String::new();
                        out += &format!(
                            "{}.{} = ",
                            &write_var(output.as_ref().unwrap()),
                            &var_get(output.as_ref().unwrap()),
                        );
                        out += &format!(
                            "{}.{}+{}.{};\n",
                            &write_var(&arguments[0]),
                            &var_get(&arguments[0]),
                            &write_var(&arguments[1]),
                            &var_get(&arguments[1])
                        );
                    }
                    "-" => {
                        binop = true;
                        //out = indent(depth);
                        out = String::new();
                        out += &format!(
                            "{}.{} = ",
                            &write_var(output.as_ref().unwrap()),
                            &var_get(output.as_ref().unwrap()),
                        );
                        out += &format!(
                            "{}.{}-{}.{}",
                            &write_var(&arguments[0]),
                            &var_get(&arguments[0]),
                            &write_var(&arguments[1]),
                            &var_get(&arguments[1])
                        );
                    }
                    "/" => {
                        binop = true;
                        //out = indent(depth);
                        out = String::new();
                        out += &format!(
                            "{}.{} = ",
                            &write_var(output.as_ref().unwrap()),
                            &var_get(output.as_ref().unwrap()),
                        );
                        out += &format!(
                            "{}.{}/{}.{}",
                            &write_var(&arguments[0]),
                            &var_get(&arguments[0]),
                            &write_var(&arguments[1]),
                            &var_get(&arguments[1])
                        );
                    }
                    "*" => {
                        binop = true;
                        //out = indent(depth);
                        out = String::new();
                        out += &format!(
                            "{}.{} = ",
                            &write_var(output.as_ref().unwrap()),
                            &var_get(output.as_ref().unwrap()),
                        );
                        out += &format!(
                            "{}.{}/{}.{}",
                            &write_var(&arguments[0]),
                            &var_get(&arguments[0]),
                            &write_var(&arguments[1]),
                            &var_get(&arguments[1])
                        );
                    }
                    "==" => {
                        binop = true;
                        //out = indent(depth);
                        out = String::new();
                        out += &format!(
                            "{}.{} = ",
                            &write_var(output.as_ref().unwrap()),
                            &var_get(output.as_ref().unwrap()),
                        );
                        out += &format!(
                            "{}.{}=={}.{}",
                            &write_var(&arguments[0]),
                            &var_get(&arguments[0]),
                            &write_var(&arguments[1]),
                            &var_get(&arguments[1])
                        );
                    }
                    "<=" => {
                        binop = true;
                        //out = indent(depth);
                        out = String::new();
                        out += &format!(
                            "{}.{} = ",
                            &write_var(output.as_ref().unwrap()),
                            &var_get(output.as_ref().unwrap()),
                        );
                        out += &format!(
                            "{}.{}<={}.{}",
                            &write_var(&arguments[0]),
                            &var_get(&arguments[0]),
                            &write_var(&arguments[1]),
                            &var_get(&arguments[1])
                        );
                    }

                    ">=" => {
                        binop = true;
                        //out = indent(depth);
                        out = String::new();
                        out += &format!(
                            "{}.{} = ",
                            &write_var(output.as_ref().unwrap()),
                            &var_get(output.as_ref().unwrap()),
                        );
                        out += &format!(
                            "{}.{}<={}.{}",
                            &write_var(&arguments[0]),
                            &var_get(&arguments[0]),
                            &write_var(&arguments[1]),
                            &var_get(&arguments[1])
                        );
                    }
                    _ => {
                        out += &format!(
                            "{}(&context)",
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
                let mut tmp = String::new();
                tmp += "out_context = context;\n";
                tmp += &indent(depth);
                tmp += "out_context.stack= out_context.stack_ptr;\n";
                for i in 0..arguments.len() {
                    tmp += &indent(depth);
                    tmp += &format!(
                        "*out_context.stack_ptr = {};out_context.stack_ptr++;\n",
                        &write_var(&arguments[i])
                    );
                }
                out = tmp + &out;
                out += ";\n";
            }
        }
        Instruction::Loop { condition, to_do } => {
            out += &format!("while({}.{}){{", write_var(condition), var_get(condition));
            for i in to_do {
                out += &compile_instruction(depth + 4, i, cinfo);
            }
            out += &indent(depth);
            out += "}\n";
        }
        Instruction::Branch {
            condition,
            if_true,
            if_false,
        } => {
            out += &format!("if({}.{}){{", write_var(condition), var_get(condition));
            for i in if_true {
                out += &compile_instruction(depth + 4, i, cinfo);
            }
            out += &indent(depth);
            out += "} else {\n";
            for i in if_false {
                out += &compile_instruction(depth + 4, i, cinfo);
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
                cinfo.var_count += 1;
                /*out += &format!("{} {};\n", mangle_type(vtype), mangle_var(*idx));*/
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
    let mut cinfo = CInfo::new();
    let mut out = format!(
        "{} {}(",
        if func.return_type == Type::Void {
            "void"
        } else {
            "bug_node_t"
        },
        mangle_function(name, func)
    );
    out += "bug_context_t * in_context)";
    /*for i in 0..func.arguments.len() {
        out += &mangle_type(&func.arguments[i]);
        out += " ";
        out += &mangle_var(i);
        if i < func.arguments.len() - 1 {
            out += ",";
        }
    }*/
    //    out += ")";
    if func.external {
        out = "extern ".to_owned() + &out;
        out += ";";
        out
    } else {
        out += "{\n    bug_context_t context = *in_context;bug_context_t out_context = context;\n";
        for i in &func.ins {
            out += &compile_instruction(4, i, &mut cinfo);
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
        out += "int main(int argc ,const char ** argv){ bug_context_t main_context = bug_create_context();int out =(int)(bug_main(&main_context).car.integer); gc_collect(main_context.stack, main_context.stack_ptr, main_context.heap); free_heap(&main_context); return out;}";
    }
    out
}
