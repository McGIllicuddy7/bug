use crate::{types, Type};

#[allow(unused)]
pub enum Registers {
    Rip,
    Rsp,
    Rax,
    Rdi,
    Rsi,
    Rdx,
    Rcx,
    R8,
    R9,
    R10,
    R11,
    Rbx,
    Rbp,
    R12,
    R13,
    R14,
    R15,
}
//RDI, RSI, RDX, RCX, R8, R9
//XMM0 - XMM7
//stack
const INT_ARG_NAMES: &[&'static str] = &["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
const FLOAT_ARG_NAMES: &[&'static str] = &[
    "xmm0", "xmm1", "xmm2", "xmm3", "xmm4", "xmm5", "xmm6", "xmm7",
];
#[derive(Debug, Clone)]
pub struct ArgCPU {
    pub int_registers: [u8; 6],
    pub float_registers: [u8; 8],
}
impl ArgCPU {
    pub fn new() -> Self {
        return Self {
            int_registers: [0, 0, 0, 0, 0, 0],
            float_registers: [0, 0, 0, 0, 0, 0, 0, 0],
        };
    }
    pub fn get_next_location(&mut self) -> Option<String> {
        for i in 0..6 {
            if self.int_registers[i] == 0 {
                self.int_registers[i] = 8;
                return Some(String::from(INT_ARG_NAMES[i]));
            }
        }
        None
    }
    pub fn handle_capacity_for(&mut self, arg_v: &str, arg_t:&Type){
        let saved_state = self.clone();
        let mut discarded_pop_stack =0;
        let _discard = self.generate_arg_internal(arg_v, arg_t, &mut discarded_pop_stack,true);
        if discarded_pop_stack==0{
            *self = saved_state;
        }
    }
    pub fn get_next_fp_location(&mut self) -> Option<String> {
        for i in 0..6 {
            if self.float_registers[i] == 0 {
                self.float_registers[i] = 8;
                return Some(String::from(FLOAT_ARG_NAMES[i]));
            }
        }
        None
    }
    fn generate_basic_arg(
        &mut self,
        op_name: &str,
        size: usize,
        offset: usize,
        to_pop_stack: &mut usize,
        is_address_of:bool
    ) -> String {
        static SIZES: &[&'static str] = &["BYTE", "WORD", "", "DWORD", "", "", "", "QWORD"]; 
        if let Some(rname) = self.get_next_location() {
            if !is_address_of{
                return format!("    mov {}, {}\n", rname, op_name);
            }
            else{
                if offset != 0{
                    return format!("    mov {}, {} [{}-{offset}]\n", rname, SIZES[size-1], op_name)
                } else{
                    return format!("    mov {}, {} [{}]\n", rname, SIZES[size-1], op_name);
                }

            }
        }
        *to_pop_stack += 1;
            if !is_address_of{
                return format!("    push {} {}\n", SIZES[size-1],op_name);
            }
            else{
                if offset != 0{
                    return format!("    push {} [{}-{offset}]\n", SIZES[size-1], op_name);
                } else{
                    return format!("    push {} [{}]\n",SIZES[size-1], op_name);
                }

            }
    }
fn generate_float_arg(
        &mut self,
        op_name: &str,
        size: usize,
        offset: usize,
        to_pop_stack: &mut usize,
        is_address_of:bool
    ) -> String {
        static SIZES: &[&'static str] = &["BYTE", "WORD", "", "DWORD", "", "", "", "QWORD"]; 
        if let Some(rname) = self.get_next_fp_location() {
            if !is_address_of{
                return format!("    mov {}, {}\n", rname, op_name);
            }
            else{
                if offset != 0{
                    return format!("    mov {}, {} [{}-{offset}]\n", rname, SIZES[size-1], op_name)
                } else{
                    return format!("    mov {}, {} [{}]\n", rname, SIZES[size-1], op_name);
                }

            }
        }
        *to_pop_stack += 1;
            if !is_address_of{
                return format!("    push {} {}\n", SIZES[size-1],op_name);
            }
            else{
                if offset != 0{
                    return format!("    push {} [{}-{offset}]\n", SIZES[size-1], op_name);
                } else{
                    return format!("    push {} [{}]\n",SIZES[size-1], op_name);
                }

            }
    }
    fn generate_arg_internal(&mut self, arg_v:&str, arg_t: &Type,to_pop_stack: &mut usize,called_from_cap:bool)->String{
        let mut out = String::new();
        let is_addr = arg_v.contains('r');
        if !called_from_cap{
            self.handle_capacity_for(arg_v, arg_t); 
        }
        match arg_t {
            types::Type::ArrayT {
                size: _,
                array_type: _,
            } => {
                unreachable!();
            }
            types::Type::BoolT => {
                return self.generate_basic_arg(arg_v, 8, 0, to_pop_stack, is_addr);
            }
            types::Type::CharT => {
                return self.generate_basic_arg(arg_v, 8, 0, to_pop_stack,is_addr);
            }
            types::Type::FloatT => {
                return self.generate_float_arg(arg_v, 8, 0,to_pop_stack, arg_v.contains("r"));
            }
            types::Type::IntegerT => { 
                return self.generate_basic_arg(arg_v, 8, 0, to_pop_stack,arg_v.contains("r"));
            }
            types::Type::PointerT { ptr_type: _ } => {
                return self.generate_basic_arg(arg_v, 8, 0, to_pop_stack, false);
            }
            types::Type::SliceT { ptr_type: _ } => {
                out += &self.generate_basic_arg(arg_v, 8, 0, to_pop_stack,true);
                out += &self.generate_basic_arg(arg_v, 8, 8, to_pop_stack, true);
                return out;
            }
            types::Type::StringT => {
                out += &self.generate_basic_arg(arg_v, 8, 0, to_pop_stack, true);
                out += &self.generate_basic_arg(arg_v, 8, 8, to_pop_stack, true);
                return out;
            }
            types::Type::StructT {
                name: _,
                components,
            } => {
                let mut offset = 0;
                for i in components {
                    let op = format!("{arg_v}-{offset}");
                    out += &self.generate_arg(&op, &i.1, to_pop_stack);
                    offset += i.1.get_size_bytes();
                }
                return out;
            }
            _ => {
                unreachable!();
            }
        }
        return out;
    }
    pub fn generate_arg(&mut self, arg_v: &str, arg_t: &Type, to_pop_stack: &mut usize) -> String {
        return self.generate_arg_internal(arg_v, arg_t, to_pop_stack, false);
    }
}
