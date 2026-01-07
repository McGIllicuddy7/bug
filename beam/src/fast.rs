use crate::heap::{Allocation, RtHeap};
use core::slice;
use std::{
    cell::UnsafeCell,
    collections::HashMap,
    fmt::Debug,
    mem::MaybeUninit,
    ops::{Index, IndexMut, Range},
    ptr::NonNull,
    string,
};
#[repr(C)]
pub struct AllocInfo {
    pub field_count: u32,
    pub type_info: u32,
}
#[repr(u8)]
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum Instr {
    Jmp,
    JmpCond,
    Call,
    CallObj,
    Ret,
    DefLocalVoid,
    DefLocalInt,
    DefLocalFloat,
    DefLocalBool,
    DefLocalPtr,
    DefLocalStr,
    LoadVoid,
    LoadInt,
    LoadFloat,
    LoadBool,
    LoadPtr,
    LoadStr,
    LoadMember,
    LoadVarAddr,
    LoadMemberAddr,
    StoreVoid,
    StoreInt,
    StoreFloat,
    StoreBool,
    StorePtr,
    StoreStr,
    ConstVoid,
    ConstInt,
    ConstFloat,
    ConstBool,
    ConstPtr,
    ConstStr,
    IntAdd,
    IntSub,
    IntMul,
    IntDiv,
    IntEq,
    IntNEq,
    IntLess,
    IntGreater,
    FloatAdd,
    FloatSub,
    FloatMul,
    FloatDiv,
    FloatEq,
    FloatNeq,
    FloatLess,
    FloatGreater,
    BoolEq,
    BoolNeq,
    BoolAnd,
    BoolOr,
    StrAdd,
    StrEq,
    StrNeq,
    New,
}
#[repr(u64)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Tag {
    Void,
    Integer,
    Float,
    Bool,
    String,
    Ptr,
    LValue,
}
#[repr(C)]
#[derive(Clone)]
pub struct BStr {
    start: *const u8,
    len: usize,
}
impl BStr {
    pub fn as_str(&self) -> &str {
        unsafe {
            let slice = std::slice::from_raw_parts(self.start, self.len);
            return str::from_utf8(slice).unwrap();
        }
    }
}

#[derive(Clone, Copy)]
pub union Value {
    pub void: (),
    pub integer: i64,
    pub float: f64,
    pub boolean: bool,
    pub string: *const BStr,
    pub ptr: *const Var,
    pub lvalue: *const Var,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct InternalVar {
    pub tag: Tag,
    pub value: Value,
}

pub struct Var(UnsafeCell<InternalVar>);
impl Clone for Var {
    fn clone(&self) -> Self {
        Self(UnsafeCell::new(*self.get()))
    }
}

use Instr::*;
use Tag::*;

use crate::mach::{Binop, Program, Type};

impl Var {
    pub const fn new() -> Self {
        Var(UnsafeCell::new(InternalVar {
            tag: Integer,
            value: Value { integer: 0 },
        }))
    }
    pub fn integer(v: i64) -> Self {
        Var(UnsafeCell::new(InternalVar {
            tag: Integer,
            value: Value { integer: v },
        }))
    }
    pub fn float(v: f64) -> Self {
        Var(UnsafeCell::new(InternalVar {
            tag: Float,
            value: Value { float: v },
        }))
    }
    pub fn boolean(v: bool) -> Self {
        Var(UnsafeCell::new(InternalVar {
            tag: Bool,
            value: Value { boolean: v },
        }))
    }
    pub fn string(v: *const BStr) -> Self {
        Var(UnsafeCell::new(InternalVar {
            tag: String,
            value: Value { string: v },
        }))
    }
    pub fn ptr(v: *mut Var) -> Self {
        Var(UnsafeCell::new(InternalVar {
            tag: Ptr,
            value: Value { ptr: v },
        }))
    }
    pub fn void(v: ()) -> Self {
        Var(UnsafeCell::new(InternalVar {
            tag: Void,
            value: Value { void: v },
        }))
    }
    pub fn l_value(v: *mut Var) -> Self {
        Var(UnsafeCell::new(InternalVar {
            tag: LValue,
            value: Value { lvalue: v },
        }))
    }
    pub fn get(&self) -> &InternalVar {
        unsafe { self.0.get().as_ref().expect("will work valid ptr") }
    }
    pub fn get_mut(&mut self) -> &mut InternalVar {
        self.0.get_mut()
    }
    pub fn get_void(&self) -> Option<()> {
        if self.get().tag == Void {
            Some(())
        } else {
            None
        }
    }
    pub fn get_int(&self) -> i64 {
        unsafe {
            if self.get().tag == Integer {
                self.get().value.integer
            } else {
                todo!()
            }
        }
    }
    pub fn get_float(&self) -> f64 {
        unsafe {
            if self.get().tag == Float {
                self.get().value.float
            } else {
                todo!()
            }
        }
    }
    pub fn get_bool(&self) -> bool {
        unsafe {
            if self.get().tag == Bool {
                self.get().value.boolean
            } else {
                todo!()
            }
        }
    }
    pub fn get_string(&self) -> *const BStr {
        unsafe {
            if self.get().tag == String {
                self.get().value.string
            } else {
                todo!()
            }
        }
    }
    pub fn get_ptr(&self) -> *const Var {
        unsafe {
            if self.get().tag == Ptr {
                self.get().value.ptr
            } else {
                println!("error expected pointer instead found:{:#?}", self);
                todo!()
            }
        }
    }
    pub fn get_l_value(&self) -> &Var {
        unsafe {
            if self.get().tag == LValue {
                self.get().value.lvalue.as_ref().unwrap_unchecked()
            } else {
                todo!()
            }
        }
    }
}
impl Debug for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.get();
        match s.tag {
            Void => {
                write!(f, "void")
            }
            Integer => {
                write!(f, "int:{:#?}", self.get_int())
            }
            Float => {
                write!(f, "float:{:#?}", self.get_float())
            }
            Bool => {
                write!(f, "bool:{:#?}", self.get_bool())
            }
            String => unsafe {
                let tmp = &*self.get_string();
                write!(f, "string:{:#?}", tmp.as_str())
            },
            Ptr => unsafe {
                let ptr = self.get_ptr();
                if ptr == std::ptr::null() {
                    write!(f, "ptr:null")
                } else {
                    let header = ptr as *const crate::heap::Allocation;
                    let len = (*header).num_objects;
                    for i in 0..len as usize {
                        write!(f, "{:#?}", &*(ptr.add(i)))?;
                    }
                    Ok(())
                }
            },
            LValue => {
                write!(f, "{:#?}", self.get_l_value())
            }
        }
    }
}
#[repr(C)]
pub struct OwnedSlice<T> {
    ptr: NonNull<T>,
    len: usize,
}
impl<T> OwnedSlice<T> {
    pub fn new<const COUNT: usize>(values: [T; COUNT]) -> Self {
        let v = Box::new(values);
        let ptr = NonNull::new(Box::leak(v).as_mut_ptr()).unwrap();
        Self { ptr, len: COUNT }
    }
    pub fn from_vec(v: Vec<T>) -> Self {
        let len = v.len();
        let ptr = NonNull::new(Box::leak(v.into_boxed_slice()).as_mut_ptr()).unwrap();
        // println!("slice created at {:#?}", ptr);
        Self { ptr, len }
    }
    pub fn from_box(v: Box<[T]>) -> Self {
        let len = v.len();
        let ptr = NonNull::new(Box::leak(v).as_mut_ptr()).unwrap();
        Self { ptr, len }
    }
    pub fn get(&self, idx: usize) -> Option<&T> {
        if idx < self.len {
            unsafe { Some(self.ptr.add(idx).as_ref()) }
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        if idx < self.len {
            unsafe { Some(self.ptr.add(idx).as_mut()) }
        } else {
            None
        }
    }
    pub unsafe fn get_unchecked(&self, idx: usize) -> &T {
        unsafe { Some(self.ptr.add(idx).as_ref()).unwrap_unchecked() }
    }
    pub unsafe fn get_mut_unchecked(&mut self, idx: usize) -> &mut T {
        unsafe { Some(self.ptr.add(idx).as_mut()).unwrap_unchecked() }
    }
    pub fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
    pub fn as_slice_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }
}
impl<T> Drop for OwnedSlice<T> {
    fn drop(&mut self) {
        unsafe {
            //  println!("slice freed at {:#?}", self.ptr);
            let b = self.as_slice_mut();
            _ = Box::from_raw(b);
        }
    }
}
impl<T: Clone> Clone for OwnedSlice<T> {
    fn clone(&self) -> Self {
        let mut r = Vec::new();
        r.reserve_exact(self.len);
        for i in self.as_slice() {
            r.push(i.clone());
        }
        Self::from_vec(r)
    }
}
impl<T> Index<usize> for OwnedSlice<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}
impl<T> IndexMut<usize> for OwnedSlice<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}
impl<T> Index<Range<usize>> for OwnedSlice<T> {
    type Output = [T];
    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.as_slice()[index]
    }
}
impl<T> IndexMut<Range<usize>> for OwnedSlice<T> {
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        &mut self.as_slice_mut()[index]
    }
}
impl<T> AsRef<[T]> for OwnedSlice<T> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}
impl<T> AsMut<[T]> for OwnedSlice<T> {
    fn as_mut(&mut self) -> &mut [T] {
        self.as_slice_mut()
    }
}
#[repr(C)]
#[derive(Clone)]
pub struct RetInfo {
    pub ip: usize,
    pub var_sp: usize,
    pub var_bp: usize,
}
#[repr(C)]
#[derive(Clone)]
pub struct GcInfo {}
impl GcInfo {
    pub fn new() -> Self {
        Self {}
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct RT {
    pub op_stack: OwnedSlice<Var>,
    pub op_stack_ptr: usize,
    pub instructions: OwnedSlice<u8>,
    pub ip: usize,
    pub var_stack: OwnedSlice<Var>,
    pub var_stack_ptr: usize,
    pub var_base_ptr: usize,
    pub ret_info: OwnedSlice<RetInfo>,
    pub ret_info_ptr: usize,
    pub halted: bool,
    pub symbol_table: HashMap<string::String, usize>,
    pub heap: RtHeap,
    pub strings: OwnedSlice<string::String>,
    pub gc_info: GcInfo,
}
impl Drop for RT {
    fn drop(&mut self) {
        crate::heap::rt_heap_mark_all_unreachable(&mut self.heap);
        crate::heap::rt_heap_free_all_unreachable(&mut self.heap);
    }
}
impl RT {
    pub fn next_instruction(&mut self) -> Instr {
        unsafe {
            let i = self.instructions.get_unchecked(self.ip).clone();
            self.ip += 1;
            std::mem::transmute(i)
        }
    }
    pub fn next_var(&mut self) -> Var {
        let mut bytes = [0; size_of::<Var>()];
        for i in 0..size_of::<Var>() {
            bytes[i] = self.instructions[self.ip + i] as u8;
        }
        self.ip += size_of::<Var>();
        unsafe { std::mem::transmute(bytes) }
    }
    pub fn next_u64(&mut self) -> u64 {
        unsafe {
            let bytes0: MaybeUninit<[u8; 8]> = MaybeUninit::zeroed();
            let mut bytes = bytes0.assume_init();
            for i in 0..8 {
                bytes[i] = std::mem::transmute(*self.instructions.get_unchecked(self.ip));
                self.ip += 1;
            }
            u64::from_le_bytes(bytes)
        }
    }
    pub fn op_pop(&mut self) -> Var {
        self.op_stack_ptr -= 1;
        unsafe { self.op_stack.get_unchecked(self.op_stack_ptr).clone() }
    }
    pub fn op_push(&mut self, var: Var) {
        unsafe {
            *self.op_stack.get_mut_unchecked(self.op_stack_ptr) = var;
            self.op_stack_ptr += 1;
        }
    }
    pub fn ret_push(&mut self, r: RetInfo) {
        self.ret_info[self.ret_info_ptr] = r;
        self.ret_info_ptr += 1;
    }
    pub fn ret_pop(&mut self) -> Option<RetInfo> {
        if self.ret_info_ptr < 1 {
            self.ret_info_ptr = 0;
            None
        } else {
            self.ret_info_ptr -= 1;
            let r = self.ret_info[self.ret_info_ptr].clone();
            Some(r)
        }
    }
    pub fn def_local_var(&mut self, vtype: Tag) {
        let mut tmp = Var::new();
        tmp.get_mut().tag = vtype;
        self.var_stack[self.var_stack_ptr] = tmp;
        self.var_stack_ptr += 1;
    }
    pub fn get_local_var(&mut self, idx: usize) -> Var {
        self.var_stack[self.var_base_ptr + idx].clone()
    }
    pub fn get_local_var_lvalue(&mut self, idx: usize) -> Var {
        let x = &mut self.var_stack[self.var_base_ptr + idx] as *mut Var;
        Var::l_value(x)
    }
    pub fn allocate_str(&mut self, s: &str) -> Var {
        unsafe {
            let slen = s.as_bytes().len();
            let s_h_ptr = crate::heap::rt_heap_allocate(&mut self.heap, slen, 0, 4) as *mut u8;
            let s_ptr = s_h_ptr.add(size_of::<crate::heap::Allocation>());
            for i in 0..slen {
                (*s_ptr.add(i)) = s.as_bytes()[i];
            }
            let bx_ptr = crate::heap::rt_heap_allocate(&mut self.heap, size_of::<BStr>(), 0, 3)
                as *mut crate::heap::Allocation;
            let bstr = bx_ptr.add(1) as *mut BStr;
            (*bstr).start = s_ptr;
            (*bstr).len = slen;
            return Var::string(bstr);
        }
    }
    pub fn try_take_ptr(&mut self, ptr: *mut Var) -> bool {
        unsafe {
            let al = ptr as *mut Allocation;
            let atm = &mut (*al).in_use;
            while let Err(_) = atm.compare_exchange_weak(
                0,
                1,
                std::sync::atomic::Ordering::Acquire,
                std::sync::atomic::Ordering::Acquire,
            ) {}
        }
        false
    }
    pub fn release_ptr(&mut self, ptr: *mut Var) {}
    pub fn step(&mut self) -> Option<bool> {
        let n = self.next_instruction();
        //        println!("{:#?}", n);
        match n {
            Jmp => {
                self.ip = self.next_u64() as usize;
            }
            JmpCond => {
                let cond = self.op_pop().get_bool();
                let to = self.next_u64();
                if cond {
                    self.ip = to as usize;
                }
            }
            Call => {
                let rst = self.next_u64() as usize;
                let old = self.ip;
                let old_var_ptr = self.var_stack_ptr;
                let old_var_bp = self.var_base_ptr;
                self.ret_push(RetInfo {
                    ip: old,
                    var_sp: old_var_ptr,
                    var_bp: old_var_bp,
                });
                self.var_base_ptr = self.var_stack_ptr;
                self.ip = rst;
            }
            CallObj => {
                todo!()
            }
            Ret => {
                if let Some(r) = self.ret_pop() {
                    self.ip = r.ip;
                    self.var_stack_ptr = r.var_sp;
                    self.var_base_ptr = r.var_bp;
                } else {
                    let s = self.op_pop();
                    println!("returned: {:#?}", s);
                    self.halted = true;
                    return Some(true);
                }
            }
            DefLocalVoid => {
                self.def_local_var(Void);
            }
            DefLocalInt => {
                self.def_local_var(Integer);
            }
            DefLocalFloat => {
                self.def_local_var(Float);
            }
            DefLocalBool => {
                self.def_local_var(Bool);
            }
            DefLocalPtr => {
                self.def_local_var(Ptr);
            }
            DefLocalStr => {
                self.def_local_var(String);
            }
            LoadVoid => {
                let idx = self.next_u64() as usize;
                // println!("{:#?} {idx}",n);
                let tmp = self.get_local_var(idx);
                self.op_push(tmp);
            }
            LoadInt => {
                let idx = self.next_u64() as usize;
                let tmp = self.get_local_var(idx);
                // println!("{:#?} {idx}",n);
                self.op_push(tmp);
            }
            LoadFloat => {
                let idx = self.next_u64() as usize;
                // println!("{:#?} {idx}",n);
                let tmp = self.get_local_var(idx);
                self.op_push(tmp);
            }
            LoadBool => {
                let idx = self.next_u64() as usize;
                // println!("{:#?} {idx}",n);
                let tmp = self.get_local_var(idx);
                self.op_push(tmp);
            }
            LoadPtr => {
                let idx = self.next_u64() as usize;
                // println!("{:#?} {idx}",n);
                let tmp = self.get_local_var(idx);
                self.op_push(tmp);
            }
            LoadStr => {
                let idx = self.next_u64() as usize;
                //println!("{:#?} {idx}",n);
                let tmp = self.get_local_var(idx);
                self.op_push(tmp);
            }
            LoadMember => {
                let s = self.op_pop();
                let offset = self.next_u64() as usize;
                unsafe {
                    let ptr = s.get_ptr().add(offset + 1);
                    self.op_push((*ptr).clone());
                }
            }
            LoadVarAddr => {
                let idx = self.next_u64() as usize;
                let tmp = self.get_local_var_lvalue(idx);
                self.op_push(tmp);
            }
            LoadMemberAddr => {
                let s = self.op_pop();
                let offset = self.next_u64() as usize;
                unsafe {
                    let ptr = s.get_ptr().add(offset + 1);
                    let v = Var::l_value(ptr as *mut Var);
                    self.op_push(v);
                }
            }
            StoreVoid => {
                let ptr = self.op_pop();
                let other = self.op_pop();
                let f = ptr.get_l_value();
                if f.get().tag != Void || other.get().tag != Void {
                    todo!();
                }
                unsafe {
                    *f.0.get() = other.get().clone();
                }
            }
            StoreInt => {
                let ptr = self.op_pop();
                let other = self.op_pop();
                let f = ptr.get_l_value();
                if f.get().tag != Integer || other.get().tag != Integer {
                    println!("{:#?},{:#?}", f, other);
                    self.debug_op_stack();
                    todo!();
                }
                unsafe {
                    *f.0.get() = other.get().clone();
                }
            }
            StoreFloat => {
                let ptr = self.op_pop();
                let other = self.op_pop();
                let f = ptr.get_l_value();
                if f.get().tag != Float || other.get().tag != Float {
                    println!("{:#?},{:#?}", f, other);
                    self.debug_op_stack();
                    todo!();
                }
                unsafe {
                    *f.0.get() = other.get().clone();
                }
            }
            StoreBool => {
                let ptr = self.op_pop();
                let other = self.op_pop();
                let f = ptr.get_l_value();
                if f.get().tag != Bool || other.get().tag != Bool {
                    println!("{:#?},{:#?}", f, other);
                    self.debug_op_stack();
                    todo!();
                }
                unsafe {
                    *f.0.get() = other.get().clone();
                }
            }
            StorePtr => {
                let ptr = self.op_pop();
                let other = self.op_pop();
                let f = ptr.get_l_value();
                if f.get().tag != Ptr || other.get().tag != Ptr {
                    todo!();
                }
                unsafe {
                    *f.0.get() = other.get().clone();
                }
            }
            StoreStr => {
                let ptr = self.op_pop();
                let other = self.op_pop();
                let f = ptr.get_l_value();
                if f.get().tag != String || other.get().tag != String {
                    todo!();
                }
                unsafe {
                    *f.0.get() = other.get().clone();
                }
            }
            ConstVoid => {
                let _ = self.next_u64();
                self.op_push(Var::void(()));
            }
            ConstInt => {
                let t = self.next_u64();
                self.op_push(Var::integer(u64::cast_signed(t)));
            }
            ConstFloat => {
                let t = self.next_u64();
                self.op_push(Var::float(f64::from_bits(t)));
            }
            ConstBool => {
                let t = self.next_u64();
                self.op_push(Var::boolean(t != 0));
            }
            ConstPtr => {
                todo!()
            }
            ConstStr => {
                let idx = self.next_u64() as usize;
                let s = self.strings[idx].clone();
                let st = self.allocate_str(&s);
                self.op_push(st);
            }
            IntAdd => {
                let r = self.op_pop();
                let l = self.op_pop();
                self.op_push(Var::integer(l.get_int() + r.get_int()));
            }
            IntSub => {
                let r = self.op_pop();
                let l = self.op_pop();
                self.op_push(Var::integer(l.get_int() - r.get_int()));
            }
            IntMul => {
                let r = self.op_pop();
                let l = self.op_pop();
                self.op_push(Var::integer(l.get_int() * r.get_int()));
            }
            IntDiv => {
                let r = self.op_pop();
                let l = self.op_pop();
                self.op_push(Var::integer(l.get_int() / r.get_int()));
            }
            IntEq => {
                let r = self.op_pop();
                let l = self.op_pop();
                self.op_push(Var::boolean(l.get_int() == r.get_int()));
            }
            IntNEq => {
                let r = self.op_pop();
                let l = self.op_pop();
                self.op_push(Var::boolean(l.get_int() != r.get_int()));
            }
            IntLess => {
                let r = self.op_pop();
                let l = self.op_pop();
                self.op_push(Var::boolean(l.get_int() < r.get_int()));
            }
            IntGreater => {
                let r = self.op_pop();
                let l = self.op_pop();
                self.op_push(Var::boolean(l.get_int() > r.get_int()));
            }
            FloatAdd => {
                let r = self.op_pop();
                let l = self.op_pop();
                self.op_push(Var::float(l.get_float() + r.get_float()))
            }
            FloatSub => {
                let r = self.op_pop();
                let l = self.op_pop();
                self.op_push(Var::float(l.get_float() - r.get_float()))
            }
            FloatMul => {
                let r = self.op_pop();
                let l = self.op_pop();
                self.op_push(Var::float(l.get_float() * r.get_float()))
            }
            FloatDiv => {
                let r = self.op_pop();
                let l = self.op_pop();
                self.op_push(Var::float(l.get_float() / r.get_float()))
            }
            FloatEq => {
                let r = self.op_pop();
                let l = self.op_pop();
                self.op_push(Var::boolean(l.get_float() == r.get_float()))
            }
            FloatNeq => {
                let r = self.op_pop();
                let l = self.op_pop();
                self.op_push(Var::boolean(l.get_float() != r.get_float()))
            }
            FloatLess => {
                let r = self.op_pop();
                let l = self.op_pop();
                self.op_push(Var::boolean(l.get_float() < r.get_float()));
            }
            FloatGreater => {
                let r = self.op_pop();
                let l = self.op_pop();
                self.op_push(Var::boolean(l.get_float() > r.get_float()));
            }
            BoolEq => {
                let r = self.op_pop();
                let l = self.op_pop();
                self.op_push(Var::boolean(l.get_bool() == r.get_bool()));
            }
            BoolNeq => {
                let r = self.op_pop();
                let l = self.op_pop();
                self.op_push(Var::boolean(l.get_bool() != r.get_bool()));
            }
            BoolAnd => {
                let r = self.op_pop();
                let l = self.op_pop();
                self.op_push(Var::boolean(l.get_bool() && r.get_bool()));
            }
            BoolOr => {
                let r = self.op_pop();
                let l = self.op_pop();
                self.op_push(Var::boolean(l.get_bool() && r.get_bool()));
            }
            StrAdd => {
                let r = self.op_pop();
                let l = self.op_pop();
                let sr = unsafe { (*r.get_string()).clone() };
                let sl = unsafe { (*l.get_string()).clone() };
                let s0 = sl.as_str().to_string() + sr.as_str();
                let ts = self.allocate_str(&s0);
                self.op_push(ts);
            }
            StrEq => {
                let r = self.op_pop();
                let l = self.op_pop();
                let sr = unsafe { (*r.get_string()).clone() };
                let sl = unsafe { (*l.get_string()).clone() };
                let b = sr.as_str() == sl.as_str();
                self.op_push(Var::boolean(b));
            }
            StrNeq => {
                let r = self.op_pop();
                let l = self.op_pop();
                let sr = unsafe { (*r.get_string()).clone() };
                let sl = unsafe { (*l.get_string()).clone() };
                let b = sr.as_str() != sl.as_str();
                self.op_push(Var::boolean(b))
            }
            New => unsafe {
                let info: AllocInfo = std::mem::transmute(self.next_u64());
                let ptr = crate::heap::rt_heap_allocate(
                    &mut self.heap,
                    info.field_count as usize * size_of::<Var>(),
                    info.field_count as u16,
                    info.type_info as u16,
                ) as *mut Var;
                for i in 0..info.field_count as usize {
                    (*ptr.add(i)) = Var::new()
                }
                self.op_push(Var::ptr(ptr));
            },
        }
        self.gc_update();
        Some(self.halted)
    }
    pub fn debug_instrs(&self) {
        let mut tmp = self.clone();
        tmp.ip = 0;
        while tmp.ip < tmp.instructions.len {
            print!("{}:", tmp.ip);
            let i = tmp.next_instruction();
            print!("{:#?} ", i);
            match i {
                Jmp => {
                    print!("{:#?}", tmp.next_u64() as usize);
                }
                JmpCond => {
                    let to = tmp.next_u64();
                    print!("{:#?}", to)
                }
                Call => {
                    let rst = tmp.next_u64() as usize;
                    print!("{:#?}", rst);
                }
                CallObj => {
                    todo!()
                }
                Ret => {}
                DefLocalVoid => {}
                DefLocalInt => {}
                DefLocalFloat => {}
                DefLocalBool => {}
                DefLocalPtr => {}
                DefLocalStr => {}
                LoadVoid => {
                    let idx = tmp.next_u64() as usize;
                    print!("{:#?}", idx);
                }
                LoadInt => {
                    let idx = tmp.next_u64() as usize;
                    print!("{:#?}", idx);
                }
                LoadFloat => {
                    let idx = tmp.next_u64() as usize;
                    print!("{:#?}", idx);
                }
                LoadBool => {
                    let idx = tmp.next_u64() as usize;
                    print!("{:#?}", idx);
                }
                LoadPtr => {
                    let idx = tmp.next_u64() as usize;
                    print!("{:#?}", idx);
                }
                LoadStr => {
                    let idx = tmp.next_u64() as usize;
                    print!("{:#?}", idx);
                }
                LoadMember => {
                    let idx = tmp.next_u64() as usize;
                    print!("{:#?}", idx);
                }
                LoadVarAddr => {
                    let idx = tmp.next_u64() as usize;
                    print!("{:#?}", idx);
                }
                LoadMemberAddr => {
                    let idx = tmp.next_u64() as usize;
                    print!("{:#?}", idx);
                }
                StoreVoid => {}
                StoreInt => {}
                StoreFloat => {}
                StoreBool => {}
                StorePtr => {}
                StoreStr => {}
                ConstVoid => {
                    let idx = tmp.next_u64() as usize;
                    print!("{:#?}", idx);
                }
                ConstInt => {
                    let idx = u64::cast_signed(tmp.next_u64());
                    print!("{:#?}", idx);
                }
                ConstFloat => {
                    let idx: f64 = f64::from_bits(tmp.next_u64());
                    print!("{:#?}", idx);
                }
                ConstBool => {
                    let idx = tmp.next_u64() as usize;
                    print!("{:#?}", idx);
                }
                ConstPtr => {
                    todo!()
                }
                ConstStr => {
                    let idx = tmp.next_u64() as usize;
                    println!("{:#?}", tmp.strings[idx]);
                }
                IntAdd => {}
                IntSub => {}
                IntMul => {}
                IntDiv => {}
                IntEq => {}
                IntNEq => {}
                IntLess => {}
                IntGreater => {}
                FloatAdd => {}
                FloatSub => {}
                FloatMul => {}
                FloatDiv => {}
                FloatEq => {}
                FloatNeq => {}
                FloatLess => {}
                FloatGreater => {}
                BoolEq => {}
                BoolNeq => {}
                BoolAnd => {}
                BoolOr => {}
                StrAdd => {}
                StrEq => {}
                StrNeq => {}
                New => {
                    let sz = tmp.next_u64();
                    println!("{:#?}", sz);
                }
            }
            println!("");
        }
    }
    pub fn debug_op_stack(&self) {
        println!("[");
        for i in 0..self.op_stack_ptr {
            println!("  {:#?}", self.op_stack[i]);
        }
        println!("]")
    }
    pub fn debug_var_stack(&self) {
        println!("[");
        for i in self.var_base_ptr..self.var_stack_ptr {
            println!("  {:#?}", self.op_stack[i]);
        }
        println!("]")
    }
    pub fn gc_update(&mut self) {
        self.gc_collect();
    }
    pub fn gc_mark(&mut self, ptr: *mut Var) {
        unsafe {
            let al = ptr as *mut Allocation;
            if al == std::ptr::null_mut() {
                return;
            }
            if (*al).reachable != 0 {
                return;
            }
            (*al).reachable = 1;
            for i in 1..(*al).num_objects as usize + 1 {
                let f = ptr.add(i);
                let v = &mut *f;
                match v.get().tag {
                    Ptr => self.gc_mark(v.get_ptr() as *mut Var),
                    String => self.gc_mark_string(v.get_string() as *mut BStr),
                    _ => {}
                }
            }
        }
    }

    pub fn gc_mark_string(&mut self, ptr: *mut BStr) {
        unsafe {
            if ptr == std::ptr::null_mut() {
                return;
            }
            let al = (ptr as *mut Allocation).sub(1);
            if (*al).reachable != 0 {
                return;
            }
            (*al).reachable = 1;
            let al_ptr = ((*ptr).start as *mut Allocation).sub(1);
            (*al_ptr).reachable = 1;
        }
    }
    pub fn gc_collect(&mut self) {
        crate::heap::rt_heap_mark_all_unreachable(&mut self.heap);
        for i in 0..self.var_stack_ptr {
            let obj = self.var_stack[i].clone();
            match obj.get().tag {
                Ptr => self.gc_mark(obj.get_ptr() as *mut Var),
                String => self.gc_mark_string(obj.get_string() as *mut BStr),
                _ => {}
            }
        }
        for i in 0..self.op_stack_ptr {
            let obj = self.op_stack[i].clone();
            match obj.get().tag {
                Ptr => self.gc_mark(obj.get_ptr() as *mut Var),
                String => self.gc_mark_string(obj.get_string() as *mut BStr),
                _ => {}
            }
        }
        crate::heap::rt_heap_free_all_unreachable(&mut self.heap);
    }
}
pub struct InstructionList {
    pub iv: Vec<u8>,
}
pub struct IntermediateRt {
    pub strings: Vec<string::String>,
    pub symbol_table: HashMap<string::String, usize>,
    pub data: InstructionList,
}
impl InstructionList {
    pub fn new() -> Self {
        Self { iv: Vec::new() }
    }
    pub fn push_instr(&mut self, ins: Instr) {
        self.iv.push(unsafe { std::mem::transmute(ins) });
    }
    pub fn push_bool(&mut self, v: bool) {
        let vs = if v { 1 } else { 0 };
        let bts = u64::to_le_bytes(vs);
        for i in bts {
            unsafe {
                self.iv.push(std::mem::transmute(i));
            }
        }
    }
    pub fn push_i64(&mut self, v: i64) {
        let vs = i64::to_le_bytes(v);
        for i in vs {
            unsafe {
                self.iv.push(std::mem::transmute(i));
            }
        }
    }
    pub fn push_float(&mut self, v: f64) {
        let vs = f64::to_le_bytes(v);
        for i in vs {
            unsafe {
                self.iv.push(std::mem::transmute(i));
            }
        }
    }
    pub fn push_u64(&mut self, v: u64) {
        let bts = u64::to_le_bytes(v);
        for i in bts {
            unsafe {
                self.iv.push(std::mem::transmute(i));
            }
        }
    }
}
pub fn compile_var(rt: &mut IntermediateRt, v: &crate::mach::Var, prg: &Program) {
    match v {
        crate::mach::Var::Stack {
            vtype,
            index,
            name: _,
        } => match vtype.as_type(&prg.types) {
            crate::mach::Type::Void => {
                rt.data.push_instr(LoadVoid);
                rt.data.push_u64(*index as u64);
            }
            crate::mach::Type::Integer => {
                rt.data.push_instr(LoadInt);
                rt.data.push_u64(*index as u64);
            }
            crate::mach::Type::Float => {
                rt.data.push_instr(LoadFloat);
                rt.data.push_u64(*index as u64);
            }
            crate::mach::Type::Bool => {
                rt.data.push_instr(LoadBool);
                rt.data.push_u64(*index as u64);
            }
            crate::mach::Type::String => {
                rt.data.push_instr(LoadStr);
                rt.data.push_u64(*index as u64);
            }
            crate::mach::Type::Ptr { to: _ } => {
                rt.data.push_instr(LoadPtr);
                rt.data.push_u64(*index as u64);
            }
            crate::mach::Type::Struct { name: _, fields: _ } => {
                todo!()
            }
            crate::mach::Type::Function {
                from: _,
                to: _,
                name: _,
            } => {
                todo!()
            }
        },
        crate::mach::Var::ConstInt { value } => {
            rt.data.push_instr(ConstInt);
            rt.data.push_i64(*value);
        }
        crate::mach::Var::ConstFloat { value } => {
            rt.data.push_instr(ConstFloat);
            rt.data.push_float(*value);
        }
        crate::mach::Var::ConstString { value } => {
            let idx = rt.strings.len();
            rt.strings.push(value.to_string());
            rt.data.push_instr(ConstStr);
            rt.data.push_u64(idx as u64);
        }
        crate::mach::Var::ConstBool { value } => {
            rt.data.push_instr(ConstBool);
            rt.data.push_bool(*value);
        }
        crate::mach::Var::Unit => {
            rt.data.push_instr(LoadVoid);
            rt.data.push_u64(0);
        }
        crate::mach::Var::FieldAccess {
            of,
            index,
            return_type: _,
        } => {
            compile_var(rt, of, prg);
            rt.data.push_instr(LoadMember);
            rt.data.push_u64(*index as u64);
        }
        crate::mach::Var::FunctionLiteral { name: _, idx: _ } => {
            todo!()
        }
        crate::mach::Var::OperatorNew { new_type } => {
            let t = new_type.as_type(&prg.types);
            match t {
                Type::Struct { name: _, fields } => {
                    let info = AllocInfo {
                        field_count: fields.len() as u32,
                        type_info: new_type.index as u32,
                    };
                    rt.data.push_instr(New);
                    unsafe {
                        rt.data.push_u64(std::mem::transmute(info));
                    }
                }
                _ => {
                    todo!()
                }
            }
        }
    }
}
pub fn compile_l_var(rt: &mut IntermediateRt, v: &crate::mach::Var, prg: &Program) {
    match v {
        crate::mach::Var::Stack {
            vtype: _,
            index,
            name: _,
        } => {
            rt.data.push_instr(Instr::LoadVarAddr);
            rt.data.push_u64(*index as u64);
        }
        crate::mach::Var::FieldAccess {
            of,
            index,
            return_type: _,
        } => {
            compile_var(rt, of, prg);
            rt.data.push_instr(Instr::LoadMemberAddr);
            rt.data.push_u64(*index as u64);
        }
        _ => {
            todo!()
        }
    }
}
pub fn compile_mach_to_rt(progs: &[Program]) -> RT {
    let mut out = IntermediateRt {
        symbol_table: HashMap::new(),
        data: InstructionList::new(),
        strings: Vec::new(),
    };
    let mut fixup_table: HashMap<usize, string::String> = HashMap::new();
    let mut idx = 0;
    let mut start_ptr = 0;
    for p in progs {
        for i in &p.functions {
            if i.1.display_name == "main" {
                start_ptr = out.data.iv.len();
            }
            let mut labels = HashMap::new();
            for j in &i.1.labels {
                labels.insert(*j.1 + idx + 1, j.0.clone());
            }
            if out.symbol_table.contains_key(i.0) {
                todo!();
            }
            out.symbol_table.insert(i.0.clone(), out.data.iv.len());
            //    println!("inserted:{:#?} at {}", i.0, out.data.iv.len());
            for j in &*i.1.arguments {
                match j.1.as_type(&p.types) {
                    crate::mach::Type::Bool => {
                        out.data.push_instr(DefLocalBool);
                    }
                    crate::mach::Type::Integer => {
                        out.data.push_instr(DefLocalInt);
                    }
                    crate::mach::Type::Float => {
                        out.data.push_instr(DefLocalFloat);
                    }
                    crate::mach::Type::Ptr { to: _ } => {
                        out.data.push_instr(DefLocalPtr);
                    }
                    crate::mach::Type::String => {
                        out.data.push_instr(DefLocalStr);
                    }
                    crate::mach::Type::Void => {
                        out.data.push_instr(DefLocalVoid);
                    }
                    _ => {
                        todo!()
                    }
                }
                idx += 1;
            }
            let mut rvs = (&*i.1.arguments).to_vec();
            rvs.reverse();
            let mut ix = rvs.len();
            for j in &rvs {
                ix -= 1;
                match j.1.as_type(&p.types) {
                    crate::mach::Type::Void => {
                        out.data.push_instr(LoadVarAddr);
                        out.data.push_u64(ix as u64);
                        out.data.push_instr(StoreVoid);
                    }
                    crate::mach::Type::Integer => {
                        out.data.push_instr(LoadVarAddr);
                        out.data.push_u64(ix as u64);
                        out.data.push_instr(StoreInt);
                    }
                    crate::mach::Type::Float => {
                        out.data.push_instr(LoadVarAddr);
                        out.data.push_u64(ix as u64);
                        out.data.push_instr(StoreFloat);
                    }
                    crate::mach::Type::Bool => {
                        out.data.push_instr(LoadVarAddr);
                        out.data.push_u64(ix as u64);
                        out.data.push_instr(StoreBool);
                    }
                    crate::mach::Type::String => {
                        out.data.push_instr(LoadVarAddr);
                        out.data.push_u64(ix as u64);
                        out.data.push_instr(StoreStr);
                    }
                    crate::mach::Type::Ptr { to: _ } => {
                        out.data.push_instr(LoadVarAddr);
                        out.data.push_u64(ix as u64);
                        out.data.push_instr(StorePtr);
                    }
                    _ => {
                        todo!()
                    }
                }
            }
            for j in &i.1.cmds {
                let rt = &mut out;
                if labels.contains_key(&idx) {
                    //          println!("inserted:{:#?} at {}", labels[&idx], rt.data.iv.len());
                    rt.symbol_table
                        .insert(labels[&idx].clone(), rt.data.iv.len());
                }
                match j {
                    crate::mach::Cmd::Binop { l, r, out, op } => {
                        compile_var(rt, l, p);
                        compile_var(rt, r, p);
                        match l.get_type(&p.types) {
                            Type::Integer => match op {
                                crate::mach::Binop::Add => {
                                    rt.data.push_instr(IntAdd);
                                    compile_l_var(rt, out, p);
                                    rt.data.push_instr(StoreInt);
                                }
                                crate::mach::Binop::Sub => {
                                    rt.data.push_instr(IntSub);
                                    compile_l_var(rt, out, p);
                                    rt.data.push_instr(StoreInt);
                                }
                                crate::mach::Binop::Mul => {
                                    rt.data.push_instr(IntMul);
                                    compile_l_var(rt, out, p);
                                    rt.data.push_instr(StoreInt);
                                }
                                crate::mach::Binop::Div => {
                                    rt.data.push_instr(IntDiv);
                                    compile_l_var(rt, out, p);
                                    rt.data.push_instr(StoreInt);
                                }
                                crate::mach::Binop::Equal => {
                                    rt.data.push_instr(IntEq);
                                    compile_l_var(rt, out, p);
                                    rt.data.push_instr(StoreBool);
                                }
                                crate::mach::Binop::NotEqual => {
                                    rt.data.push_instr(IntNEq);
                                    compile_l_var(rt, out, p);
                                    rt.data.push_instr(StoreBool);
                                }
                                crate::mach::Binop::Less => {
                                    rt.data.push_instr(IntLess);
                                    compile_l_var(rt, out, p);
                                    rt.data.push_instr(StoreBool);
                                }
                                crate::mach::Binop::Greater => {
                                    rt.data.push_instr(IntGreater);
                                    compile_l_var(rt, out, p);
                                    rt.data.push_instr(StoreBool);
                                }
                                _ => {
                                    todo!()
                                }
                            },
                            Type::Float => match op {
                                crate::mach::Binop::Add => {
                                    rt.data.push_instr(FloatAdd);
                                    compile_l_var(rt, out, p);
                                    rt.data.push_instr(StoreFloat);
                                }
                                crate::mach::Binop::Sub => {
                                    rt.data.push_instr(FloatSub);
                                    compile_l_var(rt, out, p);
                                    rt.data.push_instr(StoreFloat);
                                }
                                crate::mach::Binop::Mul => {
                                    rt.data.push_instr(FloatMul);
                                    compile_l_var(rt, out, p);
                                    rt.data.push_instr(StoreFloat);
                                }
                                crate::mach::Binop::Div => {
                                    rt.data.push_instr(FloatDiv);
                                    compile_l_var(rt, out, p);
                                    rt.data.push_instr(StoreFloat);
                                }
                                crate::mach::Binop::Equal => {
                                    rt.data.push_instr(FloatEq);
                                    compile_l_var(rt, out, p);
                                    rt.data.push_instr(StoreBool);
                                }
                                crate::mach::Binop::NotEqual => {
                                    rt.data.push_instr(FloatNeq);
                                    compile_l_var(rt, out, p);
                                    rt.data.push_instr(StoreBool);
                                }
                                crate::mach::Binop::Less => {
                                    rt.data.push_instr(FloatLess);
                                    compile_l_var(rt, out, p);
                                    rt.data.push_instr(StoreBool);
                                }
                                crate::mach::Binop::Greater => {
                                    rt.data.push_instr(FloatGreater);
                                    compile_l_var(rt, out, p);
                                    rt.data.push_instr(StoreBool);
                                }
                                _ => {
                                    todo!()
                                }
                            },
                            Type::Bool => match op {
                                Binop::Equal => {
                                    rt.data.push_instr(BoolEq);
                                    compile_l_var(rt, out, p);
                                    rt.data.push_instr(StoreBool);
                                }
                                Binop::NotEqual => {
                                    rt.data.push_instr(BoolNeq);
                                    compile_l_var(rt, out, p);
                                    rt.data.push_instr(StoreBool);
                                }
                                Binop::Or => {
                                    rt.data.push_instr(BoolOr);
                                    compile_l_var(rt, out, p);
                                    rt.data.push_instr(StoreBool);
                                }
                                Binop::And => {
                                    rt.data.push_instr(BoolAnd);
                                    compile_l_var(rt, out, p);
                                    rt.data.push_instr(StoreBool);
                                }
                                _ => {
                                    todo!()
                                }
                            },
                            Type::String => match op {
                                Binop::Add => {
                                    rt.data.push_instr(StrAdd);
                                    compile_l_var(rt, out, p);
                                    rt.data.push_instr(StoreStr);
                                }
                                Binop::Equal => {
                                    rt.data.push_instr(StrEq);
                                    compile_l_var(rt, out, p);
                                    rt.data.push_instr(StoreBool);
                                }
                                Binop::NotEqual => {
                                    rt.data.push_instr(StrNeq);
                                    compile_l_var(rt, out, p);
                                    rt.data.push_instr(StoreBool);
                                }
                                _ => {
                                    todo!()
                                }
                            },
                            _ => {
                                todo!()
                            }
                        }
                    }
                    crate::mach::Cmd::Assign { l, r } => {
                        compile_var(rt, r, p);
                        compile_l_var(rt, l, p);
                        match l.get_type(&p.types) {
                            Type::Bool => {
                                out.data.push_instr(StoreBool);
                            }
                            Type::Float => {
                                out.data.push_instr(StoreFloat);
                            }
                            Type::Integer => {
                                out.data.push_instr(StoreInt);
                            }
                            Type::Ptr { to: _ } => {
                                out.data.push_instr(StorePtr);
                            }
                            Type::Void => {
                                out.data.push_instr(StoreVoid);
                            }
                            Type::String => {
                                out.data.push_instr(StoreStr);
                            }
                            _ => {
                                todo!()
                            }
                        }
                    }
                    crate::mach::Cmd::Jmp { to, to_idx: _ } => {
                        rt.data.push_instr(Jmp);
                        fixup_table.insert(rt.data.iv.len(), to.to_string());
                        rt.data.push_u64(42069);
                    }
                    crate::mach::Cmd::JmpCond {
                        cond,
                        to,
                        to_idx: _,
                    } => {
                        compile_var(rt, cond, p);
                        rt.data.push_instr(JmpCond);
                        fixup_table.insert(rt.data.iv.len(), to.to_string());
                        rt.data.push_u64(42069);
                    }
                    crate::mach::Cmd::DeclareVariables { values } => {
                        for i in values.iter() {
                            //               println!("type dec:{:#?}", i);
                            match i {
                                Type::Bool => {
                                    rt.data.push_instr(DefLocalBool);
                                }
                                Type::Float => {
                                    rt.data.push_instr(DefLocalFloat);
                                }
                                Type::Integer => {
                                    rt.data.push_instr(DefLocalInt);
                                }
                                Type::String => {
                                    rt.data.push_instr(DefLocalStr);
                                }
                                Type::Ptr { to: _ } => {
                                    rt.data.push_instr(DefLocalPtr);
                                }
                                Type::Void => {
                                    rt.data.push_instr(DefLocalVoid);
                                }
                                _ => {
                                    todo!()
                                }
                            }
                        }
                    }
                    crate::mach::Cmd::Call {
                        to_call,
                        returned,
                        args,
                    } => {
                        for k in args.iter() {
                            compile_var(rt, k, p);
                        }
                        rt.data.push_instr(Call);
                        let c = match to_call {
                            crate::mach::Var::FunctionLiteral { name, idx: _ } => name.to_string(),
                            _ => {
                                todo!()
                            }
                        };
                        fixup_table.insert(rt.data.iv.len(), c);
                        rt.data.push_u64(42069);
                        compile_l_var(rt, returned, p);
                        match returned.get_type(&p.types) {
                            Type::Void => {
                                rt.data.push_instr(StoreVoid);
                            }
                            Type::Integer => {
                                rt.data.push_instr(StoreInt);
                            }
                            Type::Float => {
                                rt.data.push_instr(StoreFloat);
                            }
                            Type::Bool => {
                                rt.data.push_instr(StoreBool);
                            }
                            Type::String => {
                                rt.data.push_instr(StoreStr);
                            }
                            Type::Ptr { to: _ } => {
                                rt.data.push_instr(StorePtr);
                            }
                            _ => {
                                todo!()
                            }
                        }
                    }
                    crate::mach::Cmd::Return { to_return } => {
                        compile_var(rt, to_return, p);
                        rt.data.push_instr(Ret);
                    }
                }
                idx += 1;
            }
        }
    }
    for i in &fixup_table {
        let ix = u64::to_le_bytes(out.symbol_table[i.1] as u64);
        for j in 0..8 {
            out.data.iv[i.0 + j] = unsafe { std::mem::transmute(ix[j]) };
        }
    }
    let tmp = RT {
        op_stack: OwnedSlice::new([const { Var::new() }; 4096 * 16]),
        op_stack_ptr: 0,
        instructions: OwnedSlice::from_vec(out.data.iv),
        ip: start_ptr,
        var_stack: OwnedSlice::new([const { Var::new() }; 4096 * 16]),
        var_stack_ptr: 0,
        var_base_ptr: 0,
        ret_info: OwnedSlice::new(
            [const {
                RetInfo {
                    ip: 0,
                    var_sp: 0,
                    var_bp: 0,
                }
            }; 4096 * 16],
        ),
        ret_info_ptr: 0,
        halted: false,
        symbol_table: out.symbol_table,
        strings: OwnedSlice::from_vec(out.strings),
        heap: RtHeap {
            allocations: std::ptr::null_mut(),
        },
        gc_info: GcInfo::new(),
    };
    tmp
}
