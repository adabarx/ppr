#![allow(dead_code)]
use std::{collections::HashSet, fs, path::PathBuf, str};

use clap::Parser;
use rayon::prelude::*;

#[derive(Parser)]
struct Opts {
    #[arg(short, long, value_name = "FILE")]
    input: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Bold,
    Text(String),
    OpenBookmark,
    CloseBookmark,
    OpenItalic,
    CloseItalic,
    OpenUnderline,
    CloseUnderline,
    OpenStrikethrough,
    CloseStrikethrough,
    CompilerInstruction,
    Heading(usize),
    Title,
    // do footnote stuff
}

#[derive(Debug)]
struct Text {
    text: String,
    styles: HashSet<Style>,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Style {
    Bold,
    Italic,
    Underline,
    Strikethrough,
}

#[derive(Debug)]
enum Content {
    CompilerInstructions(Vec<Text>),
    Paragraph(Vec<Text>),
    Title(Vec<Text>),
    Heading { level: usize, text: Vec<Text> },
}
impl Default for Content {
    fn default() -> Self {
        Self::Paragraph(Vec::new())
    }
}

type Document = Vec<Content>;

fn parse_text(input: Vec<Token>) -> Vec<Text> {
    let mut rv = Vec::new();
    let mut curr_style = HashSet::new();
    for token in input.into_iter() {
        match token {
            Token::Text(txt) => rv.push(Text {
                text: txt,
                styles: curr_style.clone(),
            }),
            Token::Bold if curr_style.contains(&Style::Bold) => _ = curr_style.remove(&Style::Bold),
            Token::Bold if !curr_style.contains(&Style::Bold) => _ = curr_style.insert(Style::Bold),
            Token::OpenUnderline => _ = curr_style.insert(Style::Underline),
            Token::OpenStrikethrough => _ = curr_style.insert(Style::Strikethrough),
            Token::OpenItalic => _ = curr_style.insert(Style::Italic),
            Token::CloseUnderline => _ = curr_style.remove(&Style::Underline),
            Token::CloseStrikethrough => _ = curr_style.remove(&Style::Strikethrough),
            Token::CloseItalic => _ = curr_style.remove(&Style::Italic),
            _ => panic!("NO!"),
        }
    }
    rv
}

fn parse_paragraph(input: Vec<Token>) -> Content {
    if input.len() == 0 {
        return Content::default();
    }
    match input[0] {
        Token::CompilerInstruction => {
            Content::CompilerInstructions(parse_text(input[1..input.len()].to_vec()))
        }
        Token::Title => Content::Title(parse_text(input[1..input.len()].to_vec())),
        Token::Heading(level) => Content::Heading {
            level,
            text: parse_text(input[1..input.len()].to_vec()),
        },
        _ => Content::Paragraph(parse_text(input)),
    }
}

fn parse_document(input: Vec<Vec<Token>>) -> Document {
    input.into_par_iter().map(|p| parse_paragraph(p)).collect()
}

fn lexerize(input: Vec<&str>) -> Vec<Vec<Token>> {
    input
        .into_par_iter()
        .map(|para| {
            let para = para.as_bytes();
            let p_len = para.len();
            let mut input_buf = Vec::new();
            let mut rv = Vec::new();
            let mut i = 0;
            loop {
                if i == p_len {
                    if input_buf.len() > 0 {
                        rv.push(Token::Text(String::from_utf8(input_buf.clone()).unwrap()));
                    }
                    break;
                }
                let token = match para[i] {
                    b'\\' => {
                        i += 1;
                        None
                    }
                    b'@' => Some(Token::CompilerInstruction),
                    b'[' => Some(Token::OpenBookmark),
                    b']' => Some(Token::CloseBookmark),
                    b'#' if i == 0 => {
                        let j = i.clone();
                        loop {
                            i += 1;
                            match para.get(i) {
                                Some(b'!') if i - j == 1 => break Some(Token::Title),
                                Some(b'#') => (),
                                _ => break Some(Token::Heading(i - j)),
                            }
                        }
                    }
                    b'/' if i + 1 < p_len && para[i + 1] == b'*' => {
                        i += 1;
                        Some(Token::CloseItalic)
                    }
                    b'-' if i + 1 < p_len && para[i + 1] == b'*' => {
                        i += 1;
                        Some(Token::CloseStrikethrough)
                    }
                    b'_' if i + 1 < p_len && para[i + 1] == b'*' => {
                        i += 1;
                        Some(Token::CloseUnderline)
                    }
                    b'*' if i + 1 < p_len => {
                        i += 1;
                        match para[i] {
                            b'*' => Some(Token::Bold),
                            b'/' => Some(Token::OpenItalic),
                            b'_' => Some(Token::OpenUnderline),
                            b'-' => Some(Token::OpenStrikethrough),
                            _ => {
                                i -= 1;
                                None
                            }
                        }
                    }
                    _ => None,
                };

                if let Some(token) = token {
                    if input_buf.len() > 0 {
                        rv.push(Token::Text(String::from_utf8(input_buf.clone()).unwrap()));
                    }
                    input_buf.clear();
                    rv.push(token);
                } else {
                    input_buf.push(para[i]);
                }
                i += 1;
            }

            rv
        })
        .collect::<Vec<Vec<Token>>>()
}

fn main() {
    let opts = Opts::parse();

    let src = fs::read_to_string(opts.input).expect("need a valid file");

    let src = src.par_split('\n').collect::<Vec<&str>>();

    let tokens = lexerize(src);

    dbg!(&tokens);

    let ast = parse_document(tokens);

    dbg!(ast);
}
