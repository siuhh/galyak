use core::alloc::Layout;
use std::alloc::{self, dealloc};

use std::collections::hash_map::DefaultHasher;
use std::collections::{LinkedList};
use std::hash::{Hash, Hasher};
use std::ptr::write;

use super::errors::{var_not_found, err_wrong_type, var_already_exists};
use super::types::{self, Type, STACK_LAYOUT};
use super::types::get_layout;

pub struct StackHashMap {
    pub vec: Vec<(u64, StackVariable)>,
}

pub struct StackVariable {
    pub offset: usize,
    pub vtype: Type,
}

impl StackHashMap {
    pub fn new() -> Self {
        return StackHashMap { vec: Vec::new() };
    }
    
    fn hash(str: String) -> u64 {
        let hasher = &mut DefaultHasher::new();
        str.hash(hasher);
        return hasher.finish();
    }
    
    pub fn push(&mut self, key: &String, value: StackVariable) -> Result<(), String> {
        let hashed = StackHashMap::hash(key.clone());
        let mut pos = 0;
        
        loop {
            let n = self.vec.get(pos);
            if n.is_none() {
                self.vec.insert(pos, (hashed, value));
            }
            else if n.unwrap().0 == hashed {
                return Err(var_already_exists());
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
        return Ok(());
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
            } 
            else if self.vec[mid].0 < target {
                left = mid + 1;
            } 
            else {
                right = mid - 1;
            }
        }

        return None;
    }
}
#[derive(Clone)]
pub struct VarInfo {
    pub vtype: Type,
    pub name: String,
}
pub struct GlkStack {
    pub size: usize,
    pub start: *mut u8,
    pub offsets: StackHashMap,
    pub self_ptr: *mut u8,
}

impl GlkStack {
    pub unsafe fn alloc(reserve: &LinkedList<VarInfo>) -> *mut GlkStack {
        let mut size: usize = 0;
        
        let mut align = 0;
        
        let mut offsets = StackHashMap::new();
        
        for reservation in reserve {
            //TODO! call error on variable names repeat
            let _push_result = offsets.push(
                &reservation.name, 
                StackVariable { offset: size, vtype: reservation.vtype }
            );
            
            let curr_size = types::get_layout(&reservation.vtype).size();
            
            if align < curr_size {
                align = curr_size;
            }
            
            size += curr_size;
        }
        
        let fields_ptr = alloc::alloc(Layout::from_size_align(size, 8).unwrap());
        //TODO! зробити шось з вирівнюванням
        let stack_ptr = alloc::alloc(Layout::new::<GlkStack>()) as *mut GlkStack;
        
        let stack = GlkStack {
            start: fields_ptr,
            size,
            self_ptr: stack_ptr as *mut u8,
            offsets,
        };
        
        write(stack_ptr, stack);
        
        return stack_ptr;
    }
    
    pub unsafe fn nahuy(&mut self) {
        for vec_el in &self.offsets.vec {
            let var = &vec_el.1;
            
            match var.vtype {
                Type::Class => {
                    todo!(); //TODO! class dealloc here
                }
                Type::Stack => {
                    
                }
                _ => ()
            }
        }
        dealloc(self.start, Layout::from_size_align_unchecked(self.size, 0));
        dealloc(self.self_ptr, STACK_LAYOUT);
    }
    
    pub unsafe fn get_typed(&mut self, name: &String, vtype: &Type) -> Result<*mut u8, String> {
        let var = self.offsets.get(name);
        
        match var {
            Some(val) => {
                if val.vtype != *vtype {
                    return Err(err_wrong_type());
                }
                return Ok(self.start.add(val.offset));
            },
            None => {
                return Err(var_not_found());
            },
        }
    }
    
    pub unsafe fn get_dynamicaly(&mut self, name: &String) -> Result<(*mut u8, Type), String> {
        let var = self.offsets.get(name);
        
        match var {
            Some(val) => {
                return Ok((self.start.add(val.offset), val.vtype));
            },
            None => {
                return Err(var_not_found());
            },
        }
        
    }
    
}

pub struct HeapSpace {
    
}

unsafe fn heap_alloc(t: Type) -> *mut u8 {
    return std::alloc::alloc(get_layout(&t));
}