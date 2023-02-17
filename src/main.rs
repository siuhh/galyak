#![allow(dead_code)]

use error_mgr::ErrorCaller;
use runtime::{parser, interpreter};

mod error_mgr;
mod pre;
mod runtime;
mod test;

fn main() {
    //test::lexer::show_tokens();
    let f = "\"aaa\"+boba";
    let caller = ErrorCaller::new("2+2*2".to_string(), f);
    let mut parser = parser::Parser::new(f, &caller);
    
    let ast = parser.parse();
    
    interpreter::interpreter(ast);
}
