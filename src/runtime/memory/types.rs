use std::alloc::Layout;

use crate::{runtime::func::GlkFuncDeclaration, compiler::token::tokens::stat::{NULL, NUM, BOOL, CHAR}};

use super::{storage::GlkStack, list::GlkList};

pub const FLOAT_LAYOUT: Layout = Layout::new::<f64>();
pub const BOOL_LAYOUT: Layout = Layout::new::<bool>();
pub const CHAR_LAYOUT: Layout = Layout::new::<char>();
pub const STACK_LAYOUT: Layout = Layout::new::<GlkStack>();
//pub const CLASS_LAYOUT: Layout = Layout::new::<ClassInstance>();
pub const FUNC_LAYOUT: Layout = Layout::new::<GlkFuncDeclaration>();
pub const LIST_LAYOUT: Layout = Layout::new::<GlkList>();
pub const STRING_LAYOUT: Layout = Layout::new::<String>();


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Type {
    Null,
    Number,
    Char,
    Bool,
    Stack,
    Func,
    List,
    String,
    Class
}

//виділяє всі поля
pub fn get_layout(t: &Type) -> Layout {
    return match t {
        Type::Number => FLOAT_LAYOUT,
        Type::Char   => BOOL_LAYOUT,
        Type::Bool   => CHAR_LAYOUT,
        Type::Stack  => STACK_LAYOUT,
        Type::Func   => FUNC_LAYOUT,
        Type::List   => LIST_LAYOUT,
        Type::String => STRING_LAYOUT,
        Type::Class  => todo!(),
        Type::Null   => panic!(),
    }
}

pub fn get_type(t: &String) -> Type {
    return match t.as_str() {
        NUM => Type::Number,
        BOOL => Type::Bool,//TODO!
        CHAR => Type::Char,
        NULL => Type::Null,
        _ => Type::Class,
    }
}

pub fn get_type_name(t: &Type) -> String {
    return match t {
        Type::Null   => NULL.to_string(),
        Type::Number => NUM.to_string(),
        Type::Char   => CHAR.to_string(),
        Type::Bool   => BOOL.to_string(),
        Type::Stack  => todo!(),
        Type::Func   => todo!(),
        Type::List   => todo!(),
        Type::String => todo!(),
        Type::Class  => todo!(),
    }
}