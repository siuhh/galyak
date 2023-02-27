#![allow(dead_code)]

use compiler::parser::Parser;

use crate::runtime::interpreter::{Interpreter};

mod compiler;
mod runtime;
mod error_mgr;
mod test;

fn main() {
    const FILE: &str = "
штріх цифри аб = 2 + 2 * 2
базар(аб) 
аб = аб - 2 
базар(аб)
штріх цифри абв = аб / 2
абв = аб * абв + 2 - аб / 2
базар(абв)
";
    let c = error_mgr::CompilationError::new("test".to_string(), &FILE);
    let mut p = Parser::new(&FILE, &c);
    
    let asts = p.parse();
    unsafe { let mut interpreter = Interpreter::new(asts); interpreter.run(); };
    //test::lexer::show_tokens();
}