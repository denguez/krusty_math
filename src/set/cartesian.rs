use std::fmt;
use super::set::Element;
use super::set::Set;

#[derive(Debug)]
pub struct CartesianSet<T> where T: Element {
    el: Vec<(T, T)>
}

impl<T> CartesianSet<T> where T: Element {
    pub fn cardinal(&self) -> usize {
        self.el.len()
    }
}

pub fn product<T>(a: Set<T>, b: Set<T>) -> CartesianSet<T> where T: Element {
    let mut prod = Vec::new();
    for i in 0..a.cardinal() {
        for j in 0..b.cardinal() {
            prod.push((a.el(i), b.el(j)))
        }
    }
    CartesianSet { el: prod }
}

pub fn power_size<T>(prod: CartesianSet<T>) -> u32 where T: Element {
    (2 as u32).pow(prod.cardinal() as u32)
}

impl<T> fmt::Display for CartesianSet<T> where T: Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", &self.el)
    }
}
