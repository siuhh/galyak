use std::collections::LinkedList;

use crate::{compiler::parser::Ast};

pub struct Interpreter {
    pub call_stack: LinkedList<Ast>,
}
impl Interpreter {
    pub fn new(call_stack: LinkedList<Ast>) -> Interpreter {
        return Interpreter { call_stack }
    }
    fn var_num(&self) -> f64 {
        return 22.0;
    }
    fn var_str(&self) -> String {
        return "1488".to_string();
    }

    fn expression(&self, bin: Ast /*Expression*/) -> f64 {
        match bin {
            Ast::Num(value) => value,
            Ast::Expression { left, op, right } => match op.value.as_str() {
                "+" => self.expression(*left) + self.expression(*right),
                "-" => self.expression(*left) - self.expression(*right),
                "*" => self.expression(*left) * self.expression(*right),
                "/" => self.expression(*left) / self.expression(*right),
                _ => panic!(),
            },
            Ast::Keyword(_) => self.var_num(),
            _ => panic!(),
        }
    }
    fn string(&self, str: Ast /*Expression*/) -> String {
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
    fn dec_var(&self, str: Ast) {
        match str {
            Ast::DeclareVariable {
                array,
                name,
                vtype,
                value,
            } => {
                println!(
                "name: {}, vtype: {}, value: {}, array: {}",
                name,
                vtype,
                self.expression(*value),
                array
            )},
            _ => panic!(),
        };
    }

    pub fn run(&self) {
        
    }
}
