use crate::pre::token::TokenType;

use super::{Expect, Object, ObjectType};

pub struct TokenExpectation {
    pub token: TokenType,
}
impl Expect for TokenExpectation {
    fn get_type(&self) -> ObjectType {
        ObjectType::Token
    }
}
pub struct TokenObject {
    pub token: TokenType,
}
impl Object for TokenObject {
    fn get_type(&self) -> ObjectType {
        ObjectType::Token
    }
}
