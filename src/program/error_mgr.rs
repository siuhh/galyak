use core::panic;

use crate::compiler::token::Token;
use colored::Colorize;


fn print_tab(size: usize) {
    for _ in 0..size {
        print!(" ");
    }
}

fn sub_str(message: &str, start: usize, end: usize) -> String {
    if end > message.chars().count() {
        return String::from(" ");
    }

    let mut fin = String::new();

    for i in start..end {
        fin.push(message.chars().nth(i).unwrap());
    }

    return fin;
}

fn get_num_size(num: usize) -> usize {
    return num / 10 + 1;
}
pub struct ErrorCaller {
    file_name: String,
    file: String,
    debug: bool,
}
    
impl ErrorCaller {
    pub fn new(file_name: &String, file: &String, debug: bool) -> ErrorCaller {
        let file_name = file_name.clone();
        let file = file.clone();
        return ErrorCaller { file, file_name, debug };
    }
    
    fn comp_err_head(&self, line: usize, ch: usize, message: &String) {
        println!(
            "\n{} по {} строці на {} букві: {}",
            "Повний галяк".red().bold(),
            line.to_string().bold().yellow(),
            (ch + 1).to_string().bold().yellow(),
            message.bold()
        );
    }
    fn runt_err_head(&self, line: usize, message: &String) {
        println!(
            "\n{} по {} строці: {}",
            "Повний галяк".red().bold(),
            line.to_string().bold().yellow(),
            message.bold()
        );
    }
    fn comp_err_line(&self, line_num: usize, ch: usize, err_length: usize) {
        let line = self.file.split('\n').nth(line_num - 1).unwrap();
        let offset = get_num_size(line_num);
        
        //file name
        print_tab(offset + 1);
        print!("{}:\n", self.file_name.yellow().bold());
        println!();
        //line number
        
        print!("{} | ", line_num);
        
        //line content with colored error token
        println!("{}{}{}",
            sub_str(&line, 0, ch),
            sub_str(&line, ch, ch + err_length).red().bold(),
            sub_str(&line, ch + err_length, line.chars().count())
        );
    }
    
    fn runt_err_line(&self, line_num: usize) {
        let line = self.file.split('\n').nth(line_num - 1).unwrap();
        let offset = get_num_size(line_num);
        
        //file name
        print_tab(offset + 1);
        print!("{}:\n", self.file_name.yellow().bold());
        println!();
        //line number
        
        print!("{} | ", line_num.to_string().yellow().bold());
        
        //line content 
        println!("{}", &line.red().bold());
    }
    
    fn comp_err_message(&self, offset: usize, ch: usize, err_length: usize, message: &String) {
        //arrows
        
        print_tab(offset + 3 + ch);

        for _ in 0..err_length {
            print!("{}", "^".red().bold());
        }
        
        let mut msg_offset: usize = 0;

        if ch  + err_length / 2 > message.chars().count() / 2 {
            msg_offset = ch  + err_length / 2 - message.chars().count() / 2;
        }
        
        print!("\n");
        print_tab(offset + 2 + msg_offset);
        
        print!("{}", message.red());
    }
    
    fn runt_err_message(&self, offset: usize, message: &String) {
        print!("\n");
        print_tab(offset + 2);
        
        print!("{}", message.red());
    }
    
    fn end(&self) {
        if self.debug {
            panic!();
        }std::process::exit(0);
    }
    
    pub fn comp_error(&self, message: String, token: &Token) {
        let line = token.line;
        let ch = token.on_char;

        self.comp_err_head(line, ch, &message);
        self.comp_err_line(line, ch, token.value.chars().count());
        self.comp_err_message(get_num_size(line), ch, token.value.chars().count(), &message);
        
        self.end();
    }
    
    pub fn runt_error(&self, message: String, line: usize) {

        self.runt_err_head(line, &message);
        self.runt_err_line(line);
        self.runt_err_message(get_num_size(line), &message);
        
        self.end();
    }
}
