use super::{Expect, ObjectType};
use crate::runtime::constructions::construction::Construction;
use crate::runtime::constructions::object::Object;

pub struct ArgsExpectation {
    template: &'static Construction,
}
impl Expect for ArgsExpectation {
    fn get_type(&self) -> ObjectType {
        ObjectType::Args
    }
    fn
}
pub struct ArgsObject {
    template: &'static Construction,
}
impl Object for ArgsObject {
    fn get_type(&self) -> ObjectType {
        ObjectType::Args
    }
}
