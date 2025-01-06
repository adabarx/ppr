use crate::parse::Style;

struct TokenData {
    start: Pos,
    end: Pos,
    token: Token,
}

impl TokenData {
    fn new(token: Token, pos: Pos, len: usize) -> Self {
        Self {
            token,
            start: pos,
            end: Pos {
                byte: pos.byte + len,
                line: pos.line,
                char: pos.char + len,
            },
        }
    }
}

struct Pos {
    byte: usize,
    line: usize,
    char: usize,
}

impl Pos {
    fn new(byte: usize, line: usize, char: usize) -> Self {
        Self { byte, line, char }
    }
}

enum Token {
    Space,
    Tab,
    Word,
    Hyphen,
    Comma,
    Semicolon,
    Colon,
    Period,
    Question,
    EmDash,
    Exclamation,
    Style(Style),
    Quote,
    OpenParentheses,
    CloseParentheses,
    Tab,
    Space,
}

fn lex(input: Vec<char>) -> Vec<TokenData> {
    let mut rv = Vec::new();
    let mut word_buf = String::new();
    let mut i = 0_usize;
    let mut line = 0_usize;
    let mut char = 0_usize;
    let mut offset = 0_usize;

    loop {
        if i == input.len() {
            break;
        }

        let tok = match input[i] {
            '\u{2014}' => Some(Token::EmDash),
            '-' if input[i + 1] == '-' => {
                i += 1;
                char += 1;
                Some(Token::Hyphen)
            }
            ':' => Some(Token::Colon),
            ',' => Some(Token::Comma),
            '-' => Some(Token::Hyphen),
            '.' => Some(Token::Period),
            '?' => Some(Token::Question),
            ';' => Some(Token::Semicolon),
            '!' => Some(Token::Exclamation),
            '(' => Some(Token::OpenParentheses),
            ')' => Some(Token::CloseParentheses),
            '"' | '“' | '”' => Some(Token::Quote),
            ' ' if input[i..i + 4] == [' ', ' ', ' ', ' '] => {
                i += 3;
                char += 3;
                Some(Token::Tab)
            }
            ' ' => Some(Token::Space),
            '\t' => Some(Token::Tab),
            '\n' => {
                char = 0;
                line += 1;
                None
            }
            _ => None,
        };

        if let Some(tok) = tok {
            let w_len = word_buf.len();
            if w_len > 0 {
                rv.push(TokenData::new(
                    Token::Word,
                    Pos {
                        byte: offset,
                        line,
                        char,
                    },
                    input[offset..offset + w_len],
                ));
                offset += w_len;
                char += w_len;
                word_buf = String::new();
            }

            let len = input[i].len_utf8();
            offset += len;
            char += len;
            rv.push(TokenData::new(Token::Hyphen, Pos { byte, line, char }, len));
        } else {
            word_buf.push(input[i]);
        }

        char += 1;
        i += 1;
    }
}
