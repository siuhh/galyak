use std::{ ptr::null, alloc::{Layout, dealloc, alloc} };

use crate::error_mgr::ErrorType;

const NODE_NULL: *mut Node = null::<Node>() as *mut Node;
const NODE_LAYOUT: Layout = Layout::new::<Node>();

use super::types::{Type, get_layout};

pub struct Node {
    next: *mut Node,
    prev: *mut Node, 
    val: *mut u8,
}

impl Node {
    pub unsafe fn new(val: *mut u8) -> Node {
        Node {
            next: NODE_NULL,
            prev: NODE_NULL,
            val
        }
    }
    pub unsafe fn destroy_right(&mut self, t: Type) {
        if self.next != NODE_NULL {
            (*self.next).destroy_right(t);
            dealloc(self.next as *mut u8, NODE_LAYOUT);
        }
        dealloc(self.val, get_layout(&t));
    }
}

pub struct List {
    vtype: Type,
    start: *mut Node, 
    end: *mut Node,
    current: *mut Node,
    current_id: usize,
    size: usize,
}

impl List {
    pub fn new(vtype: Type) -> Self {
        return List {
            vtype,
            start: NODE_NULL,
            end: NODE_NULL,
            current: NODE_NULL,
            current_id: 0,
            size: 0,
        }    
    }
    
    unsafe fn advance_left(&mut self) {
        self.current_id -= 1;
        self.current = (*self.current).next;
    }
    
    unsafe fn advance_right(&mut self) {
        self.current_id += 1;
        self.current = (*self.current).next;
    }
    
    unsafe fn unwrap_current(&mut self) -> *mut u8 {
        return (*self.current).val;
    }
    
    unsafe fn move_from_start(&mut self, offset: i64) {
        self.current = self.end;
        
        for _ in 0..offset {
            self.advance_right();
        }
    }
    
    unsafe fn move_from_curr(&mut self, offset: i64) {
        let left = offset < 0;
        
        if left {
            for _ in 0..offset {
                self.advance_left();
            }
            return;
        }
        for _ in 0..offset {
            self.advance_right();
        }
    }
    
    unsafe fn move_from_end(&mut self, offset: i64) {
        self.current = self.start;
        
        for _ in 0..offset {
            self.advance_left();
        }
    }
    
    pub unsafe fn push<T>(&mut self, value: T) -> Result<(), ErrorType>{
        let expected_layout = get_layout(&self.vtype);
        let real_layout = Layout::new::<T>();
        
        if expected_layout != real_layout {
            return Err(ErrorType::WrongType);
        }
        
        let value_ptr = alloc(expected_layout);
        *(value_ptr as *mut T) = value;
        
        let new_node = alloc(NODE_LAYOUT) as *mut Node;
        *new_node = Node::new(value_ptr);
        
        if self.start == NODE_NULL {
            self.start = new_node;
            self.current = self.start;
            self.end = self.current;
            
            return Ok(());
        }
        
        (*self.end).next = new_node;
        self.end = (*self.end).next;
        
        return Ok(());
    }
    
    pub unsafe fn get(&mut self, id: usize) -> *mut u8 {
        let dstart = (id) as i64; // distance from tail
        let dcurr = (id - self.current_id) as i64; // distance from current
        let dend = (self.size - id) as i64; // distance from left
        
        if dcurr.abs() > dend && dcurr > dstart {
            self.move_from_curr(dcurr);
            return self.unwrap_current();
        }
        
        if dstart > dend && dstart > dcurr {
            self.move_from_start(dstart);
            return self.unwrap_current();
        }
        
        else {
            self.move_from_end(dend);
            return self.unwrap_current();
        }
    }
    
    pub unsafe fn destroy(&mut self) {
        (*self.start).destroy_right(self.vtype);
        dealloc(self.start as *mut u8, NODE_LAYOUT);
    }
}