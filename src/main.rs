#[macro_use]
extern crate lazy_static;

mod div;
mod cli;

use cli::*;

lazy_static! {
    static ref APP: Module<Module<Cmd>> = Module {
        name: "App",
        operations: vec![
            div::init(),
        ]
    };
}

fn main() {
    Module::run(&APP);
}