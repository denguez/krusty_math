use std::{
    fmt,
    convert::TryInto
};
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

    pub fn elements(el: Vec<T>) -> Self {
        Set { el: el }
    }

    pub fn power(set: Self) -> Set<Self> {
        let p_cardinal = (2 as usize).pow(set.len().try_into().unwrap());
        let mut power: Set<Set<T>> = Set::of_size(p_cardinal);
        for i in 0..power.len() {
            let mut row: Set<T> = Self::of_size(1);
            for n in 0..set.len() {
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
    fn len(&self) -> usize {
        self.el.capacity()
    }

    fn el(&self, index: usize) -> T {
        self.el[index].clone()
    }

    fn add(&mut self, el: T) {
        self.el.push(el)
    }
}

pub fn subsets_of_len(n: u32, r: u32) -> u32 {
    combinatory(n, r)
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
    let mut string = el.iter().fold("[".to_string(), |acc, e| {
        acc + &e.to_string() + ","
    });
    string.pop();
    string + "]"
}

impl Element for String {
    fn to_string(&self) -> String {
        self.clone()
    }
}


