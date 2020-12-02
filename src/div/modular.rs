#![allow(non_snake_case)]
use std::fmt;
use super::{
    bezout,
    division::{
        rest,
        quotient,
    },
    prime::*,
};
use InverseMod::*;
use RestMod::*;
use FermatResult::*;

pub enum RestMod {
    Rest(i64), NotGreaterThan1
}

impl fmt::Display for RestMod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rest(r) => write!(f, "{}", r),
            NotGreaterThan1 => write!(f, "err: modulo < 2"),
        }
    }
}

pub fn modulus(n: i64, modulo: u32) -> RestMod {
    if modulo == 0 | 1 {
        NotGreaterThan1
    } else {
        Rest(rest(n, modulo as i64))
    }
}

pub enum InverseMod {
    Class(RestMod), NotCoprime(i64, i64)
}

pub fn inverse(a: i64, m: u32) -> InverseMod {
    if coprime(a, m as i64) {
        let (u, _) = bezout::equation(a, m as i64).solve();
        Class(modulus(u, m))
    } else {
        NotCoprime(a, m as i64)
    }
}

pub enum FermatResult {
    Ok(RestMod), NotPrime
}

pub fn fermat_rest(base: i64, exp: u32, modulo: u32) -> FermatResult {
    match is_prime(modulo) {
        true => {
            let r = rest(exp as i64, (modulo - 1) as i64) as u32;
            Ok(modulus(base.pow(r), modulo))
        },
        _ => NotPrime,
    }
}

pub fn euler_rest(base: i64, exp: u32, modulo: u32) -> RestMod {
    let totient = euler_totient(modulo);
    let r = rest(exp as i64, totient as i64) as u32;
    modulus(base.pow(r), modulo)
}

pub enum ChineseAlg {
    Result(RestMod, u32), 
    NonCoprimes(Vec<i64>)
}

pub fn chinese_algorithm(mods: &[u32], rests: &[i64]) -> ChineseAlg {
    let M = mods.iter().fold(1, |acc, x| acc * x);
    let mut result = 0;
    let mut nonCoprimes: Vec<i64> = Vec::new();

    for i in 0..mods.len() {
        let mi = mods[i];
        let mk = quotient(M as i64, mi as i64);
        match inverse(mk, mi) {
            Class(r) => match r {
                Rest(uk) => result += mk * uk * rests[i],
                _ => {}
            },
            NotCoprime(_, b) => nonCoprimes.push(b),
        }
    }
    if nonCoprimes.len() == 0 {
        ChineseAlg::Result(modulus(result, M), M) 
    } else {
        ChineseAlg::NonCoprimes(nonCoprimes)
    }
}