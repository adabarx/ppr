#![allow(dead_code)]
use std::{fs, path::PathBuf};

use clap::Parser;

mod export;
mod parse;

use parse::{lexerize, tok2cont};

#[derive(Parser)]
struct Opts {
    #[arg(short, long, value_name = "FILE")]
    input: PathBuf,
}

fn main() {
    let opts = Opts::parse();

    let src = fs::read_to_string(&opts.input).expect("need a valid file");

    let tokens = lexerize(src);

    //dbg!(&tokens);

    let ast = tok2cont(tokens);

    //dbg!(ast);

    export::docx(ast, opts.input);
}
