use std::fmt;
use super::factorial::combinatory;

pub trait Element: Clone + fmt::Display + fmt::Debug {
    fn to_string(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct Set<T> where T: Element {
    pub el: Vec<T>,
}

impl<T> Set<T> where T: Element {
    pub fn of_size(len: usize) -> Self {
        Set { el: Vec::with_capacity(len) }
    }

    pub fn of_elements(el: Vec<T>) -> Self {
        Set { el: el }
    }

    pub fn power(set: Self) -> Set<Self> {
        let p_cardinal = (2 as usize).pow(set.cardinal() as u32);
        let mut power: Set<Set<T>> = Set::of_size(p_cardinal);
        for i in 0..power.cardinal() {
            let mut row: Set<T> = Self::of_size(1);
            for n in 0..set.cardinal() {
                if (i >> n) % 2 == 1 {
                    row.add(set.el(n))
                }
            }
            power.add(row)
        }
        power
    }
}

impl<T> Set<T> where T: Element {
    pub fn cardinal(&self) -> usize {
        self.el.capacity()
    }

    pub fn el(&self, index: usize) -> T {
        self.el[index].clone()
    }

    fn add(&mut self, el: T) {
        self.el.push(el)
    }

    pub fn subsets_of_size(&self, r: u32) -> u32 {
        combinatory(self.cardinal() as u32, r)
    }
}

impl<T> fmt::Display for Set<T> where T: Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", string_vec(&self.el))
    }
}

impl<T> Element for Set<T> where T: Element {
    fn to_string(&self) -> String {
        string_vec(&self.el)
    }
}

fn string_vec(el: &[impl Element]) -> String {
    let mut string = el.iter().fold("{".to_string(), |acc, e| {
        acc + &e.to_string() + ","
    });
    string.pop();
    string + "}"
}

impl Element for String {
    fn to_string(&self) -> String {
        self.clone()
    }
}


