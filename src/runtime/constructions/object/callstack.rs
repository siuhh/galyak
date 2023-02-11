use crate::pre::lexer::Lexer;

use super::{Expect, Object, ObjectType};

pub struct CallStackExpectation {}
impl Expect for CallStackExpectation {
    fn get_type(&self) -> ObjectType {
        ObjectType::CallStack
    }

    fn read(&self, lexer: &Lexer) -> Box<dyn Object> {
        todo!()
    }
}
pub struct CallStackObject {}
impl Object for CallStackObject {
    fn get_type(&self) -> ObjectType {
        ObjectType::CallStack
    }
}
