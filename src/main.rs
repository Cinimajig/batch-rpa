mod parser;
mod command;
mod types;
mod variables;

mod commands;

use parser::*;
use command::*;
use types::*;
use variables::*;

fn main() {
    println!("{:?}", VariableManager::new());
}
