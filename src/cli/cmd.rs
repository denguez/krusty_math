use super::*;
use super::module::title;

pub type Cmd = Command<fn(ArgList)>;

pub struct Command<F> where F: Fn(ArgList) {
    pub name: String,
    pub expression: String,
    operation: F,
    params: Vec<Param>,
}

impl<F> Command<F> where F: Fn(ArgList) {
    pub fn of(name: &str, expression: &str, operation: F, params: Vec<Param>) -> Self {
        Command { 
            name: String::from(name), 
            expression: String::from(expression), 
            operation: operation,
            params: params,
         }
    }

    pub fn binary(name: &str, expression: &str, operation: F) -> Self {
        Self::of(name, expression, operation, vec![int("a"), int("b")])
    }

    fn not_empty(val: &InputValue) -> bool {
        match val {
            Empty => false,
            _ => true
        }
    }
}

impl CliModule for Cmd {
    fn name(&self) -> &str {
        &self.name
    }

    fn execute(&self) -> State {
        title(&self.name());
        println!("=> {}", self.expression);
        
        let values = ArgList::init(&self.params);
        if values.iter().all(Self::not_empty) {
            (self.operation)(values); 
        } else {
            for val in values.iter() {
                match val {
                    InputValue::Empty => {
                        println!("Missing parameter {:?}", val)
                    },
                    _ => continue
                }
            }
        }
        State::Loop
    }
}



