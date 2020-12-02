use std::fmt;
use super::{
    division::quotient,
    euclid::mcd,
    bezout,
};

pub fn solve(a: i64, b: i64, c: i64) -> Solution {
    let mcd = mcd(a, b);
    let (u, v) = bezout::equation(a, b).solve();
    let n1 = quotient(c, mcd);

    Solution {
        x: (n1 * u, quotient(b, mcd)),
        y: (n1 * v, quotient(a, mcd))
    }
}


pub struct Solution {
    x: (i64, i64),
    y: (i64, i64)
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x = {} - {}t ; y = {} + {}t", self.x.0, self.x.1, self.y.0, self.y.1)
    }
}