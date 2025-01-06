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
        .map(|para| {
            let text = para.chars().collect::<Vec<_>>();
            let p_len = text.len();
            let mut styles = HashSet::new();
            let mut i = 0;
            let mut rv = match text[i] {
                'P' => Token::Paragraph(Vec::new()),
                'T' => Token::Title(Vec::new()),
                'H' => {
                    i += 1;
                    let digit = text[i].to_digit(10).unwrap() as usize;
                    Token::Heading(digit, Vec::new())
                }
                'B' => Token::Bookmark(Vec::new()),
                _ => panic!("nope"),
            };

            i += 1;
            let mut input_buff = Vec::new();

            loop {
                if i == p_len {
                    rv.add_text(String::from_iter(input_buff.iter()), styles.clone());
                    break;
                }

                if text[i].is_whitespace() {
                    let trim_start = i == 1;
                    let trim_end = loop {
                        if i + 1 == p_len {
                            break true;
                        }
                        if text[i + 1].is_whitespace() {
                            i += 1;
                            continue;
                        }
                        i += 1;
                        break false;
                    };
                    if trim_start && trim_end == false {
                        input_buff.push(' ');
                    }
                }

                if i + 1 == p_len {
                    input_buff.push(text[i]);
                    i += 1;
                    continue;
                }

                let s = match text[i] {
                    '/' if text[i + 1] == '/' => Some(Style::Italics),
                    '_' if text[i + 1] == '_' => Some(Style::Underline),
                    '*' if text[i + 1] == '*' => Some(Style::Bold),
                    _ => None,
                };

                if let Some(s) = s {
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
            rv
        })
        .collect()
}
