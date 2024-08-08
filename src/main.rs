use std::collections::HashMap;
#[derive(Clone,Debug)]
pub enum Type{
    BoolT,IntegerT,FloatT,StringT, MatrixT, StructT{name:String, components:Vec<Type>}, PointerT{ptr_type:Box<Type>},
}
#[derive(Clone,Debug)]
pub enum AstNode{
    StringLiteral{value:String}, 
    IntLiteral{value:i64}, 
    FloatLiteral{value:f64}, 
    StructLiteral{nodes:Vec<AstNode>},
    VariableUse{name:String},
    FunctionCall{function_name:String, args:Vec<AstNode>},
    Assignment{left:Box<AstNode>, right:Box<AstNode>},
    VariableDeclaration{name:String, var_type:Type, value_assigned:Option<Box<AstNode>>},
    Add{left:Box<AstNode>, right:Box<AstNode>},
    Sub{left:Box<AstNode>, right:Box<AstNode>},
    Mult{left:Box<AstNode>, right:Box<AstNode>},
    Div{left:Box<AstNode>, right:Box<AstNode>},
}
#[derive(Debug)]
pub struct Function{
    name:String, 
    return_type:Type, 
    args:Vec<Type>,
    program:Vec<AstNode>,
}
#[derive(Debug)]
pub struct Program{
    types:HashMap<String, Type>, 
    functions:HashMap<String, Function>,
    static_variables:HashMap<String, (Type,usize)>,
}
#[derive(Debug,Clone,Copy)]
pub enum GlobalTypes<'a>{
    StructDef{text:&'a [&'a str]},
    FunctionDef{text:&'a [&'a str]},
    GlobalDef{text:&'a [&'a str]},
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
pub fn tokenize<'a>(program:&'a str)->Vec<&'a str>{
    let mut out:Vec<&'a str>= program.split_whitespace().collect();
    out = out.iter().map(|i| split_by(i,'(')).flatten().collect();
    out = out.iter().map(|i| split_by(i,')')).flatten().collect();
    out = out.iter().map(|i| split_by(i,':')).flatten().collect();
    out = out.iter().map(|i| split_by(i,'+')).flatten().collect();
    out = out.iter().map(|i| split_by(i,'-')).flatten().collect();
    out = out.iter().map(|i| split_by(i,'=')).flatten().collect();
    out = out.iter().map(|i| split_by(i,'/')).flatten().collect();
    out = out.iter().map(|i| split_by(i,'[')).flatten().collect();
    out = out.iter().map(|i| split_by(i,']')).flatten().collect();
    return out;
}
pub fn extract_global<'a>(tokens:&'a[&'a str],idx:&mut usize)->Option<GlobalTypes<'a>>{
    let start = *idx;
    if tokens[*idx] != "(" {
        return None;
    } 
    let mut parens_count = 1;
    while parens_count>0{
        *idx += 1;
        if tokens[*idx] == "("{
            parens_count+= 1;
        } 
        if tokens[*idx] == ")"{
            parens_count -= 1;
        }
        if *idx>tokens.len(){
            return None;
        }
    }

    let span = &tokens[start..*idx];
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
    return None;
 }
pub fn extract_globals<'a>(tokens:&'a[&'a str])->Result<Vec<GlobalTypes<'a>>,String>{
    let mut out = vec![];
    let mut idx = 0;
    while let Some(p) = extract_global(&tokens, &mut idx){
        out.push(p.clone());
    }
    return Ok(out);
}
pub fn parse_type(text:&[&str], types:&HashMap<String,Type>)->Option<(String,Type)>{
    if *text.get(0)? != "("{
        println!("error expect parenthesis");
        return None;
    } 
    if *text.get(1)? != "struct"{
        println("expected struct declaration");
        
    }
    
    return None;
}
pub fn parse_global(text:&[&str], types:&HashMap<String,Type>)->Option<(String,Type)>{
    None
}
pub fn parse_function(text:&[&str], types:&HashMap<String,Type>, global_variables:&HashMap<String, (Type,usize)>)->Option<(String,Function)>{
    println!("{:#?}",text);
    None
}
pub fn program_to_ast(program:&str)->Option<Program>{
    let tokens = tokenize(program);
    let globals_result = extract_globals(&tokens);
    if globals_result.is_err(){
        let s  =  globals_result.expect_err("is error shouldn't break");
        println!("{}",s);
        return None;
    }
    let globals = globals_result.expect("is ok by previous call");
    let mut types:HashMap<String,Type> = HashMap::new();
    let mut global_variables:HashMap<String,(Type,usize)> = HashMap::new();
    let mut functions:HashMap<String,Function> =HashMap::new();
    for i in &globals{
        match i {
            GlobalTypes::StructDef{text} =>{
                //types.push(parse_type(*text, &types)?);
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
                if global_variables.contains_key(&tmp.0){
                    println!("error {} redeclared", tmp.0);
                    return None;
                }
                global_variables.insert(tmp.0,(tmp.1,global_count));
                global_count+= 1;
            }
            GlobalTypes::FunctionDef { text:_}=>{}
        }
    }
    for i in &globals{
        match i {
            GlobalTypes::StructDef{text:_} =>{}
            GlobalTypes::FunctionDef{text}=>{
                let tmp = parse_function(*text,&types, &global_variables)?;
                if functions.contains_key(&tmp.0){
                    println!("error {} redeclared", tmp.0);
                    return None;
                }
                functions.insert(tmp.0,tmp.1);
            }
            GlobalTypes::GlobalDef { text:_}=>{}
        }
    }
    return Some(Program{types,functions, static_variables:global_variables});
}
const PROGRAM:&str = "(fn main {}->void(print (+ 10 (* 2 15)))))";
fn main() {
    let tokens = tokenize(PROGRAM);
    for i in &tokens{
        println!("<{}>",i);
    }
    let prg = program_to_ast(PROGRAM);
    println!("{:#?}",prg);
}
