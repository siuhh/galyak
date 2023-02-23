#![allow(dead_code)]

use compiler::parser::Parser;

mod compiler;
mod runtime;
mod error_mgr;

fn main() {
    const FILE: &str = "
тємка абв (цифри а, цифри б) нарішає цифри значить 
    штріх цифри аб = а + б крч 
    рішани аб крч 
всьо
штріх цифри аа = абв(5, 10) крч
аа = абв(19, 5) крч
";
    let c = error_mgr::CompilationError::new("test".to_string(), &FILE);
    let mut p = Parser::new(&FILE, &c);
    
    p.parse();
}