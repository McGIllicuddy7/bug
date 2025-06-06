use std::sync::Arc;
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TokenType {
    Literal,
    String,
    Number,
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    OpenCurly,
    CloseCurly,
    Operator,
    DeFN,
    DeStruct,
    TokenNone,
    TokenError,
}
#[derive(Clone, Debug)]
pub struct Token {
    pub tt: TokenType,
    pub st: Arc<str>,
    pub line: usize,
    pub collumn: usize,
}
#[derive(Clone, Copy, Debug)]
pub struct Tokenizer<'a> {
    pub v: &'a str,
    pub current: &'a str,
    pub line: usize,
    pub collumn: usize,
}
impl<'a> Tokenizer<'a> {
    pub fn new(v: &'a str) -> Self {
        Tokenizer {
            v,
            current: v,
            line: 1,
            collumn: 1,
        }
    }
    pub fn parse_lit(&mut self) -> Token {
        let start_line = self.line;
        let start_col = self.collumn;
        let mut s = String::new();
        if self.current.chars().next().is_none() {
            return Token {
                tt: TokenType::TokenNone,
                st: self.v.into(),
                line: self.line,
                collumn: self.collumn,
            };
        }
        let k = self.current.chars().next().unwrap();
        let is_num = k >= '0' && k <= '9';
        if k == '"' {
            let mut last_was_slash = false;
            let mut closed = false;
            let start_col = self.collumn;
            let mut hit_nonwhitespace = true;
            self.current = &self.current[1..];
            while let Some(c) = self.current.chars().next() {
                self.current = &self.current[1..];
                if c == '\n' {
                    self.line += 1;
                    self.collumn = 1;
                    last_was_slash = false;
                    hit_nonwhitespace = false;
                    continue;
                }
                if c == '\\' && !last_was_slash {
                    self.collumn += 1;
                    last_was_slash = true;
                    continue;
                }
                if c == '"' && !last_was_slash {
                    closed = true;
                    break;
                }
                if (self.collumn >= start_col) || !c.is_whitespace() || hit_nonwhitespace {
                    if last_was_slash {
                        s.push('\\');
                    }
                    s.push(c);
                    hit_nonwhitespace = true;
                }
                last_was_slash = false;
                self.collumn += 1;
            }
            if !closed {
                return Token {
                    tt: TokenType::TokenError,
                    st: s.into(),
                    line: start_line,
                    collumn: start_col,
                };
            }
            return Token {
                tt: TokenType::String,
                st: s.into(),
                line: start_line,
                collumn: start_col,
            };
        }
        while let Some(c) = self.current.chars().next() {
            if c == ' ' || c == '\n' || c == '\t' || c == ')' || c == '(' {
                break;
            }
            self.current = &self.current[1..];
            if is_num {
                s.push(c);
                if !c.is_digit(10) || c != '.' {
                    self.collumn += 1;
                    return Token {
                        tt: TokenType::Number,
                        st: s.into(),
                        line: start_line,
                        collumn: start_col,
                    };
                }
            }
            self.collumn += 1;
            s.push(c);
        }
        if is_num {
            return Token {
                tt: TokenType::TokenError,
                st: s.into(),
                line: self.line,
                collumn: self.collumn,
            };
        }
        match s.as_str() {
            "fn" => {
                return Token {
                    tt: TokenType::DeFN,
                    st: s.into(),
                    line: start_line,
                    collumn: start_col,
                };
            }
            "struct" => {
                return Token {
                    tt: TokenType::DeStruct,
                    st: s.into(),
                    line: start_line,
                    collumn: start_col,
                };
            }
            _ => {
                return Token {
                    tt: TokenType::Literal,
                    st: s.into(),
                    line: start_line,
                    collumn: start_col,
                };
            }
        }
    }
    pub fn next_token(&mut self) -> Token {
        let mut c = '\0';
        while let Some(k) = self.current.chars().next() {
            if k == ' ' || k == '\t' {
                self.current = &self.current[1..];
                self.collumn += 1;
            } else if k == '\n' {
                self.current = &self.current[1..];
                self.collumn = 1;
                self.line += 1;
            } else {
                c = k;
                break;
            }
        }
        if c == '\0' {
            return Token {
                tt: TokenType::TokenNone,
                st: self.v.into(),
                line: self.line,
                collumn: self.collumn,
            };
        }
        let out = match c {
            '(' => {
                self.current = &self.current[1..];
                self.collumn += 1;
                Token {
                    tt: TokenType::OpenParen,
                    st: "(".into(),
                    line: self.line,
                    collumn: self.collumn - 1,
                }
            }
            ')' => {
                self.current = &self.current[1..];
                self.collumn += 1;
                Token {
                    tt: TokenType::CloseParen,
                    st: ")".into(),
                    line: self.line,
                    collumn: self.collumn - 1,
                }
            }
            '{' => {
                self.current = &self.current[1..];
                self.collumn += 1;
                Token {
                    tt: TokenType::OpenCurly,
                    st: "{".into(),
                    line: self.line,
                    collumn: self.collumn - 1,
                }
            }
            '}' => {
                self.current = &self.current[1..];
                self.collumn += 1;
                Token {
                    tt: TokenType::CloseCurly,
                    st: "}".into(),
                    line: self.line,
                    collumn: self.collumn - 1,
                }
            }
            '[' => {
                self.current = &self.current[1..];
                self.collumn += 1;
                if let Some(c) = self.current.chars().peekable().peek() {
                    if *c == ']' {
                        self.current = &self.current[1..];
                        self.collumn += 1;
                        Token {
                            tt: TokenType::Operator,
                            st: "[]".into(),
                            line: self.line,
                            collumn: self.collumn - 1,
                        }
                    } else {
                        Token {
                            tt: TokenType::OpenBracket,
                            st: "{".into(),
                            line: self.line,
                            collumn: self.collumn - 1,
                        }
                    }
                } else {
                    Token {
                        tt: TokenType::OpenBracket,
                        st: "{".into(),
                        line: self.line,
                        collumn: self.collumn - 1,
                    }
                }
            }
            ']' => {
                self.current = &self.current[1..];
                self.collumn += 1;
                Token {
                    tt: TokenType::CloseBracket,
                    st: "]".into(),
                    line: self.line,
                    collumn: self.collumn - 1,
                }
            }
            '+' => {
                self.current = &self.current[1..];
                self.collumn += 1;
                if let Some(c) = self.current.chars().peekable().peek() {
                    if *c == '=' {
                        self.current = &self.current[1..];
                        self.collumn += 1;
                        Token {
                            tt: TokenType::Operator,
                            st: "+=".into(),
                            line: self.line,
                            collumn: self.collumn - 1,
                        }
                    } else {
                        Token {
                            tt: TokenType::Operator,
                            st: "+".into(),
                            line: self.line,
                            collumn: self.collumn - 1,
                        }
                    }
                } else {
                    Token {
                        tt: TokenType::Operator,
                        st: "+".into(),
                        line: self.line,
                        collumn: self.collumn - 1,
                    }
                }
            }
            '-' => {
                self.current = &self.current[1..];
                self.collumn += 1;
                if let Some(c) = self.current.chars().peekable().peek() {
                    if *c == '=' {
                        self.current = &self.current[1..];
                        self.collumn += 1;
                        Token {
                            tt: TokenType::Operator,
                            st: "-=".into(),
                            line: self.line,
                            collumn: self.collumn - 1,
                        }
                    } else {
                        Token {
                            tt: TokenType::Operator,
                            st: "-".into(),
                            line: self.line,
                            collumn: self.collumn - 1,
                        }
                    }
                } else {
                    Token {
                        tt: TokenType::Operator,
                        st: "-".into(),
                        line: self.line,
                        collumn: self.collumn - 1,
                    }
                }
            }
            '*' => {
                self.current = &self.current[1..];
                self.collumn += 1;
                if let Some(c) = self.current.chars().peekable().peek() {
                    if *c == '=' {
                        self.current = &self.current[1..];
                        self.collumn += 1;
                        Token {
                            tt: TokenType::Operator,
                            st: "*=".into(),
                            line: self.line,
                            collumn: self.collumn - 1,
                        }
                    } else {
                        Token {
                            tt: TokenType::Operator,
                            st: "*".into(),
                            line: self.line,
                            collumn: self.collumn - 1,
                        }
                    }
                } else {
                    Token {
                        tt: TokenType::Operator,
                        st: "*".into(),
                        line: self.line,
                        collumn: self.collumn - 1,
                    }
                }
            }
            '/' => {
                self.current = &self.current[1..];
                self.collumn += 1;
                if let Some(c) = self.current.chars().peekable().peek() {
                    if *c == '=' {
                        self.current = &self.current[1..];
                        self.collumn += 1;
                        Token {
                            tt: TokenType::Operator,
                            st: "/=".into(),
                            line: self.line,
                            collumn: self.collumn - 1,
                        }
                    } else if *c == '/' {
                        while let Some(c) = self.current.chars().next() {
                            self.current = &self.current[1..];
                            if c != '\n' {
                                self.line += 1;
                            } else {
                                self.collumn = 1;
                                self.line += 1;
                            }
                        }
                        return self.next_token();
                    } else {
                        Token {
                            tt: TokenType::Operator,
                            st: "/".into(),
                            line: self.line,
                            collumn: self.collumn - 1,
                        }
                    }
                } else {
                    Token {
                        tt: TokenType::Operator,
                        st: "/".into(),
                        line: self.line,
                        collumn: self.collumn - 1,
                    }
                }
            }
            '=' => {
                self.current = &self.current[1..];
                self.collumn += 1;
                if let Some(c) = self.current.chars().peekable().peek() {
                    if *c == '=' {
                        self.current = &self.current[1..];
                        self.collumn += 1;
                        Token {
                            tt: TokenType::Operator,
                            st: "==".into(),
                            line: self.line,
                            collumn: self.collumn - 1,
                        }
                    } else {
                        Token {
                            tt: TokenType::Operator,
                            st: "=".into(),
                            line: self.line,
                            collumn: self.collumn - 1,
                        }
                    }
                } else {
                    Token {
                        tt: TokenType::Operator,
                        st: "=".into(),
                        line: self.line,
                        collumn: self.collumn - 1,
                    }
                }
            }
            '>' => {
                self.current = &self.current[1..];
                self.collumn += 1;
                if let Some(c) = self.current.chars().peekable().peek() {
                    if *c == '=' {
                        self.current = &self.current[1..];
                        self.collumn += 1;
                        Token {
                            tt: TokenType::Operator,
                            st: ">=".into(),
                            line: self.line,
                            collumn: self.collumn - 1,
                        }
                    } else {
                        Token {
                            tt: TokenType::Operator,
                            st: ">".into(),
                            line: self.line,
                            collumn: self.collumn - 1,
                        }
                    }
                } else {
                    Token {
                        tt: TokenType::Operator,
                        st: ">".into(),
                        line: self.line,
                        collumn: self.collumn - 1,
                    }
                }
            }
            '<' => {
                self.current = &self.current[1..];
                self.collumn += 1;
                if let Some(c) = self.current.chars().peekable().peek() {
                    if *c == '<' {
                        self.current = &self.current[1..];
                        self.collumn += 1;
                        Token {
                            tt: TokenType::Operator,
                            st: "<=".into(),
                            line: self.line,
                            collumn: self.collumn - 1,
                        }
                    } else {
                        Token {
                            tt: TokenType::Operator,
                            st: "<".into(),
                            line: self.line,
                            collumn: self.collumn - 1,
                        }
                    }
                } else {
                    Token {
                        tt: TokenType::Operator,
                        st: "<".into(),
                        line: self.line,
                        collumn: self.collumn - 1,
                    }
                }
            }
            _ => self.parse_lit(),
        };
        out
    }
    pub fn peek_next(&self) -> Token {
        let mut tmp = *self;
        tmp.next_token()
    }
}
impl Iterator for Tokenizer<'_> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        let s = self.next_token();
        if s.tt == TokenType::TokenNone {
            None
        } else {
            Some(s)
        }
    }
}
