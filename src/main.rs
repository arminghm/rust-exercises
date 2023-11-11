use clap::Parser;

mod command;
use command::{AppCommands, Command};

fn main() {
  println!("{}", reverse("input"))
}

#[allow(dead_code)]
pub fn reverse(input: &str) -> String {
  /*This is a better solution */

  input.chars().rev().collect::<String>()

  /*This is the previous solution */
  // let mut output = String::new();
  // for char in input.chars() {
  //   output = char.to_string() + &output;
  // }
  // return output;
}

#[allow(dead_code)]
fn command_line() {
  let arg = Command::parse();
  match arg.command {
    Some(i) => match i {
      AppCommands::User(user_command) => match user_command.command {
        Some(i) => match i {
          command::user::UserSubCommand::Create(user) => {
            println!("create a user with name: {}, age: {}", user.name, user.age)
          }
          command::user::UserSubCommand::Update => {
            println!("Update command ")
          }
        },
        None => {}
      },
    },
    None => {
      println!("No command specified!")
    }
  }
}
