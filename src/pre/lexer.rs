use crate::error_mgr::Caller;

use super::token::{
    self,
    tokens::{self, dynamic::UNKNOWN},
    Token, TokenType,
};

pub struct Lexer<'a> {
    file: &'static str,
    pos: usize,
    curr_char: char,
    curr_line: usize,
    line_char: usize,
    error_caller: &'a Caller,
    current_token: Token,
}

fn is_ariphmetic_op(ch: &char) -> bool {
    return ['+', '-', '/', '*'].contains(ch);
}

fn is_symbol(ch: &char) -> bool {
    return ['(', ')', '>', '=', '<', '!'].contains(ch);
}

impl<'a> Lexer<'a> {
    pub fn new(file: &'static str, caller: &'a Caller) -> Lexer<'a> {
        let mut lexer = Lexer {
            file: file,
            pos: 0,
            curr_char: '\0',
            curr_line: 1,
            line_char: 1,
            error_caller: caller,
            current_token: UNKNOWN,
        };

        //init first char
        lexer.curr_char = lexer.at_pos(0);
        return lexer;
    }

    fn at_pos(&self, pos: usize) -> char {
        return self.file.chars().nth(pos).unwrap();
    }

    fn advance(&mut self) {
        self.line_char += 1;
        self.pos += 1;

        if self.eof() {
            return;
        }
        self.curr_char = self.at_pos(self.pos);

        if self.curr_char == '\n' {
            self.curr_line += 1;
            self.line_char = 0;
        }
    }

    fn eof(&self) -> bool {
        return self.pos >= self.file.chars().count();
    }

    fn space(&self) -> bool {
        return self.curr_char == ' ' || self.curr_char == '\n' || self.curr_char == '\t';
    }

    fn tok_inst(&self, line: usize, ch: usize, typ: TokenType, val: String) -> Token {
        return Token::new(typ, val, line, ch);
    }

    fn get_number_token(&mut self) -> Token {
        let l = self.curr_line;
        let c = self.line_char;

        let mut num: String = String::new();

        while !self.eof() && (self.curr_char.is_numeric() || self.curr_char == '.') {
            num.push(self.curr_char);
            self.advance();
        }

        return self.tok_inst(l, c, tokens::dynamic::NUMBER, num);
    }

    fn get_word(&mut self) -> String {
        let mut word: String = String::new();

        while !self.eof() && self.curr_char.is_alphabetic() {
            word.push(self.curr_char);
            self.advance();
        }

        return word;
    }

    fn get_static_token(&mut self) -> Token {
        let l = self.curr_line;
        let c = self.line_char;

        let word = self.get_word();

        for st in token::STATIC_TOKENS {
            if st.to_string() == word {
                return self.tok_inst(l, c, st, word);
            }
        }

        return self.tok_inst(l, c, tokens::dynamic::NAME, word);
    }

    fn get_str_token(&mut self) -> Token {
        if self.curr_char == '\"' {
            self.advance();
        }

        let l = self.curr_line;
        let c = self.line_char;

        let mut str_val = String::new();

        while self.curr_char != '\"' {
            str_val.push(self.curr_char);
            self.advance();
            if self.eof() || self.curr_char == '\n' {
                //TODO: call unmatched quote error
                break;
            }
        }

        self.advance();

        return self.tok_inst(l, c, tokens::dynamic::STR, str_val);
    }

    fn get_symbol_token(&mut self) -> Token {
        let word = self.curr_char.to_string();

        let l = self.curr_line;
        let c = self.line_char;

        self.advance();

        for st in token::STATIC_TOKENS {
            if st.to_string() == word {
                return self.tok_inst(l, c, st, word);
            }
        }
        if true {
        } else {
        }

        return self.tok_inst(l, c, tokens::dynamic::UNKNOWN, word);
    }

    fn skip_space(&mut self) {
        while !self.eof() && self.space() {
            self.advance();
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_space();

        if self.eof() {
            return self.tok_inst(
                self.curr_line,
                self.line_char,
                tokens::stat::EOF,
                String::from("eof"),
            );
        }

        let curr = self.curr_char;

        if curr == '\"' {
            return self.get_str_token();
        }

        if curr.is_alphabetic() {
            return self.get_static_token();
        }

        if is_symbol(&curr) {
            return self.get_symbol_token();
        }

        if curr.is_numeric() {
            return self.get_number_token();
        }

        if is_ariphmetic_op(&curr) {
            self.advance();
            return self.tok_inst(
                self.curr_line,
                self.line_char - 1,
                tokens::dynamic::ARIPH_OP,
                String::from(curr),
            );
        }

        return self.tok_inst(
            self.curr_line,
            self.line_char,
            tokens::dynamic::UNKNOWN,
            String::from(curr),
        );
    }
}
