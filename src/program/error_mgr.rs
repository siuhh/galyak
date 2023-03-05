use crate::compiler::token::Token;
use colored::Colorize;

pub enum GlkErrorType {
    Compilation,
    Runtime,
}

pub struct ErrorCaller {
    file_name: String,
    file: &'static str,
}

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
    
impl ErrorCaller {
    pub fn new(file_name: String, file: &'static str) -> ErrorCaller {
        return ErrorCaller { file, file_name };
    }
    
    fn pr_err_head(&self, line: usize, ch: usize, message: &String) {
        println!(
            "\n{} по {} строці на {} букві: {}",
            "Повний галяк ".red().bold(),
            line.to_string().bold().yellow(),
            (ch + 1).to_string().bold().yellow(),
            message
        );
    }
    fn pr_err_line(&self, line_num: usize, ch: usize, err_length: usize) {
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
            sub_str(&line, ch, ch + err_length).red().bold().underline(),
            sub_str(&line, ch + err_length, line.chars().count())
        );
        
        print_tab(offset + 3 + ch);

        for _ in 0..err_length {
            print!("{}", "↑".red().bold());
        }
    }
    
    fn pr_message(&self, offset: usize, ch: usize, err_length: usize, message: &String) {
        //arrows


        let mut msg_offset: usize = 0;

        if ch  + err_length / 2 > message.chars().count() / 2 {
            msg_offset = ch  + err_length / 2 - message.chars().count() / 2;
        }
        
        print!("\n");
        print_tab(offset + 2 + msg_offset);
        
        print!("{}", message.red());
    }
    pub fn comp_error(&self, message: String, token: &Token) {
        let ln = token.line;
        let ch = token.on_char;

        self.pr_err_head(ln, ch, &message);
        self.pr_err_line(ln, ch, token.value.chars().count());
        self.pr_message(get_num_size(ln), ch, token.value.chars().count(), &message);
        
        std::process::exit(0);
    }
}
