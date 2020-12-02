use super::division::*;

pub fn to_decimal(n: &str, radix: u32) -> i64 {
    let decimal = i64::from_str_radix(n, radix);
    match decimal {
        Ok(n) => n,
        _ => 0,
    }
}

pub fn to_radix(n: i64, radix: u32) -> String {
    recursive_radix_div(n, radix, String::new())
}

fn recursive_radix_div(n: i64, radix: u32, result: String) -> String {
    let rad: i64 = radix.into();
    if n < rad {
        result + &n.to_string()
    } else {
        let (q, r) = complete_div(n, rad);
        recursive_radix_div(q, radix, result + &r.to_string())
    }
}