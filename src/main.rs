#[macro_use]
extern crate lazy_static;

mod set;
mod div;
mod cli;

use cli::*;

lazy_static! {
    static ref APP: Module<Module<Cmd>> = Module {
        name: "krusty math",
        operations: vec![
            set::init(),
            div::init(),
        ]
    };
}

fn main() {
    Module::run(&APP);
}