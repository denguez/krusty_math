use std::slice::Iter;
use super::{
    io::*,
    param::*
};

use ArgValue::*;
use ArgTuplet::*;

#[derive(Debug, Clone)]
pub enum ArgValue {
    Int(i64), UInt(u32), Str(String), 
    IntList(Vec<i64>), UIntList(Vec<u32>),
    Empty, Quit, Back
}

#[derive(Debug, Clone)]
pub enum ArgTuplet {
    Zero, 
    One(ArgValue), 
    Two(ArgValue, ArgValue), 
    Three(ArgValue, ArgValue, ArgValue)
}

pub struct ArgList { 
    pub vec: Vec<ArgValue> 
}

impl ArgList {
    pub fn init(params: &[Param]) -> ArgList {
        let mut values: Vec<ArgValue> = Vec::new();
        for p in params {
            match p._type {
                ParamType::Int => values.push(input_i64(&p.name)),
                ParamType::UInt => values.push(input_u32(&p.name)),
                ParamType::Str => values.push(input_str(&p.name)),
                ParamType::IntList => values.push(input_vec_i64(&p.name)),
                ParamType::UIntList => values.push(input_vec_u32(&p.name)),
            }
        }
        ArgList { vec: values }
    }
}

impl ArgList {
    pub fn unwrap(self) -> ArgTuplet {
        match &self.vec.len() {
            1 => match &self.vec[0] {
                Empty => Zero,
                a => One(a.clone()),
            },
            2 => match (&self.vec[0], &self.vec[1]) {
                (Empty, _) | (_, Empty) => Zero,
                (a, b) => Two(a.clone(), b.clone()),
            },
            3 => match (&self.vec[0], &self.vec[1], &self.vec[2]) {
                (Empty, ..) | (.., Empty) | (_, Empty, _) => Zero,
                (a, b, c) => Three(a.clone(), b.clone(), c.clone()),
            },
            _ => Zero
        }
    }

    pub fn iter(&self) -> Iter<'_, ArgValue> {
        self.vec.iter()
    }
}