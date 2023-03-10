use core::panic;
use std::{collections::LinkedList, ptr::{null, write}};

use crate::{
    compiler::ast::{deref_ast, Ast},
    program::error_mgr::ErrorCaller, runtime::{memory::var::var_fn},
};

use super::{
    func::GlkFuncDeclaration,
    memory::{
        storage::{GlkStack, VarInfo},
        types::{get_type, Type},
        var::*,
    },
};

pub struct Interpreter<'a> {
    call_stack: LinkedList<Box<Ast>>,
    mem_stack: *mut GlkStack,
    error_caller: &'a ErrorCaller,
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
            curr_line: 0,
        };
    }

    fn unwrap<T>(&mut self, res: Result<T, String>) -> T {
        match res {
            Ok(value) => value,
            Err(message) => {
                unsafe { self.end() };
                self.error_caller.runt_error(message, self.curr_line);
                panic!();
            }
        }
    }

    unsafe fn num(&mut self, bin: Ast) -> f64 {
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
            _ => panic!(),
        }
    }

    unsafe fn string(&self, str: Ast) -> String {
        match str {
            Ast::String(value) => value,
            Ast::Expression { left, op, right } => match op.value.as_str() {
                "+" => self.string(*left) + self.string(*right).as_str(),
                _ => {
                    panic!();
                }
            },
            Ast::Keyword(_name) => todo!(),//TODO!
            _ => panic!(),
        }
    }

    unsafe fn declare_variable(&mut self, _array: bool, vtype: &String, name: &String, value: Box<Ast>) {
        let vtype = get_type(vtype);
        let res = (*self.mem_stack).get_typed(name, &vtype);
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
        let res = (*self.mem_stack).get_dynamicaly(name);
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
            let res = (*self.mem_stack).get_typed(&name, &Type::Func);
            self.unwrap(res)
        };
        
        let mut parsed_args = LinkedList::<VarInfo>::new();
        
        for arg in args {
            parsed_args.push_back(VarInfo { vtype: get_type(&arg.0), name: arg.1 });
        }
        
        let func = GlkFuncDeclaration::new(compound_statement, parsed_args, get_type(&return_type));
        
        write(ptr as *mut GlkFuncDeclaration, func);
        
    }
    
    pub unsafe fn call_func(&mut self, name: String, passed_args: LinkedList<Box<Ast>>) {
        if name == "базар" {
            for arg in &passed_args {
                println!("{}", self.num(deref_ast(arg)));
            }
            return;
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
                    let ptr = self.unwrap((*intpr_mem).get_typed(&arg.name, &arg.vtype));
                    *(ptr as *mut f64) = self.num(deref_ast(&args_iter.next().unwrap()));
                }
                Type::String => {
                    let ptr = self.unwrap((*intpr_mem).get_typed(&arg.name, &arg.vtype));
                    *(ptr as *mut String) = self.string(deref_ast(&args_iter.next().unwrap()));
                }
                _ => todo!() //TODO!
            }
        }
        
        func_interpreter.run();
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
                        self.call_func(name, args);
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
