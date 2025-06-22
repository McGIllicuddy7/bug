pub enum MikuType {}
pub enum OpType {
    Add,
    Sub,
    Mul,
    Div,
    CmpG,
    CmpE,
    CmpL,
    Or,
    And,
}
pub enum MikuInstr<'a> {
    BeginFunction {
        name: &'a str,
        label_index: u32,
    },
    EndFunction {
        name: &'a str,
    },
    Label {
        name: &'a str,
        count: u32,
    },
    Jmp {
        to: &'a str,
        count: u32,
    },
    JmpNot {
        to: &'a str,
        count: u32,
        var_name: &'a str,
    },
    Call {
        to: &'a str,
        count: u32,
    },
    DeclVar {
        vtype: MikuType,
        var_name: &'a str,
        count: u32,
    },
    DeclArg {
        vtype: MikuType,
        var_name: &'a str,
    },
    Assign {
        left: &'a str,
        left_idx: u32,
        right: &'a str,
        right_idx: u32,
    },
    BinOp {
        left: &'a str,
        left_idx: u32,
        right: &'a str,
        right_idx: u32,
        op_type: OpType,
    },
    Return {
        to_return: &'a str,
        return_idx: u32,
    },
    PushArg {
        arg: &'a str,
        idx: u32,
    },
}
pub struct ParserState<'a> {
    instructions: Vec<MikuInstr<'a>>,
}
