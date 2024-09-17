use colored::Colorize;
use std::process::exit;

use crate::shared::Span;

/// Print an error to the command line.
pub fn error<S: AsRef<str>>(msg: S, source: &str, span: Option<Span>) {
    println!("{} | {}\n", "ERROR".red().bold(), msg.as_ref());

    if let Some(span) = span {
        let line = source.split('\n').nth(span.start.line - 1).unwrap();

        let length = if span.end.line > span.start.line {
            line.len() - span.start.column
        } else {
            span.end.column - span.start.column + 1
        };

        let marker = " ".repeat(span.start.column - 1) + &"~".repeat(length);
        let col_num_padding = span.start.line.to_string().len();

        println!("{}:{}", span.start.line, line);
        println!("{} {}\n", " ".repeat(col_num_padding), marker.red().bold());
    }
}
