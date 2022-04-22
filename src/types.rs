use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
pub enum Type {
    None,
    String(String),
    Char(char),
    Number(isize),
    Bool(bool),
    List(Vec<Type>),
    Object(HashMap<String, Type>),
}

impl FromStr for Type {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
