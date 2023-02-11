use super::{Expect, Object, ObjectType};
use crate::{pre::lexer::Lexer, runtime::types::Type};

pub struct ValueExpectation {}
struct ValueObject {
    _type: Type,
}
impl Object for ValueObject {
    fn get_type(&self) -> ObjectType {
        ObjectType::Value
    }
}

impl Expect for ValueExpectation {
    fn get_type(&self) -> ObjectType {
        ObjectType::Value
    }

    fn read(&self, lexer: &Lexer) -> Vec<Box<dyn Object>> {
        let objects = Vec::<Box<dyn Object>>::new();

        for expc in self.expectations {
            expc.exp.read(lexer);
        }

        return objects;
    }
}
