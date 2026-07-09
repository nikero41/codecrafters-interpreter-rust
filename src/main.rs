use clap::{Args, Parser as ClapParser, Subcommand};
use miette::{Context, Result};
use std::{io::Write, path::PathBuf};

use codecrafters_interpreter::{
    runtime::Interpreter,
    source_file::SourceFile,
    stages::{Scanner, StageResult},
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
    Tokenize(RunArgs),
    /// Display the AST of the file
    Parse(RunArgs),
    /// Evaluate file
    Evaluate(RunArgs),
    /// Print the AST of the file
    Ast(RunArgs),
    /// Run file
    Run(RunArgs),
}

#[derive(Args)]
struct RunArgs {
    #[arg(value_name = "FILE", default_value = "main.lox")]
    path: PathBuf,
}

fn main() -> Result<()> {
    miette::set_panic_hook();
    let cli = Cli::parse();

    match cli.command {
        None => {
            let stdin = std::io::stdin();
            let interpreter = Interpreter::<&str>::new();
            loop {
                print!("> ");
                std::io::stdout().flush().unwrap();
                let mut buf = String::new();
                match stdin.read_line(&mut buf) {
                    Ok(_) => interpreter.run(&buf),
                    Err(error) => eprintln!("Error: {error}"),
                }
            }
        }

        Some(Commands::Tokenize(RunArgs { path })) => {
            let file = SourceFile::new(path)?;
            let mut scanner = Scanner::new(&file.content);
            scanner.scan();
            scanner.print(file.named_source.clone());
            if scanner.has_errors() {
                std::process::exit(65)
            }
        }

        Some(Commands::Parse(RunArgs { path })) => {
            let file = SourceFile::new(path)?;
            let content = file.content.lines().next().wrap_err("No content")?;
            Interpreter::new()
                .with_debug(file.named_source)
                .print_ast(content);
        }

        Some(Commands::Evaluate(RunArgs { path })) => {
            let file = SourceFile::new(path)?;
            let content = file.content.lines().next().wrap_err("No content")?;
            Interpreter::new()
                .with_debug(file.named_source)
                .run(&format!("print {};", content));
        }

        Some(Commands::Ast(opts)) => {
            let file = SourceFile::new(opts.path)?;
            Interpreter::new()
                .with_debug(file.named_source)
                .print_ast(&file.content)
        }
        Some(Commands::Run(opts)) => {
            let file = SourceFile::new(opts.path)?;
            Interpreter::new()
                .with_debug(file.named_source)
                .run(&file.content)
        }
    }

    Ok(())
}
