use std::{collections::LinkedList, alloc::alloc};
use crate::runtime::memory::stack::StackReservation;

use super::{stack::Stack, layouts::CLASS_LAYOUT};

pub struct ClassDeclaration {
    fields: LinkedList<StackReservation>,
    name: String,
}

pub struct ClassInstance {
    stack: *mut Stack,
    name: String,
}

impl ClassInstance {
    pub unsafe fn alloc(template: ClassDeclaration) -> *mut ClassInstance {
        let stack = Stack::alloc(template.fields);
        
        let class_ptr = alloc(CLASS_LAYOUT) as *mut ClassInstance;
        
        let class = ClassInstance {
            stack,
            name: template.name,
        };
        
        *class_ptr = class;
        
        return class_ptr;
    }
    pub unsafe fn dealloc(&mut self) {
        (*self.stack).dealloc();
    }
}
