pub use std::collections::{HashMap, HashSet};
pub use std::error::Error;
pub use std::sync::Arc;
#[derive(Copy, Clone, Debug)]
pub enum BinopType {
    None,
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    Greater,
    Less,
    And,
    Or,
    Xor,
}
#[derive(Copy, Clone, Debug)]
pub enum Register {
    I0,
    I1,
    I2,
    I3,
    I4,
    I5,
    I6,
    I7,
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    F0,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    Out0,
    Out1,
}
#[derive(Clone, Debug)]
pub enum LaigoOp {
    Deref {
        to_deref: Box<LaigoOp>,
    },
    Register {
        idx: Register,
    },
    TakeRef {
        to_take_ref: Box<LaigoOp>,
    },
    StackOperand {
        word_offset: u64,
    },
    Index {
        base: Box<LaigoOp>,
        amount: Box<LaigoOp>,
        scale: u8,
    },
    Ptr {
        v: u64,
    },
    Integer {
        v: i64,
    },
    Double {
        v: f64,
    },
    Symbol {
        v: Arc<str>,
    },
}
impl LaigoOp {
    pub fn is_num_or_reg(&self) -> bool {
        match self {
            Self::Register { idx: _ } => true,
            Self::Ptr { v: _ } => true,
            Self::Integer { v: _ } => true,
            _ => false,
        }
    }
    pub fn is_reg(&self) -> bool {
        match self {
            Self::Register { idx: _ } => true,
            _ => false,
        }
    }
    pub fn get_imm_arm(&self) -> String {
        match self {
            Self::Register { idx } => match idx {
                Register::I0 => {
                    format!("x0")
                }
                Register::I1 => {
                    format!("x1")
                }
                Register::I2 => {
                    format!("x2")
                }
                Register::I3 => {
                    format!("x3")
                }
                Register::I4 => {
                    format!("x4")
                }
                Register::I5 => {
                    format!("x5")
                }
                Register::I6 => {
                    format!("x6")
                }
                Register::I7 => {
                    format!("x7")
                }
                Register::R0 => {
                    format!("x0")
                }
                Register::R1 => {
                    format!("x1")
                }
                Register::R2 => {
                    format!("x2")
                }
                Register::R3 => {
                    format!("x3")
                }
                Register::R4 => {
                    format!("x4")
                }
                Register::R5 => {
                    format!("x5")
                }
                Register::R6 => {
                    format!("x6")
                }
                Register::R7 => {
                    format!("x7")
                }
                Register::Out0 => {
                    format!("x0")
                }
                Register::Out1 => {
                    format!("x1")
                }

                _ => todo!(),
            },
            Self::Integer { v } => format!("#{v}"),
            Self::Ptr { v } => format!("#{v}"),
            Self::Symbol { v } => format!("_{v}"),
            _ => todo!(),
        }
    }
    pub fn get_mem_op_arm(&self) -> String {
        match self {
            Self::StackOperand { word_offset } => {
                format!("[fp,#-{}]", word_offset * 8)
            }
            Self::Deref { to_deref } => match to_deref.as_ref() {
                LaigoOp::Register { idx: _ } => {
                    format!("[{},#0]", to_deref.get_imm_arm())
                }
                _ => todo!(),
            },
            Self::Symbol { v } => v.as_ref().into(),
            _ => todo!(),
        }
    }
    pub fn get_as_ref_arm(&self) -> (String, String) {
        match self {
            Self::StackOperand { word_offset } => (format!("fp"), format!("{}", word_offset * 8)),
            _ => todo!(),
        }
    }
    pub fn get_imm_x86(&self) -> String {
        match self {
            Self::Register { idx } => match idx {
                Register::I0 => {
                    format!("rdi")
                }
                Register::I1 => {
                    format!("rsi")
                }
                Register::I2 => {
                    format!("rdx")
                }
                Register::I3 => {
                    format!("rcx")
                }
                Register::I4 => {
                    format!("r8")
                }
                Register::I5 => {
                    format!("r9")
                }
                Register::I6 => {
                    format!("rax")
                }
                Register::I7 => {
                    format!("rbx")
                }
                Register::R0 => {
                    format!("rdi")
                }
                Register::R1 => {
                    format!("rsi")
                }
                Register::R2 => {
                    format!("rdx")
                }
                Register::R3 => {
                    format!("rcx")
                }
                Register::R4 => {
                    format!("r8")
                }
                Register::R5 => {
                    format!("r9")
                }
                Register::R6 => {
                    format!("rax")
                }
                Register::R7 => {
                    format!("rbx")
                }
                Register::Out0 => {
                    format!("rax")
                }
                Register::Out1 => {
                    format!("rbx")
                }
                _ => todo!(),
            },
            Self::Integer { v } => format!("{v}"),
            Self::Ptr { v } => format!("{v}"),
            Self::Symbol { v } => format!("{v}"),
            _ => todo!(),
        }
    }
    pub fn get_mem_op_x86(&self) -> String {
        match self {
            Self::StackOperand { word_offset } => {
                format!("[rbp-{}]", word_offset * 8)
            }
            _ => todo!(),
        }
    }
}
#[derive(Clone, Debug)]
pub enum LaigoIns {
    Declare {
        count: usize,
    },
    Binop {
        output: LaigoOp,
        left: LaigoOp,
        right: LaigoOp,
        binop_type: BinopType,
    },
    Not {
        left: LaigoOp,
        right: LaigoOp,
    },
    Assign {
        left: LaigoOp,
        right: LaigoOp,
    },
    Jmp {
        target: LaigoOp,
    },
    If {
        condition: LaigoOp,
        left: LaigoOp,
        right: LaigoOp,
    },
    Call {
        to_call: LaigoOp,
    },
    Syscall {
        call: u64,
    },
    Ret,
    Noop,
    FnBegin,
    FnEnd,
}
#[derive(Clone, Debug)]
pub struct LaigoInterpreter {
    pub registers: [u64; 8],
    pub fp_registers: [f64; 8],
    pub memory: Box<[u8]>,
    pub sp: u64,
    pub bp: u64,
    pub ip: u64,
}
#[derive(Clone, Debug)]
pub enum LaigoValue {
    String { v: String },
    Bytes { v: Vec<u8> },
    Integer { v: i64 },
    Float { f: f64 },
    Unsigned { u: u64 },
}
#[derive(Clone, Debug)]
pub struct LaigoUnit {
    pub label_indexs: HashMap<usize, String>,
    pub labels: HashMap<String, usize>,
    pub instructions: Vec<LaigoIns>,
    pub data_table: HashMap<String, LaigoValue>,
    pub globals: Vec<String>,
    pub externs: Vec<String>,
}
#[derive(Clone, Debug)]
pub struct TokenOwned {
    s: Arc<str>,
    line: usize,
}
fn tokenize(s: &str) -> Vec<Vec<TokenOwned>> {
    let mut out = Vec::new();
    let mut tmp: Vec<TokenOwned> = Vec::new();
    let mut st = String::new();
    let delims = ['+', '-', '*', '/', ',', '=', '&', '|', '^', '<', '>'];
    let mut lc = 1;
    let mut in_string = false;
    let mut last_was_slash = false;
    for i in s.chars() {
        if in_string {
            if i == '\\' {
                if last_was_slash {
                    st.push('\\');
                    last_was_slash = false;
                } else {
                    last_was_slash = true;
                }
            } else {
                if last_was_slash {
                    if i == '\n' {
                        continue;
                    } else if i == 'n' {
                        st.push_str("\\n");
                    }
                } else if i == '\n' {
                    lc += 1;
                    st.push(i);
                } else if i == '"' {
                    st.push(i);
                    tmp.push(TokenOwned {
                        s: st.clone().into(),
                        line: lc,
                    });
                    st.clear();
                    in_string = false;
                } else {
                    st.push(i);
                }
                last_was_slash = false;
            }
        } else if i == ' ' || i == '\t' {
            tmp.push(TokenOwned {
                s: st.clone().into(),
                line: lc,
            });
            st.clear();
        } else if i == '\n' {
            tmp.push(TokenOwned {
                s: st.clone().into(),
                line: lc,
            });
            st.clear();
            lc += 1;
            out.push(tmp.clone());
            tmp.clear();
        } else if i == '"' {
            tmp.push(TokenOwned {
                s: st.clone().into(),
                line: lc,
            });
            st.clear();
            st.push(i);
            in_string = true;
        } else if delims.contains(&i) {
            tmp.push(TokenOwned {
                s: st.clone().into(),
                line: lc,
            });
            st.clear();
            st.push(i);
            tmp.push(TokenOwned {
                s: st.clone().into(),
                line: lc,
            });
            st.clear();
        } else {
            st.push(i);
        }
    }
    if !st.is_empty() {
        tmp.push(TokenOwned {
            s: st.into(),
            line: lc,
        });
    }
    if !tmp.is_empty() {
        out.push(tmp);
    }
    let mut tout = Vec::new();
    for i in out {
        let mut tmp2 = Vec::new();
        for j in i {
            if !j.s.is_empty() {
                tmp2.push(j);
            }
        }
        if !tmp2.is_empty() {
            tout.push(tmp2);
        }
    }
    tout
}
pub fn parse_to_unit(s: &str) -> Result<LaigoUnit, Box<dyn Error>> {
    let tokens = tokenize(s);
    println!("{:#?}", tokens);
    let mut compiler = Compiler::new();
    for i in tokens {
        compiler.compile_line(&i)?;
    }
    Ok(LaigoUnit {
        labels: compiler.labels,
        label_indexs: compiler.label_idxes,
        instructions: compiler.ins,
        data_table: compiler.statics,
        globals: compiler.globals,
        externs: compiler.externs,
    })
}
pub struct Compiler {
    pub in_fn: bool,
    pub ins: Vec<LaigoIns>,
    pub statics: HashMap<String, LaigoValue>,
    pub externs: Vec<String>,
    pub labels: HashMap<String, usize>,
    pub label_idxes: HashMap<usize, String>,
    pub globals: Vec<String>,
}
impl Compiler {
    pub fn expect_op(line: &[TokenOwned], idx: &mut usize) -> Result<LaigoOp, Box<dyn Error>> {
        if *idx >= line.len() {
            return Err(
                format!("line {}, expected operand found end of line", line[0].line).into(),
            );
        }
        let s = line[*idx].s.as_ref();
        let mut out: LaigoOp;
        if s == "&" {
            *idx += 1;
            out = LaigoOp::TakeRef {
                to_take_ref: Box::new(Self::expect_op(line, idx)?),
            };
            return Ok(out);
        } else if s == "*" {
            *idx += 1;
            out = LaigoOp::Deref {
                to_deref: Box::new(Self::expect_op(line, idx)?),
            };
            return Ok(out);
        } else if let Some(id) = s.strip_prefix("x") {
            let i = id.parse::<u64>()?;
            out = LaigoOp::StackOperand { word_offset: i };
            *idx += 1;
        } else if let Some(id) = s.strip_prefix("i") {
            let i = id.parse::<u64>()?;
            if i == 0 {
                out = LaigoOp::Register { idx: Register::I0 };
            } else if i == 1 {
                out = LaigoOp::Register { idx: Register::I1 };
            } else if i == 2 {
                out = LaigoOp::Register { idx: Register::I2 };
            } else if i == 3 {
                out = LaigoOp::Register { idx: Register::I3 };
            } else if i == 4 {
                out = LaigoOp::Register { idx: Register::I4 };
            } else if i == 5 {
                out = LaigoOp::Register { idx: Register::I5 };
            } else if i == 6 {
                out = LaigoOp::Register { idx: Register::I6 };
            } else if i == 7 {
                out = LaigoOp::Register { idx: Register::I7 };
            } else {
                return Err(format!("line:{}, no such register{}", line[*idx].line, s).into());
            }
            *idx += 1;
        } else if let Some(id) = s.strip_prefix("r") {
            let i = id.parse::<u64>()?;
            if i == 0 {
                out = LaigoOp::Register { idx: Register::R0 };
            } else if i == 1 {
                out = LaigoOp::Register { idx: Register::R1 };
            } else if i == 2 {
                out = LaigoOp::Register { idx: Register::R2 };
            } else if i == 3 {
                out = LaigoOp::Register { idx: Register::R3 };
            } else if i == 4 {
                out = LaigoOp::Register { idx: Register::R4 };
            } else if i == 5 {
                out = LaigoOp::Register { idx: Register::R5 };
            } else if i == 6 {
                out = LaigoOp::Register { idx: Register::R6 };
            } else if i == 7 {
                out = LaigoOp::Register { idx: Register::R7 };
            } else {
                return Err(format!("line:{}, no such register{}", line[*idx].line, s).into());
            }
            *idx += 1;
        } else if let Some(id) = s.strip_prefix("f") {
            let i = id.parse::<u64>()?;
            if i == 0 {
                out = LaigoOp::Register { idx: Register::F0 };
            } else if i == 1 {
                out = LaigoOp::Register { idx: Register::F1 };
            } else if i == 2 {
                out = LaigoOp::Register { idx: Register::F2 };
            } else if i == 3 {
                out = LaigoOp::Register { idx: Register::F3 };
            } else if i == 4 {
                out = LaigoOp::Register { idx: Register::F4 };
            } else if i == 5 {
                out = LaigoOp::Register { idx: Register::F5 };
            } else if i == 6 {
                out = LaigoOp::Register { idx: Register::F6 };
            } else if i == 7 {
                out = LaigoOp::Register { idx: Register::F7 };
            } else {
                return Err(format!("line:{}, no such register{}", line[*idx].line, s).into());
            }
            *idx += 1;
        } else if s == "out0" {
            out = LaigoOp::Register {
                idx: Register::Out0,
            };
            *idx += 1;
        } else if s == "out1" {
            out = LaigoOp::Register {
                idx: Register::Out1,
            };
            *idx += 1;
        } else {
            if let Ok(p) = s.parse() {
                out = LaigoOp::Ptr { v: p };
            } else if let Ok(p) = s.parse() {
                out = LaigoOp::Integer { v: p };
            } else if let Ok(p) = s.parse() {
                out = LaigoOp::Double { v: p };
            } else {
                out = LaigoOp::Symbol { v: s.into() };
            }

            *idx += 1;
        }
        if *idx < line.len() && line[*idx].s.as_ref() == "[" {
            *idx += 1;
            let tmp = Self::expect_op(line, idx)?;
            let t2 = out.clone();
            out = LaigoOp::Index {
                base: Box::new(t2.clone()),
                amount: Box::new(tmp.clone()),
                scale: 1,
            };
            if *idx < line.len() {
                if line[*idx].s.as_ref() == "*" {
                    *idx += 1;
                    if *idx >= line.len() {
                        return Err(format!(
                            "line:{}, expected ] instead found end of line",
                            line[0].line,
                        )
                        .into());
                    }
                    let n = line[*idx].s.parse::<u8>()?;
                    out = LaigoOp::Index {
                        base: Box::new(t2),
                        amount: Box::new(tmp),
                        scale: n,
                    };
                    *idx += 1;
                }
                if line[*idx].s.as_ref() != "]" {
                    return Err(format!(
                        "line:{}, expected ] instead found{}",
                        line[*idx].line, line[*idx].line
                    )
                    .into());
                } else {
                    *idx += 1;
                }
            } else {
                return Err(format!(
                    "line:{}, expected ] instead found end of line",
                    line[0].line,
                )
                .into());
            }
        }
        Ok(out)
    }
    pub fn new() -> Self {
        Self {
            in_fn: false,
            ins: Vec::new(),
            statics: HashMap::new(),
            labels: HashMap::new(),
            label_idxes: HashMap::new(),
            globals: Vec::new(),
            externs: Vec::new(),
        }
    }
    pub fn compile_line(&mut self, mut line: &[TokenOwned]) -> Result<(), Box<dyn Error>> {
        if let Some(l) = line[0].s.strip_suffix(":") {
            self.labels.insert(l.into(), self.ins.len());
            self.label_idxes.insert(self.ins.len(), l.into());
            line = &line[1..];
            if line.is_empty() {
                return Ok(());
            }
        }
        let is_let = line[0].s.as_ref() == "let";
        if is_let {
            line = &line[1..];
            self.ins.push(LaigoIns::Declare { count: 1 });
        } else if line[0].s.as_ref() == "call" {
            let op = Self::expect_op(line, &mut 1)?;
            self.ins.push(LaigoIns::Call { to_call: op });
            return Ok(());
        } else if line[0].s.as_ref() == "ret" {
            self.ins.push(LaigoIns::Ret);
            return Ok(());
        } else if line[0].s.as_ref() == "int" {
            if line.len() < 2 {
                return Err(format!("line:{} expected operand", line[0].line).into());
            }
            let op = line[1].s.parse()?;
            self.ins.push(LaigoIns::Syscall { call: op });
            return Ok(());
        } else if line[0].s.as_ref() == "noop" {
            self.ins.push(LaigoIns::Noop);
            return Ok(());
        } else if line[0].s.as_ref() == "begin" {
            if line.len() < 2 {
                return Err(format!("line:{} expected func", line[0].line).into());
            }
            if line[1].s.as_ref() == "func" {
                if self.in_fn {
                    return Err(format!(
                        "line:{} cannot declare function in function",
                        line[0].line
                    )
                    .into());
                }
                self.in_fn = true;
                self.ins.push(LaigoIns::FnBegin);
                return Ok(());
            }
        } else if line[0].s.as_ref() == "end" {
            if line.len() < 2 {
                return Err(format!("line:{} expected operand", line[0].line).into());
            }
            if !self.in_fn {
                return Err(
                    format!("line:{} cannot end function in function", line[0].line).into(),
                );
            }
            if line[1].s.as_ref() == "func" {
                self.ins.push(LaigoIns::FnEnd);
                return Ok(());
            } else {
                return Err(format!("line:{} expected func", line[0].line).into());
            }
        } else if line[0].s.as_ref() == "global" {
            self.globals.push(line[1].s.to_string());
            return Ok(());
        } else if line[0].s.as_ref() == "extern" {
            self.externs.push(line[1].s.to_string());
            return Ok(());
        } else if line[0].s.as_ref() == "if" {
            let mut idx = 1;
            let cond = Self::expect_op(line, &mut idx)?;
            let l1 = Self::expect_op(line, &mut idx)?;
            let l2 = Self::expect_op(line, &mut idx)?;
            self.ins.push(LaigoIns::If {
                condition: cond,
                left: l1,
                right: l2,
            });
            return Ok(());
        } else if line[0].s.as_ref() == "jmp" {
            let mut idx = 1;
            let to = Self::expect_op(line, &mut idx)?;
            self.ins.push(LaigoIns::Jmp { target: to });
            return Ok(());
        }
        let mut idx = 0;
        let op1 = Self::expect_op(line, &mut idx)?;
        if idx >= line.len() {
            return Ok(());
        }
        if line[idx].s.as_ref() == "=" {
            idx += 1;
            if line[idx].s.as_ref() == "!" {
                let op2 = Self::expect_op(line, &mut idx)?;
                idx += 1;
                self.ins.push(LaigoIns::Not {
                    left: op1,
                    right: op2,
                });
                if idx < line.len() {
                    return Err(
                        format!("line {}, excess tokens after instruction", line[0].line).into(),
                    );
                }

                return Ok(());
            }
            let op2 = Self::expect_op(line, &mut idx)?;
            if idx >= line.len() {
                self.ins.push(LaigoIns::Assign {
                    left: op1,
                    right: op2,
                });
            } else {
                let mut operator = BinopType::None;
                let s = line[idx].s.as_ref();
                if s == "+" {
                    operator = BinopType::Add;
                } else if s == "-" {
                    operator = BinopType::Sub;
                } else if s == "*" {
                    operator = BinopType::Mul;
                } else if s == "/" {
                    operator = BinopType::Div;
                } else if s == "=" {
                    operator = BinopType::Equal;
                } else if s == "<" {
                    operator = BinopType::Less;
                } else if s == ">" {
                    operator = BinopType::Greater;
                } else if s == "|" {
                    operator = BinopType::Or;
                } else if s == "&" {
                    operator = BinopType::And;
                } else if s == "^" {
                    operator = BinopType::Xor;
                }
                idx += 1;
                let op3 = Self::expect_op(line, &mut idx)?;
                self.ins.push(LaigoIns::Binop {
                    output: op1,
                    left: op2,
                    right: op3,
                    binop_type: operator,
                });
                if idx < line.len() {
                    return Err(
                        format!("line {}, excess tokens after instruction", line[0].line).into(),
                    );
                }
            }
        } else {
            return Err("expected assignment found not that".into());
        }
        Ok(())
    }
}
