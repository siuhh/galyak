use crate::pre::lexer::Lexer;

mod CallStack;
pub mod args;
pub mod callstack;
pub mod token;
pub mod value;

#[derive(PartialEq)]
pub enum ObjectType {
    Value,
    CallStack,
    Args,
    Token,
    Construction,
}
pub trait Expect {
    fn get_type(&self) -> ObjectType;
    fn read(&self, lexer: &Lexer) -> Vec<Box<dyn Object>>;
}
pub trait Object {
    fn get_type(&self) -> ObjectType;
}
pub struct Expectation {
    pub opt: bool,
    pub exp: &'static dyn Expect,
}
