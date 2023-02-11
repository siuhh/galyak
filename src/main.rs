#![allow(dead_code)]

mod error_mgr;
mod pre;
mod runtime;
mod test;

fn main() {
    test::lexer::test();
}
