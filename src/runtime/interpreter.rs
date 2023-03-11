use core::panic;
use std::{collections::LinkedList, ptr::{null, write}, alloc::{alloc, dealloc}};

use crate::{
    compiler::ast::{deref_ast, Ast},
    program::error_mgr::ErrorCaller, runtime::{memory::var::var_fn},
};

use super::{
    func::GlkFuncDeclaration,
    memory::{
        storage::{GlkStack, VarInfo},
        types::{get_type, Type, FLOAT_LAYOUT, STRING_LAYOUT, get_layout}, var::{var_num, var_str}, errors::err_wrong_type,
    },
};

pub struct Interpreter<'a> {
    call_stack: LinkedList<Box<Ast>>,
    mem_stack: *mut GlkStack,
    error_caller: &'a ErrorCaller,
    return_type: Type,
    curr_line: usize,
}

unsafe fn is_null(ptr: *mut u8) -> bool {
    return ptr == null::<u8>() as *mut u8;
}

impl<'a> Interpreter<'a> {
    pub unsafe fn new(func: *mut GlkFuncDeclaration, error_caller: &'a ErrorCaller) -> Self {
        let mem_stack = 
            if (*func).stack_reservation.len() != 0 {
                GlkStack::alloc(&(*func).stack_reservation)
            }
            else {
                null::<GlkStack>() as *mut GlkStack
            };
            
        
        return Interpreter {
            call_stack: (*func).call_stack.clone(),
            mem_stack,
            error_caller,
            return_type: (*func).return_type,
            curr_line: 0,
        };
    }
    
    unsafe fn end_with_error(&mut self, message: String) {
        self.end();
        self.error_caller.runt_error(message, self.curr_line);
        panic!();
    }

    unsafe fn unwrap<T>(&mut self, res: Result<T, String>) -> T {
        match res {
            Ok(value) => value,
            Err(message) => {
                self.end_with_error(message);
                panic!();
            }
        }
    }
    
    pub unsafe fn auto(&mut self, expr: Ast) -> (*mut u8, Type) {  
        let left = {
            let mut res = expr.clone();
            
            while let Ast::Expression { left, op: _, right: _ } = res {
                res = deref_ast(&left)
            }
            
            res
        };
        
        match left {
            Ast::Num(_) => {
                let value = self.num(expr);
                let value_ptr = alloc(FLOAT_LAYOUT) as *mut f64;
                
                write(value_ptr, value);
                
                return (value_ptr as *mut u8, Type::Number);
            }
            Ast::String(_) => {
                let value = self.string(expr);
                let value_ptr = alloc(STRING_LAYOUT) as *mut String;
                
                write(value_ptr, value);
                
                return (value_ptr as *mut u8, Type::String);
            }
            Ast::Keyword(name) => {
                let var = {
                    let res = (*self.mem_stack).get_dynamicaly(&name, true);
                    self.unwrap(res)
                };
                
                match var.1 {
                    Type::String => {
                        let value = self.string(expr);
                        let value_ptr = alloc(STRING_LAYOUT) as *mut String;
                        
                        write(value_ptr, value);
                        
                        return (value_ptr as *mut u8, Type::String);
                    },
                    Type::Number => {
                        let value = self.num(expr);
                        let value_ptr = alloc(FLOAT_LAYOUT) as *mut f64;
                        
                        write(value_ptr, value);
                        
                        return (value_ptr as *mut u8, Type::Number);
                    }
                    _ => todo!(),
                }
            }
            Ast::CallFunc { name, args } => {
                let declared_func = self.unwrap(var_fn(self.mem_stack, &name));
                
                match (*declared_func).return_type {
                    Type::String => {
                        let val = self.call_func::<String>(name, args, Type::String).unwrap();
                        let value_ptr = alloc(STRING_LAYOUT) as *mut String;
                        
                        write(value_ptr, val);
                        
                        return (value_ptr as *mut u8, Type::String);
                    },
                    Type::Number => {
                        let val = self.call_func::<f64>(name, args, Type::Number).unwrap();
                        let value_ptr = alloc(FLOAT_LAYOUT) as *mut f64;
                        
                        write(value_ptr, val);
                        
                        return (value_ptr as *mut u8, Type::Number);
                    }
                    _ => todo!(),
                }
            }
            _ => {
                println!("not implemented for type:");
                dbg!(expr);
                panic!();
            }
        }
    }

    pub unsafe fn num(&mut self, bin: Ast) -> f64 {
        match bin {
            Ast::Num(value) => value,
            Ast::Expression { left, op, right } => match op.value.as_str() {
                "+" => self.num(*left) + self.num(*right),
                "-" => self.num(*left) - self.num(*right),
                "*" => self.num(*left) * self.num(*right),
                "/" => self.num(*left) / self.num(*right),
                _ => panic!(),
            },
            Ast::Keyword(value) => self.unwrap(var_num(self.mem_stack, &value)),
            Ast::CallFunc { name, args } => self.call_func::<f64>(name, args, Type::Number).unwrap(),
            _ => panic!(),//TODO! тут нори помилку викинути
        }
    }

    pub unsafe fn string(&mut self, str: Ast) -> String {
        match str {
            Ast::String(value) => value,
            Ast::Expression { left, op, right } => match op.value.as_str() {
                "+" => self.string(*left) + self.string(*right).as_str(),
                _ => {
                    panic!();
                }
            },
            Ast::Keyword(name) => self.unwrap(var_str(self.mem_stack, &name)),
            Ast::CallFunc { name, args } => self.call_func::<String>(name, args, Type::String).unwrap(),
            _ => panic!(),//TODO! тут нори помилку викинути
        }
    }

    unsafe fn declare_variable(&mut self, _array: bool, vtype: &String, name: &String, value: Box<Ast>) {
        let vtype = get_type(vtype);
        let res = (*self.mem_stack).get_typed(name, &vtype, false);
        let var_ptr = self.unwrap(res);
        
        match vtype {
            Type::Number => {
                let val = self.num(deref_ast(&value));
                write(var_ptr as *mut f64, val);
            }
            Type::String =>  {
                let val = self.string(deref_ast(&value));
                write(var_ptr as *mut String, val);
            }
            //TODO! add other types
            _ => panic!(),
        }
    }

    unsafe fn set_variable(&mut self, name: &String, value: Box<Ast>) {
        let res = (*self.mem_stack).get_dynamicaly(name, true);
        let (var_ptr, vtype) = self.unwrap(res);

        match vtype {
            Type::Number => {
                let val = self.num(deref_ast(&value));
                write(var_ptr as *mut f64, val);
            }
            Type::String => {
                let val = self.string(deref_ast(&value));
                write(var_ptr as *mut String, val);
            }
            _ => panic!(),
        }
    }
    
    unsafe fn declare_function(&mut self, 
        name: String, 
        args: LinkedList<(String, String)>, 
        return_type: String, 
        compound_statement: LinkedList<Box<Ast>>
    ){
        
        let ptr = {
            let res = (*self.mem_stack).get_typed(&name, &Type::Func, false);
            self.unwrap(res)
        };
        
        let mut parsed_args = LinkedList::<VarInfo>::new();
        
        for arg in args {
            parsed_args.push_back(VarInfo { vtype: get_type(&arg.0), name: arg.1 });
        }
        
        let func = GlkFuncDeclaration::new(compound_statement, parsed_args, get_type(&return_type));
        
        write(ptr as *mut GlkFuncDeclaration, func);
        
    }
    
    unsafe fn call_func<T: Clone> (
        &mut self, 
        name: String, 
        passed_args: LinkedList<Box<Ast>>, 
        expected_return: Type
    ) -> Option<T> {
        if let Some((pointer, vtype)) = self.kf(&name, passed_args.clone()) {
            if expected_return != Type::Null && expected_return == vtype {
                let value = (*(pointer as *mut T)).clone();
                dealloc(pointer, get_layout(&vtype));
                return Some(value);
            }
            
            if vtype != Type::Null {
                dealloc(pointer, get_layout(&vtype));
            }
            if expected_return == Type::Null {
                return None;
            }
            
            self.end_with_error(
                err_wrong_type(&name, &expected_return, &vtype)
            );
            panic!();
        }  
        let declared_func = self.unwrap(var_fn(self.mem_stack, &name));
        
        let mut args_iter = passed_args.iter();
        
        let mut func_interpreter = Interpreter::new(declared_func, self.error_caller);
        let intpr_mem = func_interpreter.mem_stack;
        
        //TODO! перевіряти кількість аргів
        //move arguments values to call stack
        for arg in &(*declared_func).args {
            match arg.vtype {
                Type::Number => {
                    let ptr = self.unwrap((*intpr_mem).get_typed(&arg.name, &arg.vtype, false));
                    *(ptr as *mut f64) = self.num(deref_ast(&args_iter.next().unwrap()));
                }
                Type::String => {
                    let ptr = self.unwrap((*intpr_mem).get_typed(&arg.name, &arg.vtype, false));
                    *(ptr as *mut String) = self.string(deref_ast(&args_iter.next().unwrap()));
                }
                _ => todo!() //TODO!
            }
        }
        
        func_interpreter.run();
        
        if expected_return != Type::Null {
            let val = { 
                let res = (*func_interpreter.mem_stack).get_typed(&"#".to_string(), &expected_return, true);
                (*(self.unwrap(res) as *mut T)).clone()
            };
            
            func_interpreter.end();
        
            return Some(val);   
        }
        
        func_interpreter.end();
        
        return None;
    }


    unsafe fn retrn(&mut self, expression: Box<Ast>) {
        //TODO! викидати норм помилку якшо ретурн функції це нулл і вона шось ретурнить
        let ptr = { 
            let res = (*self.mem_stack).get_typed(&"#".to_string(), &self.return_type, false);
            self.unwrap(res)
        };
        match self.return_type {
            Type::Number => {
                write(ptr as *mut f64, self.num(deref_ast(&expression)));
            },
            Type::String => {
                write(ptr as *mut String, self.string(deref_ast(&expression)));
            },
            _ => todo!(),
        }
    }

    pub unsafe fn end(&mut self) {
        (*self.mem_stack).nahuy();
    }

    pub unsafe fn run(&mut self) {
        for cs in self.call_stack.clone() {
            let ast = deref_ast(&cs);
            
            if let Ast::Statement { line, statement } = ast {
                let stat = deref_ast(&statement);
                
                self.curr_line = line;
                
                match stat {
                    Ast::CallFunc { name, args } => {
                        self.call_func::<u8>(name, args, Type::Null);
                    }
                    Ast::Return { expression } => {
                        self.retrn(expression);
                    }
                    Ast::Function { name, args, return_type, compound_statement } => {
                        self.declare_function(name, args, return_type, compound_statement);
                    }
                    Ast::DeclareVariable { array, name, vtype, value } => {
                        self.declare_variable(array, &vtype, &name, value);
                    }
                    Ast::SetVariable { name, value } => {
                        self.set_variable(&name, value);
                    }
                    _ => panic!(),
                }  
            }
        }
    }
}
