pub use core::str;
pub use std::collections::HashMap;
use std::rc::Rc;
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

#[derive(Clone, Debug,Eq, Hash,PartialEq)]
pub enum Type {
    BoolT,
    CharT,
    IntegerT,
    FloatT,
    StringT,
    StructT {
        name: Rc<str>,
        components: Vec<(String, Type)>,
    },
    PointerT {
        ptr_type:Rc<Type>,
    },
    ArrayT {
        size: usize,
        array_type: Rc<Type>,
    },
    VoidT,
    SliceT {
        ptr_type: Rc<Type>,
    },
    PartiallyDefined{
        name:Rc<str>,
    }
}
impl Type{
    pub fn get_array_type(&self)->Option<Type>{
        match &self{
            &Self::ArrayT { size:_, array_type }=>{
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
    #[allow(unused)]
    pub fn is_array(&self)->bool{
        match &self{
            &Self::ArrayT { size:_, array_type:_ }=>{
                return true;
            }
            &Self::SliceT { ptr_type:_ }=>{
                return true;
            }
            _=>{
                return false;
            }
        }
    }
   pub fn is_basic_number(&self)->bool {
    match &self{
        &Self::IntegerT { }=>{
            return true;
        }
        &Self::FloatT {}=>{
            return true;
        }
        _=>{
            return false;
        }
    }
   }
   pub fn get_size_bytes(&self)->usize{
        match self{
            Self::BoolT=>{
                1
            }
            Self::CharT=>{
                1
            }
            Self::FloatT=>{
                8
            }
            Self::IntegerT=>{
                8
            }
            Self::PointerT { ptr_type:_ }=>{
                8
            }
            Self::SliceT { ptr_type:_ }=>{
                16
            }
            Self::StringT{}=>{
                16
            }
            Self::ArrayT { size, array_type }=>{
                size*array_type.get_size_bytes()
            }
            Self::VoidT=>{
                0
            }
            Self::StructT { name:_, components }=>{
                let mut size = 0;
                for i in components{size += i.1.get_size_bytes();}
                size
            }
            Self::PartiallyDefined { name:_ }=>{
                0
            }
        }
   }
   pub fn is_partially_defined(&self)->bool{
        match self{
            Type::PartiallyDefined { name:_ }=>{
                true
            }
            Type::PointerT { ptr_type }=>{
                ptr_type.is_partially_defined()
            }
            Type::ArrayT { size:_, array_type }=>{
                array_type.is_partially_defined()
            }
            Type::SliceT { ptr_type }=>{
                ptr_type.is_partially_defined()
            }
            _=>{
                false
            }
        }
   }
   pub fn get_variable_offset(&self, name:&str)->Option<usize>{
    let mut count = 0;
    match self{
        Self::SliceT { ptr_type:_ }=>{
            return Some(8);
        }
        Self::StringT=>{
            return Some(8);
        }
        Self::StructT { name: _, components }=>{
            for i in components{
                if &i.0 == name{
                    return Some(self.get_size_bytes()-count-i.1.get_size_bytes());
                } else{
                    count += i.1.get_size_bytes();
                }
            }
        }
        _=>{
            return None;
        }
    }
    return None;
}
pub fn get_variable_type(&self, name:&str)->Option<Type>{
    match self{
        Self::SliceT { ptr_type:_ }=>{
            return Some(Type::IntegerT);
        }
        Self::StringT=>{
            return Some(Type::IntegerT);
        }
        Self::StructT { name: _, components }=>{
            for i in components{
                if &i.0 == name{
                    return Some(i.1.clone())
                }
            }
        }
        _=>{
            return None;
        }
    }
    return None; 
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
        Type::CharT=>match b{
            Type::CharT=>{
                return true;
            }
            Type::IntegerT=>{
                return true;
            }
            _=>{
                return false;
            }
        }
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
                    if name.as_ref() == "" || aname.as_ref() == "" {
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
                Type::PartiallyDefined { name }=>{
                    return aname == name;
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
                Type::BoolT{}=>{
                    return true;
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
                Type::SliceT { ptr_type} => {
                    return is_compatible_type(&at, &ptr_type);
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
                Type::ArrayT { array_type, size :_} => {
                    return is_compatible_type(&at, &array_type); 
                }   
                Type::SliceT { ptr_type } => {
                    return is_compatible_type(&at, ptr_type);
                }
                _ => {
                    return false;
                }
            }
        }
        Type::PartiallyDefined { name }=>{
            let aname = name;
            match b{
                Type::StructT { name, components:_ }=>{
                    return aname == name;
                }
                Type::PartiallyDefined { name }=>{
                    return aname == name;
                }
                _=>{
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
        Type::CharT=>match b{
            Type::CharT=>{
                return true;
            }
            _=>{
                return false;
            }
        }
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
                    if name.as_ref() == "" || aname.as_ref() == "" {
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
                Type::PartiallyDefined { name }=>{
                    return aname == name;
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
        Type::PartiallyDefined { name }=>{
            let aname = name;
            match b {
                Type::StructT { name, components:_ }=>{
                    return name == aname;
                }
                Type::PartiallyDefined { name }=>{
                    return aname == name;
                }
                _=>{
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
        vtype:Type,
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
        data:Option<AstNodeData>,
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
    NotEquals {
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
        data:Option<AstNodeData>,
    },
    Paren{
        internals:Box<AstNode>,
    },
    OperatorNew{
        vtype:Type,
    },
    OperatorMake{
        vtype:Type, 
        size:Box<AstNode>,
    },
}

impl AstNode {
    pub fn get_type(
        &self,
        function_table: &HashMap<String, FunctionTable>,
        types: &HashMap<String, Type>,
    ) -> Option<Type> {
        match self {
            Self::VoidLiteral {} => Some(Type::VoidT),
            Self::BoolLiteral { value: _ } => Some(Type::BoolT),
            Self::StringLiteral { value: _ } => Some(Type::StringT),
            Self::IntLiteral { value: _ } => Some(Type::IntegerT),
            Self::FloatLiteral { value: _ } => Some(Type::FloatT),
            Self::StructLiteral { vtype,nodes:_ } => {
                return Some(vtype.clone());
            }
            Self::ArrayLiteral { nodes } => {
                let mut out_types = vec![];
                for i in nodes {
                    out_types.push((("").to_owned(), i.get_type(function_table, types)?));
                }
                return Some(Type::ArrayT { size:out_types.len(), array_type: Rc::new(out_types[0].1.clone()) } );
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
                args,
                data:_,
            } => {
                let fn_args:Vec<Type> = args.iter().map(|i| i.get_type(function_table, types).expect("should have type")).collect();
                return Some(get_function_by_args(function_name, &fn_args,function_table)?.return_type.clone());
            }
            Self::Assignment { left, right: _ ,data:_} => {
                return left.get_type(function_table, types);
            }
            Self::VariableDeclaration {
                name: _,
                var_type,
                value_assigned: _,
                data:_
            } => {
                return Some(var_type.clone());
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
            Self::NotEquals { left:_, right:_, data:_ }=>{
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
                        ptr_type: Rc::new(vtype.clone()),
                    });
                }
                Self::ArrayAccess { variable, index:_ }=> {
                    return Some(Type::PointerT{ptr_type: Rc::new(variable.get_type(function_table, types).expect("should have type").get_array_type()?)});
                }
                Self::FieldUsage { base, field_name }=>{
                    let vtype = base.get_type(function_table, types)?;
                    match vtype{
                        Type::ArrayT { size:_, array_type }=>{
                            return Some(array_type.as_ref().to_owned());
                        }
                        Type::SliceT { ptr_type }=>{
                            return Some(ptr_type.as_ref().to_owned());
                        }
                        Type::StructT { name:_, components }=>{
                            for i in &components{
                                if *i.0.clone() == *field_name{
                                    match i.1.clone(){
                                        Type::PartiallyDefined { name }=>{
                                            return Some(types[name.as_ref()].clone());
                                        }_=>{
                                            return Some( i.1.clone());
                                        }
                                    }

                                }
                            }
                            return None;
                        }
                        _=>{
                            return None;
                        }
                    }
                }
                _=>{
                    return None;
                }
            },
            Self::FieldUsage {
                base,
                field_name,
            } => {
                let base0 = base.get_type(function_table, types)?;
                match base0{
                    Type::StructT { name:_, components }=>{
                        for i in &components{
                            if i.0 == *field_name{
                                match &i.1{
                                    Type::PartiallyDefined { name }=>{
                                        return Some(types[name.as_ref()].clone());
                                    }_=>{
                                        return Some(i.1.clone());
                                    } 
                                }

                            }
                        }
                        return None;
                    }
                    Type::SliceT { ptr_type:_ }=>{
                        return Some(Type::IntegerT);
                    }
                    Type::ArrayT { size:_, array_type:_ }=>{
                        return Some(Type::IntegerT);
                    }
                    _=>{
                        return None;
                    }
                }
            }
            Self::ArrayAccess { variable, index:_ }=>{
                return  Some(variable.get_type(function_table, types)?.get_array_type()?);
            }
            Self::BoundFunctionCall { variable:_, function_name, args, data:_}=>{
                let fn_args:Vec<Type> = args.iter().map(|i| i.get_type(function_table, types).expect("should have type")).collect();
                return Some(get_function_by_args(function_name, &fn_args,function_table)?.return_type.clone());
            }
            Self::Paren { internals }=>{
                return internals.get_type(function_table, types)
            }
            Self::OperatorNew { vtype }=>{
                match vtype{
                    Type::ArrayT { size:_, array_type }=>{
                        return Some(Type::SliceT { ptr_type: array_type.clone() });
                    }
                    _=>{
                        return Some(Type::PointerT { ptr_type: Rc::new(vtype.clone()) });
                    }
                }
            }
            Self::OperatorMake { vtype , size:_}=>{
                return Some(Type::SliceT { ptr_type: Rc::new(vtype.clone()) });
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
            Self::StructLiteral {vtype:_ ,nodes: _ } => {
                return 0;
            }
            Self::ArrayLiteral { nodes: _ } => {
                return 0;
            }
            Self::Paren { internals:_ }=>{
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
                data:_,
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
            Self::NotEquals { left:_, right:_, data:_ }=>{
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
                return 2;
            }
            Self::TakeRef { thing_to_ref: _ } => {
                return 2;
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
            Self::BoundFunctionCall { variable:_, function_name:_, args:_ , data:_}=>{
                return 1;
            }
            Self::OperatorNew { vtype:_ }=>{
                return 0;
            }
            Self::OperatorMake { vtype:_, size:_ }=>{
                return 0;
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
    #[allow(unused)]
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
    pub forward_declared:bool,
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
    #[allow(unused)]
    pub fn push_scope(&mut self) {
        self.scope.push(HashMap::new());
    }
    #[allow(unused)]
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
#[derive(Debug,Clone)]
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
        Type::CharT=>{
            return String::from("char");
        }
        Type::PointerT { ptr_type }=>{
            return name_mangle_type(ptr_type)+"*";
        }
        Type::ArrayT { size:_, array_type }=>{
            return name_mangle_type(array_type)+"Slice_t";
        }
        Type::SliceT { ptr_type }=>{
            return name_mangle_type(ptr_type)+"Slice_t";
        }
        Type::StructT { name, components:_ }=>{
            return String::from("u_")+&name;
        }
        Type::PartiallyDefined { name }=>{
            return String::from("u_")+&name;
        }
    }
}
pub fn name_mangle_type_for_names(var:&Type)->String{
    match var{
        Type::BoolT=>{
            return String::from("bool");
        }
        Type::CharT=>{
            return String::from("char");
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
            return name_mangle_type(ptr_type)+"_ptr";
        }
        Type::ArrayT { size:_, array_type }=>{
            return name_mangle_type(array_type)+"Slice_t";
        }
        Type::SliceT { ptr_type }=>{
            return name_mangle_type(ptr_type)+"Slice_t";
        }
        Type::StructT { name, components:_ }=>{
            return String::from("u_")+&name;
        }
        Type::PartiallyDefined { name }=>{
            return String::from("u_")+&name;
        }
    }
}
pub fn name_mangle_type_for_struct(var:&Type)->String{
    let out = 
    match var{
        Type::BoolT=>{
            String::from("bool")
        }
        Type::FloatT=>{
            String::from("double")
        }
        Type::IntegerT=>{
            String::from("long")
        }
        Type::StringT=>{
                String::from("String")
        }
        Type::VoidT=>{
            String::from("void")
        }
        Type::CharT=>{
             String::from("char")
        }
        Type::PointerT { ptr_type }=>{
            name_mangle_type(ptr_type)+"*"
        }
        Type::ArrayT { size:_, array_type }=>{
            name_mangle_type(array_type)+"Slice_t"
        }
        Type::SliceT { ptr_type }=>{
            name_mangle_type(ptr_type)+"Slice_t"
        }
        Type::StructT { name, components:_ }=>{
             String::from("u_")+&name
        }
        Type::PartiallyDefined { name }=>{
            String::from("u_")+&name
        }
    };
    return if var.is_partially_defined(){
        "struct ".to_owned()+&out
    } else{
        out
    }
}
pub fn name_mangle_function(var:&Function, _filename:&str)->String{
    let mut args = String::new();
    let name = var.name.to_owned();
    if name == "main"{
        return String::from("user_main");
    }
    for i in &var.args{
        args+= "_";
        args += &name_mangle_type_for_names(i);
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

pub fn get_function_by_args(name:&str, args:&[Type], functions:&HashMap<String, FunctionTable>)->Option<Function>{
    let table = functions.get(name)?;
    for i in &table.functions{
        if args.len() != i.args.len(){
            continue;
        }
        let mut matched = true;
        for j in 0..args.len(){
            if !is_equal_type(&args[j], &i.args[j]){
                matched = false;
                break; 
            }
        }
        if !matched{
            continue;
        }
        let mut out = i.clone();
        out.name = name_mangle_function(&out, "");
        return Some(out);
    }
    return None;
}
#[allow(unused)]
pub enum Target{
    MacOs{arm:bool}, Linux{arm:bool}, Windows{arm:bool}
}