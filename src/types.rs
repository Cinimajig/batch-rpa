use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
pub enum Type {
    None,
    String(String),
    Char(char),
    Number(NumberValue),
    Bool(bool),
    List(Vec<Type>),
    Object(HashMap<String, Type>),
}

#[derive(Debug)]
pub enum NumberValue {
    Float(f64),
    Signed(isize),
    Unsigned(usize),
}

impl FromStr for Type {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl FromStr for NumberValue {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
