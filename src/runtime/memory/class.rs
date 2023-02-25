// use std::{alloc, collections::{LinkedList, HashMap}, alloc::{alloc, Layout}};
// use crate::runtime::memory::storage::StackReservation;

// use super::{storage::{Stack, StackVariable}, types::{STACK_LAYOUT, self}};

// pub struct ClassDeclaration {
//     fields: LinkedList<StackReservation>,
//     name: String,
// }

// pub struct ClassInstance {
//     stack: *mut Stack,
//     name: String,
// }

// impl ClassInstance {
//     pub unsafe fn alloc(template: ClassDeclaration) -> *mut ClassInstance {
//         let stack = {
//             let reserve = template.fields;
//             let mut size: usize = 0;
    
//             let mut r_a_nodes = HashMap::<String, StackVariable>::new();
    
//             for reservation in reserve {
//                 r_a_nodes.insert(reservation.name, StackVariable { offset: size, vtype: reservation.vtype });
//                 size += types::get_layout(&reservation.vtype).size();
//             }
    
//             let fields_ptr = alloc::alloc(Layout::from_size_align_unchecked(size, 0));
//             let stack_ptr = alloc::alloc(Layout::new::<Stack>()) as *mut Stack;
    
//             let stack = Stack {
//                 start: fields_ptr,
//                 offsets: r_a_nodes,
//                 size,
//                 self_ptr: stack_ptr as *mut u8,
//             };
    
//             //*stack_ptr = stack;
    
//             return stack_ptr as *mut ClassInstance;
//         };
        
//         let class_ptr = alloc(STACK_LAYOUT) as *mut ClassInstance;
        
//         let class = ClassInstance {
//             stack,
//             name: template.name,
//         };
        
//         *class_ptr = class;
        
//         return class_ptr;
//     }
//     pub unsafe fn dealloc(&mut self) {
//         (*self.stack).dealloc();
//     }
// }
