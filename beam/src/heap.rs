use libc::c_void;
use std::{ptr::null_mut, sync::atomic::AtomicU16};
#[repr(C)]
pub struct Allocation {
    pub in_use: AtomicU16,
    pub reachable: u16,
    pub type_idx: u16,
    pub num_objects: u16,
    pub next: *mut Allocation,
}

#[repr(C)]
#[derive(Clone)]
pub struct RtHeap {
    pub allocations: *mut Allocation,
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
        (*out).next = heap.allocations;
        (*out).type_idx = type_ptr;
        heap.allocations = out;
        return out as *mut c_void;
    }
}
#[unsafe(no_mangle)]
pub fn rt_heap_mark_all_unreachable(heap: &mut RtHeap) {
    unsafe {
        let mut a = heap.allocations;
        while a != null_mut() {
            //println!("marking:3 {:#?}", a);
            (*a).reachable = 0;
            a = (*a).next;
        }
    }
}
#[unsafe(no_mangle)]
pub fn rt_heap_free_all_unreachable(heap: &mut RtHeap) {
    unsafe {
        let mut prev: *mut Allocation = null_mut();
        let mut a = heap.allocations;
        while a != null_mut() {
            //println!("checking:{:#?}", a);
            if (*a).reachable == 0 {
                if prev != null_mut() {
                    (*prev).next = (*a).next;
                } else {
                    heap.allocations = (*a).next;
                }
                let old = a;
                a = (*a).next;
                println!("freeing:{:#?}", old);
                libc::free(old as *mut c_void);
            } else {
                prev = a;
                a = (*a).next;
            }
        }
    }
}
