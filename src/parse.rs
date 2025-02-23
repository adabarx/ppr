use rayon::prelude::*;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Text {
    pub(crate) style: HashSet<Style>,
    pub(crate) str: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Token {
    Paragraph(Vec<Text>),
    Heading(usize, Vec<Text>),
    Title(Vec<Text>),
    Bookmark(Vec<Text>),
}

// document:

impl Token {
    fn add_text(&mut self, str: String, style: HashSet<Style>) {
        match self {
            Token::Title(text) => text.push(Text { style, str }),
            Token::Paragraph(text) => text.push(Text { style, str }),
            Token::Heading(_, text) => text.push(Text { style, str }),
            Token::Bookmark(text) => text.push(Text { style, str }),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub(crate) enum Style {
    Bold,
    Italics,
    Underline,
    Strikethrough,
}

#[derive(Debug)]
pub(crate) enum Content {
    Paragraph(Vec<Text>),
    Title(Vec<Text>),
    Heading { level: usize, text: Vec<Text> },
}
impl Default for Content {
    fn default() -> Self {
        Self::Paragraph(Vec::new())
    }
}

pub(crate) type Document = Vec<Content>;

pub(crate) fn tok2cont(input: Vec<Token>) -> Document {
    input
        .into_par_iter()
        .filter_map(|token| {
            Some(match token {
                Token::Title(text) => Content::Title(text),
                Token::Paragraph(text) => Content::Paragraph(text),
                Token::Heading(level, text) => Content::Heading { level, text },
                _ => return None,
            })
        })
        .collect()
}

pub(crate) fn lexerize(input: String) -> Vec<Token> {
    input
        .par_split('\\')
        .into_par_iter()
        .filter_map(|para| {
            let mut text = para.chars().collect::<Vec<_>>();
            let mut styles = HashSet::new();
            let mut i = 0;
            //dbg!(&text);
            let mut rv = match text[i] {
                'P' => Token::Paragraph(Vec::new()),
                'T' => Token::Title(Vec::new()),
                'H' => {
                    i += 1;
                    let digit = text[i].to_digit(10).unwrap() as usize;
                    Token::Heading(digit, Vec::new())
                }
                'B' => Token::Bookmark(Vec::new()),
                '\u{feff}' => return None,
                _ => {
                    dbg!(text[i]);
                    println!("\"{}\"", text[i]);
                    panic!("nope");
                }
            };

            i += 1;
            let mut input_buff = Vec::new();

            //remove trailing whitespace
            while let Some(c) = text.last() {
                if c.is_whitespace() {
                    text.pop();
                } else {
                    break;
                }
            }

            //remove leading whitespace
            while let Some(c) = text.get(i) {
                if c.is_whitespace() {
                    i += 1;
                } else {
                    break;
                }
            }

            let p_len = text.len();

            loop {
                if i == p_len {
                    rv.add_text(String::from_iter(input_buff.iter()), styles.clone());
                    break;
                }

                if i + 1 == p_len {
                    input_buff.push(text[i]);
                    i += 1;
                    continue;
                }

                if text[i].is_whitespace() {
                    input_buff.push(' ');
                    loop {
                        i += 1;
                        if !text[i].is_whitespace() {
                            break;
                        }
                    }
                }

                let style = match text[i] {
                    '/' if text[i + 1] == '/' => Some(Style::Italics),
                    '_' if text[i + 1] == '_' => Some(Style::Underline),
                    '*' if text[i + 1] == '*' => Some(Style::Bold),
                    _ => None,
                };

                if let Some(s) = style {
                    rv.add_text(String::from_iter(input_buff.iter()), styles.clone());
                    input_buff.clear();
                    i += 2;

                    if styles.contains(&s) {
                        styles.remove(&s);
                    } else {
                        styles.insert(s);
                    }
                }

                input_buff.push(text[i]);
                i += 1;
            }
            Some(rv)
        })
        .collect()
}
