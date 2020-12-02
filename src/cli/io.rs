use std::io::{
    stdin, 
    stdout,
    Error,
    Write
};
use super::arg::*;

pub enum State {
    Loop, Exit, Back
}

pub fn input_i64(arg: &str) -> ArgValue {
    input(arg, parse_i64)
}

pub fn input_u32(arg: &str) -> ArgValue {
    input(arg, parse_u32)
}

pub fn input_str(arg: &str) -> ArgValue {
    input(arg, |_, val| ArgValue::Str(val.to_string()))
}

pub fn input_vec_i64(arg: &str) -> ArgValue {
    input(arg, |_, val| {
        let parse = |s: &str| s.parse::<i64>().unwrap();
        let list: Vec<i64> = val.split(" ").map(parse).collect();
        ArgValue::IntList(list)
    })
}

pub fn input_vec_u32(arg: &str) -> ArgValue {
    input(arg, |_, val| {
        let parse = |s: &str| s.parse::<u32>().unwrap();
        let list: Vec<u32> = val.split(" ").map(parse).collect();
        ArgValue::UIntList(list)
    })
}

fn input(arg: &str, parse: fn(&str, &str) -> ArgValue) -> ArgValue {
    request_input(arg);
    match nextln() {
        Ok(ln) if ln == "q" => ArgValue::Quit,
        Ok(ln) if ln == "b" => ArgValue::Back,
        Ok(ln) => parse(arg, &ln),
        Err(e) => {
            error(e);
            ArgValue::Empty
        }
    }
}

fn parse_i64(arg: &str, val: &str) -> ArgValue {
    match val.parse::<i64>() {
        Ok(i) => ArgValue::Int(i),
        _ => {
            invalid("int", val);
            try_again();
            input_i64(arg)
        }
    }
}

fn parse_u32(arg: &str, val: &str) -> ArgValue {
    match val.parse::<u32>() {
        Ok(u) => ArgValue::UInt(u),
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