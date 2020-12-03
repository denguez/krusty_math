use super::*;

use State::*;

pub enum State {
    Loop, Exit, Detach
}

pub trait CliModule {
    fn name(&self) -> &str;
    fn execute(&self) -> State;
}

pub struct Module<T: CliModule> {
    pub name: &'static str,
    pub operations: Vec<T>,
}

impl<T: CliModule> Module<T> {
    pub fn run(module: &Self) -> State {
        module.promt_options()
    }
}

impl<T: CliModule> Module<T> {
    fn print_menu(&self) {
        title(self.name);
        println!("(q)uit");
        if self.name != "krusty math" {
            println!("(b)ack");
        }
        self.operations.iter().enumerate().for_each(|(i, op)|
            println!("{}. {}", i + 1, op.name())
        );
    }

    fn promt_options(&self) -> State {
        self.print_menu();
        match io::input_u32("Enter option") {
            UInt(u) => match self.run_operation(u as usize) {
                Loop => self._continue(u as usize),
                Detach => self.promt_options(),
                _ => Exit
            },
            Quit => {
                println!("bye");
                Exit
            },
            Back => Detach,
            _ => self.promt_options(),
        }
    }

    fn run_operation(&self, index: usize) -> State {
        match index {
            0 => self.retry_options(index),
            i => match self.operations.get(i - 1) {
                Some(cmd) => cmd.execute(),
                None => self.retry_options(index)
            }
        }
    }

    fn retry_options(&self, index: usize) -> State {
        invalid_option(index);
        self.promt_options()
    }

    fn _continue(&self, index: usize) -> State {
        println!("Press enter to try again or any key to go back");
        match io::nextln() {
            Ok(ln) if ln == "" => match self.run_operation(index) {
                Loop => self._continue(index),
                state => state,
            }, 
            Ok(_) => self.promt_options(),
            Err(e) => {
                io::error(e);
                Loop
            }
        }
    }
}

impl CliModule for Module<Cmd> {
    fn name(&self) -> &str {
        self.name
    }

    fn execute(&self) -> State {
        Self::run(self)
    }
}

pub fn title(t: &str) {
    println!("---------------- {} ---------------", t);
}

fn invalid_option(i: usize) {
    println!("Invalid option {}", i);
}