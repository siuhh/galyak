use core::panic;
use std::{collections::LinkedList, ptr::{null, write}, alloc::{alloc, dealloc, Layout}};

use crate::{
    compiler::{ast::Ast, token::{tokens::stat::{EQUALS, MORE, LESS, NOT_EQUALS}, TokenType}},
    program::error_mgr::ErrorCaller, runtime::{memory::var::var_fn},
    program::errors::runtime::*
};

use super::{
    func::GlkFuncDeclaration,
    memory::{
        storage::{GlkStack, VarInfo},
        types::{get_type, Type, get_layout}, var::{var_num, var_str}
    },
};

pub struct Interpreter<'a> {
    call_stack: LinkedList<Box<Ast>>,
    mem_stack: *mut GlkStack,
    error_caller: &'a ErrorCaller,
    return_type: Type,
    curr_line: usize,
    name: String
}

unsafe fn is_null(ptr: *mut u8) -> bool {
    return ptr == null::<u8>() as *mut u8;
}

impl<'a> Interpreter<'a> {
    pub unsafe fn new(func: *mut GlkFuncDeclaration, error_caller: &'a ErrorCaller, parent: *mut GlkStack) -> Self {
        let mem_stack = GlkStack::alloc(&(*func).stack_reservation, parent);
            
        
        return Interpreter {
            call_stack: (*func).call_stack.clone(),
            mem_stack,
            error_caller,
            return_type: (*func).return_type,
            curr_line: 0,
            name: (*func).name.clone()
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
    
    
    unsafe fn write_in_heap<T>(&mut self, value: T) -> *mut u8 {
        
        let ptr = alloc(Layout::new::<T>()) as *mut T;
        write(ptr, value);
        
        return ptr as *mut u8;
    }
    
    unsafe fn auto_string_or_num(&mut self, expr: Ast) -> (*mut u8, Type) {
        let left = {
            let mut res = expr.clone();
            
            while let Ast::Expression { left, op: _, right: _ } = res {
                res = *left
            }
            
            res
        };
        
        let vtype = match left {
            Ast::Num(_) => Type::Number,
            Ast::String(_) => Type::String,
            Ast::Keyword(name) => {
                let var = {
                    let res = (*self.mem_stack).get_dynamicaly(&name, true);
                    self.unwrap(res)
                };
                var.1
            }
            Ast::CallFunc { name, args: _ } => {
                let declared_func = self.unwrap(var_fn(self.mem_stack, &name));
                (*declared_func).return_type
            }
            _ => {
                println!("not implemented for type:");
                dbg!(expr);
                panic!();
            }
        };
        match vtype {
            Type::Number => {
                let val = self.num(expr);
                (self.write_in_heap(val), Type::Number)
            },
            Type::String => {
                let val = self.string(expr);
                (self.write_in_heap(val), Type::String)
            },
            _ => panic!()
        }
    }
    
    
    pub unsafe fn auto(&mut self, expr: Ast) -> (*mut u8, Type) {
        match &expr {
            Ast::Expression { left: _, op, right: _ } => {
                return match op.value.as_str() {
                    "+" => self.auto_string_or_num(expr),
                    "-" | "*" | "/" => {
                        let val = self.num(expr);
                        (self.write_in_heap(val), Type::Number)
                    },
                    //EQUALS => (self.write_in_heap(self.bool(expr)), Type::Bool),
                    //NOT_EQUALS => (self.write_in_heap(self.bool(expr)), Type::Bool),
                    //MORE => (self.write_in_heap(self.bool(expr)), Type::Bool),
                    //LESS => (self.write_in_heap(self.bool(expr)), Type::Bool),
                    _ => {
                        println!("not implemented for type:");
                        dbg!(expr);
                        panic!();
                    }
                }
            },
            Ast::Num(num) => {
                return (self.write_in_heap(*num), Type::Number);
            },
            Ast::String(str) => {
                return (self.write_in_heap(str.clone()), Type::String);
            },
            Ast::Keyword(name) => {
                let var = {
                    let res = (*self.mem_stack).get_dynamicaly(&name, true);
                    self.unwrap(res)
                };
                
                match var.1 {
                    Type::Number => {
                        let val = self.num(expr);
                        (self.write_in_heap(val), Type::Number)
                    },
                    Type::String => {
                        let val = self.string(expr);
                        (self.write_in_heap(val), Type::String)
                    },
                    _ => panic!()
                }
            },
            Ast::CallFunc { name, args } => {
                if let Some((pointer, vtype)) = self.kf(&name, args.clone()) {
                    return (pointer, vtype);
                }  
                
                let declared_func = self.unwrap(var_fn(self.mem_stack, &name));
                
                match (*declared_func).return_type {
                    Type::Number => {
                        let val = self.call_func::<f64>((*name).clone(), (*args).clone(), Type::Number).unwrap();
                        (self.write_in_heap(val), Type::Number)
                    },
                    Type::String => {
                        let val = self.call_func::<String>((*name).clone(), (*args).clone(), Type::String).unwrap();
                        (self.write_in_heap(val), Type::String)
                    },
                    _ => panic!()
                }
            }
            _ => {
                println!("not implemented for type:");
                dbg!(expr);
                panic!();
            }
        }
    }
    
    pub unsafe fn bool(&mut self, expr: Ast) -> bool {
        if let Ast::Expression { left, op, right } = &expr {
            let left_au = self.auto((**left).clone());
            let right_au = self.auto((**right).clone());
            
            if left_au.1 != right_au.1 {
                dealloc(left_au.0, get_layout(&left_au.1));
                dealloc(right_au.0, get_layout(&right_au.1));
                
                return false;
            }
            
            let comparsion_result: TokenType;
            
            match left_au.1 {
                Type::String => {
                    let val_left = (*(left_au.0 as *mut String)).clone();
                    let val_right = (*(right_au.0 as *mut String)).clone();
                    
                    if val_left == val_right {
                        comparsion_result = EQUALS;
                    }
                    else if val_left > val_right {
                        comparsion_result = MORE;
                    }
                    else if val_left < val_right {
                        comparsion_result = LESS;
                    }
                    else {
                        comparsion_result = NOT_EQUALS;
                    }
                }
                Type::Number => {
                    let val_left = *(left_au.0 as *mut f64);
                    let val_right = *(right_au.0 as *mut f64);
                    
                    if val_left == val_right {
                        comparsion_result = EQUALS;
                    }
                    else if val_left > val_right {
                        comparsion_result = MORE;
                    }
                    else if val_left < val_right {
                        comparsion_result = LESS;
                    }
                    else {
                        comparsion_result = NOT_EQUALS;
                    }
                }
                _ => panic!()
            }
            
            dealloc(left_au.0, get_layout(&left_au.1));
            dealloc(right_au.0, get_layout(&right_au.1));
            
            return op.value == comparsion_result;
        }
        panic!();
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
            _ => {
                self.end_with_error(type_expected(&Type::Number));
                panic!();
            },
        }
    }

    pub unsafe fn string(&mut self, str: Ast) -> String {
        match str.clone() {
            Ast::String(value) => value,
            Ast::Expression { left, op, right } => match op.value.as_str() {
                "+" => self.string(*left) + self.string(*right).as_str(),
                _ => self.num(str).to_string()
            },
            Ast::Num(_) => {
                let n = self.num(str);
                n.to_string()
            },
            Ast::Keyword(name) => {
                let var = {
                    let res = (*self.mem_stack).get_dynamicaly(&name, true);
                    self.unwrap(res)
                };
                
                let string = match var.1 {
                    Type::String => {
                        self.unwrap(var_str(self.mem_stack, &name))
                    },
                    Type::Number => {
                        self.num(str).to_string()
                    }
                    _ => todo!(),
                };
                
                return string;
            },
            Ast::CallFunc { name, args } => self.call_func::<String>(name, args, Type::String).unwrap(),
            _ =>  {
                self.end_with_error(type_expected(&Type::String));
                panic!();
            },
        }
    }

    unsafe fn declare_variable(&mut self, _array: bool, vtype: &String, name: &String, value: Box<Ast>) {
        let vtype = get_type(vtype);
        let res = (*self.mem_stack).get_typed(name, &vtype, false);
        let var_ptr = self.unwrap(res);
        
        if let Ast::Nothing = *value {
            return;
        }
        
        match vtype {
            Type::Number => {
                let val = self.num(*value);
                write(var_ptr as *mut f64, val);
            }
            Type::String =>  {
                let val = self.string(*value);
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
                let val = self.num(*value);
                write(var_ptr as *mut f64, val);
            }
            Type::String => {
                let val = self.string(*value);
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
        
        let func = GlkFuncDeclaration::new(compound_statement, parsed_args, get_type(&return_type), name);
        
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
                wrong_type(&name, &expected_return, &vtype)
            );
            panic!();
        }  
        let declared_func = self.unwrap(var_fn(self.mem_stack, &name));
        
        if (*declared_func).return_type != expected_return {
            self.end_with_error(wrong_type(&name, &expected_return, &(*declared_func).return_type))
        }
        
        let mut args_iter = passed_args.iter();
        
        let mut func_interpreter = Interpreter::new(declared_func, self.error_caller, self.mem_stack);
        let intpr_mem = func_interpreter.mem_stack;
        
        if passed_args.len() != (*declared_func).args.len() {
            self.end_with_error(
                wrong_arguments_count(
                    &name, 
                    (*declared_func).args.len(),
                    passed_args.len(), 
                )
            );
        }
        
        //move arguments values to function memory
        for arg in &(*declared_func).args {
            match arg.vtype {
                Type::Number => {
                    let ptr = self.unwrap((*intpr_mem).get_typed(&arg.name, &arg.vtype, false));
                    let val = self.num((**(args_iter.next().unwrap())).clone());
                    write(ptr as *mut f64,val);
                }
                Type::String => {
                    let ptr = self.unwrap((*intpr_mem).get_typed(&arg.name, &arg.vtype, false));
                    let val = self.string((**(args_iter.next().unwrap())).clone());
                    write(ptr as *mut String, val);
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
        let ptr = { 
            let res = (*self.mem_stack).get_typed(&"#".to_string(), &self.return_type, false);
            if let Err(_) = res {
                self.end_with_error(wrong_return_type(&self.name, &self.return_type));
            }
            res.unwrap()
        };
        match self.return_type {
            Type::Number => {
                self.call_stack.clear();
                write(ptr as *mut f64, self.num(*expression));
            },
            Type::String => {
                self.call_stack.clear();
                write(ptr as *mut String, self.string(*expression));
            },
            _ => todo!(),
        }
    }
    
    unsafe fn ifst(
        &mut self,
        condition: Box<Ast>, 
        compound_statement: LinkedList<Box<Ast>>, 
        else_statement: Option<LinkedList<Box<Ast>>>
    ) {
        let ok = self.bool(*condition);
        
        let mut anon_func: GlkFuncDeclaration;
        if ok {
            anon_func = GlkFuncDeclaration::new(
                compound_statement, 
                LinkedList::new(), 
                Type::Null, 
                "~anonif~".to_string()
            );
        }
        else {
            if let Some(stat) = else_statement {
                anon_func = GlkFuncDeclaration::new(
                    stat, 
                    LinkedList::new(), 
                    Type::Null, 
                    "~if~".to_string()
                );
            }
            else {
                return;
            }
        }
        let mut if_interpreter = Interpreter::new(&mut anon_func, self.error_caller, self.mem_stack);
        
        if_interpreter.run();
        if_interpreter.end();
    }

    pub unsafe fn end(&mut self) {
        if !self.mem_stack.is_null() {
            (*self.mem_stack).nahuy();
        }
    }

    pub unsafe fn run(&mut self) {
        for cs in self.call_stack.clone() {
            let ast = *cs;
            
            if let Ast::Statement { line, statement } = ast {
                let stat = *statement;
                
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
                    Ast::If { condition, compound_statement, else_statement } => {
                        self.ifst(condition, compound_statement, else_statement)
                    }
                    _ => panic!(),
                }  
            }
        }
    }
}
