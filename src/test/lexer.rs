use crate::{
    error_mgr::ErrorCaller,
    pre::{lexer::Lexer, token::tokens::stat::EOF},
};

pub fn show_tokens() {
    const FILE: &str = "тіп цифри число = 55 крч
базар(число + 92 / (32 + число)) крч
базар(\"12341234) крч";

    let c = ErrorCaller::new(String::from("test"), FILE);
    let mut l = Lexer::new(FILE, &c);

    let mut t = l.next_token();

    while t.name != EOF {
        println!("{}", t);
        t = l.next_token();
    }
}

pub fn unmatched_quote() {
    const FILE: &str = "тіп цифри число = 55 крч
базар(число + 92 / (32 + число)) крч
базар(\"12341234) крч"; // here is unmatched quote
    let c = ErrorCaller::new(String::from("test"), FILE);
    let mut l = Lexer::new(FILE, &c);

    let mut t = l.next_token();

    while t.name != EOF {
        t = l.next_token();
    }
}

pub fn unexpected_token() {
    const FILE: &str = "тіп цифри число = 55 крч
базар(число + 92 / (32 + числ$о)) крч
базар(\"12341234\") крч";
    let c = ErrorCaller::new(String::from("тест.глк"), FILE);
    let mut l = Lexer::new(FILE, &c);

    let mut t = l.next_token();

    while t.name != EOF {
        t = l.next_token();
        c
    }
}
