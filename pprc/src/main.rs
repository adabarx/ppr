#![allow(dead_code)]
use std::fs;
use std::path::PathBuf;
use std::str;

use clap::Parser;

#[derive(Parser)]
struct Opts {
    #[arg(short, long, value_name = "FILE")]
    input: PathBuf,
}

#[derive(Default)]
enum Token {
    #[default]
    Start,
    Word(String),
    Period,
    Comma,
    Semicolon,
    Colon,
    Dash,
    QuestionMark,
    ExclaimationPoint,
    Emdash,
    FSlash,
    OpenSurround(Surround),
    ClosedSurround(Surround),
    Escape,
    Crunch,
    Bang,
    At,
}

enum Surround {
    Bold,
    Italic,
    Underline,
    Strikethrough,
    Footnote,
    Bookmark,
    Reference,
}

fn main() {
    let opts = Opts::parse();

    let src: Vec<char> = fs::read_to_string(opts.input)
        .expect("need a valid file")
        .chars()
        .collect();

    let mut input_buf = String::new();
    let mut rv = Vec::new();
    for i in 0..src.len() {
        let input = src.get(i).unwrap();
        match input {
            ' ' => {
                rv.push(Token::Word(input_buf.clone()));
                input_buf = String::new();
            }
            '*' => {
                if let Some(ch) = src.get(i + 1) {
                    match ch {
                        '*' => todo!(),
                        '/' => todo!(),
                        '_' => todo!(),
                        '-' => todo!(),
                        _ => input_buf = format!("{}{}", input_buf, input),
                    }
                }
            }
            _ => input_buf = format!("{}{}", input_buf, input),
        }
    }
    if input_buf.len() > 0 {
        rv.push(Token::Word(input_buf.clone()));
    }
}
