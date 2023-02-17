use std::alloc::Layout;

// pub trait Type {
//     fn get_layout(&self) -> Layout;
// }

pub struct Type {
    pub name: &'static str,
    pub layout: Layout,
}
// impl Type for ClassT {
//     fn get_layout(&self) -> Layout {
//         return self.layout;
//     }
// }

pub mod bt {
    use std::alloc::Layout;

    use super::Type;

    pub const CHAR: &Type = &Type {
        name: "базар",
        layout: Layout::new::<char>(),
    };

    pub const NUMBER: &Type = &Type {
        name: "цифри",
        layout: Layout::new::<f64>(),
    };

    pub const BOOL: &Type = &Type {
        name: "бул",
        layout: Layout::new::<bool>(),
    };
}
