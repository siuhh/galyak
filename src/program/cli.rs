use std::env;

use colored::Colorize;

use super::program::Prog;

pub fn cli() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("{}", "А де файл".red().bold());
        return;
    }
    
    let prog = Prog::new(args[1].clone());
    
    prog.run(false, false);
}