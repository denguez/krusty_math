

pub fn combinatory(n: u32, r: u32) -> u32 {
    factorial(n) / (factorial(r) * factorial(n - r))
}


pub fn factorial(n: u32) -> u32 {
    factorial_rec(n - 1, n)
}

fn factorial_rec(n: u32, res: u32) -> u32 {
    match n {
        0 | 1 => res,
        _ => factorial_rec(n - 1, n * res)
    }
}