use crate::{
    error_mgr::ErrorCaller,
    pre::{
        lexer::Lexer,
        token::{
            tokens::{
                dynamic::{ARIPH_OP, NUMBER, UNKNOWN, NAME, STR},
                stat::{LEFT_PARENTHESIS, RIGHT_PARENTHESIS},
            },
            Token, TokenType,
        },
    },
};
#[derive(PartialEq)]
pub enum AST {
    Nothing,
    Num(f64),
    Var(String),
    Chars(String),
    Str {
        base: Box<AST>,//CHARS | VAR
        next: Box<AST>//STR | NOTHING
    },
    AriphExpression {
        left: Box<AST>,//NUMBER | VAR | ARIPH_EXPRESSION
        op: String,//+ - / *
        right: Box<AST>,//NUMBER | VAR | ARIPH_EXPRESSION
    },
    DecVar {
        array: bool,
        name: String,
        _type: String,
    },
    Statement {
        array: bool,
        name: String,
        _type: String,
    }
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

    //factor : INTEGER | VAR | LEFT_PARENTHESIS expr RIGHT_PARENTHESIS
    fn factor(&mut self) -> AST {
        let token = self.current_token.clone();

        if token.name == NUMBER {
            self.eat(NUMBER);
            return AST::Num(token.val.parse::<f64>().unwrap());
        } else if token.name == NAME {
            self.eat(NAME);
            return AST::Var(token.val);
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
            if token.val == "*" || token.val  == "/" {
                self.eat(ARIPH_OP);
            } else {
                break;
            }
            node = AST::AriphExpression{left: Box::new(node), op: token.val, right: Box::new(self.factor())};
        }

        return node;
    }
    //expr   : term ((PLUS | MINUS) term)*
    fn expr(&mut self) -> AST {
        let mut node = self.term();
        while self.current_token.name == ARIPH_OP {
            let token = self.current_token.clone();
            if token.val == "+" || token.val  == "-" {
                self.eat(ARIPH_OP);
            } else {
                break;
            }
            node = AST::AriphExpression{left: Box::new(node), op: token.val, right: Box::new(self.term())};
        }
        return node;
    }
    //string    : "str" | VAR (+ string)*
    pub fn string(&mut self) -> AST {
        let token = self.current_token.clone();
        let base = Box::new(match token.name {
            NAME => AST::Var(token.val),
            STR => AST::Chars(token.val),
            _ => panic!()
        });
        if self.current_token.val == "+" {
            self.eat(ARIPH_OP);
            return AST::Str { base, next: Box::new(self.string()) };
        }
        return AST::Str { base, next: Box::new(AST::Nothing) };
    }
    
    pub fn parse(&mut self) -> AST {
        self.current_token = self.lexer.next_token();
        return self.string();
    }
}
