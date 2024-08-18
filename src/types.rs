pub use core::str;
pub use std::collections::HashMap;
pub use std::slice;
#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub string: &'a str,
    pub line: usize,
}

impl<'a> Into<&'a str> for Token<'a> {
    fn into(self) -> &'a str {
        return self.string;
    }
}
impl PartialEq<&str> for Token<'_> {
    fn eq(&self, other: &&str) -> bool {
        return self.string == *other;
    }
}

#[derive(Clone, Debug)]
pub enum Type {
    BoolT,
    IntegerT,
    FloatT,
    StringT,
    StructT {
        name: String,
        components: Vec<(String, Type)>,
    },
    PointerT {
        ptr_type: Box<Type>,
    },
    ArrayT {
        size: usize,
        array_type: Box<Type>,
    },
    VoidT,
    SliceT {
        ptr_type: Box<Type>,
    },
}
impl Type{
    pub fn get_array_type(&self)->Option<Type>{
        match &self{
            &Self::ArrayT { size, array_type }=>{
                return Some(array_type.as_ref().clone());
            }
            &Self::SliceT { ptr_type }=>{
                return Some(ptr_type.as_ref().clone());
            }
            _=>{
                return None;
            }
        }
    }
}
pub fn is_compatible_type(a: &Type, b: &Type) -> bool {
    match a {
        Type::BoolT => match b {
            Type::BoolT => {
                return true;
            }
            _ => {
                return false;
            }
        },
        Type::IntegerT => match b {
            Type::FloatT => {
                return true;
            }
            Type::IntegerT => {
                return true;
            }
            _ => {
                return false;
            }
        },
        Type::FloatT => match b {
            Type::FloatT => {
                return true;
            }
            Type::IntegerT => {
                return true;
            }
            _ => {
                return false;
            }
        },
        Type::StringT => match b {
            Type::StringT => {
                return true;
            }
            _ => {
                return false;
            }
        },
        Type::StructT { name, components } => {
            let aname = name;
            let acomponents = components;
            match b {
                Type::StructT { name, components } => {
                    if name == "" || aname == "" {
                        if acomponents.len() != components.len() {
                            return false;
                        }
                        for i in 0..acomponents.len() {
                            if !is_compatible_type(&acomponents[i].1, &components[i].1) {
                                return false;
                            }
                        }
                        return true;
                    } else {
                        return aname == name;
                    }
                }
                _ => {
                    return false;
                }
            }
        }
        Type::PointerT { ptr_type } => {
            let at = ptr_type;
            match b {
                Type::PointerT { ptr_type } => {
                    return is_compatible_type(&at, &ptr_type);
                }
                _ => {
                    return false;
                }
            }
        }
        Type::ArrayT { array_type, size } => {
            let at = array_type;
            let asize = size;
            match b {
                Type::ArrayT { array_type, size } => {
                    return is_compatible_type(&at, &array_type) && asize == size;
                }
                _ => {
                    return false;
                }
            }
        }
        Type::VoidT => match b {
            Type::VoidT {} => {
                return true;
            }
            _ => {
                return false;
            }
        },
        Type::SliceT { ptr_type } => {
            let at = ptr_type;
            match b {
                Type::SliceT { ptr_type } => {
                    return is_compatible_type(&at, ptr_type);
                }
                _ => {
                    return false;
                }
            }
        }
    }
}
#[allow(unused)]
pub fn is_equal_type(a:&Type, b:&Type)->bool{
    match a {
        Type::BoolT => match b {
            Type::BoolT => {
                return true;
            }
            _ => {
                return false;
            }
        },
        Type::IntegerT => match b {
            Type::IntegerT => {
                return true;
            }
            _ => {
                return false;
            }
        },
        Type::FloatT => match b {
            Type::FloatT => {
                return true;
            }
            _ => {
                return false;
            }
        },
        Type::StringT => match b {
            Type::StringT => {
                return true;
            }
            _ => {
                return false;
            }
        },
        Type::StructT { name, components } => {
            let aname = name;
            let acomponents = components;
            match b {
                Type::StructT { name, components } => {
                    if name == "" || aname == "" {
                        if acomponents.len() != components.len() {
                            return false;
                        }
                        for i in 0..acomponents.len() {
                            if !is_equal_type(&acomponents[i].1, &components[i].1) {
                                return false;
                            }
                        }
                        return true;
                    } else {
                        return aname == name;
                    }
                }
                _ => {
                    return false;
                }
            }
        }
        Type::PointerT { ptr_type } => {
            let at = ptr_type;
            match b {
                Type::PointerT { ptr_type } => {
                    return is_equal_type(&at, &ptr_type);
                }
                _ => {
                    return false;
                }
            }
        }
        Type::ArrayT { array_type, size } => {
            let at = array_type;
            let asize = size;
            match b {
                Type::ArrayT { array_type, size } => {
                    return is_equal_type(&at, &array_type) && asize == size;
                }
                _ => {
                    return false;
                }
            }
        }
        Type::VoidT => match b {
            Type::VoidT {} => {
                return true;
            }
            _ => {
                return false;
            }
        },
        Type::SliceT { ptr_type } => {
            let at = ptr_type;
            match b {
                Type::SliceT { ptr_type } => {
                    return is_equal_type(&at, ptr_type);
                }
                _ => {
                    return false;
                }
            }
        }
    }
}
#[allow(unused)]
#[derive(Clone,Debug)]
pub struct AstNodeData{
    pub line:usize,
    pub temporary_index:Option<usize>,
}
#[allow(unused)]
#[derive(Clone, Debug)]
pub enum AstNode {
    VoidLiteral,
    BoolLiteral {
        value: bool,
    },
    StringLiteral {
        value: String,
    },
    IntLiteral {
        value: i64,
    },
    FloatLiteral {
        value: f64,
    },
    StructLiteral {
        nodes: Vec<AstNode>,
    },
    ArrayLiteral {
        nodes: Vec<AstNode>,
    },
    VariableUse {
        name: String,
        index: usize,
        vtype: Type,
is_arg: bool,
        data:Option<AstNodeData>, 
    },
    FunctionCall {
        function_name: String,
        args: Vec<AstNode>,
        data:Option<AstNodeData>,
    },
    Assignment {
        left: Box<AstNode>,
        right: Box<AstNode>,
        data:Option<AstNodeData>,
    },
    VariableDeclaration {
        name: String,
        var_type: Type,
        value_assigned: Option<Box<AstNode>>,
    },
    Add {
        left: Box<AstNode>,
        right: Box<AstNode>,
        data:Option<AstNodeData>
    },
    Sub {
        left: Box<AstNode>,
        right: Box<AstNode>,
        data:Option<AstNodeData>
    },
    Mult {
        left: Box<AstNode>,
        right: Box<AstNode>,
        data:Option<AstNodeData>
    },
    Div {
        left: Box<AstNode>,
        right: Box<AstNode>,
        data:Option<AstNodeData>
    },
    Equals {
        left: Box<AstNode>,
        right: Box<AstNode>,
        data:Option<AstNodeData>
    },
    GreaterThan {
        left: Box<AstNode>,
        right: Box<AstNode>,
        data:Option<AstNodeData>
    },
    LessThan {
        left: Box<AstNode>,
        right: Box<AstNode>,
        data:Option<AstNodeData>
    },
    GreaterOrEq {
        left: Box<AstNode>,
        right: Box<AstNode>,
        data:Option<AstNodeData>
    },
    LessOrEq {
        left: Box<AstNode>,
        right: Box<AstNode>,
        data:Option<AstNodeData>
    },
    Not {
        value: Box<AstNode>,
        data:Option<AstNodeData>
    },
    And {
        left: Box<AstNode>,
        right: Box<AstNode>,
        data:Option<AstNodeData>
    },
    Or {
        left: Box<AstNode>,
        right: Box<AstNode>,
        data:Option<AstNodeData>
    },
    If {
        condition: Box<AstNode>,
        thing_to_do: Vec<AstNode>,
        r#else: Option<Vec<AstNode>>,
    },
    Loop {
        condition: Box<AstNode>,
        body: Vec<AstNode>,
    },
    ForLoop {
        variable: Box<AstNode>,
        condition: Box<AstNode>,
        post_op: Box<AstNode>,
        body:Vec<AstNode>,
    },
    Return {
        body: Box<AstNode>,
    },
    Deref {
        thing_to_deref: Box<AstNode>,
    },
    TakeRef {
        thing_to_ref: Box<AstNode>,
    },
    FieldUsage {
        base: Box<AstNode>,
        field_name: String,
    },
    ArrayAccess{
        variable:Box<AstNode>,
        index:Box<AstNode>
    },
    BoundFunctionCall{
        variable:Box<AstNode>,
        function_name:String,
        args:Vec<AstNode>,
    }
}

impl AstNode {
    pub fn get_type(
        &self,
        function_table: &HashMap<String, Function>,
        types: &HashMap<String, Type>,
    ) -> Option<Type> {
        match self {
            Self::VoidLiteral {} => Some(Type::VoidT),
            Self::BoolLiteral { value: _ } => Some(Type::BoolT),
            Self::StringLiteral { value: _ } => Some(Type::StringT),
            Self::IntLiteral { value: _ } => Some(Type::IntegerT),
            Self::FloatLiteral { value: _ } => Some(Type::FloatT),
            Self::StructLiteral { nodes } => {
                let mut out_types = vec![];
                for i in nodes {
                    out_types.push((("").to_owned(), i.get_type(function_table, types)?));
                }
                return Some(Type::StructT {
                    name: "".to_owned(),
                    components: out_types,
                });
            }
            Self::ArrayLiteral { nodes } => {
                let mut out_types = vec![];
                for i in nodes {
                    out_types.push((("").to_owned(), i.get_type(function_table, types)?));
                }
                return Some(Type::StructT {
                    name: "".to_owned(),
                    components: out_types,
                });
            }
            Self::VariableUse {
                name: _,
                index: _,
                vtype,
                is_arg: _,
                data:_,
            } => {
                return Some(vtype.clone());
            }
            Self::FunctionCall {
                function_name,
                args: _,
                data:_,
            } => {
                return Some(function_table.get(function_name)?.return_type.clone());
            }
            Self::Assignment { left: _, right: _ ,data:_} => {
                return Some(Type::VoidT);
            }
            Self::VariableDeclaration {
                name: _,
                var_type: _,
                value_assigned: _,
            } => {
                return Some(Type::VoidT);
            }
            Self::Add { left, right ,data:_} => {
                if is_compatible_type(
                    &left.get_type(function_table, types)?,
                    &right.get_type(function_table, types)?,
                ) {
                    return left.get_type(function_table, types);
                }
                return None;
            }
            Self::Sub { left, right,data:_ } => {
                if is_compatible_type(
                    &left.get_type(function_table, types)?,
                    &right.get_type(function_table, types)?,
                ) {
                    return left.get_type(function_table, types);
                }
                return None;
            }
            Self::Mult { left, right,data:_ } => {
                if is_compatible_type(
                    &left.get_type(function_table, types)?,
                    &right.get_type(function_table, types)?,
                ) {
                    return left.get_type(function_table, types);
                }
                return None;
            }
            Self::Div { left, right ,data:_} => {
                if is_compatible_type(
                    &left.get_type(function_table, types)?,
                    &right.get_type(function_table, types)?,
                ) {
                    return left.get_type(function_table, types);
                }
                return None;
            }
            Self::Equals { left: _, right: _ ,data:_} => {
                return Some(Type::BoolT);
            }
            Self::LessThan { left: _, right: _ ,data:_} => {
                return Some(Type::BoolT);
            }
            Self::GreaterThan { left: _, right: _ ,data:_} => {
                return Some(Type::BoolT);
            }
            Self::GreaterOrEq { left: _, right: _ ,data:_} => {
                return Some(Type::BoolT);
            }
            Self::LessOrEq { left: _, right: _ ,data:_} => {
                return Some(Type::BoolT);
            }
            Self::Not { value: _ ,data:_} => {
                return Some(Type::BoolT);
            }
            Self::And { left: _, right: _ ,data:_} => {
                return Some(Type::BoolT);
            }
            Self::Or { left: _, right: _ ,data:_} => {
                return Some(Type::BoolT);
            }
            Self::If {
                condition: _,
                thing_to_do: _,
                r#else: _,
            } => {
                return Some(Type::VoidT);
            }
            Self::Loop {
                condition: _,
                body: _,
            } => {
                return Some(Type::VoidT);
            }
            Self::ForLoop {
                variable: _,
                condition: _,
                post_op: _,
                body: _,
            } => {
                return Some(Type::VoidT);
            }
            Self::Return { body: _ } => {
                return Some(Type::VoidT);
            }
            Self::Deref { thing_to_deref } => match (*thing_to_deref).as_ref() {
                Self::VariableUse {
                    name: _,
                    index: _,
                    vtype,
                    is_arg: _,
                    data:_,
                } => match vtype {
                    Type::PointerT { ptr_type } => {
                        return Some(ptr_type.as_ref().clone());
                    }
                    _ => {
                        return None;
                    }
                },
                _ => {
                    return None;
                }
            },
            Self::TakeRef { thing_to_ref } => match (*thing_to_ref).as_ref() {
                Self::VariableUse {
                    name: _,
                    index: _,
                    vtype,
                    is_arg: _,
                    data:_,
                } => {
                    return Some(Type::PointerT {
                        ptr_type: Box::new(vtype.clone()),
                    });
                }
                _ => {
                    return None;
                }
            },
            Self::FieldUsage {
                base: _,
                field_name: _,
            } => {
                return None;
            }
            Self::ArrayAccess { variable, index:_ }=>{
                return  Some(variable.get_type(function_table, types)?.get_array_type()?);
            }
            Self::BoundFunctionCall { variable:_, function_name, args:_ }=>{
                return Some(function_table.get(function_name)?.return_type.clone());
            }
        }
    }
    pub fn get_priority(&self) -> usize {
        match self {
            Self::VoidLiteral {} => {
                return 0;
            }
            Self::BoolLiteral { value: _ } => {
                return 0;
            }
            Self::StringLiteral { value: _ } => {
                return 0;
            }
            Self::IntLiteral { value: _ } => {
                return 0;
            }
            Self::FloatLiteral { value: _ } => {
                return 0;
            }
            Self::StructLiteral { nodes: _ } => {
                return 0;
            }
            Self::ArrayLiteral { nodes: _ } => {
                return 0;
            }
            Self::VariableUse {
                name: _,
                index: _,
                vtype: _,
                is_arg: _,
                data:_,
            } => {
                return 0;
            }
            Self::FunctionCall {
                function_name: _,
                args: _,
                data:_,
            } => {
                return 0;
            }
            Self::Assignment { left: _, right: _ ,data:_} => {
                return 8;
            }
            Self::VariableDeclaration {
                name: _,
                var_type: _,
                value_assigned: _,
            } => {
                return 0;
            }
            Self::Add { left: _, right: _,data:_ } => {
                return 5;
            }
            Self::Sub { left: _, right: _ ,data:_} => {
                return 5;
            }
            Self::Mult { left: _, right: _ ,data:_} => {
                return 3;
            }
            Self::Div { left: _, right: _ ,data:_} => {
                return 3;
            }
            Self::Equals { left: _, right: _ ,data:_} => {
                return 6;
            }
            Self::LessThan { left: _, right: _ ,data:_} => {
                return 6;
            }
            Self::GreaterThan { left: _, right: _ ,data:_} => {
                return 6;
            }
            Self::GreaterOrEq { left: _, right: _ ,data:_} => {
                return 6;
            }
            Self::LessOrEq { left: _, right: _,data:_ } => {
                return 6;
            }
            Self::Not { value: _,data:_ } => {
                return 0;
            }
            Self::And { left: _, right: _ ,data:_} => {
                return 7;
            }
            Self::Or { left: _, right: _ ,data:_} => {
                return 7;
            }
            Self::If {
                condition: _,
                thing_to_do: _,
                r#else: _,
            } => {
                return 0;
            }
            Self::Loop {
                condition: _,
                body: _,
            } => {
                return 0;
            }
            Self::ForLoop {
                variable: _,
                condition: _,
                post_op: _,
                body: _,
            } => {
                return 0;
            }
            Self::Return { body: _ } => {
                return 8;
            }
            Self::Deref { thing_to_deref: _ } => {
                return 1;
            }
            Self::TakeRef { thing_to_ref: _ } => {
                return 1;
            }
            Self::FieldUsage {
                base: _,
                field_name: _,
            } => {
                return 1;
            }
            Self::ArrayAccess {
                variable: _,
                index: _,
            } => {
                return 1;
            }
            Self::BoundFunctionCall { variable:_, function_name:_, args:_ }=>{
                return 1;
            }
        }
    }
    pub fn get_data(&self)->Option<&AstNodeData>{
        match self{
            Self::VariableUse{name:_, index:_, vtype:_, is_arg:_, data}=>{
                data.as_ref()
            }
            Self::FunctionCall{function_name:_, args:_, data}=>{
                data.as_ref()
            }
            Self::Assignment{left:_, right:_, data}=>{
                data.as_ref()
            }
            Self::Add{left:_, right:_, data}=>{
                data.as_ref()
            }
            Self::Sub{left:_, right:_, data}=>{
                data.as_ref()
            }
            Self::Mult{left:_, right:_, data}=>{
                data.as_ref()
            }
            Self::Div{left:_, right:_, data}=>{
                data.as_ref()
            }
            Self::Equals{left:_, right:_, data}=>{
                data.as_ref()
            }
            Self::GreaterThan{left:_, right:_, data}=>{
                data.as_ref()
            }
            Self::LessThan{left:_, right:_, data}=>{
                data.as_ref()
            }
            Self::GreaterOrEq{left:_, right:_, data}=>{
                data.as_ref()
            }
            Self::LessOrEq{left:_, right:_, data}=>{
                data.as_ref()
            }
            Self::Not{value:_, data}=>{
                data.as_ref()
            }
            Self::And{left:_, right:_, data}=>{
                data.as_ref()
            }
            Self::Or{left:_, right:_, data}=>{
                data.as_ref()
            }
            _=>{
                None
            }
        }
    }
    pub fn get_data_mut(&mut self)->Option<&mut AstNodeData>{
        match self{
            Self::VariableUse{name:_, index:_, vtype:_, is_arg:_, data}=>{
                data.as_mut()
            }
            Self::FunctionCall{function_name:_, args:_, data}=>{
                data.as_mut()
            }
            Self::Assignment{left:_, right:_, data}=>{
                data.as_mut()
            }
            Self::Add{left:_, right:_, data}=>{
                data.as_mut()
            }
            Self::Sub{left:_, right:_, data}=>{
                data.as_mut()
            }
            Self::Mult{left:_, right:_, data}=>{
                data.as_mut()
            }
            Self::Div{left:_, right:_, data}=>{
                data.as_mut()
            }
            Self::Equals{left:_, right:_, data}=>{
                data.as_mut()
            }
            Self::GreaterThan{left:_, right:_, data}=>{
                data.as_mut()
            }
            Self::LessThan{left:_, right:_, data}=>{
                data.as_mut()
            }
            Self::GreaterOrEq{left:_, right:_, data}=>{
                data.as_mut()
            }
            Self::LessOrEq{left:_, right:_, data}=>{
                data.as_mut()
            }
            Self::Not{value:_, data}=>{
                data.as_mut()
            }
            Self::And{left:_, right:_, data}=>{
                data.as_mut()
            }
            Self::Or{left:_, right:_, data}=>{
                data.as_mut()
            }
            _=>{
                None
            }
        }
    }
}
#[allow(unused)]
#[derive(Debug,Clone)]
pub struct Function {
    pub name: String,
    pub return_type: Type,
    pub args: Vec<Type>,
    pub arg_names: Vec<String>,
    pub program: Vec<AstNode>,
}
impl PartialEq for Function{
    fn eq(&self, other: &Self) -> bool {
        if self.name == other.name && self.args.len() == other.args.len(){
            for i in 0..self.args.len(){
                if !is_equal_type(&self.args[i], &other.args[i]){
                    return false;
                }
            }
            return true;
        }
        return false;
    }
}
#[derive(Debug)]
pub struct Scope {
    pub scope: Vec<HashMap<String, (Type, usize, bool)>>,
    pub count: usize,
}
impl Scope {
    pub fn new(statics: &HashMap<String, (Type, usize)>) -> Self {
        let mut base = HashMap::new();
        let mut count = 0;
        for r in statics {
            base.insert(r.0.clone(), (r.1 .0.clone(), count, false));
            count += 1;
        }
        Self {
            scope: vec![base],
            count,
        }
    }
    pub fn push_scope(&mut self) {
        self.scope.push(HashMap::new());
    }
    pub fn pop_scope(&mut self) {
        self.scope.pop();
    }
    pub fn declare_variable(&mut self, vtype: Type, name: String) {
        let cur = &mut self.scope[0];
        cur.insert(name, (vtype, self.count, false));
        self.count += 1;
    }
    pub fn declare_variable_arg(&mut self, vtype: Type, name: String) {
        let cur = &mut self.scope[0];
        cur.insert(name, (vtype, self.count, false));
        self.count += 1;
    }
    pub fn variable_idx(&mut self, name: String) -> Option<(Type, usize, bool)> {
        for i in &self.scope {
            if i.contains_key(&name) {
                return Some(i.get(&name)?.clone());
            }
        }
        return None;
    }
}
#[derive(Debug)]
pub struct FunctionTable{
    pub functions:Vec<Function>,
}
impl FunctionTable{
    pub fn new()->Self{
        return Self{functions:vec![]};
    }
    pub fn push(&mut self, func:Function){
        self.functions.push(func);
    }
}
#[allow(unused)]
#[derive(Debug)]
pub struct Program {
    pub types: HashMap<String, Type>,
    pub functions: HashMap<String, FunctionTable>,
    pub static_variables: HashMap<String, (Type, usize)>,
    pub global_initializers: Vec<(String, Option<AstNode>)>,
}
pub fn name_mangle_type(var:&Type)->String{
    match var{
        Type::BoolT=>{
            return String::from("bool");
        }
        Type::FloatT=>{
            return String::from("double");
        }
        Type::IntegerT=>{
            return String::from("long");
        }
        Type::StringT=>{
            return String::from("String");
        }
        Type::VoidT=>{
            return String::from("void");
        }
        Type::PointerT { ptr_type }=>{
            return name_mangle_type(ptr_type)+"*";
        }
        Type::ArrayT { size, array_type }=>{
            return name_mangle_type(array_type)+&format!("[{size}]");
        }
        Type::SliceT { ptr_type }=>{
            return name_mangle_type(ptr_type)+"Slice_t";
        }
        Type::StructT { name, components:_ }=>{
            return String::from("u_")+&name;
        }
    }
}

pub fn name_mangle_function(var:&Function, _filename:&str)->String{
    let mut args = String::new();
    let name = var.name.to_owned();
    for i in &var.args{
        args+= "_";
        args += &name_mangle_type(i);
    }
    match name.as_ref(){
        "+"=>{
            return String::from("operator_plus")+&args;
        }
        "-"=>{
            return String::from("operator_minus")+&args;
        }
        "*"=>{
            return String::from("operator_mult")+&args;
        }
        "/"=>{
            return String::from("operator_divide")+&args;
        }
        "=="=>{
            return String::from("operator_equals")+&args;
        }
        "<"=>{
            return String::from("operator_less_than")+&args;
        }
        ">"=>{
            return String::from("operator_greater_than")+&args;
        }
        "<="=>{
            return String::from("operator_less_than_or_eq")+&args;
        }
        ">="=>{
            return String::from("operator_greater_than_eq")+&args;
        }
        _=>{
            return String::from("user_")+&name+&args;
        }
    }
}
