use std::convert::TryInto;
use super::euclid::mcd;

pub fn is_prime(n: u32) -> bool {
    match n {
        0..=1 => false,
        _ => (2..n).all(|a| n % a != 0)
    }
}

pub fn coprime(a: i64, b: i64) -> bool {
    mcd(a, b) == 1
}

pub fn euler_totient(n: u32) -> u32 {
    // Create and initialize an array to store phi or totient values 
    let mut phi: Vec<u32> = (0..=n).map(|i| i).collect();
    // Compute other Phi values 
    for p in 2..phi.len() {
        let p32 = p as u32;
        // If phi[p] is not computed already, then number p is prime 
        if phi[p] == p32 {
            // Phi of a prime number p is always equal to p-1. 
            phi[p] = p32 - 1; 
            // Update phi values of all multiples of p
            let mut i = 2 * p;
            while i <= n.try_into().unwrap() {
                phi[i] = (phi[i] / p32) * (p32 - 1); 
                i += p;
            }
        }
    }
    phi[n as usize]
}