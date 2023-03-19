use core::alloc::Layout;
use std::alloc::{self, dealloc};

use std::collections::hash_map::DefaultHasher;
use std::collections::{LinkedList};
use std::hash::{Hash, Hasher};
use std::ptr::{write, null_mut};

use crate::program::errors::runtime::*;
use super::types::{self, Type, STACK_LAYOUT};
#[derive(Debug)]
pub struct StackHashMap {
    pub vec: Vec<(u64, StackVariable)>,
}
#[derive(Debug)]
pub struct StackVariable {
    pub offset: usize,
    pub vtype: Type,
    pub initialized: bool,
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
    
    pub fn get(&mut self, key: &String) -> Option<&mut StackVariable> {
        if self.vec.len() == 0 {
            return None;
        }
        let target = StackHashMap::hash(key.clone());
        
        let mut left: i32 = 0;
        let mut right: i32 = (self.vec.len() - 1) as i32;

        //bin search
        while left <= right {
            let mid: i32 = left + (right - left) / 2;

            if self.vec[mid as usize].0 == target {
                return Some(&mut self.vec[mid as usize].1);
            } 
            else if self.vec[mid as usize].0 < target {
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
    pub align: usize,
    pub parent: *mut GlkStack,
}

impl GlkStack {
    pub unsafe fn alloc(reserve: &LinkedList<VarInfo>, parent: *mut GlkStack) -> *mut GlkStack {
        let mut size: usize = 0;
        
        let mut align = 0;
        
        let mut offsets = StackHashMap::new();
        
        for reservation in reserve {
            offsets.push(
                &reservation.name, 
                StackVariable { offset: size, vtype: reservation.vtype, initialized: false }
            );
            
            let curr_size = types::get_layout(&reservation.vtype).size();
            
            if align < curr_size {
                align = curr_size;
            }
            
            size += curr_size;
        }
        
        //TODO! зробити шось з вирівнюванням
        let align = 8;
        let fields_ptr = if size > 0 {
            alloc::alloc(Layout::from_size_align(size, align).unwrap())
        }
        else {
            null_mut()
        };
        let stack_ptr = alloc::alloc(Layout::new::<GlkStack>()) as *mut GlkStack;
        
        let stack = GlkStack {
            start: fields_ptr,
            align,
            size,
            self_ptr: stack_ptr as *mut u8,
            offsets,
            parent,
        };
        
        write(stack_ptr, stack);
        
        return stack_ptr;
    }
    
    pub unsafe fn nahuy(&mut self) {
        for vec_el in &self.offsets.vec {
            let var = &vec_el.1;
            
            match var.vtype {
                _ => ()
            }
        }
        dealloc(self.start, Layout::from_size_align(self.size, self.align).unwrap());
        dealloc(self.self_ptr, STACK_LAYOUT);
    }
    
    pub unsafe fn get_typed(
        &mut self, 
        name: &String, 
        expected_type: &Type, 
        expected_init_status: bool
    ) -> Result<*mut u8, String> {
        let var = self.offsets.get(name);
        
        match var {
            Some(mut val) => {
                if expected_init_status == true && val.initialized == false {
                    return Err(var_not_found(name));
                }
                if expected_init_status == false && val.initialized == true {
                    return Err(var_already_exists(name));
                }
                if val.vtype != *expected_type {
                    return Err(wrong_type(name, expected_type, &val.vtype));
                }
                if expected_init_status == false {
                    val.initialized = true;
                }
                return Ok(self.start.add(val.offset));
            },
            None => {
                if !self.parent.is_null() {
                    return (*self.parent).get_typed(name, expected_type, expected_init_status);
                }
                return Err(var_not_found(name));
            },
        }
    }
    
    pub unsafe fn get_dynamicaly(&mut self, name: &String, expected_init_status: bool) 
    -> Result<(*mut u8, Type), String> {
        let var = self.offsets.get(name);
        
        match var {
            Some(val) => {
                if expected_init_status == true && val.initialized == false {
                    return Err(var_not_found(name));
                }
                if expected_init_status == false && val.initialized == true {
                    return Err(var_already_exists(name));
                }
                if expected_init_status == false {
                    val.initialized = true;
                }

                return Ok((self.start.add(val.offset), val.vtype));
            },
            None => {
                if !self.parent.is_null() {
                    return (*self.parent).get_dynamicaly(name, expected_init_status);
                }
                return Err(var_not_found(name));
            },
        }   
    }
    
    pub unsafe fn uninit_all(&mut self) {
        for mut e in &mut self.offsets.vec {
            e.1.initialized = false;
        }
    }
}