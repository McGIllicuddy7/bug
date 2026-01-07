use libc::c_void;
use std::{ptr::null_mut, sync::atomic::AtomicU16};
#[repr(C)]
pub struct Allocation {
    pub in_use: AtomicU16,
    pub reachable: u16,
    pub type_idx: u16,
    pub num_objects: u16,
}

#[repr(C)]
#[derive(Clone)]
pub struct RtHeap {
    allocations: Vec<*mut Allocation>,
}
impl RtHeap {
    pub fn new() -> Self {
        Self {
            allocations: Vec::new(),
        }
    }
}
#[unsafe(no_mangle)]
pub fn rt_heap_allocate(
    heap: &mut RtHeap,
    size: usize,
    num_objects: u16,
    type_ptr: u16,
) -> *mut c_void {
    unsafe {
        let out = libc::malloc(size + size_of::<Allocation>()) as *mut Allocation;
        (*out).reachable = 0;
        (*out).num_objects = num_objects;
        (*out).type_idx = type_ptr;
        heap.allocations.push(out);
        return out as *mut c_void;
    }
}
#[unsafe(no_mangle)]
pub fn rt_heap_mark_all_unreachable(heap: &mut RtHeap) {
    unsafe {
        for i in &heap.allocations {
            (**i).reachable = 0;
        }
    }
}
#[unsafe(no_mangle)]
pub fn rt_heap_free_all_unreachable(heap: &mut RtHeap) {
    unsafe {
        let mut new_allocs = Vec::new();
        for i in &heap.allocations {
            if (**i).reachable == 0 {
                libc::free(*i as *mut c_void);
            } else {
                new_allocs.push(*i);
            }
        }
    }
}
