use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
pub enum Type {
    None,
    String(String),
    Char(char),
    Number(f64),
    Bool(bool),
    List(Vec<Type>),
    Object(HashMap<String, Type>),
}

// #[derive(Debug)]
// pub enum NumberValue {
//     Float(f64),
//     Signed(isize),
//     Unsigned(usize),
// }

impl FromStr for Type {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

// impl FromStr for NumberValue {
//     type Err = std::num::ParseIntError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         if let Ok(float) = s.parse::<f64>() {
//             return Ok(NumberValue::Float(float));
//         }
//         if let Ok(int_t) = s.parse::<usize>() {
//             return Ok(NumberValue::Unsigned(int_t));
//         }
//         let result = s.parse::<isize>();

//         if result.is_err() {
//             return Err(result.unwrap_err());
//         }
        
//         Ok(NumberValue::Signed(result.unwrap()))
//     }
// }
