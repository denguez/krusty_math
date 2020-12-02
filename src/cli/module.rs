use super::{
    arg::ArgValue,
    io,
    io::{
        State,
        State::*,
    },
    cmd::Cmd
};

pub trait CliModule {
    fn name(&self) -> &str;
    fn execute(&self) -> State;
    fn expression(&self) -> String;
}

pub struct Module<T: CliModule> {
    pub name: &'static str,
    pub operations: Vec<T>,
}

impl<T: CliModule> Module<T> {
    pub fn run(module: &Self) -> State {
        module.promt_options()
    }

    fn print_menu(&self) {
        title(self.name);
        self.operations.iter().enumerate().for_each(|(i, op)|
            println!("{}. {}", i + 1, op.name())
        );
        println!("(q)uit");
        println!("(b)ack")
    }

    fn promt_options(&self) -> State {
        self.print_menu();
        match io::input_u32("Enter option") {
            ArgValue::UInt(u) => match self.run_operation(u as usize) {
                Loop => self._continue(u as usize),
                Back => self.promt_options(),
                _ => Exit
            },
            ArgValue::Quit => {
                println!("bye");
                Exit
            },
            ArgValue::Back => Back,
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

    fn expression(&self) -> String {
        "".to_string()
    }
}

pub fn title(t: &str) {
    println!("---------------- {} ---------------", t);
}

fn invalid_option(i: usize) {
    println!("Invalid option {}", i);
}