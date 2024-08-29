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
    Plus{stack_idx:usize, left:IrOperand, right:IrOperand},
    Minus{stack_idx:usize, left:IrOperand, right:IrOperand},
    Mult{stack_idx:usize, left:IrOperand, right:IrOperand},
    Div{stack_idx:usize, left:IrOperand, right:IrOperand}, 
    And{stack_idx:usize, left:IrOperand, right:IrOperand}, 
    Or{stack_idx:usize, left:IrOperand, right:IrOperand},
    Not{stack_idx:usize, value:IrOperand},
    GreaterThan{stack_idx:usize, left:IrOperand, right:IrOperand},
    LessThan{stack_idx:usize, left:IrOperand, right:IrOperand},
    EqualTo{stack_idx:usize, left:IrOperand, right:IrOperand},
    Label{name:String},
    Goto{name:String},
    CondGoto{cond:IrOperand, true_lable:String, false_label:String},
    StackFramePush{},
    StackFramePop{to_clear:Vec<Type>},
    Ret{value:IrOperand},
}
