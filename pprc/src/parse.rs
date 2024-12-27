use rayon::prelude::*;
use std::{collections::HashSet, fmt::Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Token {
    Text(String),
    OpenBookmark,
    CloseBookmark,
    Bold,
    Italic,
    Underline,
    Strikethrough,
    SetVar { ident: String, value: String },
    GetVar(String),
    Heading(usize),
    Title,
    // do footnote stuff
}

impl Token {
    fn to_style(&self) -> Option<Style> {
        match self {
            Token::Bold => Some(Style::Bold),
            Token::Italic => Some(Style::Italic),
            Token::Underline => Some(Style::Underline),
            Token::Strikethrough => Some(Style::Strikethrough),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub(crate) struct Text {
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
pub(crate) enum Content {
    SetVar { ident: String, value: String },
    GetVar(String),
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

pub(crate) fn parse_text(input: Vec<Token>) -> Vec<Text> {
    let mut rv = Vec::new();
    let mut curr_style = HashSet::new();
    for token in input.into_iter() {
        match token {
            Token::Text(txt) => rv.push(Text {
                text: txt,
                styles: curr_style.clone(),
            }),
            Token::Italic | Token::Bold | Token::Underline | Token::Strikethrough => {
                if curr_style.contains(&token.to_style().unwrap()) {
                    curr_style.remove(&token.to_style().unwrap());
                } else {
                    curr_style.insert(token.to_style().unwrap());
                }
            }
            _ => panic!("NO!"),
        }
    }
    rv
}

fn parse_content(input: Vec<Token>) -> Content {
    if input.len() == 0 {
        return Content::default();
    }
    match &input[0] {
        Token::SetVar { ident, value } => Content::SetVar {
            ident: ident.clone(),
            value: value.clone(),
        },
        Token::GetVar(ident) => Content::GetVar(ident.clone()),
        Token::Title => Content::Title(parse_text(input[1..input.len()].to_vec())),
        Token::Heading(level) => Content::Heading {
            level: *level,
            text: parse_text(input[1..input.len()].to_vec()),
        },
        _ => Content::Paragraph(parse_text(input)),
    }
}

pub(crate) fn parse_document(input: Vec<Vec<Token>>) -> Document {
    input.into_par_iter().map(|p| parse_content(p)).collect()
}

pub(crate) fn lexerize(input: String) -> Vec<Vec<Token>> {
    let input = input.par_split('\n').collect::<Vec<&str>>();
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
                    b'@' => {
                        let mut ident = String::new();
                        let mut value = String::new();
                        let mut write_to_ident = true;
                        loop {
                            i += 1;
                            if i == p_len {
                                break;
                            }
                            if para[i] == b'=' {
                                i += 1;
                                write_to_ident = false;
                            }
                            if write_to_ident {
                                write!(ident, "{:02x}", para[i]).unwrap();
                            } else {
                                write!(value, "{:02x}", para[i]).unwrap();
                            }
                        }
                        Some(if write_to_ident {
                            Token::GetVar(ident)
                        } else {
                            Token::SetVar { ident, value }
                        })
                    }
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
                    b'/' if i + 1 < p_len && para[i + 1] == b'/' => {
                        i += 1;
                        Some(Token::Italic)
                    }
                    b'-' if i + 1 < p_len && para[i + 1] == b'-' => {
                        i += 1;
                        Some(Token::Strikethrough)
                    }
                    b'_' if i + 1 < p_len && para[i + 1] == b'_' => {
                        i += 1;
                        Some(Token::Underline)
                    }
                    b'*' if i + 1 < p_len && para[i + 1] == b'*' => {
                        i += 1;
                        Some(Token::Bold)
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
        .collect()
}
