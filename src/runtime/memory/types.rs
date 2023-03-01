use std::alloc::Layout;

use super::{storage::Stack};

pub const FLOAT_LAYOUT: Layout = Layout::new::<f64>();
pub const BOOL_LAYOUT: Layout = Layout::new::<bool>();
pub const CHAR_LAYOUT: Layout = Layout::new::<char>();
pub const STACK_LAYOUT: Layout = Layout::new::<Stack>();
//pub const CLASS_LAYOUT: Layout = Layout::new::<ClassInstance>();

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Type {
    Null,
    Float,
    Char,
    Bool,
    Stack,
}

//виділяє всі поля
pub fn get_layout(t: &Type) -> Layout {
    return match t {
        Type::Float => FLOAT_LAYOUT,
        Type::Char => BOOL_LAYOUT,
        Type::Bool => CHAR_LAYOUT,
        Type::Stack => STACK_LAYOUT,
        Type::Null => panic!(),
    }
}

pub fn get_type(t: &String) -> Type {
    return match t.as_str() {
        "цифри" => Type::Float,
        "базар" => Type::Bool,
        "буква" => Type::Char,
        _ => Type::Stack,
    }
}