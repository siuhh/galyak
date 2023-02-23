use std::{alloc::{Layout}};

use super::{stack::Stack, class::ClassInstance};

pub const FLOAT_LAYOUT: Layout = Layout::new::<f64>();
pub const BOOL_LAYOUT: Layout = Layout::new::<bool>();
pub const CHAR_LAYOUT: Layout = Layout::new::<char>();
pub const STACK_LAYOUT: Layout = Layout::new::<Stack>();
pub const CLASS_LAYOUT: Layout = Layout::new::<ClassInstance>();

#[derive(PartialEq, Clone, Copy)]
pub enum Type {
    Null,
    Float,
    Char,
    Bool,
    Class,
}

//виділяє всі поля
pub fn get_layout(t: &Type) -> Layout {
    return match t {
        Type::Float => FLOAT_LAYOUT,
        Type::Char => BOOL_LAYOUT,
        Type::Bool => CHAR_LAYOUT,
        Type::Class => STACK_LAYOUT,
        Type::Null => panic!(),
    }
}