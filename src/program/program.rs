use std::{
    alloc::{alloc, dealloc, Layout},
    collections::LinkedList, fs, process::exit,
};
use std::time::{Instant};

use colored::Colorize;
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
    pub file_content: String,
}

impl Prog {
    fn fill_content(&mut self) {
        self.file_content = {
            match fs::read_to_string(&self.file_name) {
                Ok(mut file) => { 
                    file = file.replace("\r\n", "\n");
                    
                    file 
                },
                Err(message) => {println!("{}", message); exit(1)},
            }
        }
    }
    
    pub fn new(file_name: String) -> Self {
        let mut prog = Prog {
            file_name,
            file_content: "".to_string()
        };
        
        prog.fill_content();
        
        return prog;
    }
    
    pub fn run(&self, show_time: bool, debug: bool) {
        let c = ErrorCaller::new(&self.file_name, &self.file_content, debug);
        
        let mut p = Parser::new(&self.file_content, &c);

        let mut now = Instant::now();
        let asts = p.parse();
        
        unsafe {
            
            let mainfptr = alloc(Layout::new::<GlkFuncDeclaration>()) as *mut GlkFuncDeclaration;
            let mainfd = GlkFuncDeclaration::new(asts, LinkedList::new(), Type::Null, "main".to_string());
            std::ptr::write(mainfptr, mainfd);

            let mut interpreter = Interpreter::new(mainfptr, &c);
        
            if show_time {
                println!("{}", format!(
                    "Забілдилось за {}сек", 
                    now.elapsed().as_secs_f32()).green().bold()
                );
            }
            now = Instant::now();
            
            interpreter.run();
            interpreter.end();
            dealloc(mainfptr as *mut u8, Layout::new::<GlkFuncDeclaration>());
            
            if show_time {
                print!("{}", format!(
                    "\nГаляк готово за {}сек", 
                    now.elapsed().as_secs_f32()).green().bold()
                );
            }
        }
    }
}
