use core::panic;
use std::collections::LinkedList;

use crate::{
    compiler::{
        lexer::Lexer,
        token::{
            tokens::{
                dynamic::*,
                stat::*,
            },
            Token, TokenType,
        },
    }, error_mgr::CompilationError,
};

use super::ast::Ast;
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    error_caller: &'a CompilationError,
    current_token: Token,
    peaked_token: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(file: &'static str, caller: &'a CompilationError) -> Parser<'a> {
        let mut parser = Parser {
            lexer: Lexer::new(file, caller),
            error_caller: caller,
            current_token: Token::new(UNKNOWN, "EMPTY TOKEN".to_string(), 0, 0),
            peaked_token: None,
            
        };
        parser.eat(UNKNOWN);
        return parser;
    }
    
    fn peak(&mut self) -> Token {
        if self.peaked_token.is_some() {
            panic!();
        }
        self.peaked_token = Some(self.lexer.next_token());
        return self.peaked_token.clone().unwrap();
    }
    
    fn eat(&mut self, tok: TokenType) -> Token {
        if self.current_token.name == tok {
            let prev = self.current_token.clone();
            
            //if token was peaked 
            if self.peaked_token.is_some() {
                self.current_token = self.peaked_token.clone().unwrap();
                self.peaked_token = None;
            }
            else {
                self.current_token = self.lexer.next_token();
            }
            
            return prev;
        }
        self.error_caller.unexpected_token(&self.current_token);
        panic!();
    }

    //factor : INTEGER | STRING | VAR | CALL_FUNC | LEFT_PARENTHESIS expr RIGHT_PARENTHESIS
    fn st_factor(&mut self) -> Ast {
        let token = self.current_token.clone();

        if token.name == NUMBER {
            let num = self.eat(NUMBER).value;
            return Ast::Num(num.parse::<f64>().unwrap());
        } else if token.name == NAME {
            let next = self.peak().name;
            
            return match next {
                LPAR => self.st_call_func(),
                _ => Ast::Keyword(self.eat(NAME).value)
            };
        } else if token.name == LPAR {
            self.eat(LPAR);
            let node = self.st_expr();
            self.eat(RPAR);
            return node;
        }
        self.error_caller.unexpected_token(&self.current_token);
        panic!();
    }
    //term   : factor ((MUL | DIV) factor)*
    fn st_term(&mut self) -> Ast {
        let mut node = self.st_factor();

        while self.current_token.name == ARIPH_OP {
            let token = self.current_token.clone();
            if token.value == "*" || token.value == "/" {
                self.eat(ARIPH_OP);
            }
            else {
                break;
            }
            node = Ast::Expression {
                left: Box::new(node),
                op: token,
                right: Box::new(self.st_factor()),
            };
        }

        return node;
    }
    //expr   : term ((PLUS | MINUS) term)*
    fn st_expr(&mut self) -> Ast {
        let mut node = self.st_term();
        while self.current_token.name == ARIPH_OP {
            let token = self.current_token.clone();
            if token.value == "+" || token.value == "-" {
                self.eat(ARIPH_OP);
            } else {
                break;
            }
            node = Ast::Expression {
                left: Box::new(node),
                op: token,
                right: Box::new(self.st_term()),
            };
        }
        return node;
    }
    //dec_var   :  (штріх #TYPE) | (штріхи #TYPE) #NAME = expr крч
    pub fn st_dec_var(&mut self) -> Ast {
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

        let vtype = self.eat(NAME).value;

        let name = self.eat(NAME).value;
        
        let value;
        
        if self.current_token.name == EOL {
            value = Ast::Nothing;
        }
        else {
            self.eat(SET);
            value = self.st_expr();
        }

        return Ast::DeclareVariable {
            array,
            name,
            vtype,
            value: Box::new(value),
        };
        
    }
    //set_var   : #NAME = expr
    pub fn st_set_var(&mut self) -> Ast {
        let name = self.eat(NAME).value;
        
        self.eat(SET);
        
        let value = self.st_expr();
        
        return Ast::SetVariable { name, value: Box::new(value) }
    }
    pub fn st_call_func(&mut self) -> Ast {
        let name = self.eat(NAME).value;
        self.eat(LPAR);
        
        let mut args = LinkedList::<Box<Ast>>::new();
        
        loop {
            let arg = Box::new(self.st_expr());
            
            args.push_back(arg);
            
            if self.current_token.name == RPAR {
                self.eat(RPAR);
                break;
            }
            
            self.eat(COMA);
        }
        
        return Ast::CallFunc { name, args };
    }
    fn st_return(&mut self) -> Ast {
        self.eat(RETURN);
        return Ast::Return { expression: Box::new(self.st_expr()) };
    }
    
    // dec_func  :  тємка #NAME LPAR (#TYPE #NAME,)* RPAR (нарішає #TYPE)? LBRACK statement_list RBRACK
    pub fn st_def_func(&mut self) -> Ast {
        self.eat(FUNC); 
        
        let name = self.eat(NAME).value; 
        
        self.eat(LPAR);
        
        let mut args = LinkedList::<(String, String)>::new();
        
        loop {
            let vtype = self.eat(NAME).value;
            let vname = self.eat(NAME).value;
            
            args.push_back((vname, vtype));
            
            if self.current_token.name == RPAR {
                self.eat(RPAR);
                break;
            }
            
            self.eat(COMA);
        }
        
        let return_type;
        
        if self.current_token.name == RET_RYPE {
            self.eat(RET_RYPE);
            return_type = self.eat(NAME).value;
        }
        else {
            return_type = NULL.to_string();
        }
        self.eat(COMPOUND_START);
        let compound_statement = self.statement_list();
        self.eat(COMPOUND_END);
        
        return Ast::Function { 
            name, args, return_type, 
            compound_statement: Box::new(compound_statement)
        };
    }
    //(dec_var | dec_func | dec_class)
    pub fn declaration_statement(&mut self) -> Ast {
        
                println!("here is decstat");
        return match self.current_token.name {
            FUNC => self.st_def_func(),
            VAR => {
                println!("here is decvar");
                let dec_var = self.st_dec_var();
                self.eat(EOL);
                return dec_var;
            },
            CLASS => todo!(), //TODO!
            _ => {
                self.error_caller.unexpected_token(&self.current_token);
                panic!();
            }
        }
    }
    //statement : (dec_var | dec_func | call_func | dec_class | set_var | retrn)
    pub fn statement(&mut self) -> Ast {
        return match self.current_token.name {
            NAME => {
                let peak = self.peak().name;
                let expr;
                
                if peak == SET {
                    expr = self.st_set_var()
                }
                else if peak == LPAR {
                    expr =  self.st_call_func();
                }
                else {
                    self.error_caller.unexpected_token(&self.current_token);
                    panic!();
                }
                self.eat(EOL);
                return expr;
            },
            RETURN => {
                let ret = self.st_return();
                self.eat(EOL);
                return ret;
            },
            EOL => { //skip empty line
                self.eat(EOL);
                return Ast::Nothing;
            }
            _ => self.declaration_statement(),
        } 
    }
    
    pub fn statement_list(&mut self) -> Ast {
        let mut statements = LinkedList::<Box<Ast>>::new(); 
        
        while self.current_token.name != COMPOUND_END && self.current_token.name != EOF {
            let statement = self.statement();
            
            if let Ast::Nothing = statement {
                continue;
            }
            
            statements.push_back(Box::new(statement));
        } 
        
        return Ast::StatementList { statements };
    }
    
    pub fn parse(&mut self) -> LinkedList<Box<Ast>> { 
        if let Ast::StatementList { statements } = self.statement_list() {
            return statements;
        }
        else { 
            panic!();
        }
    }
}
