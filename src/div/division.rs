
pub fn rest(n: i64, div: i64) -> i64 {
    match div {
        0 => 0,
        _ => n.rem_euclid(div)
    }
}

pub fn quotient(n: i64, div: i64) -> i64 {
    n.div_euclid(div)
}

pub fn complete_div(n: i64, div: i64) -> (i64, i64) {
    (quotient(n, div), rest(n, div))
}