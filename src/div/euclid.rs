use super::division::{
    rest, quotient
};

type IntPair = (i64, i64);

pub fn mcd(d: i64, div: i64) -> i64 {
    euclid((d, div), |n| n.0, |_, _| {})
}

pub fn mcm(a: i64, b: i64) -> i64 {
    quotient(a * b, mcd(a, b))
}

pub fn euclid<F, E, R>(n: IntPair, end: E, mut next: F) -> R
where F: FnMut(IntPair, i64), E: Fn(IntPair) -> R {
    match &n.1 {
        0 => end(n),
        _ => {
            let r = rest(n.0, n.1);
            next(n, r);
            euclid((n.1, r), end, next)
        }
    }
}