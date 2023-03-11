use core::fmt::{Display, Formatter, Result};

pub type TokenType = &'static str;

pub mod tokens {
    pub mod dynamic {
        use crate::compiler::token::TokenType;

        pub const NUMBER: TokenType = "цифри";
        pub const ARIPH_OP: TokenType = "мт";
        pub const STR: TokenType = "букви";
        pub const NAME: TokenType = "назва";
        pub const UNKNOWN: TokenType = "якась хуйня";
    }

    pub mod stat {
        use crate::compiler::token::TokenType;

        pub const EOF: TokenType = "кінець";
        pub const EOL: TokenType = "кінець строки";

        //brackets
        pub const LPAR: TokenType = "(";
        pub const RPAR: TokenType = ")";
        pub const COMPOUND_START: TokenType = ":";
        pub const COMPOUND_END: TokenType = ".";

        pub const COMA: TokenType = ",";

        //declatarions
        pub const FUNC: TokenType = "тємка";
        pub const RET_RYPE: TokenType = "нарішає";
        pub const RETURN: TokenType = "рішани";
        pub const CLASS: TokenType = "масть";
        pub const CONSTRUCTOR: TokenType = "вилупився";
        pub const PRIVATE: TokenType = "тіхарь";
        pub const PUBLIC: TokenType = "кент";

        //variables
        pub const VAR: TokenType = "штріх";
        pub const ARRAY: TokenType = "штріхи";
        pub const SET: TokenType = "=";
        pub const GET: TokenType = ".";

        //Statements
        pub const IF: TokenType = "варік";
        pub const ELSE: TokenType = "найоб";
        pub const WHILE: TokenType = "поки";

        //bool operators
        pub const EQUALS: TokenType = "внатурі";
        pub const NOT_EQUALS: TokenType = "не";
        pub const LESS: TokenType = "меньше";
        pub const MORE: TokenType = "більше";

        //static values
        pub const TRUE: TokenType = "верняк";
        pub const FALSE: TokenType = "гон";
    }
}

use self::tokens::stat::*;

pub const STATIC_TOKENS: &[TokenType] = &[
    LPAR,
    RPAR,
    COMPOUND_START,
    COMPOUND_END,
    COMA,
    FUNC,
    RET_RYPE,
    RETURN,
    CLASS,
    PRIVATE,
    PUBLIC,
    ARRAY,
    IF,
    ELSE,
    WHILE,
    VAR,
    SET,
    GET,
    EQUALS,
    NOT_EQUALS,
    LESS,
    MORE,
    TRUE,
    FALSE,
];
#[derive(Clone, Debug)]
pub struct Token {
    pub name: TokenType,
    pub value: String,
    pub line: usize,
    pub on_char: usize,
}

impl Token {
    pub fn new(line: usize, ch: usize, name: TokenType, val: String) -> Token {
        return Token {
            name,
            value: val,
            line,
            on_char: ch,
        };
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}:{} {} \"{}\"",
            self.line, self.on_char, self.name, self.value,
        )
    }
}
