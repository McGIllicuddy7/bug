use core::str;
use std::collections::HashMap;
use std::hash::Hash;
use std::io::Cursor;
use std::slice;
#[derive(Debug, Clone)]
pub struct Token<'a>{
    string:&'a str,
    line:usize
}

impl <'a>Into<&'a str> for Token<'a>{
    fn into(self)->&'a str{
       return self.string; 
    }
}
impl PartialEq<&str> for Token<'_>{
    fn eq(&self, other:&&str) ->bool{
        return self.string == *other;
    }
}

#[derive(Clone,Debug)]
pub enum Type{
    BoolT,IntegerT,FloatT,StringT, MatrixT,StructT{name:String, components:Vec<(String,Type)>}, PointerT{ptr_type:Box<Type>},
    ArrayT{size:usize, array_type:Box<Type>},VoidT, VecT{ptr_type:Box<Type>},
}

#[derive(Clone,Debug)]
pub enum AstNode{
    VoidLiteral{},
    BoolLiteral{value:bool},
    StringLiteral{value:String}, 
    IntLiteral{value:i64}, 
    FloatLiteral{value:f64}, 
    StructLiteral{nodes:Vec<AstNode>},
    ArrayLiteral{nodes:Vec<AstNode>},
    VariableUse{name:String, index:usize,vtype:Type, is_arg:bool},
    FunctionCall{function_name:String, args:Vec<AstNode>},
    Assignment{left:Box<AstNode>, right:Box<AstNode>},
    VariableDeclaration{name:String, var_type:Type, value_assigned:Option<Box<AstNode>>},
    Add{left:Box<AstNode>, right:Box<AstNode>},
    Sub{left:Box<AstNode>, right:Box<AstNode>},
    Mult{left:Box<AstNode>, right:Box<AstNode>},
    Div{left:Box<AstNode>, right:Box<AstNode>},
    Equals{left:Box<AstNode>, right:Box<AstNode>},
    GreaterThan{left:Box<AstNode>, right:Box<AstNode>},
    LessThan{left:Box<AstNode>, right:Box<AstNode>},
    GreaterOrEq{left:Box<AstNode>, right:Box<AstNode>},
    LessOrEq{left:Box<AstNode>, right:Box<AstNode>},
    Not{value:Box<AstNode>},
    And{left:Box<AstNode>, right:Box<AstNode>},
    Or{left:Box<AstNode>, right:Box<AstNode>},
    If{condition:Box<AstNode>,thing_to_do:Box<AstNode>, r#else:Option<Box<AstNode>>,},
    Loop{condition:Box<AstNode>, body:Box<AstNode>,},
    ForLoop{variable:Box<AstNode>, condition:Box<AstNode>, post_op:Box<AstNode>, body:Box<AstNode>,},
    Return{body:Box<AstNode>},
    Deref{thing_to_deref:Box<AstNode>},
    TakeRef{thing_to_ref:Box<AstNode>},
}

#[derive(Debug)]
pub struct Function{
    name:String, 
    return_type:Type, 
    args:Vec<Type>,
    arg_names:Vec<String>,
    program:Vec<AstNode>,
}
#[derive(Debug)]
pub struct Scope{
    pub scope:Vec<HashMap<String, (Type,usize, bool)>>, 
    pub count:usize,
}
impl Scope{
    pub fn new(statics:&HashMap<String,(Type,usize)>)->Self{
        let mut  base = HashMap::new();
        let mut count = 0;
        for r in statics{
            base.insert(r.0.clone(), (r.1.0.clone(), count, false));
            count +=1;
        }
        Self{scope:vec![base],count}
    }   
    pub fn push_scope(&mut self){
        self.scope.push(HashMap::new());
    } 
    pub fn pop_scope(&mut self){
        self.scope.pop();
    }
    pub fn declare_variable(&mut self, vtype:Type, name:String){
        let cur = &mut self.scope[0];
        cur.insert(name,(vtype, self.count,false));
        self.count +=1;
    }
    pub fn declare_variable_arg(&mut self, vtype:Type, name:String){
        let cur = &mut self.scope[0];
        cur.insert(name,(vtype, self.count,false));
        self.count +=1;
    }
    pub fn variable_idx(&mut self, name:String)->Option<(Type,usize,bool)>{
        for i in &self.scope{
            if i.contains_key(&name){
                return Some(i.get(&name)?.clone());
            }
        }
        return None;
    }
}
#[derive(Debug)]
pub struct Program{
    types:HashMap<String, Type>, 
    functions:HashMap<String, Function>,
    static_variables:HashMap<String, (Type,usize)>,
}


#[derive(Debug,Clone,Copy)]
pub enum GlobalTypes<'a>{
    StructDef{text:&'a [Token<'a>]},
    FunctionDef{text:&'a [Token<'a>]},
    GlobalDef{text:&'a [Token<'a>]},
}

pub fn calc_close_paren(tokens:&[Token<'_>], base_idx:usize)->Option<usize>{
    let mut idx = base_idx+1;
    let mut paren_count = 1;
    if tokens[base_idx] != "("{
        println!("base wasn't a paren");
        return None;
    }
    while idx<tokens.len(){
        if tokens[idx] == "("{
            paren_count += 1;
        } else if tokens[idx] == ")"{
            paren_count -= 1;
        }
        if paren_count == 0{
            return Some(idx);
        }
        idx +=1;
    }
    println!("failed to find next paren");
    return None;
}

pub fn calc_close_block(tokens:&[Token<'_>], base_idx:usize)->Option<usize>{
    let mut idx = base_idx;
    let mut paren_count = 1;
    if tokens[base_idx] != "["{
        return None;
    }
    while idx<tokens.len(){
        if tokens[idx] == "["{
            paren_count += 1;
        } else if tokens[idx] == "]"{
            paren_count -= 1;
        }
        if paren_count == 0{
            return Some(idx);
        }
        idx +=1;
    }
    return None;
}

pub fn split_by<'a>(string:&'a str, value:char)->Vec<&'a str>{
    let mut out:Vec<&'a str> = vec![];
    let mut last = 0;
    let mut current_idx = 0;
    for i in string.chars(){
        if i == value{
            if last != current_idx{
                out.push(&string[last..current_idx] as &str);
            }
            out.push(&string[current_idx..current_idx+1] as &str);
            last = current_idx+1;
        }
        current_idx += 1;
    }
    if last !=current_idx{
        out.push(&string[last..] as &str);
    }
    return out;
}

pub fn token_split_by<'a>(token:&Token<'a>, value:char)->Vec<Token<'a>>{
    split_by(token.string, value).into_iter().map(|i| Token{string:i, line:token.line}).collect()
}

pub fn collect_tokens<'a>(tokens:&[Token<'a>])->Vec<Token<'a>>{
    let mut out = vec![];
    let mut count = 0;
    while count<tokens.len(){
        if count<tokens.len()-1{
            if tokens[count] == "=" && tokens[count+1] == "="{
                let mut token = Token{string:tokens[count].string, line:tokens[count].line};
                unsafe { 
                    let strr = token.string.as_ref() as &[u8];
                    let ptr = strr.as_ptr();
                    let len = strr.len()+tokens[count+1].string.len();
                    let ptr0 = tokens[count].string.as_ptr();
                    if ptr as usize +len == ptr0 as usize{
                        let new_str = slice::from_raw_parts(ptr,len);
                        let new_string = str::from_utf8(new_str);
                        if let Ok(s) = new_string{
                            token.string = s;
                        }
                    }
                }
                out.push(token);
            } else if tokens[count] == "-" && tokens[count+1] == ">"{
                let mut token = Token{string:tokens[count].string, line:tokens[count].line};
                unsafe { 
                    let strr = token.string.as_ref() as &[u8];
                    let ptr = strr.as_ptr();
                    let len = strr.len()+tokens[count+1].string.len();
                    let ptr0 = tokens[count+1].string.as_ptr();
                    if ptr as usize +len-1== ptr0 as usize{
                        let new_str = slice::from_raw_parts(ptr,len);
                        let new_string = str::from_utf8(new_str);
                        if let Ok(s) = new_string{
                            token.string = s;
                            count += 1;
                        }

                    }
                }
                out.push(token);
            }else{
                out.push(tokens[count].clone());
            }
        } else{
            out.push(tokens[count].clone());
        }
        count+=1;
    }
    return out;
}
fn is_numbers(s:&str)->bool{
    for r in s.chars(){
        if r == '0' ||r == '1' ||r== '2' ||r== '3' ||r== '4' ||r== '5' ||r== '6' || r== '7' || r=='8' || r == '9' || r == '.'{
            continue;
        }
        return false;
    }
    true
}
fn handle_numbers<'a>(tokens:&[Token<'a>])->Vec<Token<'a>>{
    let mut out = vec![];
    for i in tokens{
        if i.string.contains("."){
            if !is_numbers(i.string){
                let tmp = split_by(i.string, '.');
                for t in &tmp{
                    out.push(Token { string: t, line: i.line });
                }
            } else{
                out.push(i.clone());
            }
        } else{
            out.push(i.clone());
        }
    }
    return out;
}
fn compress_quotes<'a>(tokens:&[Token<'a>])->Option<Vec<Token<'a>>>{
    fn str_compress<'a>(tokens:&[Token<'a>], cursor:&mut usize)->Option<Token<'a>>{
        let start = tokens[*cursor].clone();
        let mut count = 0_usize;
        let mut last_was_slash = false;
        while *cursor<tokens.len(){
            if tokens[*cursor].string == "\"" && !last_was_slash{
                let out = unsafe{slice::from_raw_parts(start.string.as_ptr(), start.string.len()+count)};
                if let Ok(out_str) = &str::from_utf8(out){
                    return Some(Token { string:out_str, line: start.line });
                }

            }else{ 
                if tokens[*cursor].string == "\\" && !last_was_slash{
                    last_was_slash = true;
                }
                count += tokens[*cursor].string.len();
            }
            *cursor+=1;
        }
        return None;
    }
    let mut out = vec![];
    let mut cursor = 0;
    while cursor<tokens.len(){
        if tokens[cursor] == "\""{
            out.push(str_compress(tokens, &mut cursor)?);
        } else{
            out.push(tokens[cursor].clone());
        }
        cursor+=1;
    }
    return Some(out);
}
pub fn tokenize<'a>(program:&'a str)->Vec<Token<'a>>{
    let lines:Vec<&'a str> = program.split("\n").collect();
    let mut out:Vec<Token<'a>> = vec![];
    for i in 0..lines.len(){
        let tokens:Vec<&'a str> = lines[i].split_whitespace().collect();
        for j in tokens{
            out.push(Token{string:j, line:i+1});
        } 
    }
    out = out.iter().map(|i| token_split_by(i,'(')).flatten().collect();
    out = out.iter().map(|i| token_split_by(i,')')).flatten().collect();
    out = out.iter().map(|i| token_split_by(i,':')).flatten().collect();
    out = out.iter().map(|i| token_split_by(i,'+')).flatten().collect();
    out = out.iter().map(|i| token_split_by(i,'-')).flatten().collect();
    out = out.iter().map(|i| token_split_by(i,'=')).flatten().collect();
    out = out.iter().map(|i| token_split_by(i,'/')).flatten().collect();
    out = out.iter().map(|i| token_split_by(i,'[')).flatten().collect();
    out = out.iter().map(|i| token_split_by(i,']')).flatten().collect();
    out = out.iter().map(|i| token_split_by(i,'<')).flatten().collect();
    out = out.iter().map(|i| token_split_by(i,'>')).flatten().collect();
    out = out.iter().map(|i| token_split_by(i,'"')).flatten().collect();
    out = collect_tokens(&out);
    out = handle_numbers(&out);
    out = compress_quotes(&out).expect("quoutes should work\n");
    return out;
}

pub fn extract_global<'a>(tokens:&'a[Token],idx:&mut usize)->Option<GlobalTypes<'a>>{
    let start = *idx;
    if start>=tokens.len(){
        return None;
    }
    if tokens[*idx] != "(" {
        return None;
    } 
    let mut parens_count = 1;
    while parens_count>0{
        *idx += 1;
        if *idx>=tokens.len(){
            return None;
        }
        if tokens[*idx] == "("{
            parens_count+= 1;
        } 
        if tokens[*idx] == ")"{
            parens_count -= 1;
        }
        if *idx>tokens.len(){
            println!("returned none 1");
            return None;
        }
    }

    let span = &tokens[start..*idx];
    *idx +=1;
    if span[1] == "struct"{
        let out = GlobalTypes::StructDef { text:span };
        return Some(out);
    } 
    if span[1] == "let"{
        let out = GlobalTypes::GlobalDef  { text:span };
        return Some(out);
    } 
    if span[1] == "fn"{
        let out = GlobalTypes::FunctionDef { text:span };
        return Some(out);
    }
    println!("returned none 2");
    return None;
 }

pub fn extract_globals<'a>(tokens:&'a[Token<'a>])->Result<Vec<GlobalTypes<'a>>,String>{
    let mut out = vec![];
    let mut idx = 0;
    while let Some(p) = extract_global(&tokens, &mut idx){
        out.push(p.clone());
    }
    return Ok(out);
}
fn parse_declared_type(tokens:&[Token], idx:&mut usize, types:&HashMap<String, Type>)->Option<Type>{
    let base = *idx;
    let current = &tokens[base];
    if let Some(st) = types.get(current.string){
        *idx += 1;
        return Some(st.clone());
    }
    if current.string == "*"{
        *idx +=1;
        return Some(parse_declared_type(tokens, idx, types)).flatten().map(|i| Type::PointerT { ptr_type:Box::new(i) });
    }
    if current.string == "["{
        if tokens.get(base+1)?.string == "]"{
            *idx += 2;
            return Some(parse_declared_type(tokens, idx, types)).flatten().map(|i| Type::VecT { ptr_type:Box::new(i) });
        }  else if tokens.get(base+2)?.string == "]"{
            if let Ok(count) = tokens[base+1].string.parse::<usize>(){
                return Some(parse_declared_type(tokens, idx, types)).flatten().map(|i| Type::ArrayT { size:count,array_type:Box::new(i) });
            } else{
                return None;
            }
        }
        else{
            return None;
        }
    }
    println!("error unknown type:{:#?}", tokens[base].string);
    return None;
}
pub fn parse_type(text:&[Token], types:&HashMap<String,Type>)->Option<(String,Type)>{
    if text.len()<3{
        println!("error requires more than three tokens to declare struct");
    }
    if *text.get(0)? != "("{
        println!("error expected parenthesis, line: {}", text.get(0)?.line);
        return None;
    } 
    if *text.get(1)? != "struct"{
        println!("expected struct declaration line{}", text.get(1)?.line);
        
    }
    let name = String::from(text.get(2)?.string);
    let mut out_types = vec![];
    let mut idx = 3;
    while idx<text.len(){
        let ident_name = &text[idx];
        if text[idx+1] != ":"{
            println!("error unexpected non : character {} at line:{}", text[idx+1].string, text[idx+1].line);
            return None;
        }
        idx += 2;
        let comp_type = parse_declared_type(text, &mut idx, types);
        if comp_type.is_none(){
            println!("error: unknown type:{} at line:{}", text[idx].string, text[idx].line);
            return None;
        }
        out_types.push((String::from(ident_name.string),comp_type.unwrap().clone()));
    }
    return Some((name.clone(),Type::StructT{name, components:out_types.clone()}));
}

pub fn parse_expression(text:&[Token], cursor:&mut usize, types:&HashMap<String,Type>, scope:&mut Scope, function_table:&HashMap<String, Function>)->Option<AstNode>{
    if text[*cursor] == "("{
        if function_table.contains_key(text[*cursor+1].string){
            let mut args = vec![];
            let last_idx = calc_close_paren(text, *cursor)?;
            while *cursor<last_idx{
                args.push(parse_expression(text, cursor, types, scope, function_table)?);
            }
            *cursor+=1;
            return Some(AstNode::FunctionCall { function_name: text[*cursor].string.to_owned(), args:  args});
        } else if text[*cursor+1] == "+"{
            let mut args = vec![];
            let last_idx = calc_close_paren(text, *cursor)?;
            while *cursor<last_idx{
                args.push(parse_expression(text, cursor, types, scope, function_table)?);
            }
            *cursor+=1;
            return Some(AstNode::Add { left: Box::new(args[0].clone()), right:Box::new(args[1].clone()) });
        } else if text[*cursor+1] == "-"{
            let mut args = vec![];
            let last_idx = calc_close_paren(text, *cursor)?;
            while *cursor<last_idx{
                args.push(parse_expression(text, cursor, types, scope, function_table)?);
            }
            *cursor+=1;
            return Some(AstNode::Sub{ left: Box::new(args[0].clone()), right:Box::new(args[1].clone()) });
        } else if text[*cursor+1] == "*"{
            let mut args = vec![];
            let last_idx = calc_close_paren(text, *cursor)?;
            while *cursor<last_idx{
                args.push(parse_expression(text, cursor, types, scope, function_table)?);
            }
            *cursor+=1;
            if args.len() == 1{
                return Some(AstNode::Deref { thing_to_deref: Box::new(args[0].clone())});
            } else{
                return Some(AstNode::Mult { left: Box::new(args[0].clone()), right:Box::new(args[1].clone()) });
            }
        } else if text[*cursor+1] == "/"{
            let mut args = vec![];
            let last_idx = calc_close_paren(text, *cursor)?;
            while *cursor<last_idx{
                args.push(parse_expression(text, cursor, types, scope, function_table)?);
            }
            *cursor+=1;
            return Some(AstNode::Div { left: Box::new(args[0].clone()), right:Box::new(args[1].clone()) });
        } else if text[*cursor+1] == "=="{
            let mut args = vec![];
            let last_idx = calc_close_paren(text, *cursor)?;
            while *cursor<last_idx{
                args.push(parse_expression(text, cursor, types, scope, function_table)?);
            }
            *cursor+=1;
            return Some(AstNode::Equals { left: Box::new(args[0].clone()), right:Box::new(args[1].clone()) });
        } else if text[*cursor+1] == "<="{
            let mut args = vec![];
            let last_idx = calc_close_paren(text, *cursor)?;
            while *cursor<last_idx{
                args.push(parse_expression(text, cursor, types, scope, function_table)?);
            }
            *cursor+=1;
            return Some(AstNode::LessOrEq { left: Box::new(args[0].clone()), right:Box::new(args[1].clone()) });
        } else if text[*cursor+1] == ">="{
            let mut args = vec![];
            let last_idx = calc_close_paren(text, *cursor)?;
            while *cursor<last_idx{
                args.push(parse_expression(text, cursor, types, scope, function_table)?);
            }
            *cursor+=1;
            return Some(AstNode::GreaterOrEq { left: Box::new(args[0].clone()), right:Box::new(args[1].clone()) });
        } else if text[*cursor+1] == "<"{
            let mut args = vec![];
            let last_idx = calc_close_paren(text, *cursor)?;
            while *cursor<last_idx{
                args.push(parse_expression(text, cursor, types, scope, function_table)?);
            }
            *cursor+=1;
            return Some(AstNode::LessThan { left: Box::new(args[0].clone()), right:Box::new(args[1].clone()) });
        } else if text[*cursor+1] == ">"{
            let mut args = vec![];
            let last_idx = calc_close_paren(text, *cursor)?;
            while *cursor<last_idx{
                args.push(parse_expression(text, cursor, types, scope, function_table)?);
            }
            *cursor+=1;
            return Some(AstNode::GreaterThan {left: Box::new(args[0].clone()), right:Box::new(args[1].clone()) });
        } else if text[*cursor+1] == "&&"{
            let mut args = vec![];
            let last_idx = calc_close_paren(text, *cursor)?;
            while *cursor<last_idx{
                args.push(parse_expression(text, cursor, types, scope, function_table)?);
            }
            *cursor+=1;
            return Some(AstNode::And{left: Box::new(args[0].clone()), right:Box::new(args[1].clone()) });
        } else if text[*cursor+1] == "||"{
            let mut args = vec![];
            let last_idx = calc_close_paren(text, *cursor)?;
            while *cursor<last_idx{
                args.push(parse_expression(text, cursor, types, scope, function_table)?);
            }
            *cursor+=1;
            return Some(AstNode::Or{left: Box::new(args[0].clone()), right:Box::new(args[1].clone()) });
        } else if text[*cursor+1] == "="{
            let mut args = vec![];
            let last_idx = calc_close_paren(text, *cursor)?;
            while *cursor<last_idx{
                args.push(parse_expression(text, cursor, types, scope, function_table)?);
            }
            *cursor+=1;
            return Some(AstNode::Assignment{left: Box::new(args[0].clone()), right:Box::new(args[1].clone()) });
        }
        else if text[*cursor+1] == "let"{
            let name = text[*cursor+1].string.to_owned();
            let var_type = parse_declared_type(text, cursor, types)?;
            *cursor +=1;
            scope.declare_variable(var_type.clone(), name.clone());
            if text[*cursor] != "="{
                return  Some(AstNode::VariableDeclaration { name , var_type,value_assigned:None });
            } 
            let next = parse_expression(text, cursor, types, scope, function_table)?;
            return Some(AstNode::VariableDeclaration { name , var_type, value_assigned: Some(Box::new(next)) });
        }
        else if text[*cursor+1] == "if"{
            *cursor+=2;
            let condition=  parse_expression(text, cursor, types, scope, function_table)?;
            scope.push_scope();
            let to_do = parse_expression(text, cursor, types, scope, function_table)?;
            scope.pop_scope();
            if text[*cursor] == "else"{
                let to_do_else = parse_expression(text, cursor, types, scope, function_table)?;
                *cursor+=1;
                return Some(AstNode::If { condition:Box::new(condition), thing_to_do: Box::new(to_do), r#else: Some(Box::new(to_do_else))});
            }
            return Some(AstNode::If { condition:Box::new(condition), thing_to_do: Box::new(to_do), r#else: None});
        }
        else{
            let mut out = vec![];
            let last_idx = calc_close_paren(text, *cursor)?;
            *cursor+=1;
            while *cursor<last_idx{
                out.push(parse_expression(text, cursor, types, scope, function_table)?);
                *cursor+=1;
        }
        return Some(AstNode::StructLiteral { nodes: out });}
    } else if text[*cursor] == "["{
        let mut out = vec![];
        let last_idx = calc_close_block(text, *cursor)?;
        while *cursor<last_idx{
            out.push(parse_expression(text, cursor, types, scope, function_table)?);
        }
        return Some(AstNode::ArrayLiteral { nodes: out });
    } else if is_numbers(text[*cursor].string){
        if text[*cursor].string.contains('.'){
            return Some(AstNode::FloatLiteral { value: text[*cursor].string.parse::<f64>().unwrap() });
        } else{
            return Some(AstNode::IntLiteral { value: text[*cursor].string.parse::<i64>().unwrap() });
        }
    } else if text[*cursor].string.chars().collect::<Vec<char>>()[0] == '"'{
        return Some(AstNode::StringLiteral { value: text[*cursor].string.to_owned() });
    } else if let Some(v) = scope.variable_idx(text[*cursor].string.to_owned()){
        return Some(AstNode::VariableUse { name:text[*cursor].string.to_owned(), index:v.1, vtype:v.0 , is_arg:v.2});
    }
    println!("returned none on {:#?}", text[*cursor]);
    return None;
}

pub fn parse_global(text:&[Token], types:&HashMap<String,Type>)->Option<(String,Type,AstNode)>{
    let mut idx = 0;
    if text[idx] != "("{
        println!("error expected ( line:{}",text[idx].line);
        return None;
    }
    idx +=1;
    if text[idx] != "let"{
        println!("error expected let: line:{}", text[idx].line);
        return None;
    }
    idx +=1 ;
    let name = text[idx].string;
    idx += 1;
    if text[idx] != ":"{
        println!("error expected : line:{}", text[idx].line);
        return None;
    }
    idx +=1;
    let vtype = parse_declared_type(text, &mut idx, types)?;
    let mut scope = Scope::new(&HashMap::new());
    let function_table = HashMap::new();
    idx += 1;
    let node = parse_expression(text, &mut idx, types, &mut scope, &function_table);
    if node.is_none(){
        println!("failed to parse global variable assignment");
    }
    return Some((String::from(name), vtype, node?));
}


pub fn parse_function(text:&[Token], types:&HashMap<String,Type>, globals:&HashMap<String, (Type,usize)>, function_table:&HashMap<String, Function>)->Option<(String,Function)>{
    
    let fn_end = text.len();
    let mut args = vec![];
    let mut arg_names = vec![];
    let mut nodes = vec![];
    let mut cursor = 2_usize;
    let name = text[2].string.to_owned();
    cursor+=1;
    if text[cursor] != "("{
        println!("error expected paren");
    }
    let args_end = calc_close_paren(text, cursor)?;
    cursor+=1;
    while cursor<args_end{
        let name = text[cursor].to_owned();
        cursor +=1;
        if text[cursor] != ":"{
            println!("error expected :");
        }
        cursor+=1;
        let vtype = parse_declared_type(text, &mut cursor, types)?;
        arg_names.push(name.string.to_owned());
        args.push(vtype);
    }
    cursor +=1;
    if text[cursor] != "->"{
        println!("error requires -> for return type of function");
    }
    cursor+=1;
    let return_type = parse_declared_type(text, &mut cursor, types)?;
    let mut scope = Scope::new(globals);
    while cursor<fn_end{
        nodes.push(parse_expression(text, &mut cursor, types,&mut  scope, function_table)?);
    }
    return Some((name.clone(),Function{name, return_type:return_type, args:args, arg_names:arg_names, program:nodes}));
}

pub fn program_to_ast(program:&str)->Option<Program>{
    let tokens = tokenize(program);
    //println!("{:#?}", tokens);
    let globals_result = extract_globals(&tokens);
    if globals_result.is_err(){
        let s  =  globals_result.expect_err("is error shouldn't break");
        println!("{}",s);
        return None;
    }
    let globals = globals_result.expect("is ok by previous call");
    //println!("globals:{:#?}",globals);
    let mut types:HashMap<String,Type> = HashMap::new();
    types.insert(String::from("bool"), Type::BoolT);
    types.insert(String::from("int"),Type::IntegerT );
    types.insert(String::from("float"), Type::FloatT);
    types.insert(String::from("matrix"), Type::MatrixT);
    types.insert(String::from("string"), Type::StringT);
    types.insert(String::from("void"), Type::VoidT);
    let mut scope:HashMap<String,(Type,usize)> = HashMap::new();
    let mut functions:HashMap<String,Function> =HashMap::new();
    for i in &globals{
        match i {
            GlobalTypes::StructDef{text} =>{
                let tmp = parse_type(*text,&types)?;
                if types.contains_key(&tmp.0){
                    println!("error {} redeclared", tmp.0);
                    return None;
                }
                types.insert(tmp.0,tmp.1);
            }
            GlobalTypes::FunctionDef{text:_}=>{}
            GlobalTypes::GlobalDef { text:_}=>{}
        }
    }
    let mut global_count = 0;
    for i in &globals{
        match i {
            GlobalTypes::StructDef{text:_} =>{}
            GlobalTypes::GlobalDef{text}=>{
                let tmp = parse_global(*text,&types)?;
                if scope.contains_key(&tmp.0){
                    println!("error {} redeclared", tmp.0);
                    return None;
                }
                scope.insert(tmp.0,(tmp.1,global_count));
                global_count+= 1; 
            }
            GlobalTypes::FunctionDef { text:_}=>{}
        }
    }
    for i in &globals{
        match i {
            GlobalTypes::StructDef{text:_} =>{}
            GlobalTypes::FunctionDef{text}=>{
                let tmp = parse_function(*text,&types, &scope, &functions)?;
                if functions.contains_key(&tmp.0){
                    println!("error {} redeclared", tmp.0);
                    return None;
                }
                functions.insert(tmp.0,tmp.1);
            }
            GlobalTypes::GlobalDef { text:_}=>{}
        }
    }
    return Some(Program{types,functions, static_variables:scope});
}
		

fn main() { 
    let tprg = std::fs::read_to_string("test.risp").expect("testing expect");
    let prg = program_to_ast(&tprg);
    println!("{:#?}",prg);
}
