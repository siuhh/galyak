use crate::compiler::token::Token;
use colored::Colorize;

pub struct CompilationError {
    file_name: String,
    file: &'static str,
}

impl CompilationError {
    pub fn new(file_name: String, file: &'static str) -> CompilationError {
        return CompilationError { file, file_name };
    }
    fn sub_str(&self, message: &str, start: usize, end: usize) -> String {
        if end > message.chars().count() {
            return String::from(" ");
        }

        let mut fin = String::new();

        for i in start..end {
            fin.push(message.chars().nth(i).unwrap());
        }

        return fin;
    }
    fn pr_err_head(&self, line: usize, ch: usize, message: &str) {
        println!(
            "\n{} по {} строці на {} букві: {}\n",
            "Повний галяк ".red().bold(),
            line.to_string().bold().yellow(),
            (ch + 1).to_string().bold().yellow(),
            message
        );
    }
    fn pr_err_line(&self, line_num: usize, ch: usize, err_length: usize) {
        let line = self.file.split('\n').nth(line_num - 1).unwrap();

        println!("| -> {}", self.file_name.yellow().bold());
        println!(
            "|        {}{}{}",
            self.sub_str(&line, 0, ch),
            self.sub_str(&line, ch, ch + err_length).red().bold(),
            self.sub_str(&line, ch + err_length, line.chars().count())
        );
    }
    fn pr_message(&self, ch: usize, err_length: usize, message: String) {
        //arrows
        print!("|");
        for _ in 0..ch + 8 {
            print!(" ");
        }

        for _ in 0..err_length {
            print!("{}", "^".red().bold());
        }

        let mut msg_offset: usize = 0;

        if ch + 8 + err_length / 2 > message.chars().count() / 2 {
            msg_offset = ch + 8 + err_length / 2 - message.chars().count() / 2;
        }

        print!("\n|");
        for _ in 0..msg_offset {
            print!(" ");
        }
        println!("{}", message.red());
    }
    pub fn call(&self, message: String, token: &Token) {
        let ln = token.line + 1;
        let ch = token.on_char - 1;

        self.pr_err_head(ln, ch, &message);
        self.pr_err_line(ln, ch, token.value.chars().count());
        self.pr_message(ch, token.value.chars().count(), message);
        //std::process::exit(0);
    }

    pub fn unknown_token(&self, t: &Token) {
        let msg = format!("якийсь кучерявий базар \"{}\"", t.value);
        self.call(msg, t);
    }

    pub fn unexpected_token(&self, t: &Token) {
        let msg = format!("\"{}\" - цього тіпа сюда ніхто не кликав", t.value);
        self.call(msg, t);
    }

    pub fn unmatched_quote(&self, t: &Token) {
        let msg = String::from("не закрита \"");
        self.call(msg, t);
    }

    pub fn unallowed_operation(&self, t: &Token, vtype: &str) {
        let msg = format!("якась кучерява операція для штріха масті {}", vtype);
        self.call(msg, t);
    }
    
    pub fn inner_compilation_error(&self, t: &Token) {
        let msg = String::from("Якась залупа тут кароче закинь пж сюда https://github.com/siuhh/galyak/issues шо сталось");
        self.call(msg, t);
    }
    
}