use crate::{
    error_mgr::Caller,
    pre::{lexer::Lexer, token::tokens},
};
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    exc_caller: &'a Caller,
}

impl<'a> Parser<'a> {
    pub fn new(file: &'static str, caller: &'a Caller) -> Parser<'a> {
        return Parser {
            lexer: Lexer::new(file, caller),
            exc_caller: caller,
        };
    }

    fn next_construction(&mut self) {
        let token = self.lexer.next_token();

        match token.name {
            tokens::dynamic::NAME => {}
            _ => todo!(),
        }
    }
}
