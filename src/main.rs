use clap::Parser;

mod command;

use command::{Command, AppCommands};

fn main() {
    let arg = Command::parse();
    match arg.command {
        Some(i) => match i {
            AppCommands::User(user_command) => {
                match user_command.command {
                    Some(i) => match i {
                        command::user::UserSubCommand::Create(user) => {
                            println!("create a user with name: {}, age: {}", user.name, user.age)
                        }
                        command::user::UserSubCommand::Update => {
                            println!("Update command ")
                        }
                    },
                    None => {}
                }
            }
        },
        None => {
            println!("No command specified!")
        }
    }
}
