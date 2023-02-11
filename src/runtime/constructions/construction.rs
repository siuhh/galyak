use crate::pre::lexer::Lexer;

use super::object::{Expect, Expectation, Object, ObjectType};

pub struct Construction {
    expectations: &'static [Expectation],
}
impl Expect for Construction {
    fn get_type(&self) -> ObjectType {
        ObjectType::Construction
    }

    fn read(&self, lexer: &Lexer) -> Vec<Box<dyn Object>> {
        let objects = Vec::<Box<dyn Object>>::new();

        for expc in self.expectations {
            expc.exp.read(lexer);
        }

        return objects;
    }
}
impl Construction {}
