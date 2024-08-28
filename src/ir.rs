use crate::Type;

/*
    variable_declaration

    operands: number, thing at address or hardcoded value
*/
#[allow(unused)]
pub enum IrOperand{
    GlobalName{name:String}, 
    StackVariable{idx:usize},
    Deref{to_deref:Box<IrOperand>},
    IntConstant{value:i64},
    FloatConstant{value:f64},
}
#[allow(unused)]
pub enum IrNode{
    FunctionCall{args:Vec<IrOperand>},
    ValueEqualsFunctionCall{left:IrOperand, right:IrOperand},
    Assignment{left:IrOperand, right:IrOperand},
    VarDeclaration{stack_idx:usize, vtype:Type},
}