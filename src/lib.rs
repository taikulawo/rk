use clap::Command;

pub fn basic_command(name: &str) -> Command {
    return Command::new(name.to_string()).arg_required_else_help(true);
}
