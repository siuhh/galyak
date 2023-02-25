use core::alloc::Layout;
use std::alloc::{self, dealloc};

use std::collections::{HashMap, LinkedList};
use std::ptr::null;

use super::layouts::{self, Type, STACK_LAYOUT};


#[derive(PartialEq)]
pub struct StackReservation {
    pub vtype: Type,
    pub name: String,
}

pub struct StackVariable {
    pub offset: usize,
    pub vtype: Type,
}

pub struct Stack {
    pub size: usize,
    pub start: *mut u8,
    pub offsets: HashMap<String, StackVariable>,
    pub self_ptr: *mut u8,
}

impl Stack {
    pub unsafe fn alloc(reserve: LinkedList<StackReservation>) -> *mut Stack {
        let mut size: usize = 0;
        
        let mut r_a_nodes = HashMap::<String, StackVariable>::new();
        
        for reservation in reserve {
            r_a_nodes.insert(reservation.name, StackVariable { offset: size, vtype: reservation.vtype });
            size += layouts::get_layout(&reservation.vtype).size();
        }
        
        let fields_ptr = alloc::alloc(Layout::from_size_align_unchecked(size, 0));
        let stack_ptr = alloc::alloc(Layout::new::<Stack>()) as *mut Stack;
        
        let stack = Stack {
            start: fields_ptr,
            offsets: r_a_nodes,
            size,
            self_ptr: stack_ptr as *mut u8,
        };
        
        *stack_ptr = stack;
        
        return stack_ptr;
    }
    
    pub unsafe fn dealloc(&mut self) {
        dealloc(self.start, Layout::from_size_align_unchecked(self.size, 0));
        dealloc(self.self_ptr, STACK_LAYOUT);
    }
    
    pub unsafe fn get_var(&mut self, name: String) -> (*mut u8, Type) {
        if !self.offsets.contains_key(&name) {
            return (null::<u8>() as *mut u8, Type::Null);
        }
        
        let var = self.offsets.get(&name).unwrap();
        
        return (self.start.add(var.offset), var.vtype);
    }
}