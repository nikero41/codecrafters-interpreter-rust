use clap::{Parser, Subcommand};
use miette::{Context, IntoDiagnostic, NamedSource, Report, Result};
use std::path::PathBuf;
use std::{fs, io::Write};

use codecrafters_interpreter::lexer;
use codecrafters_interpreter::parser;

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
    /// Display the AST of the file
    Parse {
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
                    Ok(_) => {
                        lexer::tokenize(&buf).iter().for_each(|token| match token {
                            Ok(token) => println!("{}", token),
                            Err(err) => eprintln!("{}", err),
                        });
                    }
                    Err(error) => eprintln!("Error: {error}"),
                }
            }
        }

        Some(Commands::Tokenize { path }) => {
            let file_content = fs::read_to_string(&path)
                .into_diagnostic()
                .wrap_err(format!("Failed to read file {}", path.display()))?;

            let mut failed = false;
            lexer::tokenize(&file_content)
                .into_iter()
                .for_each(|token| match token {
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

        Some(Commands::Parse { path }) => {
            let file_contents = fs::read_to_string(&path).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", path.display());
                String::new()
            });

            let result = lexer::tokenize(&file_contents);
            let (tokens, errors): (Vec<_>, Vec<_>) = result.into_iter().partition(Result::is_ok);

            let error_count = errors
                .into_iter()
                .filter_map(|result| result.err())
                .inspect(|err| eprintln!("{:?}", err))
                .count();
            if error_count > 0 {
                std::process::exit(65)
            }

            let tokens = tokens.into_iter().map(Result::unwrap).collect();
            let _ = parser::parse(tokens)?;
        }
    }

    Ok(())
}
