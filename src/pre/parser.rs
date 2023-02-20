use core::panic;
use std::collections::LinkedList;

use crate::{
    pre::{
        lexer::Lexer,
        token::{
            tokens::{
                dynamic::{ARIPH_OP, NAME, NUMBER, STR, UNKNOWN},
                stat::{ARRAY, LEFT_PARENTHESIS, RIGHT_PARENTHESIS, SET, VAR},
            },
            Token, TokenType,
        },
    }, error_mgr::ErrorCaller,
};
pub enum AstNodeValue {
    Nothing,
    Num(f64),
    Keyword(String),
    String(String),
    AriphExpression {
        left: Box<AstNodeValue>,  //NUMBER | VAR | ARIPH_EXPRESSION | STRING
        op: Token,       //+ - / *
        right: Box<AstNodeValue>, //NUMBER | VAR | ARIPH_EXPRESSION | STRING
    },
    DecVar {
        array: bool,
        name: Box<AstNodeValue>,  // VAR
        _type: Box<AstNodeValue>, // VAR
        value: Box<AstNodeValue>,
    },
    Statement {
        line: usize,
        statement: Box<AstNodeValue>, // DecVar
    },
    StatementList {
        statements: LinkedList<Box<AstNodeValue>>, //Statement
    },
    CompoundStatement {
        statements: LinkedList<Box<AstNodeValue>>, //
    },
}
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    error_caller: &'a ErrorCaller,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(file: &'static str, caller: &'a ErrorCaller) -> Parser<'a> {
        return Parser {
            lexer: Lexer::new(file, caller),
            error_caller: caller,
            current_token: Token::new(UNKNOWN, "EMPTY TOKEN".to_string(), 0, 0),
        };
    }
    fn eat(&mut self, tok: TokenType) {
        if self.current_token.name == tok {
            self.current_token = self.lexer.next_token();
            return;
        }
        self.error_caller.unexpected_token(&self.current_token);
        panic!();
    }

    //factor : INTEGER | STRING | VAR | LEFT_PARENTHESIS expr RIGHT_PARENTHESIS
    fn factor(&mut self) -> AstNodeValue {
        let token = self.current_token.clone();

        if token.name == NUMBER {
            self.eat(NUMBER);
            return AstNodeValue::Num(token.value.parse::<f64>().unwrap());
        } else if token.name == NAME {
            self.eat(NAME);
            return AstNodeValue::Keyword(token.value)
        } else if token.name == LEFT_PARENTHESIS {
            self.eat(LEFT_PARENTHESIS);
            let node = self.expr();
            self.eat(RIGHT_PARENTHESIS);
            return node;
        }
        println!("!!{}", self.current_token);
        self.error_caller.unexpected_token(&self.current_token);
        panic!();
    }
    //term   : factor ((MUL | DIV) factor)*
    fn term(&mut self) -> AstNodeValue {
        let mut node = self.factor();

        while self.current_token.name == ARIPH_OP {
            let token = self.current_token.clone();
            if token.value == "*" || token.value == "/" {
                self.eat(ARIPH_OP);
            } else {
                break;
            }
            node = AstNodeValue::AriphExpression {
                left: Box::new(node),
                op: token,
                right: Box::new(self.factor()),
            };
        }

        return node;
    }
    //expr   : term ((PLUS | MINUS) term)*
    fn expr(&mut self) -> AstNodeValue {
        let mut node = self.term();
        while self.current_token.name == ARIPH_OP {
            let token = self.current_token.clone();
            if token.value == "+" || token.value == "-" {
                self.eat(ARIPH_OP);
            } else {
                break;
            }
            node = AstNodeValue::AriphExpression {
                left: Box::new(node),
                op: token,
                right: Box::new(self.term()),
            };
        }
        return node;
    }
    //dec_var   :  (штріх TYPE) | (штріхи TYPE) NAME = EXPR крч
    pub fn dec_var(&mut self) -> AstNodeValue {
        let array = match self.current_token.name {
            ARRAY => {
                self.eat(ARRAY);
                true
            }
            VAR => {
                self.eat(VAR);
                false
            }
            _ => panic!(),
        };

        let _type = self.current_token.clone();
        self.eat(NAME);

        let name = self.current_token.clone();
        self.eat(NAME);

        self.eat(SET);

        let value = self.expr();

        return AstNodeValue::DecVar {
            array,
            name: Box::new(AstNodeValue::Keyword(name.value)),
            _type: Box::new(AstNodeValue::Keyword(name.value)),
            value: Box::new(value),
        };
    }

    pub fn parse(&mut self) -> AstNodeValue {
        self.current_token = self.lexer.next_token();
        return self.dec_var();
    }
}
