use std::alloc::{alloc, dealloc};

use super::types::Type;
pub struct Var<'a> {
    pub name: String,
    pub alive: bool,
    pub _type: &'a Type,
    pub mem_addr: *mut u8,
}

impl<'a> Var<'a> {
    pub unsafe fn init(_type: &'a Type, name: String) -> Var {
        let addr = alloc(_type.layout);

        return Var {
            name: name,
            alive: true,
            _type,
            mem_addr: addr,
        };
    }

    pub unsafe fn kill(&mut self) {
        self.alive = false;
        dealloc(self.mem_addr, self._type.layout);
    }
}
