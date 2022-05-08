use crate::{command::*, types};

#[derive(Debug)]
pub struct Var;

impl Command for Var {
    const NAME: &'static str = "VAR";

    fn execute(&self) -> types::Type {
        Type::String("VAR\n\tTYPE")
    }

    fn get_subcommands(&self) -> Vec<Command> {
        todo!()
    }

    fn get_args(&self) -> Args {
        Args::new()
    }

    fn get_rest(&self) -> RestArgs {
        vec![]
    }

    fn get_flags(&self) -> Flags {
        Flags::new()
    }

    fn can_run(&self) -> bool {
        true
    }
}

#[derive(Debug)]
pub struct VarGet;

impl Command for VarType {
    const NAME: &'static str = "VAR TYPE";

    fn execute(&self) -> types::Type {
        match self.get_args().get("name").unwrap_or_default() {
            
        }
    }

    fn get_args(&self) -> Args {
        todo!()
    }

    fn get_rest(&self) -> RestArgs {
        todo!()
    }

    fn get_flags(&self) -> Flags {
        todo!()
    }

    fn can_run(&self) -> bool {
        todo!()
    }

    fn get_subcommands(&self) -> Vec<Command> {
        todo!()
    }
}
