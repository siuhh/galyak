use core::panic;
use std::collections::LinkedList;

use crate::{
    error_mgr::ErrorCaller,
    pre::{
        lexer::Lexer,
        token::{
            tokens::{
                dynamic::{ARIPH_OP, NAME, NUMBER, STR, UNKNOWN},
                stat::{ARRAY, LEFT_PARENTHESIS, RIGHT_PARENTHESIS, SET, VAR},
            },
            Token, TokenType,
        },
    },
};
pub enum AST {
    Nothing,
    Num(f64),
    Var(String),
    String(String),
    AriphExpression {
        left: Box<AST>,  //NUMBER | VAR | ARIPH_EXPRESSION | STRING
        op: Token,       //+ - / *
        right: Box<AST>, //NUMBER | VAR | ARIPH_EXPRESSION | STRING
    },
    DecVar {
        array: bool,
        name: Box<AST>,  // VAR
        _type: Box<AST>, // VAR
        value: Box<AST>,
    },
    Statement {
        statement: Box<AST>, // DecVar
    },
    StatementList {
        statements: LinkedList<Box<AST>>, //Statement
    },
    CompoundStatement {
        statements: LinkedList<Box<AST>>, //
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
    fn factor(&mut self) -> AST {
        let token = self.current_token.clone();

        if token.name == NUMBER {
            self.eat(NUMBER);
            return AST::Num {
                token,
                value: token.value.parse::<f64>().unwrap(),
            };
        } else if token.name == NAME {
            self.eat(NAME);
            return AST::Var {
                token,
                value: token.value,
            };
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
    fn term(&mut self) -> AST {
        let mut node = self.factor();

        while self.current_token.name == ARIPH_OP {
            let token = self.current_token.clone();
            if token.value == "*" || token.value == "/" {
                self.eat(ARIPH_OP);
            } else {
                break;
            }
            node = AST::AriphExpression {
                left: Box::new(node),
                op: token,
                right: Box::new(self.factor()),
            };
        }

        return node;
    }
    //expr   : term ((PLUS | MINUS) term)*
    fn expr(&mut self) -> AST {
        let mut node = self.term();
        while self.current_token.name == ARIPH_OP {
            let token = self.current_token.clone();
            if token.value == "+" || token.value == "-" {
                self.eat(ARIPH_OP);
            } else {
                break;
            }
            node = AST::AriphExpression {
                left: Box::new(node),
                op: token,
                right: Box::new(self.term()),
            };
        }
        return node;
    }
    //dec_var   :  (штріх TYPE) | (штріхи TYPE) NAME = EXPR крч
    pub fn dec_var(&mut self) -> AST {
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

        return AST::DecVar {
            array,
            name: Box::new(AST::Var {
                value: name.value,
                token: name,
            }),
            _type: Box::new(AST::Var {
                value: name.value,
                token: name,
            }),
            value: Box::new(value),
        };
    }

    pub fn parse(&mut self) -> AST {
        self.current_token = self.lexer.next_token();
        return self.dec_var();
    }
}
