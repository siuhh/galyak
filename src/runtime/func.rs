use std::collections::LinkedList;

use crate::compiler::ast::Ast;

use super::memory::storage::Stack;

pub struct Func {
    todo: LinkedList<Box<Ast>>,
    stack: Stack,
}