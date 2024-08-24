pub use crate::types::*;
use crate::validation::alide_parens;
use crate::validation::validate_ast;
#[derive(Debug, Clone, Copy)]
pub enum GlobalTypes<'a> {
    StructDef { text: &'a [Token<'a>] },
    FunctionDef { text: &'a [Token<'a>] },
    GlobalDef { text: &'a [Token<'a>] },
    IncludeDirective{text:&'a [Token<'a>]}
}

pub fn calc_close_paren(tokens: &[Token<'_>], base_idx: usize) -> Option<usize> {
    let mut idx = base_idx + 1;
    let mut paren_count = 1;
    if tokens[base_idx] != "(" {
        println!("base wasn't a paren, instead found {} line {}",tokens[base_idx].string, tokens[base_idx].line);
        return None;
    }
    while idx < tokens.len() {
        if tokens[idx] == "(" {
            paren_count += 1;
        } else if tokens[idx] == ")" {
            paren_count -= 1;
        }
        if paren_count == 0 {
            return Some(idx);
        }
        idx += 1;
    }
    println!("failed to find next paren");
    return None;
}

pub fn calc_close_scope(tokens: &[Token<'_>], base_idx: usize) -> Option<usize> {
    let mut idx = base_idx;
    let mut paren_count = 0;
    let mut hit_paren = false;
    while idx < tokens.len() {
        //println!("calc close scope:{:#?}", tokens[idx]);
        if tokens[idx] == "{" {
            paren_count += 1;
            hit_paren = true;
        } else if tokens[idx] == "}" && hit_paren {
            paren_count -= 1;
        }
        if paren_count < 1 {
            return Some(idx);
        }
        idx += 1;
    }
    println!("failed to find next scope end");
    return None;
}
#[allow(unused)]
pub fn calc_close_block(tokens: &[Token<'_>], base_idx: usize) -> Option<usize> {
    let mut idx = base_idx + 1;
    let mut paren_count = 1;
    if tokens[base_idx] != "[" {
        println!("base wasn't a [");
        return None;
    }
    while idx < tokens.len() {
        if tokens[idx] == "[" {
            paren_count += 1;
        } else if tokens[idx] == "]" {
            paren_count -= 1;
        }
        if paren_count == 0 {
            return Some(idx);
        }
        idx += 1;
    }
    println!("failed to find next ]");
    return None;
}

pub fn split_by<'a>(string: &'a str, value: char) -> Vec<&'a str> {
    let mut out: Vec<&'a str> = vec![];
    let mut last = 0;
    let mut current_idx = 0;
    for i in string.chars() {
        if i == value {
            if last != current_idx {
                out.push(&string[last..current_idx] as &str);
            }
            out.push(&string[current_idx..current_idx + 1] as &str);
            last = current_idx + 1;
        }
        current_idx += 1;
    }
    if last != current_idx {
        out.push(&string[last..] as &str);
    }
    return out;
}

pub fn token_split_by<'a>(token: &Token<'a>, value: char) -> Vec<Token<'a>> {
    split_by(token.string, value)
        .into_iter()
        .map(|i| Token {
            string: i,
            line: token.line,
        })
        .collect()
}

pub fn collect_tokens<'a>(tokens: &[Token<'a>]) -> Vec<Token<'a>> {
    let mut out = vec![];
    let mut count = 0;
    while count < tokens.len() {
        if count < tokens.len() - 1 {
            if tokens[count] == "=" && tokens[count + 1] == "=" {
                let mut token = Token {
                    string: tokens[count].string,
                    line: tokens[count].line,
                };
                unsafe {
                    let strr = token.string.as_ref() as &[u8];
                    let ptr = strr.as_ptr();
                    let len = strr.len() + tokens[count + 1].string.len();
                    let new_str = slice::from_raw_parts(ptr, len);
                    let new_string = str::from_utf8(new_str);
                    if let Ok(s) = new_string {
                        token.string = s;
                        count += 1;
                    }
                }
                out.push(token);
            } else if tokens[count] == "-" && tokens[count + 1] == ">" {
                let mut token = Token {
                    string: tokens[count].string,
                    line: tokens[count].line,
                };
                unsafe {
                    let strr = token.string.as_ref() as &[u8];
                    let ptr = strr.as_ptr();
                    let len = strr.len() + tokens[count + 1].string.len();
                    let new_str = slice::from_raw_parts(ptr, len);
                    let new_string = str::from_utf8(new_str);
                    if let Ok(s) = new_string {
                        token.string = s;
                        count += 1;
                    }
                }
                out.push(token);
            } else if tokens[count] == "<" && tokens[count + 1] == "=" {
                let mut token = Token {
                    string: tokens[count].string,
                    line: tokens[count].line,
                };
                unsafe {
                    let strr = token.string.as_ref() as &[u8];
                    let ptr = strr.as_ptr();
                    let len = strr.len() + tokens[count + 1].string.len();
                    let new_str = slice::from_raw_parts(ptr, len);
                    let new_string = str::from_utf8(new_str);
                    if let Ok(s) = new_string {
                        token.string = s;
                        count += 1;
                    }
                }
                out.push(token);
            } else if tokens[count] == ">" && tokens[count + 1] == "=" {
                let mut token = Token {
                    string: tokens[count].string,
                    line: tokens[count].line,
                };
                unsafe {
                    let strr = token.string.as_ref() as &[u8];
                    let ptr = strr.as_ptr();
                    let len = strr.len() + tokens[count + 1].string.len();
                    let new_str = slice::from_raw_parts(ptr, len);
                    let new_string = str::from_utf8(new_str);
                    if let Ok(s) = new_string {
                        token.string = s;
                        count += 1;
                    }
                }
                out.push(token);
            } else {
                out.push(tokens[count].clone());
            }
        } else {
            out.push(tokens[count].clone());
        }
        count += 1;
    }
    return out;
}

fn is_numbers(s: &str) -> bool {
    for r in s.chars() {
        if r == '0'
            || r == '1'
            || r == '2'
            || r == '3'
            || r == '4'
            || r == '5'
            || r == '6'
            || r == '7'
            || r == '8'
            || r == '9'
            || r == '.'
        {
            continue;
        }
        return false;
    }
    true
}

fn handle_numbers<'a>(tokens: &[Token<'a>]) -> Vec<Token<'a>> {
    let mut out = vec![];
    for i in tokens {
        if i.string.contains(".") {
            if !is_numbers(i.string) {
                let tmp = split_by(i.string, '.');
                for t in &tmp {
                    out.push(Token {
                        string: t,
                        line: i.line,
                    });
                }
            } else {
                out.push(i.clone());
            }
        } else {
            out.push(i.clone());
        }
    }
    return out;
}

fn compress_quotes<'a>(tokens: &[Token<'a>]) -> Option<Vec<Token<'a>>> {
    fn str_compress<'a>(tokens: &[Token<'a>], cursor: &mut usize) -> Option<Token<'a>> {
        let start = tokens[*cursor].clone();
        let mut count = 0;
        *cursor += 1;
        let mut last_was_slash = false;
        while *cursor < tokens.len() {
            if tokens[*cursor].string == "\"" && !last_was_slash {
                let out = unsafe {
                    slice::from_raw_parts(start.string.as_ptr(), start.string.len() + count+tokens[*cursor].string.len()+1)
                };
                if let Ok(out_str) = &str::from_utf8(out) {
                    return Some(Token {
                        string: out_str,
                        line: start.line,
                    });
                }
            } else {
                if tokens[*cursor].string == "\\" && !last_was_slash {
                    last_was_slash = true;
                }
                count += tokens[*cursor].string.len();
            }
            *cursor += 1;
        }
        return None;
    }
    let mut out = vec![];
    let mut cursor = 0;
    while cursor < tokens.len() {
        if tokens[cursor] == "\"" {
            out.push(str_compress(tokens, &mut cursor)?);
        } else {
            out.push(tokens[cursor].clone());
        }
        cursor += 1;
    }
    return Some(out);
}

pub fn tokenize<'a>(program: &'a str) -> Vec<Token<'a>> {
    let lines: Vec<&'a str> = program.split("\n").collect();
    let mut out: Vec<Token<'a>> = vec![];
    for i in 0..lines.len() {
        let tokens: Vec<&'a str> = lines[i].split_whitespace().collect();
        for j in tokens {
            out.push(Token {
                string: j,
                line: i + 1,
            });
        }
    }
    out = out
        .iter()
        .map(|i| token_split_by(i, '('))
        .flatten()
        .collect();
    out = out
        .iter()
        .map(|i| token_split_by(i, ')'))
        .flatten()
        .collect();
    out = out
        .iter()
        .map(|i| token_split_by(i, ':'))
        .flatten()
        .collect();
    out = out
        .iter()
        .map(|i| token_split_by(i, ';'))
        .flatten()
        .collect();
    out = out
        .iter()
        .map(|i| token_split_by(i, '+'))
        .flatten()
        .collect();
    out = out
        .iter()
        .map(|i| token_split_by(i, '-'))
        .flatten()
        .collect();
    out = out
        .iter()
        .map(|i| token_split_by(i, '='))
        .flatten()
        .collect();
    out = out
        .iter()
        .map(|i| token_split_by(i, '/'))
        .flatten()
        .collect();
    out = out
        .iter()
        .map(|i| token_split_by(i, '*'))
        .flatten()
        .collect();
    out = out
        .iter()
        .map(|i| token_split_by(i, '['))
        .flatten()
        .collect();
    out = out
        .iter()
        .map(|i| token_split_by(i, ']'))
        .flatten()
        .collect();
    out = out
        .iter()
        .map(|i| token_split_by(i, '<'))
        .flatten()
        .collect();
    out = out
        .iter()
        .map(|i| token_split_by(i, '>'))
        .flatten()
        .collect();
    out = out
        .iter()
        .map(|i| token_split_by(i, '"'))
        .flatten()
        .collect();
    out = out
        .iter()
        .map(|i| token_split_by(i, '!'))
        .flatten()
        .collect();
    out = out
        .iter()
        .map(|i| token_split_by(i, '{'))
        .flatten()
        .collect();
    out = out
        .iter()
        .map(|i| token_split_by(i, '}'))
        .flatten()
        .collect();
    out = out
        .iter()
        .map(|i| token_split_by(i, ','))
        .flatten()
        .collect();
    out = out
        .iter()
        .map(|i| token_split_by(i, '^'))
        .flatten()
        .collect();
    out = out
        .iter()
        .map(|i| token_split_by(i, '&'))
        .flatten()
        .collect();
    out = collect_tokens(&out);
    out = handle_numbers(&out);
    out = compress_quotes(&out).expect("quotes should work\n");
    return out;
}

pub fn extract_global<'a>(tokens: &'a [Token], idx: &mut usize) -> Option<GlobalTypes<'a>> {
    let start = *idx;
    if start >= tokens.len() { 
        return None;
    }
    if tokens[start] != "let" && tokens[start] != "import" {
        let mut parens_count = 0;
        let mut hit_paren = false;
        while parens_count > 0 || !hit_paren {
            *idx += 1;
            if *idx >= tokens.len() {
                return None;
            }
            if tokens[*idx] == "{" {
                hit_paren = true;
                parens_count += 1;
            }
            if tokens[*idx] == "}" {
                parens_count -= 1;
                if parens_count < 1 {
                    *idx += 1;
                    break;
                }
            }
            if tokens[*idx] == ";"&&!hit_paren{
                *idx += 1;
                break;
            }
            if *idx > tokens.len() {
                println!("returned none 1");
                return None;
            }
        }
    } else {
        while tokens[*idx] != ";" {
            *idx += 1;
        }
    }
    let span = &tokens[start..*idx];
    if span[0] == "struct" ||span[1] == "struct"{
        let out = GlobalTypes::StructDef { text: span };
        *idx = *idx + 1;
        return Some(out);
    }
    if span[0] == "let"{
        let out = GlobalTypes::GlobalDef { text: span };
        *idx += 1;
        return Some(out);
    }
    if span[0] == "fn" ||span[1] == "fn"{
        let out = GlobalTypes::FunctionDef { text: span };
        return Some(out);
    }
    if span[0] == "import"{
        let out = GlobalTypes::IncludeDirective { text: span };
        *idx +=1;
        return Some(out);
    }
    println!("returned none 2 span :{:#?}", span);
    assert!(false);
    return None;
}


pub fn extract_globals<'a>(tokens: &'a [Token<'a>]) -> Result<Vec<GlobalTypes<'a>>, String> {
    let mut out = vec![];
    let mut idx = 0;
    while let Some(p) = extract_global(&tokens, &mut idx) {
        out.push(p.clone());
    }
    return Ok(out);
}

fn parse_declared_type(
    tokens: &[Token],
    idx: &mut usize,
    types: &mut HashMap<String, Type>,
) -> Option<Type> {
    let base = *idx;
    let current = &tokens[base];
    if let Some(st) = types.get(current.string) {
        *idx += 1;
        return Some(st.clone());
    }
    if current.string == "^" {
        *idx += 1;
        return Some(parse_declared_type(tokens, idx, types))
            .flatten()
            .map(|i| Type::PointerT {
                ptr_type: Box::new(i),
            });
    }
    if current.string == "[" {
        if tokens.get(base + 1)?.string == "]" {
            *idx += 2;
            let out =Some(parse_declared_type(tokens, idx, types))
                .flatten()
                .map(|i| Type::SliceT {
                    ptr_type: Box::new(i),
                });
            types.insert(name_mangle_type_for_names(&out.clone()?), out.clone()?);
            return out;
        } else if tokens.get(base + 2)?.string == "]" {
            *idx +=2;
            if let Ok(count) = tokens[base + 1].string.parse::<usize>() {
                *idx += 1;
                let base = parse_declared_type(tokens, idx, types)?;
                let out= Type::SliceT { ptr_type: Box::new(base.clone()) };
                types.insert(name_mangle_type_for_names(&out.clone()), out.clone());
                return Some(Type::ArrayT { size: count, array_type: Box::new(base) });
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
    println!(
        "error unknown type:{}, line:{}",
        tokens[base].string, tokens[base].line
    );
    return None;
}

pub fn parse_type(base_text: &[Token], types: &mut HashMap<String, Type>) -> Option<(String, Type,bool)> {
    if base_text.len() < 3 {
        println!("error requires at least three tokens to declare struct");
    }
    let is_pub = *base_text.get(0)? == "pub";   
    let text = if is_pub{
        &base_text[1..]
    } else{
        &base_text[0..]
    };
    if *text.get(0)? != "struct" {
        println!("expected struct declaration line{}", text.get(1)?.line);
    }
    let name = String::from(text.get(1)?.string);
    let mut out_types = vec![];
    let mut idx = 3;
    while idx < text.len() - 1 {
        let ident_name = &text[idx];
        if text[idx + 1] != ":" {
            println!(
                "error unexpected non : character {} at line:{}",
                text[idx + 1].string,
                text[idx + 1].line
            );
            return None;
        }
        idx += 2;
        let comp_type = parse_declared_type(text, &mut idx,types);
        if comp_type.is_none() {
            println!(
                "error: unknown type:{} at line:{}",
                text[idx].string, text[idx].line
            );
            return None;
        }
        out_types.push((String::from(ident_name.string), comp_type.unwrap().clone()));
    }
    return Some((
        name.clone(),
        Type::StructT {
            name,
            components: out_types.clone(),
        },
        is_pub
    ));
}

fn get_arms(expr: &mut AstNode) -> (Option<&mut AstNode>, Option<&mut AstNode>) {
    match expr {
        AstNode::Assignment { left, right,data:_ } => {
            return (Some(left), Some(right));
        }
        AstNode::Add { left, right, data:_} => {
            return (Some(left), Some(right));
        }
        AstNode::Sub { left, right,data:_ } => {
            return (Some(left), Some(right));
        }
        AstNode::Mult { left, right , data:_} => {
            return (Some(left), Some(right));
        }
        AstNode::Div { left, right,data:_ } => {
            return (Some(left), Some(right));
        }
        AstNode::Equals { left, right, data:_ } => {
            return (Some(left), Some(right));
        }
        AstNode::LessThan { left, right, data:_ } => {
            return (Some(left), Some(right));
        }
        AstNode::GreaterThan { left, right, data:_ } => {
            return (Some(left), Some(right));
        }
        AstNode::GreaterOrEq { left, right, data:_ } => {
            return (Some(left), Some(right));
        }
        AstNode::LessOrEq { left, right , data:_} => {
            return (Some(left), Some(right));
        }
        AstNode::And { left, right, data:_ } => {
            return (Some(left), Some(right));
        }
        AstNode::Or { left, right , data:_} => {
            return (Some(left), Some(right));
        }
        AstNode::Deref { thing_to_deref } => {
            return (None, Some(thing_to_deref.as_mut()));
        }
        AstNode::TakeRef { thing_to_ref } => {
            return (None, Some(thing_to_ref.as_mut()));
        }
        AstNode::FieldUsage { base, field_name:_ }=>{
            return (Some(base),None);
        }
        AstNode::ArrayAccess { variable, index:_ }=>{
            return (Some(variable),None);
        }
        AstNode::BoundFunctionCall { variable, function_name:_, args:_ }=>{
            return (Some(variable), None);
        }
        _ => {
            return (None, None);
        }
    }
}

fn place_expr(_text: &[Token], _start: usize, left: AstNode, right: AstNode) -> Option<AstNode> {
    let mut left = left;
    let mut right = right;
    if left.get_priority() >= right.get_priority() {
        let mut current = &mut left;
        while get_arms(current).1.expect("616").get_priority() > right.get_priority() {
            current = get_arms(current).1.expect("629");
        }
        *get_arms(current).1.expect("631") = right;
        return Some(left);
    } else {
        let mut current = &mut right;
        while get_arms(current).0.expect("616").get_priority() > left.get_priority() {
            current = get_arms(current).0.expect("629");
        }
        if let Some(tr) = get_arms(current).0 {
            if let Some(tl) = get_arms(&mut left).1 {
                *tl = tr.clone();
            }
        }
        *get_arms(current).0.expect("631") = left;
        return Some(right);
    }
}

pub fn parse_list(
    text: &[Token],
    list_start: usize,
    list_end: usize,
    types: &mut HashMap<String, Type>,
    scope: &mut Scope,
    function_table: &HashMap<String, FunctionTable>,
) -> Option<Vec<AstNode>> {
    fn calc_next_end(text: &[Token], start: usize, list_end: usize) -> usize {
        let mut idx = start;
        while idx < list_end {
            if text[idx] == "," {
                return idx;
            }
            idx += 1;
        }
        return idx;
    }
    let mut out = vec![];
    let mut cursor = list_start;
    while cursor < list_end {
        let end = calc_next_end(text, cursor, list_end);
        out.push(parse_expression(
            text,
            &mut cursor,
            end,
            types,
            scope,
            function_table,
        )?);
        cursor = end + 1;
    }
    return Some(out);
}

pub fn parse_expression(
    text: &[Token],
    cursor: &mut usize,
    last: usize,
    types: &mut HashMap<String, Type>,
    scope: &mut Scope,
    function_table: &HashMap<String, FunctionTable>,
) -> Option<AstNode> {
    let start = *cursor;
    let mut out = None;
    if is_numbers(text[*cursor].string) && text[*cursor] != "."{
        if text[*cursor].string.contains('.') {
            let fout = text[*cursor]
                .string
                .parse::<f64>()
                .expect("should be numbers");
            *cursor += 1;
            out = Some(AstNode::FloatLiteral { value: fout });
        } else {
            let fout = text[*cursor]
                .string
                .parse::<i64>()
                .expect("should be numbers");
            *cursor += 1;
            out = Some(AstNode::IntLiteral { value: fout });
        }
    } else if text[*cursor] == "true" {
        *cursor += 1;
        out = Some(AstNode::BoolLiteral { value: true });
    } else if text[*cursor] == "false" {
        *cursor += 1;
        out = Some(AstNode::BoolLiteral { value: false });
    } else if text[*cursor] == "{" {
        let mut vout = vec![];
        *cursor += 1;
        while text[*cursor] != "}" && *cursor < last -1 {
            if text[*cursor] == "," {
                *cursor += 1;
                if *cursor >= last {
                    break;
                }
                continue;
            }
            let mut next_indx = *cursor;
            while text[next_indx] != "," && text[next_indx] != "}" &&text[next_indx] != ";" && next_indx < last {
                next_indx += 1;
                if next_indx >= last {
                    break;
                }
            }
            next_indx -= 1;
            //println!("last:{}, next_indx:{} cursor:{}", last, next_indx, cursor);
            let next = parse_expression(text, cursor, next_indx, types, scope, function_table).expect("should compiler");
            vout.push(next);
        }
        *cursor += 1;
        out = Some(AstNode::ArrayLiteral { nodes: vout });
    } else if text[*cursor] == "let" {
        let name = text[*cursor + 1].string.to_owned();
        *cursor += 3;
        let vtype = parse_declared_type(text, cursor, types)?;
        scope.declare_variable(vtype.clone(), name.clone());
        if text[*cursor] != ";" {
            let mut tmp_out = parse_expression(text, cursor, last, types, scope, function_table)?;
            match &mut tmp_out {
                AstNode::Assignment { left, right: _ ,data:_} => {
                    let v = scope.variable_idx(name.clone())?;
                    *left = Box::new(AstNode::VariableUse {
                        name: name.clone(),
                        index: v.1.clone(),
                        vtype: v.0.clone(),
                        is_arg: v.2.clone(),
                        data:Some(AstNodeData { line: text[*cursor-2].line, temporary_index: None })
                    });
                }
                _ => {}
            }
            out = Some(AstNode::VariableDeclaration {
                name,
                var_type: vtype,
                value_assigned: Some(Box::new(tmp_out.clone())),
                data:Some(AstNodeData{line:text[*cursor-2].line, temporary_index:None})
            });
        } else {
            out = Some(AstNode::VariableDeclaration {
                name,
                var_type: vtype,
                value_assigned: None,  
                data:Some(AstNodeData{line:text[*cursor-2].line, temporary_index:None})
            });
            *cursor +=1;
        };
    } else if text[*cursor] == "+" {
        *cursor += 1;
        out = Some(AstNode::Add {
            left: Box::new(AstNode::VoidLiteral),
            right: Box::new(AstNode::VoidLiteral),
            data:Some(AstNodeData{line:text[*cursor-1].line, temporary_index:None}),
        })
    } else if text[*cursor] == "-" {
        *cursor += 1;
        out = Some(AstNode::Sub {
            left: Box::new(AstNode::VoidLiteral),
            right: Box::new(AstNode::VoidLiteral),
            data:Some(AstNodeData{line:text[*cursor-1].line, temporary_index:None}),
        })
    } else if text[*cursor] == "*" {
        *cursor += 1;
        out = Some(AstNode::Mult {
            left: Box::new(AstNode::VoidLiteral),
            right: Box::new(AstNode::VoidLiteral),
            data:Some(AstNodeData{line:text[*cursor-1].line, temporary_index:None}),
        })
    } else if text[*cursor] == "/" {
        *cursor += 1;
        out = Some(AstNode::Div {
            left: Box::new(AstNode::VoidLiteral),
            right: Box::new(AstNode::VoidLiteral),
            data:Some(AstNodeData{line:text[*cursor-1].line, temporary_index:None}),
        })
    } else if text[*cursor] == "&" {
        *cursor += 1;
        out = Some(AstNode::TakeRef {
            thing_to_ref: Box::new(AstNode::VoidLiteral),
        })
    } else if text[*cursor] == "^" {
        *cursor += 1;
        out = Some(AstNode::Deref {
            thing_to_deref: Box::new(AstNode::VoidLiteral),
        })
    } else if text[*cursor] == "return" {
        *cursor += 1;
        out = Some(AstNode::Return {
            body: Box::new(parse_expression(
                text,
                cursor,
                last,
                types,
                scope,
                function_table,
            )?),
        });
        *cursor += 1;
        if *cursor >= last {
            return out;
        }
        if text[*cursor + 1] == "}" {
            *cursor += 1;
            return out;
        }
    } else if text[*cursor] == "=" {
        *cursor += 1;
        out = Some(AstNode::Assignment {
            left: Box::new(AstNode::VoidLiteral),
            right: Box::new(AstNode::VoidLiteral),
            data:Some(AstNodeData{line:text[*cursor-1].line, temporary_index:None}),
        })
    } else if text[*cursor] == "<" {
        *cursor += 1;
        out = Some(AstNode::LessThan {
            left: Box::new(AstNode::VoidLiteral),
            right: Box::new(AstNode::VoidLiteral),
            data:Some(AstNodeData{line:text[*cursor-1].line, temporary_index:None}),
        })
    } else if text[*cursor] == ">" {
        *cursor += 1;
        out = Some(AstNode::GreaterThan {
            left: Box::new(AstNode::VoidLiteral),
            right: Box::new(AstNode::VoidLiteral),
            data:Some(AstNodeData{line:text[*cursor-1].line, temporary_index:None}),
        })
    } else if text[*cursor] == "==" {
        *cursor += 1;
        out = Some(AstNode::Equals {
            left: Box::new(AstNode::VoidLiteral),
            right: Box::new(AstNode::VoidLiteral),
            data:Some(AstNodeData{line:text[*cursor-1].line, temporary_index:None}),
        })
    } else if text[*cursor] == "<=" {
        *cursor += 1;
        out = Some(AstNode::LessOrEq {
            left: Box::new(AstNode::VoidLiteral),
            right: Box::new(AstNode::VoidLiteral),
            data:Some(AstNodeData{line:text[*cursor-1].line, temporary_index:None}),
        })
    } else if text[*cursor] == ">=" {
        *cursor += 1;
        out = Some(AstNode::LessThan {
            left: Box::new(AstNode::VoidLiteral),
            right: Box::new(AstNode::VoidLiteral),
            data:Some(AstNodeData{line:text[*cursor-1].line, temporary_index:None}),
        })
    } else if text[*cursor] == "if" {
        *cursor += 1;
        let cond_end = calc_close_paren(text, *cursor).expect("failed to parse paren");
        if text[*cursor] != "(" {
            println!(
                "error expected ( line {} instead found {}",
                text[*cursor].line, text[*cursor].string
            );
        }
        *cursor += 1;
        let cond = parse_expression(text, cursor, cond_end, types, scope, function_table)
            .expect("expression should work");
        *cursor += 1;
        let new_scope = parse_scope(text, cursor, types, scope, function_table).expect("bruh");
        let else_scope = if *cursor < last {
            if text[*cursor] == "else" {
                *cursor += 1;
                if text[*cursor] == "if" {
                    Some(vec![parse_expression(
                        text,
                        cursor,
                        last,
                        types,
                        scope,
                        function_table,
                    )?])
                } else {
                    Some(
                        parse_scope(text, cursor, types, scope, function_table)
                            .expect("parsing scope should work"),
                    )
                }
            } else {
                None
            }
        } else {
            None
        };
        out = Some(AstNode::If {
            condition: Box::new(cond),
            thing_to_do: new_scope,
            r#else: else_scope,
        });
        return out;
    } else if text[*cursor] == "while" {
        *cursor += 1;
        let cond_end = calc_close_paren(text, *cursor).expect("failed to parse paren");
        if text[*cursor] != "(" {
            println!(
                "error expected ( line {} instead found {}",
                text[*cursor].line, text[*cursor].string
            );
        }
        *cursor += 1;
        let cond = parse_expression(text, cursor, cond_end, types, scope, function_table)
            .expect("expression should work");
        *cursor += 1;
        let new_scope = parse_scope(text, cursor, types, scope, function_table).expect("bruh");
        out = Some(AstNode::Loop {
            condition: Box::new(cond), 
            body: new_scope,
        });
        return out;
    }  else if text[*cursor] == "for" {
        *cursor += 1;
        let paren_end = calc_close_paren(text, *cursor).expect("failed to parse paren");
        if text[*cursor] != "(" {
            println!(
                "error expected ( line {} instead found {}",
                text[*cursor].line, text[*cursor].string
            );
        }
        *cursor += 1;
        let variable_end = calc_expr_end(text,paren_end, *cursor)?;
        let variable = Box::new(parse_expression(text, cursor, variable_end, types, scope, function_table)?);
        *cursor = variable_end+1;
        let cond_end =  calc_expr_end(text, paren_end, *cursor)?;
        let condition = Box::new(parse_expression(text, cursor, cond_end, types, scope, function_table)?);
        *cursor = cond_end+1;
        let post_op = Box::new(parse_expression(text, cursor, paren_end, types, scope, function_table)?);
        *cursor += 1;
        let new_scope = parse_scope(text, cursor, types, scope, function_table).expect("bruh");
        out = Some(AstNode::ForLoop { variable, condition, post_op, body:new_scope });
        return out;
    } else if text[*cursor] == "."{
        *cursor += 1;
        if text[*cursor +1] == "(" {
            if function_table.contains_key(text[*cursor].string){
                let name = text[*cursor].string.to_owned();
                *cursor += 1;
                let args_end = calc_close_paren(text, *cursor)?;
                *cursor += 1;
                let args = parse_list(text, *cursor, args_end, types, scope, function_table)?;
                out = Some(AstNode::BoundFunctionCall { variable: Box::new(AstNode::VoidLiteral), function_name: name, args:args });
                *cursor = args_end + 1;
            } else{
                println!("error unknown function {}", text[*cursor].string);
                return None;
            }
        } else{
            out = Some(AstNode::FieldUsage { base: Box::new(AstNode::VoidLiteral), field_name:text[*cursor].string.to_owned() });
            *cursor += 1
        }

    } else if text[*cursor] == "["{
        let expr_end = calc_close_block(text, *cursor).expect("block must close");
        *cursor += 1;
        let expr = parse_expression(text, cursor, expr_end, types, scope, function_table)?;
        *cursor +=1;
        out = Some(AstNode::ArrayAccess { variable: Box::new(AstNode::VoidLiteral), index: Box::new(expr) });
    }
     else if text[*cursor] == "("{
        let expr_end = calc_close_paren(text, *cursor).expect("parens must close");
        *cursor +=1;
        let inner = parse_expression(text, cursor, last, types, scope, function_table)?;
        *cursor = expr_end+1;
        out = Some(AstNode::Paren { internals: Box::new(inner) });
     }else if text[*cursor] == "new"{
        let vtype = parse_declared_type(text, cursor, types)?;
        *cursor += 1;
        out = Some(AstNode::OperatorNew { vtype: vtype });
     }else {
        if function_table.contains_key(text[*cursor].string) {
            let name = text[*cursor].string.to_owned();
            *cursor += 1;
            let args_end = calc_close_paren(text, *cursor)?;
            *cursor += 1;
            let args = parse_list(text, *cursor, args_end, types, scope, function_table)?;
            out = Some(AstNode::FunctionCall {
                function_name: name,
                args: args,
                data:Some(AstNodeData{line:text[*cursor-1].line, temporary_index:None}),
            });
            *cursor = args_end + 1;
        } else if let Some(v) = scope.variable_idx(text[*cursor].string.to_owned()) {
            out = Some(AstNode::VariableUse {
                name: text[*cursor].string.to_owned(),
                index: v.1.clone(),
                vtype: v.0.clone(),
                is_arg: v.2.clone(),
                data:Some(AstNodeData{line:text[*cursor-1].line, temporary_index:None}),
            });
            *cursor += 1;
        } else if text[*cursor].string.chars().collect::<Vec<char>>()[0] == '"'{
            out = Some(AstNode::StringLiteral { value: text[*cursor].string[1..text[*cursor].string.len() ].to_owned()});
            *cursor+=1;
        } else if types.contains_key(text[*cursor].string){
            let vtype = types.get(text[*cursor].string)?.clone();
            if text[*cursor+1] != "{"{
                println!("error line:{} expected struct literal",text[*cursor].line );
            }
            *cursor+=1;
            let mut vout = vec![];
            *cursor += 1;
            while text[*cursor] != "}" && *cursor < last -1 {
                if text[*cursor] == "," {
                    *cursor += 1;
                    if *cursor >= last {
                        break;
                    }
                    continue;
                }
                let mut next_indx = *cursor;
                while text[next_indx] != "," && text[next_indx] != "}" &&text[next_indx] != ";" && next_indx < last {
                    next_indx += 1;
                    if next_indx >= last {
                        break;
                    }
                }
                next_indx -= 1;
                //println!("last:{}, next_indx:{} cursor:{}", last, next_indx, cursor);
                let next = parse_expression(text, cursor, next_indx, types, scope, function_table).expect("should compiler");
                vout.push(next);
            }
            *cursor += 1;
            out = Some(AstNode::StructLiteral { vtype, nodes: vout });
        }
    }
    if out.is_none() {
        println!(
            "error unknown token {} line {}",
            text[*cursor].string, text[*cursor].line
        );
        return None;
    }
    if *cursor < last {
        let right = parse_expression(text, cursor, last, types, scope, function_table)?;
        return place_expr(text, start, out?, right);
    } else {
        if out.is_none() {
            println!("returned none, for some reason");
        }
        return out;
    }
}

fn calc_expr_end(text: &[Token], end: usize, cursor: usize) -> Option<usize> {
    if cursor == text.len() {
        return Some(cursor);
    }
    if text[cursor] == "while" || text[cursor] == "for" || text[cursor] == "if" {
        return Some(end);
    }
    let mut indx = cursor;
    while indx <= end {
        if text[indx].string == ";" {
            return Some(indx);
        }
        indx += 1;
    }
    return None;
}

pub fn parse_scope(
    text: &[Token],
    cursor: &mut usize,
    types: &mut HashMap<String, Type>,
    scope: &mut Scope,
    function_table: &HashMap<String, FunctionTable>,
) -> Option<Vec<AstNode>> {
    let start = *cursor;
    if text[*cursor] != "{" {
        println!(
            "error expected curly brace line{}, instead found {}",
            text[*cursor].line, text[*cursor].string
        );
        return None;
    }
    let end = calc_close_scope(text, *cursor).expect("scope must end");
    let mut out = vec![];
    if *cursor + 1 == end {
        return Some(vec![]);
    }
    *cursor += 1;
    while *cursor < end {
        let mut expr_end = calc_expr_end(text, end, *cursor).expect("expression must end");
        if *cursor == start {
            expr_end += 1;
        }
        if expr_end <= *cursor {
            *cursor += 1;
            continue;
        }
        let mut tmp = parse_expression(text, cursor, expr_end, types, scope, function_table)
        .expect("expression must be valid");
        alide_parens(&mut tmp);
        out.push(
            tmp
        );
    }
    *cursor = end + 1;
    return Some(out);
}

pub fn parse_global(
    text: &[Token],
    types: &mut HashMap<String, Type>,
) -> Option<(String, Type, AstNode)> {
    let mut idx = 0;
    if text[idx] != "let" {
        println!("error expected let: line:{}", text[idx].line);
        return None;
    }
    idx += 1;
    let name = text[idx].string;
    idx += 1;
    if text[idx] != ":" {
        println!("error expected : line:{}", text[idx].line);
        return None;
    }
    idx += 1;
    let vtype = parse_declared_type(text, &mut idx, types)?;
    idx += 1;
    let mut scope = Scope::new(&HashMap::new());
    let function_table = HashMap::new();
    let node = parse_expression(
        text,
        &mut idx,
        text.len(),
        types,
        &mut scope,
        &function_table,
    );
    if node.is_none() {
        println!("failed to parse global variable assignment");
    }
    let mut n = node?;
    alide_parens(&mut n);
    return Some((String::from(name), vtype, n));
}

pub fn parse_function(
    base_text: &[Token],
    types: &mut HashMap<String, Type>,
    globals: &HashMap<String, (Type, usize)>,
    function_table: &HashMap<String, FunctionTable>,
) -> Option<(String, Function,bool)> {
    let is_pub = base_text[0] == "pub";
    let text = if is_pub{
        &base_text[1..]
    } else{
        &base_text[0..]
    };
    let mut args = vec![];
    let mut arg_names = vec![];
    let mut cursor = 1_usize;
    let name = text[1].string.to_owned();
    cursor += 1;
    let args_end = calc_close_paren(text, cursor)?;
    cursor += 1;
    while cursor < args_end {
        let name = text[cursor].to_owned();
        cursor += 1;
        if text[cursor] != ":" {
            println!("error expected : line:{}", text[cursor].line);
            return None;
        }
        cursor += 1;
        let vtype = parse_declared_type(text, &mut cursor, types)?;
        arg_names.push(name.string.to_owned());
        args.push(vtype);
    }
    cursor += 1;
    if text[cursor] != "->" {
        println!("error requires -> for return type of function");
    }
    cursor += 1;
    let return_type = parse_declared_type(text, &mut cursor, types)?;
    let mut scope = Scope::new(globals);
    for i in 0..args.len() {
        scope.declare_variable_arg(args[i].clone(), arg_names[i].clone());
    }
    let forward = text[cursor] ==";";
    let out = if !forward {parse_scope(text, &mut cursor, types, &mut scope, function_table)?} else{vec![]};
    return Some((
        name.clone(),
        Function {
            name,
            return_type: return_type,
            args: args,
            arg_names: arg_names,
            program: out,
            forward_declared:false,
        },forward
    ));
}

pub fn parse_function_stub(
    base_text: &[Token],
    types: &mut HashMap<String, Type>,
    _globals: &HashMap<String, (Type, usize)>,
    _function_table: &HashMap<String, FunctionTable>,
) -> Option<(String, Function,bool)> {
    let is_pub = base_text[0] == "pub";
    let text = if is_pub{
        &base_text[1..]
    } else{
        base_text
    };
    let mut args = vec![];
    let mut arg_names = vec![];
    let mut cursor = 1_usize;
    let name = text[1].string.to_owned();
    cursor += 1;
    let args_end = calc_close_paren(text, cursor)?;
    cursor += 1;
    while cursor < args_end {
        let name = text[cursor].to_owned();
        cursor += 1;
        if text[cursor] != ":" {
            println!("error expected : line:{}", text[cursor].line);
            return None;
        }
        cursor += 1;
        let vtype = parse_declared_type(text, &mut cursor, types)?;
        arg_names.push(name.string.to_owned());
        args.push(vtype);
    }
    cursor += 1;
    if text[cursor] != "->" {
        println!("error requires -> for return type of function");
    }
    cursor +=1;
    let return_type = parse_declared_type(text, &mut cursor, types).expect("bruh");
    return Some((
        name.clone(),
        Function {
            name,
            return_type: return_type,
            args: args,
            arg_names: arg_names,
            program: vec![],
            forward_declared:true,
        },
        is_pub,
    ));
}
pub fn get_public_members(program:&str,to_compile:&mut Vec<String>, types:&mut HashMap<String,Type>)->Option<Program>{
    let tokens = tokenize(program);
    //println!("{:#?}", tokens);
    let globals_result = extract_globals(&tokens);
    if globals_result.is_err() {
        let s = globals_result.expect_err("is error shouldn't break");
        println!("{}", s);
        return None;
    }
    let globals = globals_result.expect("is ok by previous call");
    let mut pub_types:HashMap<String,Type> = HashMap::new();
    let mut scope: HashMap<String, (Type, usize)> = HashMap::new();
    let mut functions: HashMap<String, FunctionTable> = HashMap::new();
    let mut pub_functions:HashMap<String, FunctionTable> = HashMap::new();
    for i in &globals{
        match i{
            GlobalTypes::IncludeDirective { text }=>{
                let pubs = parse_include_directive(*text,to_compile, types)?;
                for i in pubs.0{
                    types.insert(i.0, i.1);
                }
                for i in pubs.1{
                    if !functions.contains_key(&i.0){
                        functions.insert(i.0.clone(), FunctionTable::new());
                    } 
                    for j in i.1.functions{
                        functions.get_mut(&i.0)?.push(j);
                    }
                }
                if !to_compile.contains(&(pubs.2.clone()+".risp")){
                    to_compile.push(pubs.2+".risp");
                }

            } 
            _=>{

            }
        }
    }
    for i in &globals {
        match i {
            GlobalTypes::StructDef { text } => {
                let tmp = parse_type(*text, types)?;
                if types.contains_key(&tmp.0) {
                    println!("error {} redeclared", tmp.0);
                    return None;
                }
                types.insert(tmp.0.clone(), tmp.1.clone());
                if tmp.2{
                    pub_types.insert(tmp.0.clone(), tmp.1.clone());
                }
            }
            _=>{}
        }
    }
    let mut global_count = 0;
    let mut global_initializers:Vec<(String,Option<AstNode>)> = vec![];
    for i in &globals {
        match i {
            GlobalTypes::GlobalDef { text } => {
                let tmp = parse_global(*text, types)?;
                if scope.contains_key(&tmp.0) {
                    println!("error {} redeclared", tmp.0);
                    return None;
                }
                global_initializers.push((tmp.0.clone(), Some(tmp.2.clone())));
                scope.insert(tmp.0, (tmp.1, global_count));
                global_count += 1;
            }
            _=>{

            }
        }
    }
    for i in &globals {
        match i {
            GlobalTypes::FunctionDef { text } => {
                let tmp = parse_function_stub(*text, types, &scope, &functions)?;
                if !functions.contains_key(&tmp.0){
                    let table = FunctionTable::new();
                    functions.insert(tmp.0.clone(), table);
                }
                functions.get_mut(&tmp.0)?.push(tmp.1.clone());
                if tmp.2{
                        if !pub_functions.contains_key(&tmp.0){
                            let table = FunctionTable::new();
                            pub_functions.insert(tmp.0.clone(), table);
                        }
                        pub_functions.get_mut(&tmp.0)?.push(tmp.1.clone());
                }
            }
        _=>{

        }
    }
}
    return Some(Program {
        types:pub_types,
        functions:pub_functions,
        static_variables: scope,
        global_initializers,
    });
}
pub fn parse_include_directive<'a>(span:&[Token<'a>],to_compile:&mut Vec<String>, types:&mut HashMap<String,Type>)->Option<(HashMap<String,Type>,HashMap<String,FunctionTable>,String)>{
    let file = span[1].string.to_owned()+".risp";
    let tprg = std::fs::read_to_string(&file).expect("testing expect");
    let base_out = get_public_members(&tprg,to_compile, types)?;
    return Some((base_out.types, base_out.functions, span[1].string.to_owned()));
}
pub fn program_to_ast(program: &str,compile_queue:&mut Vec<String>,file:&str) -> Option<Program> {
    let tokens = tokenize(program);
    let globals_result = extract_globals(&tokens);
    if globals_result.is_err() {
        let s = globals_result.expect_err("is error shouldn't break");
        println!("{}", s);
        return None;
    }
    let globals = globals_result.expect("is ok by previous call");
    let mut types: HashMap<String, Type> = HashMap::new();
    let mut pub_types:HashMap<String,Type> = HashMap::new();
    types.insert(String::from("bool"), Type::BoolT);
    types.insert(String::from("char"), Type::CharT);
    types.insert(String::from("int"), Type::IntegerT);
    types.insert(String::from("float"), Type::FloatT);
    types.insert(String::from("string"), Type::StringT);
    types.insert(String::from("void"), Type::VoidT);
    let mut scope: HashMap<String, (Type, usize)> = HashMap::new();
    let mut functions: HashMap<String, FunctionTable> = HashMap::new();
    let mut pub_functions:HashMap<String, FunctionTable> = HashMap::new();
    for i in &globals{
        match i{
            GlobalTypes::IncludeDirective { text }=>{
                let pubs = parse_include_directive(*text,compile_queue,&mut types)?;
                for i in pubs.0{
                    types.insert(i.0, i.1);
                }
                for i in pubs.1{
                    if !functions.contains_key(&i.0){
                        functions.insert(i.0.clone(), FunctionTable::new());
                    } 
                    for j in i.1.functions{
                        functions.get_mut(&i.0)?.push(j);
                    }
                }
                if !compile_queue.contains(&(pubs.2.clone()+".risp")){
                    compile_queue.push(pubs.2+".risp");
                }
            } 
            _=>{

            }
        }
    }
    for i in &globals {
        match i {
            GlobalTypes::StructDef { text } => {
                let tmp = parse_type(*text, &mut types)?;
                if types.contains_key(&tmp.0) {
                    println!("error {} redeclared", tmp.0);
                    return None;
                }
                types.insert(tmp.0.clone(), tmp.1.clone());
                if tmp.2{
                    pub_types.insert(tmp.0.clone(), tmp.1.clone());
                }
            }
            _=>{}
        }
    }
    let mut global_count = 0;
    let mut global_initializers:Vec<(String,Option<AstNode>)> = vec![];
    for i in &globals {
        match i {
            GlobalTypes::GlobalDef { text } => {
                let tmp = parse_global(*text, &mut types)?;
                if scope.contains_key(&tmp.0) {
                    println!("error {} redeclared", tmp.0);
                    return None;
                }
                global_initializers.push((tmp.0.clone(), Some(tmp.2.clone())));
                scope.insert(tmp.0, (tmp.1, global_count));
                global_count += 1;
            }
            _=>{

            }
        }
    }
    for i in &globals {
        match i {
            GlobalTypes::FunctionDef { text } => {
                let tmp = parse_function_stub(*text, &mut types, &scope, &functions)?;
                if !functions.contains_key(&tmp.0){
                    let table = FunctionTable::new();
                    functions.insert(tmp.0.clone(), table);
                }
                functions.get_mut(&tmp.0)?.push(tmp.1.clone());
                if tmp.2{
                        if !pub_functions.contains_key(&tmp.0){
                            let table = FunctionTable::new();
                            pub_functions.insert(tmp.0.clone(), table);
                        }
                        pub_functions.get_mut(&tmp.0)?.push(tmp.1.clone());
                }
            }
        _=>{

        }
    }
    }
    for i in &globals {
        match i {
            GlobalTypes::FunctionDef { text } => {
                let tmp = parse_function(*text, &mut types, &scope, &functions)?;
                if tmp.2{
                    continue;
                }
                for i in &mut functions.get_mut(&tmp.0)?.functions{
                    if i == &tmp.1{
                        *i = tmp.1;
                        break;
                    }
                }
            }
            _=>{}
        }
    }
    let out = validate_ast(Program{
        types,
        functions,
        static_variables:scope,
        global_initializers}
    );
    if let Ok(out) = out{
        return Some(out);
    } else if let Err(out) = out{
        println!("Error {file}: {out} ");
        return None;
    } else{
        unreachable!();
    }
}
