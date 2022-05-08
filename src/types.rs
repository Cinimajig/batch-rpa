use std::{collections::HashMap, str::FromStr};

use windows::Win32::Foundation::HWND;

#[derive(Debug)]
pub enum Type {
    None,
    String(String),
    Char(char),
    Number(NumberValue),
    Bool(bool),
    List(Vec<Type>),
    Object(HashMap<String, Type>),
    HWND(HWND),
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

impl Type {
    pub fn type_name(&self) -> &'static str {
        match self {
            Type::None => "None",
            Type::String(_) => "String",
            Type::Char(_) => "Char",
            Type::Number(_) => "Number",
            Type::Bool(_) => "Bool",
            Type::List(_) => "List",
            Type::Object(_) => "Object",
            Type::HWND(_) => "HWND",
            _ => "Unnkown",
        }
    }
}

type NumberValue = isize; 

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
