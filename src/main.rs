use clap::{Parser as ClapParser, Subcommand};
use miette::Result;
use std::{io::Write, path::PathBuf};

use codecrafters_interpreter::{
    expression::interpret::Interpretable,
    source_file::SourceFile,
    stages::{Parser, Scanner, StageResult},
};

#[derive(ClapParser)]
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
    /// Display the AST of the file
    Parse {
        #[arg(value_name = "FILE", default_value = "main.lox")]
        path: PathBuf,
    },
    /// Evaluate file
    Evaluate {
        #[arg(value_name = "FILE", default_value = "main.lox")]
        path: PathBuf,
    },
}

fn main() -> Result<()> {
    miette::set_panic_hook();
    let cli = Cli::parse();

    match cli.command {
        None => {
            let stdin = std::io::stdin();
            loop {
                print!("> ");
                std::io::stdout().flush().unwrap();
                let mut buf = String::new();
                match stdin.read_line(&mut buf) {
                    Ok(_) => {}
                    Err(error) => eprintln!("Error: {error}"),
                }
            }
        }

        Some(Commands::Tokenize { path }) => {
            let file = SourceFile::new(path)?;

            let mut scanner = Scanner::new(&file);
            scanner.scan();
            scanner.print();
            if scanner.has_errors() {
                std::process::exit(65)
            }
        }

        Some(Commands::Parse { path }) => {
            let file = SourceFile::new(path)?;

            let mut scanner = Scanner::new(&file);
            scanner.scan();
            if scanner.has_errors() {
                scanner.print_errors();
                std::process::exit(65)
            }
            let tokens = scanner.tokens();

            let mut parser = Parser::new(&file, tokens);
            parser.parse();
            parser.print();
            if parser.has_errors() {
                std::process::exit(65)
            }
        }

        Some(Commands::Evaluate { path }) => {
            let file = SourceFile::new(path)?;

            let mut scanner = Scanner::new(&file);
            scanner.scan();
            if scanner.has_errors() {
                scanner.print_errors();
                std::process::exit(65)
            }
            let tokens = scanner.tokens();

            let mut parser = Parser::new(&file, tokens);
            parser.parse();
            if parser.has_errors() {
                parser.print_errors();
                std::process::exit(65)
            }

            let expressions = parser.expressions();
            if !expressions.is_empty() {
                match expressions[0].interpret() {
                    Ok(value) => println!("{}", value),
                    Err(err) => {
                        eprintln!("{}", err);
                        std::process::exit(70)
                    }
                }
            }
        }
    }

    Ok(())
}
