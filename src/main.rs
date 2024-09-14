#![feature(concat_idents)]

mod ast;
mod intermediate;
mod lexer;
mod shared;
mod utilities;

use std::{fs, path::PathBuf, process::exit};

use clap::Parser;
use lexer::Lexer;
use utilities::error;

/// The Tricycle compiler.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the source file.
    #[arg(short, long)]
    source: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    // Read the source file
    let source = fs::read_to_string("examples/main.tri").unwrap();

    // We'll begin by lexing the source
    let slice = source.chars().collect::<Vec<char>>();

    let mut lexer = Lexer::new(&slice);
    let tokens = match lexer.lex() {
        Ok(tokens) => tokens,
        Err(err) => {
            error(err.reason, &source, err.span);
            exit(1);
        }
    };

    println!("{:#?}", tokens);
}
