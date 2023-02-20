use std::collections::LinkedList;

use crate::error_mgr::ErrorCaller;

use super::parser::AST;

pub struct Interpreter<'a> {
    pub error_caller: &'a ErrorCaller,
    pub call_stack: LinkedList<AST>,
}
impl<'a> Interpreter<'a> {
    pub fn new(error_caller: &ErrorCaller, call_stack: LinkedList<AST>) -> Interpreter {
        return Interpreter { error_caller, call_stack }
    }
    fn var_num(&self) -> f64 {
        return 22.0;
    }
    fn var_str(&self) -> String {
        return "1488".to_string();
    }

    fn ariph(&self, bin: AST) -> f64 {
        match bin {
            AST::Num { value, token } => value,
            AST::AriphExpression { left, op, right } => match op.value.as_str() {
                "+" => self.ariph(*left) + self.ariph(*right),
                "-" => self.ariph(*left) - self.ariph(*right),
                "*" => self.ariph(*left) * self.ariph(*right),
                "/" => self.ariph(*left) / self.ariph(*right),
                _ => panic!(),
            },
            AST::Var { value, token } => self.var_num(),
            _ => panic!(),
        }
    }
    fn string(&self, str: AST) -> String {
        match str {
            AST::String { value, token } => value,
            AST::AriphExpression { left, op, right } => match op.value.as_str() {
                "+" => self.string(*left) + self.string(*right).as_str(),
                _ => {
                    let AST::String { token, value } = *left;
                    self.error_caller.unallowed_operation(&token, "букви");
                    panic!();
                }
            },
            AST::Var { value, token } => self.var_str(),
            _ => panic!(),
        }
    }
    fn dec_var(&self, str: AST) {
        match str {
            AST::DecVar {
                array,
                name,
                _type,
                value,
            } => {
                println!(
                "name: {}, _type: {}, value: {}, array: {}",
                
                if let AST::Var { value, token } = *name { value } else {panic!()},
                if let AST::Var { value, token } = *_type { value } else {panic!()},
                
                self.ariph(*value),
                array
            )},
            _ => panic!(),
        };
    }

    pub fn run(&self) {}
}
