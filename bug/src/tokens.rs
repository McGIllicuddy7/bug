pub use crate::utils;
pub use std::sync::Arc;
const TokenNames: [&str; 17] = [
    "TokenNone",
    "TokenStr",
    "TokenChar",
    "TokenInt",
    "TokenFloat",
    "TokenOpenCurl",
    "TokenCloseCurl",
    "TokenOpenParen",
    "TokenCloseParen",
    "TokenSemi",
    "TokenColon",
    "TokenOperator",
    "TokenIdent",
    "TokenComma",
    "TokenDot",
    "TokenOpenBracket",
    "TokenCloseBracket",
];
#[derive(Clone, Debug)]
pub struct Token {
    pub st: Arc<str>,
    pub file: Arc<str>,
    pub line: usize,
    pub tt: TokenType,
}
impl Token {
    pub fn print(&self) {
        println!("{:#?}", self);
    }

    pub fn equals(&self, st: &str) -> bool {
        self.st.as_ref() == st
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum TTState {
    TTEmpty,
    TTChar,
    TTNum,
    TTString,
    TTIdent,
}
#[derive(PartialEq, Clone, Debug)]
pub enum TokenType {
    TokenNone,
    TokenStr,
    TokenChar,
    TokenInt,
    TokenFloat,
    TokenOpenCurl,
    TokenCloseCurl,
    TokenOpenParen,
    TokenCloseParen,
    TokenSemi,
    TokenColon,
    TokenOperator,
    TokenIdent,
    TokenComma,
    TokenDot,
    TokenOpenBracket,
    TokenCloseBracket,
    TokenTypeCount,
}
pub fn is_whitespace(c: char) -> bool {
    if c == ' ' {
        true
    } else if c == '\n' {
        return true;
    } else if c == '\t' {
        return true;
    } else {
        return c == '\r';
    }
}
pub fn is_number(c: char) -> bool {
    if c == '0' {
        true
    } else if c == '1' {
        return true;
    } else if c == '2' {
        return true;
    } else if c == '3' {
        return true;
    } else if c == '4' {
        return true;
    } else if c == '5' {
        return true;
    } else if c == '6' {
        return true;
    } else if c == '7' {
        return true;
    } else if c == '8' {
        return true;
    } else if c == '9' {
        return true;
    } else {
        return c == '.';
    }
}
pub fn is_entire_token(c: char, n: char) -> bool {
    if n == '=' {
        if c == '+' {
            return true;
        } else if c == '-' {
            return true;
        } else if c == '*' {
            return true;
        } else if c == '/' {
            return true;
        } else if c == '=' {
            return true;
        } else if c == '>' {
            return true;
        } else if c == '<' {
            return true;
        }
    } else if n == '>' && c == '-' {
        return true;
    }
    false
}
pub fn is_reserved(c: char) -> bool {
    if c == '(' {
        true
    } else if c == ')' {
        return true;
    } else if c == '+' {
        return true;
    } else if c == '-' {
        return true;
    } else if c == '*' {
        return true;
    } else if c == '/' {
        return true;
    } else if c == '{' {
        return true;
    } else if c == '}' {
        return true;
    } else if c == ';' {
        return true;
    } else if c == '.' {
        return true;
    } else if c == '[' {
        return true;
    } else if c == ']' {
        return true;
    } else if c == '<' {
        return true;
    } else if c == '=' {
        return true;
    } else if c == '>' {
        return true;
    } else {
        return c == ':';
    }
}
pub fn calc_tt(c: char) -> TokenType {
    if c == '(' {
        TokenType::TokenOpenParen
    } else if c == ')' {
        return TokenType::TokenCloseParen;
    } else if c == '{' {
        return TokenType::TokenOpenCurl;
    } else if c == '}' {
        return TokenType::TokenCloseCurl;
    } else if c == '[' {
        return TokenType::TokenOpenBracket;
    } else if c == ']' {
        return TokenType::TokenCloseBracket;
    } else if c == ';' {
        return TokenType::TokenSemi;
    } else if c == ':' {
        return TokenType::TokenColon;
    } else if c == '.' {
        return TokenType::TokenDot;
    } else if c == ',' {
        return TokenType::TokenComma;
    } else {
        return TokenType::TokenOperator;
    }
}
#[derive(Clone, Debug)]
pub struct TokenStream {
    idx: usize,
    line: usize,
    file: Arc<str>,
    chars: Vec<char>,
}
impl TokenStream {
    pub fn new(chars: &str, file: &str) -> Self {
        Self {
            chars: chars.chars().collect::<Vec<char>>(),
            idx: 0,
            line: 1,
            file: file.into(),
        }
    }
    pub fn next(&mut self) -> Option<Token> {
        let mut s = String::new();
        let mut state = TTState::TTEmpty;
        let mut out: Token = Token {
            st: "".into(),
            file: "".into(),
            line: 0,
            tt: TokenType::TokenNone,
        };
        let mut was_slash = false;
        if self.idx >= self.chars.len() {
            return None;
        }
        let mut done = false;
        while self.idx < self.chars.len() {
            if done {
                break;
            }
            let c = self.chars[self.idx];
            self.idx += 1;
            if c == '\n' {
                self.line += 1;
            }
            if state == TTState::TTEmpty {
                if is_whitespace(c) {
                    continue;
                } else if c == '"' {
                    state = TTState::TTString;
                    s.push(c);
                    continue;
                } else if c == '\'' {
                    state = TTState::TTChar;
                    s.push(c);
                    continue;
                } else if is_number(c) {
                    state = TTState::TTNum;
                    s.push(c);
                    continue;
                } else if c == ',' {
                    s.push(c);
                    out.tt = TokenType::TokenComma;
                    done = true;
                    break;
                } 
                 else if is_reserved(c) {
                    let has_next = self.idx < self.chars.len();
                    s.push(c);
                    if has_next {
                        let n = self.chars[self.idx];
                        if is_entire_token(c, n) {
                            self.idx += 1;
                            s.push(c);
                        }
                    }
                    out.tt = calc_tt(c);
                    done = true;
                    continue;
                } else {
                    state = TTState::TTIdent;
                    s.push(c);
                    continue;
                }
            } else if state == TTState::TTChar {
                if was_slash {
                    if c == 't' {
                        s.push('\t');
                    } else if c == '"' {
                        s.push('"');
                    } else if c == 'n' {
                        s.push('\n');
                    } else if c == '0' {
                        s.push('\0');
                    }
                    was_slash = false;
                } else if c == '\\' {
                    was_slash = true;
                } else {
                    s.push(c);
                    if c == '\'' {
                        break;
                    }
                    was_slash = false;
                }
            } else if state == TTState::TTString {
                if was_slash {
                    if c == 't' {
                        s.push('\t');
                    } else if c == '"' {
                        s.push('"');
                    } else if c == 'n' {
                        s.push('\n');
                    } else if c == '0' {
                        s.push('\0');
                    }
                    was_slash = false;
                } else if c == '\\' {
                    was_slash = true;
                } else {
                    s.push(c);
                    if c == '"' {
                        break;
                    }
                    was_slash = false;
                }
            } else if state == TTState::TTIdent {
                if is_whitespace(c) || is_reserved(c) {
                    self.idx -= 1;
                    if c == '\n' {
                        self.line -= 1;
                    }
                    break;
                } else {
                    s.push(c);
                }
            } else if state == TTState::TTNum {
                if is_number(c) {
                    s.push(c);
                } else {
                    self.idx -= 1;
                    break;
                }
            }
        }
        if self.idx == self.chars.len() {
            self.idx = self.chars.len() + 1;
        }
        out.file = self.file.clone();
        out.line = self.line;
        out.st = s.clone().into();
        if s == "."{
            out.tt = TokenType::TokenDot;
        }
        if state == TTState::TTNum {
            if s.contains('.') && s != "."{
                out.tt = TokenType::TokenFloat;
            } else if s != "."{
                out.tt = TokenType::TokenInt;
            }
        } else if state == TTState::TTChar {
            out.tt = TokenType::TokenChar;
        } else if state == TTState::TTString {
            out.tt = TokenType::TokenStr;
        } else if state == TTState::TTIdent {
            out.tt = TokenType::TokenIdent;
        }
        if (out.st.len() == 0) {
            return None;
        }
        Some(out)
    }
    pub fn peek(&self) -> Option<Token> {
        let mut prev = self.clone();
        let out = prev.next()?;
        Some(out)
    }
    pub fn collect(&mut self) -> Vec<Token> {
        let mut out = Vec::new();
        while let Some(v) = self.next() {
            out.push(v);
        }
        out
    }
}
