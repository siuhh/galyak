#![allow(dead_code)]

use crate::program::program::Prog;

mod compiler;
mod program;
mod runtime;
mod test;

fn main() {
    let file: String = "
тємка а():
    штріх цифри аб = 2 + 2 * 2
    базар(аб) 
    аб = аб - 2 
    базар(аб)
    штріх цифри абв = аб / 2
    абв = аб * абв + 2 - аб / 2
    базар(абв)
.
тємка базарсуму(цифри а, цифри б) нарішає галяк:
    тємка базарсуму(цифри а, цифри б) нарішає галяк:
        базар(а + б + 1)
    . 
    базарсуму(а * 2, б * 2)  
.
базарсуму(5 + 5, 10 + 10)
а()
".to_string();
    
    let prog = Prog {
        file_content: file,
        file_name: "test.glk".to_string(),
        current_path: "".to_string(),
    };
    prog.run();
}
