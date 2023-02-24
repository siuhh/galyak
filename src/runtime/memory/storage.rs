use core::alloc::Layout;
use std::alloc::{self, dealloc, alloc};

use std::collections::hash_map::DefaultHasher;
use std::collections::{LinkedList};
use std::hash::{Hash, Hasher};
use std::ptr::null;

use super::types::{self, Type, STACK_LAYOUT};
use super::types::get_layout;

pub struct StackHashMap {
    pub vec: Vec<(u64, StackVariable)>,
}
impl StackHashMap {
    pub fn new() -> Self {
        return  StackHashMap { vec: Vec::new() };
    }
    fn hash(str: String) -> u64 {
        let hasher = &mut DefaultHasher::new();
        str.hash(hasher);
        return hasher.finish();
    }
    pub fn push(&mut self, key: &String, value: StackVariable) {
        let hashed = StackHashMap::hash(key.clone());
        let mut pos = 0;
        
        loop {
            let n = self.vec.get(pos);
            
            if n.is_none() {
                self.vec.insert(pos, (hashed, value));
            }
            else if n.unwrap().0 > hashed {
                self.vec.insert(pos, (hashed, value));
            }
            else {
                pos += 1;
                continue;
            }
            break;
        }
    }
    pub fn get(&mut self, key: &String) -> Option<&StackVariable> {
        let target = StackHashMap::hash(key.clone());
        
        let mut left = 0;
        let mut right = self.vec.len() - 1;

        //bin search
        while left <= right {
            let mid = left + (right - left) / 2;

            if self.vec[mid].0 == target {
                return Some(&self.vec[mid].1);
            } else if self.vec[mid].0 < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        return None;
    }
}


#[derive(PartialEq)]
pub struct StackReservation {
    pub vtype: Type,
    pub name: String,
}
#[derive(Debug)]
pub struct StackVariable {
    pub offset: usize,
    pub vtype: Type,
}

pub struct Stack {
    pub size: usize,
    pub start: *mut u8,
    pub offsets: StackHashMap,
    pub self_ptr: *mut u8,
}

impl Stack {
    pub unsafe fn alloc(reserve: LinkedList<StackReservation>) -> *mut Stack {
        let mut size: usize = 0;
        
        let mut align = 0;
        
        let mut offsets = StackHashMap::new();
        
        for reservation in &reserve {
            
            offsets.push(&reservation.name, StackVariable { offset: size, vtype: reservation.vtype });
            
            let curr_size = types::get_layout(&reservation.vtype).size();
            
            if align < curr_size {
                align = curr_size;
            }
            
            size += curr_size;
        }
        
        let fields_ptr = alloc::alloc(Layout::from_size_align(size, align).unwrap());
        let stack_ptr = alloc::alloc(Layout::new::<Stack>()) as *mut Stack;
        
        (*stack_ptr).start = fields_ptr;
        (*stack_ptr).size = size;
        (*stack_ptr).self_ptr = stack_ptr as *mut u8;
        (*stack_ptr).offsets = offsets;
        
        
        return stack_ptr;
    }
    
    pub unsafe fn dealloc(&mut self) {
        dealloc(self.start, Layout::from_size_align_unchecked(self.size, 0));
        dealloc(self.self_ptr, STACK_LAYOUT);
    }
    
    pub unsafe fn get_wt(&mut self, name: &String, vtype: &Type) -> *mut u8 {
        let var = self.offsets.get(name);
        
        match var {
            Some(val) => {
                if val.vtype != *vtype {
                    //TODO! call error here
                    panic!();
                }
                return self.start.add(val.offset);
            },
            None => {
                return null::<u8>() as *mut u8;
            },
        }
        
        
    }
    
    pub unsafe fn get(&mut self, name: &String) -> (*mut u8, Type) {
        let var = self.offsets.get(name);
        
        match var {
            Some(val) => {
                return (self.start.add(val.offset), val.vtype);
            },
            None => {
                return (null::<u8>() as *mut u8, Type::Null);
            },
        }
        
    }
}

pub struct HeapSpace {
    
}

unsafe fn heap_alloc(t: Type) -> *mut u8 {
    return std::alloc::alloc(get_layout(&t));
}