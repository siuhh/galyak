#![allow(dead_code)]

use program::cli::cli;

mod compiler;
mod program;
mod runtime;

fn main() {
    cli();
//     let file: String = "
// тємка а():
//     штріх цифри аб = 2 + 2 * 2
//     базар(аб) 
//     аб = аб - 2 
//     базар(аб)
//     штріх цифри абв = аб / 2
//     абв = аб * абв + 2 - аб / 2
//     базар(абв)
// .
// тємка сума(цифри а, цифри б) нарішає цифри:
//     рішани а + б
// .
// штріх цифри a1 = сума(5, 10)

// базарлн(a1, a1, a1, a1)

// а()
// ".to_string();
//     let prog = Prog {
//         file_content: file,
//         file_name: "test.glk".to_string(),
//     };
//     prog.run(false, false);
}
