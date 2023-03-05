use std::{collections::LinkedList, ptr::null};

use crate::{compiler::ast::{Ast, deref_ast}};

use super::memory::{storage::{StackReservation, Stack}, types::{get_type, Type}};

pub unsafe fn init_stack(call_stack: &LinkedList<Box<Ast>>) -> *mut Stack {
    let mut reserve = LinkedList::<StackReservation>::new();
    
    for cs in call_stack {
        let ast = deref_ast(cs);
        if let Ast::DeclareVariable { array: _, name, vtype, value: _ } = ast {
            reserve.push_back(StackReservation { vtype: get_type(&vtype), name })
        }
    }
    
    return Stack::alloc(reserve);
}

pub struct Interpreter {
    call_stack: LinkedList<Box<Ast>>,
    mem_stack: *mut Stack,
}

unsafe fn is_null(ptr: *mut u8) -> bool {
    return ptr == null::<u8>() as *mut u8;
}

impl Interpreter {
    pub unsafe fn new(call_stack: LinkedList<Box<Ast>>) -> Interpreter {
        return Interpreter { call_stack, mem_stack: null::<Stack>() as *mut Stack}
    }
    
    unsafe fn var_num(&self, name: &String) -> f64 {
        let ptr = (*self.mem_stack).get_wt(name, &Type::Float);
        
        if is_null(ptr) {
            panic!();
        }
		
		return *(ptr as *mut f64);
    }
	
    fn var_str(&self) -> String {
        return "1488".to_string();
    }

    unsafe fn expression(&self, bin: Ast) -> f64 {
        match bin {
            Ast::Num(value) => value,
            Ast::Expression { left, op, right } => match op.value.as_str() {
                "+" => self.expression(*left) + self.expression(*right),
                "-" => self.expression(*left) - self.expression(*right),
                "*" => self.expression(*left) * self.expression(*right),
                "/" => self.expression(*left) / self.expression(*right),
                _ => panic!(),
            },
            Ast::Keyword(value) => self.var_num(&value),
            _ => panic!(),
        }
    }
    fn string(&self, str: Ast) -> String {
        match str {
            Ast::String(value) => value,
            Ast::Expression { left, op, right } => match op.value.as_str() {
                "+" => self.string(*left) + self.string(*right).as_str(),
                _ => {
                    panic!();
                }
            },
            Ast::Keyword(_) => self.var_str(),
            _ => panic!(),
        }
    }
    unsafe fn declare_variable(&self, vtype: &String, name: &String, value: Box<Ast>) {
		let vtype = get_type(vtype);
		let var_ptr = (*self.mem_stack).get_wt(name, &vtype);
		
		match vtype {
			Type::Float => {
				*(var_ptr as *mut f64) = self.expression(deref_ast(&value));
			}
			_ => panic!(),
		}
	}
	
	unsafe fn set_variable(&self, name: &String, value: Box<Ast>) {
		let (var_ptr, vtype) = (*self.mem_stack).get(name);
		
		match vtype {
			Type::Float => {
                let val = self.expression(deref_ast(&value));
				*(var_ptr as *mut f64) = val;
			}
			_ => panic!(),
		}
	}
    
	pub unsafe fn end(&mut self) {
		(*self.mem_stack).nahuy();
	}
    
    pub unsafe fn run(&mut self) {
		self.mem_stack = init_stack(&self.call_stack);
		
		for cs in &self.call_stack {
			let ast = deref_ast(cs);
			
			match ast {
				Ast::CallFunc { name: _, args } => {
					println!("{}", self.expression(deref_ast(args.front().unwrap())));
				},
				Ast::DeclareVariable { array: _, name, vtype, value } => {
					self.declare_variable(&vtype, &name, value);
				}
				Ast::SetVariable { name, value } => {
					self.set_variable(&name, value);
				}
				_ => panic!(),
			}
        }
		self.end();
        
    }
}
