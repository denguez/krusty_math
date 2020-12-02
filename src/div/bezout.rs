use std::fmt;
use super::{
    euclid::euclid,
    division::quotient,
};

pub fn equation(a: i64, b: i64) -> Bezout {
    let mut bezout = Bezout::init();
    euclid((a, b), |_| {}, |n, rest| {
        match rest {
            0 => {},
            _ => {
                bezout.a.push(n.0);
                bezout.b.push(n.1);
                bezout.c.push(rest);
                bezout.v.push(-(quotient(n.0, n.1)));
            }
        }
    });
    bezout
}

pub struct Bezout {
    a: Vec<i64>,
    b: Vec<i64>,
    c: Vec<i64>,
    v: Vec<i64>,
}

impl Bezout {
    pub fn init() -> Self {
        Bezout{ a: Vec::new(), b: Vec::new(), c: Vec::new(), v: Vec::new() }
    }

    fn size(&self) -> usize {
        self.a.len()
    }

    fn last(&mut self) -> (i64, i64) {
        match self.v.pop() {
            Some(v) => {
                self.a.pop();
                self.b.pop();
                (1, v)
            },
            None => (0, 0),
        }
    }

    pub fn solve(&mut self) -> (i64, i64) {
        let (mut u, mut v) = self.last();
        for i in (0..self.size()).rev() {
            let temp_u = u;
            u = v;
            v = v * self.v[i] + temp_u;
        }
        (u, v)
    }
}

impl fmt::Display for Bezout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut format = String::new();
        for i in 0..self.size() {
            format += &(
                " · ".to_owned() 
                + &self.c[i].to_string() + " = "
                + &self.a[i].to_string() + " + "
                + &self.b[i].to_string() + "("
                + &self.v[i].to_string() + ")"
            );
            if i != self.size() - 1 {
                format += "\n"
            }
        }
        write!(f, "{}", format)
    }
} 