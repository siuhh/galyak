use core::panic;
use std::{collections::LinkedList};

use crate::{
    compiler::{
        lexer::Lexer,
        token::{
            tokens::{
                dynamic::*,
                stat::*,
            },
            Token, TokenType,
        }
    }, 
    program::error_mgr::ErrorCaller, runtime::memory::types::T_NULL,
};
use crate::program::errors::compilation::unexpected_token;

use super::ast::Ast;
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    error_caller: &'a ErrorCaller,
    current_token: Token,
    peaked_token: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(file: &String, caller: &'a ErrorCaller) -> Parser<'a> {
        let file = file.clone();
        
        let mut parser = Parser {
            lexer: Lexer::new(file, caller),
            error_caller: caller,
            current_token: Token::new(0, 0, UNKNOWN, "EMPTY TOKEN".to_string(), ),
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
        self.error_caller.comp_error(unexpected_token(&self.current_token), &self.current_token);
        panic!();
    }
    
    fn break_line(&mut self) -> bool {
        if self.current_token.name == EOL {
            self.eat(EOL);
            return true;
        }
        return false;
    }

    //factor : INTEGER | STRING | VAR | BOOL | CALL_FUNC | LEFT_PARENTHESIS expr RIGHT_PARENTHESIS
    fn st_factor(&mut self) -> Ast {
        let token = self.current_token.clone();

        if token.name == NUMBER {
            let num = self.eat(NUMBER).value;
            return Ast::Num(num.parse::<f64>().unwrap());
        } 
        else if token.name == STR {
            let str = self.eat(STR).value;
            return Ast::String(str);
        } 
        else if token.name == NAME {
            let next = self.peak().name;
            
            return match next {
                LPAR => self.st_call_func(),
                _ => Ast::Keyword(self.eat(NAME).value)
            };
        } 
        else if token.name == TRUE {
            self.eat(TRUE);
            return Ast::Bool(true);
        } 
        else if token.name == FALSE {
            self.eat(FALSE);
            return Ast::Bool(false);
        } 
        else if token.name == LPAR {
            self.eat(LPAR);
            let node = self.st_expr();
            self.eat(RPAR);
            return node;
        }
        self.error_caller.comp_error(unexpected_token(&self.current_token), &self.current_token);
        panic!();
    }
    //term   : factor ((MUL | DIV) factor)*
    fn st_term(&mut self) -> Ast {
        let mut node = self.st_factor();

        loop {
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
        loop {
            let token = self.current_token.clone();
            if token.value == "+" || token.value == "-" {
                self.eat(ARIPH_OP);
            } 
            else if token.name == EQUALS {
                self.eat(EQUALS);
            }
            else if token.name == NOT_EQUALS {
                self.eat(NOT_EQUALS);
            }
            else if token.name == MORE {
                self.eat(MORE);
            }
            else if token.name == LESS {
                self.eat(LESS);
            }
            else {
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
    //dec_var   :  (штріх #TYPE) | (штріхи #TYPE) #NAME (= expr)?
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
            if self.current_token.name == RPAR {
                self.eat(RPAR);
                break;
            }
            
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
    pub fn st_dec_func(&mut self) -> Ast {
        self.eat(FUNC); 
        
        let name = self.eat(NAME).value; 
        
        self.eat(LPAR);
        
        let mut args = LinkedList::<(String, String)>::new();
        
        loop {
            if self.current_token.name == RPAR {
                self.eat(RPAR);
                break;
            }
            
            let vtype = self.eat(NAME).value;
            let vname = self.eat(NAME).value;
            
            args.push_back((vtype, vname));
            
            if self.current_token.name == RPAR {
                self.eat(RPAR);
                break;
            }
            
            self.eat(COMA);
        }
        
        let return_type: String;
        
        if self.current_token.name == RET_RYPE {
            self.eat(RET_RYPE);
            return_type = self.eat(NAME).value;
        }
        else {
            return_type = T_NULL.to_string();
        }
        
        self.eat(COMPOUND_START);
        let compound_statement = self.statement_list();
        self.eat(COMPOUND_END);
        
        return Ast::Function { 
            name, args, return_type, 
            compound_statement: compound_statement
        };
    }
    //(dec_var | dec_func | dec_class)
    pub fn declaration_statement(&mut self) -> Ast {
        return match self.current_token.name {
            FUNC => self.st_dec_func(),
            VAR => {
                let dec_var = self.st_dec_var();
                self.break_line();
                return dec_var;
            },
            CLASS => todo!(), //TODO!
            _ => {
                self.error_caller.comp_error(unexpected_token(&self.current_token), &self.current_token);
                panic!();
            }
        }
    }
    pub fn st_if(&mut self) -> Ast {
        self.eat(IF);
        let condtition = self.st_expr();
        
        self.eat(COMPOUND_START);
        let compound_statement = self.statement_list();
        self.eat(COMPOUND_END);
        
        //skip empty
        
        while self.break_line() { }
        
        let else_statement = 
            if self.current_token.name == ELSE {
                self.eat(ELSE);
                self.eat(COMPOUND_START);
                let stat = self.statement_list();
                self.eat(COMPOUND_END);
                Some(stat)
            }
            else {
                None
            };
        
        return Ast::If { condition: Box::new(condtition), compound_statement, else_statement }
    }
    
    pub fn st_while(&mut self) -> Ast {
        self.eat(WHILE);
        let condtition = self.st_expr();
        
        self.eat(COMPOUND_START);
        let compound_statement = self.statement_list();
        self.eat(COMPOUND_END);
        
        return Ast::While { condition: Box::new(condtition), compound_statement }
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
                    expr = self.st_call_func();
                }
                else {
                    self.error_caller.comp_error(unexpected_token(&self.current_token), &self.current_token);
                    panic!();
                }
                self.break_line();
                return expr;
            },
            RETURN => {
                let ret = self.st_return();
                self.break_line();
                return ret;
            },
            IF => {
                let ifst = self.st_if();
                self.break_line();
                
                return ifst;
            },
            WHILE => {
                let st_while = self.st_while();
                self.break_line();
                
                return st_while;
            },
            BREAK => { 
                self.eat(BREAK);
                self.break_line();
                return Ast::Break;
            },
            CONTINUE => {
                self.eat(CONTINUE);
                self.break_line();
                return Ast::Continue;
            }
            EOL => { //skip empty line
                self.break_line();
                return Ast::Nothing;
            }
            _ => self.declaration_statement(),
        };
    }
    
    pub fn statement_list(&mut self) -> LinkedList::<Box<Ast>> {
        let mut statements = LinkedList::<Box<Ast>>::new(); 
        
        while self.current_token.name != COMPOUND_END && self.current_token.name != EOF {
            let line = self.current_token.line;
            let statement = self.statement();
            
            if let Ast::Nothing = statement {
                continue;
            }
            
            statements.push_back(Box::new(Ast::Statement { line, statement: Box::new(statement) }));
        } 
        
        return statements;
    }
    
    pub fn parse(&mut self) -> LinkedList<Box<Ast>> { 
        return self.statement_list();
    }
}
