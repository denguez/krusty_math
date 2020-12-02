use std::io::{
    stdin, 
    stdout,
    Error,
    Write
};
use InputValue::*;

#[derive(Debug, Clone)]
pub enum InputValue {
    Int(i64), UInt(u32), Str(String), 
    IntList(Vec<i64>), UIntList(Vec<u32>),
    Empty, Quit, Back
}

pub fn input_i64(arg: &str) -> InputValue {
    input(arg, parse_i64)
}

pub fn input_u32(arg: &str) -> InputValue {
    input(arg, parse_u32)
}

pub fn input_str(arg: &str) -> InputValue {
    input(arg, |_, val| Str(val.to_string()))
}

pub fn input_vec_i64(arg: &str) -> InputValue {
    input(arg, |_, val| {
        let parse = |s: &str| s.parse::<i64>().unwrap();
        let list: Vec<i64> = val.split(" ").map(parse).collect();
        IntList(list)
    })
}

pub fn input_vec_u32(arg: &str) -> InputValue {
    input(arg, |_, val| {
        let parse = |s: &str| s.parse::<u32>().unwrap();
        let list: Vec<u32> = val.split(" ").map(parse).collect();
        UIntList(list)
    })
}

fn input(arg: &str, parse: fn(&str, &str) -> InputValue) -> InputValue {
    request_input(arg);
    match nextln() {
        Ok(ln) if ln == "q" => Quit,
        Ok(ln) if ln == "b" => Back,
        Ok(ln) => parse(arg, &ln),
        Err(e) => {
            error(e);
            Empty
        }
    }
}

fn parse_i64(arg: &str, val: &str) -> InputValue {
    match val.parse::<i64>() {
        Ok(i) => Int(i),
        _ => {
            invalid("int", val);
            try_again();
            input_i64(arg)
        }
    }
}

fn parse_u32(arg: &str, val: &str) -> InputValue {
    match val.parse::<u32>() {
        Ok(u) => UInt(u),
        _ => {
            invalid("uint", &val);
            try_again();
            input_u32(arg)
        }
    }
}

fn request_input(req: &str) {
    print!("{}: ", req);
    stdout().flush().unwrap();
}

pub fn nextln() -> Result<String, Error> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    Ok(buffer.trim_end().to_string())
}

fn invalid(kind: &str, v: &str) {
    println!("Invalid {} {}", kind, v);
}

fn try_again() {
    println!("Try again? Press 'q' to exit");
}

pub fn error(e: Error) {
    println!("Error {}", e);
}