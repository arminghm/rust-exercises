use clap::{Parser, Subcommand};

pub mod user;
use user::UserCommand;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Command {
    #[command(subcommand)]
    pub command: Option<AppCommands>,
}

#[derive(Debug, Subcommand)]
pub enum AppCommands {
    User(UserCommand),
}
