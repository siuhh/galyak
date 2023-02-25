#![allow(dead_code)]

use compiler::parser::Parser;

use crate::runtime::interpreter::{Interpreter};

mod compiler;
mod runtime;
mod error_mgr;

fn main() {
    const FILE: &str = "
штріх цифри аб = 2 + 2 * 2 крч
аб = аб - 2 крч
базар(абв) крч
базар(аб) крч
штріх цифри абв = аб / 2 крч
абв = аб * абв + 2 - аб / 2 крч
базар(абв) крч
";
    let c = error_mgr::CompilationError::new("test".to_string(), &FILE);
    let mut p = Parser::new(&FILE, &c);
    
    let asts = p.parse();
    unsafe { let mut interpreter = Interpreter::new(asts); interpreter.run(); };
}