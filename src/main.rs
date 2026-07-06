use clap::{Parser as ClapParser, Subcommand};
use miette::Result;
use std::{io::Write, path::PathBuf};

use codecrafters_interpreter::{
    expression::interpret::Interpretable,
    source_file::SourceFile,
    stages::{Parser, Scanner, StageResult},
    statements::Executable,
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
    /// Run file
    Run {
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

            let mut scanner = Scanner::new(&file.content);
            scanner.scan();
            scanner.print(file.named_source.clone());
            if scanner.has_errors() {
                std::process::exit(65)
            }
        }

        Some(Commands::Parse { path }) => {
            let file = SourceFile::new(path)?;

            let mut scanner = Scanner::new(&file.content);
            scanner.scan();
            if scanner.has_errors() {
                scanner.print_errors(file.named_source.clone());
                std::process::exit(65)
            }
            let tokens = scanner.tokens();

            let mut parser = Parser::new(tokens);
            parser.parse_expr();
            parser.print_expr(file.named_source.clone());
            if parser.has_expr_errors() {
                std::process::exit(65)
            }
        }

        Some(Commands::Evaluate { path }) => {
            let file = SourceFile::new(path)?;

            let mut scanner = Scanner::new(&file.content);
            scanner.scan();
            if scanner.has_errors() {
                scanner.print_errors(file.named_source.clone());
                std::process::exit(65)
            }
            let tokens = scanner.tokens();

            let mut parser = Parser::new(tokens);
            parser.parse_expr();
            if parser.has_expr_errors() {
                parser.print_expr_errors(file.named_source.clone());
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

        Some(Commands::Run { path }) => {
            let file = SourceFile::new(path)?;

            let mut scanner = Scanner::new(&file.content);
            scanner.scan();
            if scanner.has_errors() {
                scanner.print_errors(file.named_source.clone());
                std::process::exit(65)
            }
            let tokens = scanner.tokens();

            let mut parser = Parser::new(tokens);
            parser.parse();
            if parser.has_errors() {
                parser.print_errors(file.named_source.clone());
                std::process::exit(65)
            }

            parser
                .statements()
                .iter()
                .try_for_each(|stmt| stmt.execute())?;
        }
    }

    Ok(())
}
