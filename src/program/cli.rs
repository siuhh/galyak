use std::env;

use colored::Colorize;

use super::program::Prog;

pub fn cli() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 1 {
        println!("{}", "А де файл".red().bold());
        return;
    }
    
    //let prog = Prog::new(args[1].clone());
    let prog = Prog::new("C:\\Users\\bjejb\\Desktop\\тест.глк".to_string());
    
    let show_time = false;
    let debug = false;
    
    prog.run(show_time, debug);
}