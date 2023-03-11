use std::env;

use super::program::Prog;

pub fn cli() {
    let _args: Vec<String> = env::args().collect();
    
    // if args.len() < 2 {
    //     println!("{}", "А де файл".red().bold());
    //     return;
    // }
    
    //let prog = Prog::new(args[1].clone());
    let prog = Prog::new("C:\\Users\\bjejb\\Desktop\\тест.глк".to_string());
    prog.run(false, false);
}