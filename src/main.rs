use std::path::PathBuf;
use std::{fs, io::Write};

use codecrafters_interpreter::lexer;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Display the tokens of the file
    Tokenize {
        #[arg(value_name = "FILE", default_value = "main.lox")]
        path: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        None => {
            let stdin = std::io::stdin();
            loop {
                print!("> ");
                std::io::stdout().flush().unwrap();
                let mut buf = String::new();
                match stdin.read_line(&mut buf) {
                    Ok(_) => {
                        let tokens = lexer::tokenize(buf);
                        tokens.iter().for_each(|token| match token {
                            Ok(token) => println!("{}", token),
                            Err(err) => eprintln!("{}", err),
                        });
                    }
                    Err(error) => eprintln!("Error: {error}"),
                }
            }
        }
        Some(Commands::Tokenize { path }) => {
            let file_contents = fs::read_to_string(&path).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", path.display());
                String::new()
            });

            let tokens = lexer::tokenize(file_contents);

            let mut failed = false;
            tokens.iter().for_each(|token| match token {
                Ok(token) => println!("{}", token),
                Err(err) => {
                    failed = true;
                    eprintln!("{}", err)
                }
            });

            if failed {
                std::process::exit(65)
            }
        }
    }
}
