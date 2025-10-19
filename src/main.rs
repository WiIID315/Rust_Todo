//rustc main.rs & ./main
use std::io::{stdin, self, Write};
use clap::{CommandFactory, Parser, Subcommand};

// const LISTS: &str = "list";
// const EXIT: &str = "exit";
// const COMMANDS: &str = "commands";

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(disable_help_flag = true)]
#[command(disable_help_subcommand = true)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add,
    List,
    Done,
    Help,
    Exit,
}


fn main() {
    // let mut buffer = String::new();
    // let stdin = io::stdin();
    // let cli = CLI::parse();
    loop {
        println!("Please input a command (type \"help\" for a list of commands):");
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        let input = stdin().read_line(&mut buffer);
        if input.is_err() {
            println!("There was an error reading in your message, please try again.");
            continue;
        }

        let args = std::iter::once("my_todo".to_string()).chain(buffer.split_whitespace().map(|s| s.to_string()));

        match CLI::try_parse_from(args) {
            Ok(cli) => {
                match cli.command {
                    Commands::Add => {

                    },
                    Commands::List => {

                    },
                    Commands::Done => {

                    },
                    Commands::Help => {
                        let mut commands = CLI::command();
                        commands.print_help().unwrap();
                    },
                    Commands::Exit => {
                        println!("Ciao!");
                        break;
                    }
                }
            }
            Err(e) => {
                e.print().unwrap();
            }
        }
    }
}
