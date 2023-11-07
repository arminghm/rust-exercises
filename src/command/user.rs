use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct UserCommand {
    #[command(subcommand)]
    pub command: Option<UserSubCommand>,
}

#[derive(Debug, Subcommand)]
pub enum UserSubCommand {
    Create(CreateUser),
    Update,
}

#[derive(Debug, Args)]
pub struct CreateUser {
    #[arg(short, long)]
    pub name: String,
    #[arg(short, long)]
    pub age: u32,
}
