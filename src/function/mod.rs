
pub struct Producer<T: 'static>(Box<dyn Fn() -> T>);

impl<T> Producer<T> {
    /*pub fn of(value: T) -> Self {
        Producer(Box::new(|| value))
    }*/

    /*pub fn from(f: &'static dyn Fn() -> T) -> Self {
        Producer(Box::new(f))
    }*/

    pub fn unwrap(&self) -> T {
        self.0()
    }
}

pub struct Function<I, O>(pub Box<dyn Fn(I) -> O>);

impl<I, O> Function<I, O> {
    pub fn of(f: &'static dyn Fn(I) -> O) -> Self {
        Function(Box::new(f))
    }

    pub fn apply(&self, x: I) -> O {
        self.0(x)
    }
}