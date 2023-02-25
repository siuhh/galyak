use super::layouts::{get_layout, Type};

pub struct HeapSpace {
    
}

unsafe fn heap_alloc(t: Type) -> *mut u8 {
    return std::alloc::alloc(get_layout(&t));
}