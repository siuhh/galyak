use std::alloc::Layout;

use crate::runtime::func::GlkFuncDeclaration;

use super::{list::GlkList, storage::GlkStack};

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
    Class,
}

//виділяє всі поля
pub fn get_layout(t: &Type) -> Layout {
    return match t {
        Type::Number => FLOAT_LAYOUT,
        Type::Char => BOOL_LAYOUT,
        Type::Bool => CHAR_LAYOUT,
        Type::Stack => STACK_LAYOUT,
        Type::Func => FUNC_LAYOUT,
        Type::List => LIST_LAYOUT,
        Type::String => STRING_LAYOUT,
        Type::Class => todo!(),
        Type::Null => panic!(),
    };
}

pub const T_NULL: &str = "галяк";
pub const T_NUM: &str = "цифри";
pub const T_STR: &str = "букви";
pub const T_BOOL: &str = "факт";
pub const T_CHAR: &str = "буква";
pub const T_STRING: &str = "букви";

pub fn get_type(t: &String) -> Type {
    return match t.as_str() {
        T_NUM => Type::Number,
        T_BOOL => Type::Bool, //TODO!
        T_CHAR => Type::Char,
        T_NULL => Type::Null,
        T_STRING => Type::String,
        _ => Type::Class,
    };
}

pub fn get_type_name(t: &Type) -> String {
    return match t {
        Type::Null => T_NULL.to_string(),
        Type::Number => T_NUM.to_string(),
        Type::Char => T_CHAR.to_string(),
        Type::Bool => T_BOOL.to_string(),
        Type::Stack => todo!(),
        Type::Func => todo!(),
        Type::List => todo!(),
        Type::String => T_STRING.to_string(),
        Type::Class => todo!(),
    };
}
