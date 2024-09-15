#![feature(concat_idents)]

mod asm;
mod ast;
mod ir;
mod lexer;
mod parser;
mod shared;
mod utilities;

use clap::Parser as ClapParser;
use ir::LoweringEngine;

use std::fs;
use std::path::PathBuf;
use std::process::exit;

use lexer::Lexer;
use parser::Parser;
use utilities::error;

/// The Scooter compiler.
#[derive(ClapParser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the source file.
    #[arg(short, long)]
    source: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let source = match args.source {
        Some(source) => source,
        None => PathBuf::from("."),
    };

    // Read the source file
    let source = fs::read_to_string(source).unwrap();

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

    // Now, parse the tokens into a syntax tree
    let mut parser = Parser::new(&tokens);
    let ast = match parser.parse_file() {
        Ok(ast) => ast,
        Err(err) => {
            error(err.reason, &source, err.span);
            exit(1);
        }
    };

    // Next, we'll lower the AST to IR and generate a human readable IR file
    let mut lower = LoweringEngine::new(&ast);
    let ir = lower.lower();

    let _ = ir.human_readable("./out.ir");
}
