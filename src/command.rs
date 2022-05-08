use std::collections::HashMap;

use crate::types::Type;

// pub struct Group(String);
// pub struct Options(String, Option<Type>);
// pub struct Arg(String);

// pub struct ArgValue(String, Type);
// pub struct RestArgs(Vec<Type>);

pub type Args = std::collections::HashMap<&'static str, Option<types::Type>>;
pub type RestArgs = Vec<types::Type>;
pub type Flags = HashMap<&'static str, Option<Type>>;

pub trait Command {
    const NAME: &'static str;

    fn execute(&self) -> Type;
    fn get_args(&self) -> Args;
    fn get_rest(&self) -> RestArgs;
    fn get_flags(&self) -> Flags;
    fn can_run(&self) -> bool;
    fn get_subcommands(&self) -> Vec<Command>;
}

// pub struct CommandGroup(String, Vec<impl Command>);
