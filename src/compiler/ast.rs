use std::collections::LinkedList;

use super::token::Token;
#[derive(Clone, Debug)]
pub enum Ast {
    Nothing,
    Num(f64),
    Keyword(String),
    String(String),
    Expression {
        left: Box<Ast>,  //NUMBER | VAR | ARIPH_EXPRESSION | STRING
        op: Token,       //+ - / *
        right: Box<Ast>, //NUMBER | VAR | ARIPH_EXPRESSION | STRING
    },
    DeclareVariable {
        array: bool,
        name: String,
        vtype: String,
        value: Box<Ast>,//Expression
    },
    SetVariable {
        name: String,
        value: Box<Ast>, //Expression
    },
    Return {
        expression: Box<Ast>, //Expression
    },
    CallFunc {
        name: String,
        args: LinkedList<Box<Ast>>, //Expression
    },
    Statement {
        line: usize,
        statement: Box<Ast>, // DecVar | Function | Class | SetVariable | CallFunc
    },
    Function {
        name: String,
        args: LinkedList<(String, String)>,//type + name
        return_type: String,
        compound_statement: LinkedList<Box<Ast>>, //StatementList
    },
    Class {
        name: String,
        args: LinkedList<(String, String)>,
        return_type: String,
        compound_statement: LinkedList<Box<Ast>>, //Declarations only
    },
}
pub fn deref_ast(ast: &Box<Ast>) -> Ast {
    return *(*ast).clone();
}