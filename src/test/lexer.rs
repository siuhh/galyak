use crate::{
    error_mgr::Caller,
    pre::{lexer::Lexer, token::tokens::stat::EOF},
};

pub fn test() {
    const FILE: &str = "тіп цифри число = 55 крч
базар(число + 92 / (32 + число)) крч
базар(\"12341234\") крч";

    let c = Caller::new(String::from("test"), FILE);
    let mut l = Lexer::new(FILE, &c);

    let mut t = l.next_token();

    while t.name != EOF {
        println!("{}", t);
        t = l.next_token();
    }
}
