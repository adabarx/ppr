#![allow(dead_code)]
use std::{fs, path::PathBuf};

use clap::Parser;

mod parse;

use parse::{lexerize, parse_document};

#[derive(Parser)]
struct Opts {
    #[arg(short, long, value_name = "FILE")]
    input: PathBuf,
}

fn main() {
    let opts = Opts::parse();

    let src = fs::read_to_string(opts.input).expect("need a valid file");

    let tokens = lexerize(src);

    //dbg!(&tokens);

    let ast = parse_document(tokens);

    //dbg!(ast);
}
