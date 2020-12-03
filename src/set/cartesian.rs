use std::fmt;
use super::set::Element;
use super::set::Set;

#[derive(Debug)]
pub struct CartesianSet<T> where T: Element {
    el: Vec<(T, T)>
}

pub fn product<T>(a: Set<T>, b: Set<T>) -> CartesianSet<T> where T: Element {
    let mut prod = Vec::new();
    for i in 0..a.len() {
        for j in 0..b.len() {
            prod.push((a.el(i), b.el(j)))
        }
    }
    CartesianSet { el: prod }
}

impl<T> fmt::Display for CartesianSet<T> where T: Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", &self.el)
    }
}
