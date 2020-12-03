#[derive(Debug, Clone)]
pub enum ParamType {
    Int, UInt, Str, 
    IntList, UIntList, StrList
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub _type: ParamType,
}

impl Param {
    fn from(name: &str, _type: ParamType, ) -> Self {
        Param { name: String::from(name), _type: _type }
    }
}

pub fn int(name: &str) -> Param {
    Param::from(name, ParamType::Int)
}

pub fn int_list(name: &str) -> Param {
    Param::from(name, ParamType::IntList)
}

pub fn uint(name: &str) -> Param {
    Param::from(name, ParamType::UInt)
}

pub fn uint_list(name: &str) -> Param {
    Param::from(name, ParamType::UIntList)
}

pub fn str(name: &str) -> Param {
    Param::from(name, ParamType::Str)
}

pub fn str_list(name: &str) -> Param {
    Param::from(name, ParamType::StrList)
}