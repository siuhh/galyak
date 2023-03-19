use std::{collections::LinkedList, io::{stdout, Write}};

use colored::Colorize;

use crate::{compiler::ast::{Ast}};

use super::{memory::{storage::{VarInfo}, types::{get_type, Type}}, interpreter::{Interpreter, TempValue}};

unsafe fn init_stack_res(call_stack: &LinkedList<Box<Ast>>) -> LinkedList<VarInfo> {
    let mut reserve = LinkedList::<VarInfo>::new();
    
    for cs in call_stack {
        if let Ast::Statement { line: _, statement } = &**cs {
            match &**statement {
                Ast::DeclareVariable { array: _, name, vtype, value: _ } => {
                    reserve.push_back(VarInfo { vtype: get_type(&vtype), name: name.clone() })
                }
                Ast::Function { name, args: _, return_type: _, compound_statement: _ } => {
                    reserve.push_back(VarInfo { vtype: Type::Func, name: name.clone() })
                }
                _ => (),
            } 
        }
    }
    
    return reserve;
}

pub struct GlkFuncDeclaration {
    pub call_stack: LinkedList<Box<Ast>>,
    pub stack_reservation: LinkedList<VarInfo>,
    pub args: LinkedList<VarInfo>,
    pub return_type: Type,
    pub name: String,
}

impl GlkFuncDeclaration {
    pub unsafe fn new(
        call_stack: LinkedList<Box<Ast>>, 
        args: LinkedList<VarInfo>, 
        return_type: Type,
        name: String
    ) -> Self {
        let mut stack_reservation = init_stack_res(&call_stack);
        
        //добавити на початок стеку аргументи
        for arg in &args {
            stack_reservation.push_back(arg.clone());
        }
        
        if return_type != Type::Null {
            //тут зберігається ретурн функції
            let result_value = VarInfo { name: "#".to_string(), vtype: return_type };
            stack_reservation.push_back(result_value);
        }
        
        return GlkFuncDeclaration { call_stack, stack_reservation, args, return_type, name };
    }
}

impl<'a> Interpreter<'a> {
    unsafe fn kf_print(&mut self, passed_args: LinkedList<Box<Ast>>) {
        let mut color: char = 'б';
        for arg in passed_args {
            let expr = *arg;
            
            let value = self.auto(expr);
            
            let mut prnt: String = String::new(); 
            
            match value {
                TempValue::Number(num) => {
                    prnt = num.to_string();
                },
                TempValue::String(str) => {
                    if str.starts_with(":?") {
                        color = str.chars().nth(2).unwrap();
                    }
                    else {
                        prnt = str;
                    }
                },
                _ => todo!()
            };
            
            match color {
                'б' => print!("{}", prnt),
                'ч' => print!("{}", prnt.red()),
                'з' => print!("{}", prnt.green()),
                'ж' => print!("{}", prnt.yellow()),
                'с' => print!("{}", prnt.blue()),
                _ => todo!()
            }
        }
    }
    
    unsafe fn kf_println(&mut self, passed_args: LinkedList<Box<Ast>>) {
        self.kf_print(passed_args);
        println!();
    }
    
    unsafe fn kf_input(&mut self) -> String {
        let mut line = String::new();
        let stdin = std::io::stdin();
        stdin.read_line(&mut line).unwrap();
        
        return line.replace("\r\n", "").replace("\n", "");
    }
    
    unsafe fn as_num(&mut self, expr: Box<Ast>) -> f64 {
        return self.string(*expr).parse::<f64>().unwrap();
    }
    //виконує функції прописані вище, якшо така функція не прописана вертає нулл
    pub unsafe fn kf(&mut self, fn_name: &String, passed_args: LinkedList<Box<Ast>>) 
    -> Option<TempValue> {
        match fn_name.as_str() {
            "базар" => {
                self.kf_print(passed_args);
                return Some(TempValue::Null);
            },
            "базарлн" => {
                self.kf_println(passed_args);
                return Some(TempValue::Null);
            },
            "шоти" => {
                self.kf_print(passed_args);
                
                stdout().flush().unwrap();
                
                let val = self.kf_input();
                
                return Some(TempValue::String(val));
            },
            "тіпацифри" => {
                let val = self.as_num(passed_args.into_iter().next().unwrap());
                return Some(TempValue::Number(val));
            },
            _ => return None
        }
    }
}