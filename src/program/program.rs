use std::{
    alloc::{alloc, dealloc, Layout},
    collections::LinkedList,
};

use compiler::parser::Parser;

use crate::{
    compiler,
    runtime::{func::GlkFuncDeclaration, interpreter::Interpreter, memory::types::Type},
};

use super::error_mgr::ErrorCaller;

pub fn run(file_path: String) -> String {
    return file_path;
}
pub struct Prog {
    pub file_name: String,
    pub current_path: String,
    pub file_content: String,
}

impl Prog {
    pub fn run(&self) {
        let c = ErrorCaller::new(&self.file_name, &self.file_content);
        
        let mut p = Parser::new(&self.file_content, &c);

        let asts = p.parse();

        unsafe {
            let mainf = alloc(Layout::new::<GlkFuncDeclaration>()) as *mut GlkFuncDeclaration;
            let gfd = GlkFuncDeclaration::new(asts, LinkedList::new(), Type::Null);

            std::ptr::write(mainf, gfd);

            let mut interpreter = Interpreter::new(mainf, &c);
            interpreter.run();

            dealloc(mainf as *mut u8, Layout::new::<GlkFuncDeclaration>());
            interpreter.end();
        }
    }
}
