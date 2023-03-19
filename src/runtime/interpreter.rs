use core::panic;
use std::{collections::LinkedList, ptr::{null, write}};

use crate::{
    compiler::{ast::Ast, token::{tokens::stat::{EQUALS, MORE, LESS, NOT_EQUALS}, TokenType}},
    program::error_mgr::ErrorCaller, runtime::{memory::{var::var_fn, types::get_value_type}},
    program::errors::runtime::*
};

use super::{
    func::GlkFuncDeclaration,
    memory::{
        storage::{GlkStack, VarInfo},
        types::{get_type, Type}, var::{var_num, var_str}
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
#[derive(Debug)]
pub enum InterpreterResult {
    Returned(TempValue), End, Break, Continue
}
#[derive(PartialEq, Clone, Debug)]
pub enum TempValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

impl<'a> Interpreter<'a> {
    pub unsafe fn new(
        func: *mut GlkFuncDeclaration, 
        error_caller: &'a ErrorCaller, 
        parent: *mut GlkStack) 
    -> Self {
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
    //приймає аст і вертає або строку або число
    unsafe fn auto_string_or_num(&mut self, expr: Ast) -> TempValue {
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
                let res = (*self.mem_stack).get_dynamicaly(&name, true);
                self.unwrap(res).1
            }
            Ast::CallFunc { name, args } => {
                if let Some(value) = self.kf(&name, args.clone()) {
                    return value;
                }  
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
                TempValue::Number(val)
            },
            Type::String => {
                let val = self.string(expr);
                TempValue::String(val)
            },
            _ => panic!()
        }
    }
    
    pub unsafe fn auto(&mut self, expr: Ast) -> TempValue {
        match &expr {
            Ast::Expression { left: _, op, right: _ } => {
                return match op.value.as_str() {
                    "+" => self.auto_string_or_num(expr),
                    "-" | "*" | "/" | "%" => {
                        let val = self.num(expr);
                        return TempValue::Number(val);
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
                return TempValue::Number(*num);
            },
            Ast::String(str) => {
                return TempValue::String((*str).clone());
            },
            Ast::Keyword(name) => {
                let var = {
                    let res = (*self.mem_stack).get_dynamicaly(&name, true);
                    self.unwrap(res)
                };
                
                match var.1 {
                    Type::Number => {
                        let val = self.num(expr);
                        return TempValue::Number(val);
                    },
                    Type::String => {
                        let val = self.string(expr);
                        return TempValue::String(val);
                    },
                    _ => panic!()
                }
            },
            Ast::CallFunc { name, args } => {
                if let Some(value) = self.kf(&name, args.clone()) {
                    return value;
                }  
                
                let declared_func = self.unwrap(var_fn(self.mem_stack, &name));
                
                match self.call_func(name.clone(), args.clone(), (*declared_func).return_type) {
                    Some(value) => return value,
                    None => return TempValue::Null,
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
            if op.name == MORE || op.name == LESS {
                let left_num = self.num((**left).clone());
                let right_num = self.num((**right).clone());
                
                if op.name == MORE {
                    return left_num > right_num;
                }
                return left_num < right_num;
            }
            let left_au = self.auto((**left).clone());
            let right_au = self.auto((**right).clone());
            
            let comparsion_result: TokenType;
            
            if left_au == right_au {
                comparsion_result = EQUALS;
            }
            else {
                comparsion_result = NOT_EQUALS;
            }
                   
            if op.value == NOT_EQUALS && comparsion_result != EQUALS {
                return true;
            }
            
            return op.value == comparsion_result;
        }
        else if let Ast::Bool(b) = &expr {
            return *b;
        }
        self.end_with_error(type_expected(&Type::Bool));
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
                "%" => self.num(*left) % self.num(*right),
                _ => panic!(),
            },
            Ast::Keyword(value) => self.unwrap(var_num(self.mem_stack, &value)),
            Ast::CallFunc { name, args } => { 
                if let TempValue::Number(num) = self.call_func(name, args, Type::Number).unwrap() {
                    return num;
                }
                else {
                    panic!()
                }
            },
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
            Ast::CallFunc { name, args } => { 
                if let TempValue::String(str) = self.call_func(name, args, Type::String).unwrap() {
                    return str
                }
                else {
                    panic!();
                }
                
                },
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
    
    unsafe fn call_func(
        &mut self, 
        name: String, 
        passed_args: LinkedList<Box<Ast>>, 
        expected_return: Type
    ) -> Option<TempValue> {
        if let Some(value) = self.kf(&name, passed_args.clone()) {
            let returned_type = get_value_type(&value);
            if expected_return != Type::Null && expected_return == returned_type {
                return Some(value.clone());
            }
            
            if expected_return == Type::Null {
                return None;
            }
            
            self.end_with_error(
                wrong_type(&name, &expected_return, &returned_type)
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
        
        let result = func_interpreter.run();
        func_interpreter.end();
        
        if expected_return != Type::Null {
            if let InterpreterResult::Returned(val) = result {
                return Some(val);   
            }
            else {
                self.end_with_error(func_has_no_return(&name))
            }
        }
        return None;
    }
    
    unsafe fn ifst(
        &mut self,
        condition: Box<Ast>, 
        compound_statement: LinkedList<Box<Ast>>, 
        else_statement: Option<LinkedList<Box<Ast>>>
    ) -> InterpreterResult {
        let ok = self.bool(*condition);
        
        let mut anon_func: GlkFuncDeclaration;
        if ok {
            anon_func = GlkFuncDeclaration::new(
                compound_statement, 
                LinkedList::new(), 
                self.return_type, 
                "~if~".to_string()
            );
        }
        else {
            if let Some(stat) = else_statement {
                anon_func = GlkFuncDeclaration::new(
                    stat, 
                    LinkedList::new(), 
                    Type::Null, 
                    "~else~".to_string()
                );
            }
            else {
                return InterpreterResult::End;
            }
        }
        let mut if_interpreter = Interpreter::new(&mut anon_func, self.error_caller, self.mem_stack);
        
        let res = if_interpreter.run();
        
        if_interpreter.end();
        
        return res;
    }
    
    unsafe fn whilest(
        &mut self,
        condition: Box<Ast>, 
        compound_statement: LinkedList<Box<Ast>>
    ) -> InterpreterResult {
        let mut anon_func = GlkFuncDeclaration::new(
            compound_statement, 
            LinkedList::new(), 
            Type::Null, 
            "~while~".to_string()
        );
        
        let mut if_interpreter = Interpreter::new(&mut anon_func, self.error_caller, self.mem_stack);
        
        loop {
            let ok = self.bool((*condition).clone());
            
            if !ok {
                break;
            }
            
            let res = if_interpreter.run();
            if !if_interpreter.mem_stack.is_null() { 
                (*if_interpreter.mem_stack).uninit_all();
            }
            
            if let InterpreterResult::Break = res {
                break;
            }
            if let InterpreterResult::Returned(_) = &res {
                return res;
            }
        }
        
        if_interpreter.end();
        return InterpreterResult::End;
    }

    pub unsafe fn end(&mut self) {
        if !self.mem_stack.is_null() {
            (*self.mem_stack).nahuy();
        }
    }

    pub unsafe fn run(&mut self) -> InterpreterResult {
        for cs in self.call_stack.clone() {
            let ast = *cs;
            
            if let Ast::Statement { line, statement } = ast {
                let stat = *statement;
                
                self.curr_line = line;
                
                match stat {
                    Ast::CallFunc { name, args } => {
                        self.call_func(name, args, Type::Null);
                    }
                    Ast::Return { expression } => {
                        return InterpreterResult::Returned(self.auto(*expression));
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
                        let res = self.ifst(condition, compound_statement, else_statement);
                        match res {
                            InterpreterResult::End => (),
                            _ => return res,
                        }
                    }
                    Ast::While { condition, compound_statement } => {
                        let res = self.whilest(condition, compound_statement);
                        if let InterpreterResult::Returned(_) = &res {
                            return res;
                        }
                    }
                    Ast::Break => {
                        return InterpreterResult::Break;
                    }
                    Ast::Continue => {
                        return InterpreterResult::Continue;
                    }
                    _ => panic!(),
                }  
            }
        }
        return InterpreterResult::End;
    }
}