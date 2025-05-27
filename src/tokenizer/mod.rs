use std::{collections::VecDeque, fmt::Display};

macro_rules! token {
    ($arg:expr, $actual:expr, $line:expr, $column:expr) => {{
        Token {
            kind: $arg,
            start: $actual,
            line: $line,
            column: $column,
        }
    }};
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    //Bool means if they got a '=' after it.
    Add(bool),
    Sub(bool),
    Star(bool),
    Slash(bool),
    Pow(bool),

    Gt(bool),
    Lt(bool),
    Eq(bool),

    Bang(bool),
    Mod(bool),
    And(bool),
    Or(bool),

    BitAnd(bool),
    BitOr(bool),
    Xor(bool),

    BitLeft(bool),
    BitRight(bool),

    Arrow,

    Increment,
    Decrement,
}
impl Operator {
    pub fn has_eq(&self) -> bool {
        match self {
            Self::Arrow | Self::Increment | Self::Decrement => false,
            Self::Add(f)
            | Self::Sub(f)
            | Self::Star(f)
            | Self::Slash(f)
            | Self::Pow(f)
            | Self::Gt(f)
            | Self::Lt(f)
            | Self::Eq(f)
            | Self::Bang(f)
            | Self::Mod(f)
            | Self::And(f)
            | Self::Or(f)
            | Self::BitAnd(f)
            | Self::BitOr(f)
            | Self::Xor(f)
            | Self::BitLeft(f)
            | Self::BitRight(f) => *f,
        }
    }
}
impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = {
            let flag = if self.has_eq() { "=" } else { "" };
            match self {
                Self::Add(_) => format!("+{flag}"),
                Self::Sub(_) => format!("-{flag}"),
                Self::Star(_) => format!("*{flag}"),
                Self::Slash(_) => format!("/{flag}"),
                Self::Pow(_) => format!("**{flag}"),
                Self::Gt(_) => format!(">{flag}"),
                Self::Lt(_) => format!("<{flag}"),
                Self::Eq(_) => format!("={flag}"),
                Self::Bang(_) => format!("!{flag}"),
                Self::Mod(_) => format!("%{flag}"),
                Self::And(_) => format!("&&{flag}"),
                Self::Or(_) => format!("||{flag}",),
                Self::BitAnd(_) => format!("&{flag}"),
                Self::BitOr(_) => format!("|{flag}"),
                Self::Xor(_) => format!("^{flag}"),
                Self::BitLeft(_) => format!("<<{flag}"),
                Self::BitRight(_) => format!(">>{flag}"),
                Self::Arrow => "->".to_string(),
                Self::Increment => "++".to_string(),
                Self::Decrement => "--".to_string(),
            }
        };
        write!(f, "{content}")
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum Reserved {
    Trait,
    Struct,
    Let,
    Mut,
    Function,
    Macro,
    Type,
    If,
    Else,
    Loop,
    While,
    For,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Colon,
    SemiColon,
    Comma,

    Operator(Operator),
    Reserved(Reserved),
    Identifier(String),
    Int(String),
    Float(String),

    FnArrow,

    EOF,
}
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub start: usize,
    pub line: usize,
    pub column: usize,
}
fn is_operator(c: &char) -> bool {
    matches!(
        c,
        '+' | '-' | '*' | '/' | '%' | '&' | '|' | '>' | '<' | '!' | '='
    )
}

//Checks if the current char is a valid operator, if so, gets an operator based on the next chars and return the corresponding token and amount of chars walked by
fn check_operator(c: &char, chars: &[char], idx: usize) -> Option<(Operator, usize)> {
    if !is_operator(c) {
        return None;
    }
    let next = chars.get(idx + 1);

    if let Some(next) = next {
        let flag = matches!(next, '=');
        let mut n = if flag { 1 } else { 0 } + 1;
        let operator = match c {
            '+' => match next {
                '+' => {
                    n += 1;
                    Operator::Increment
                }
                _ => Operator::Add(flag),
            },
            '-' => match next {
                '-' => {
                    n += 1;
                    Operator::Decrement
                }
                '>' => {
                    n += 1;
                    Operator::Arrow
                }
                _ => Operator::Sub(flag),
            },
            '*' => match next {
                '*' => {
                    n += 1;
                    let next = chars.get(idx + 2);
                    if matches!(next, Some('=')) {
                        n += 1;
                        Operator::Pow(true)
                    } else {
                        Operator::Pow(false)
                    }
                }
                _ => Operator::Star(flag),
            },
            '/' => Operator::Slash(flag),
            '=' => Operator::Eq(flag),
            '!' => Operator::Bang(flag),
            '&' => match next {
                '&' => {
                    n += 1;
                    let next = chars.get(idx + 2);
                    if matches!(next, Some('=')) {
                        n += 1;
                        Operator::And(true)
                    } else {
                        Operator::And(false)
                    }
                }
                _ => Operator::BitAnd(flag),
            },
            '|' => match next {
                '|' => {
                    n += 1;
                    let next = chars.get(idx + 2);
                    if matches!(next, Some('=')) {
                        n += 1;
                        Operator::Or(true)
                    } else {
                        Operator::Or(false)
                    }
                }
                _ => Operator::Or(flag),
            },
            '>' => match next {
                '>' => {
                    n += 1;
                    let next = chars.get(idx + 2);
                    if matches!(next, Some('=')) {
                        n += 1;
                        Operator::BitRight(true)
                    } else {
                        Operator::BitRight(false)
                    }
                }
                _ => Operator::Gt(flag),
            },
            '<' => match next {
                '<' => {
                    n += 1;
                    let next = chars.get(idx + 2);
                    if matches!(next, Some('=')) {
                        n += 1;
                        Operator::BitLeft(true)
                    } else {
                        Operator::BitLeft(false)
                    }
                }
                _ => Operator::Lt(flag),
            },

            '^' => Operator::Xor(flag),
            '%' => Operator::Mod(flag),
            _ => return None,
        };
        Some((operator, n))
    } else {
        let operator = match c {
            '+' => Operator::Add(false),
            '-' => Operator::Sub(false),
            '*' => Operator::Star(false),
            '/' => Operator::Slash(false),
            '=' => Operator::Eq(false),
            '!' => Operator::Bang(false),
            '&' => Operator::BitAnd(false),
            '|' => Operator::Or(false),
            '>' => Operator::Gt(false),
            '<' => Operator::Lt(false),
            '^' => Operator::Xor(false),
            '%' => Operator::Mod(false),
            _ => return None,
        };
        Some((operator, 1))
    }
}

//Checks if the current char is a symbol initializer, if so, gets all symbol chars and return the corresponding token and amount of chars walked by
fn check_symbol(c: &char, chars: &[char], mut idx: usize) -> Option<(TokenKind, usize)> {
    if c.is_alphabetic() {
        let mut buffer = String::new();
        while let Some(c) = chars.get(idx) {
            if c.is_alphanumeric() {
                buffer.push(*c);
                idx += 1;
            } else {
                break;
            }
        }
        let len = buffer.len();
        Some((
            match buffer.as_ref() {
                "let" => TokenKind::Reserved(Reserved::Let),
                "mut" => TokenKind::Reserved(Reserved::Mut),
                "function" => TokenKind::Reserved(Reserved::Function),
                "macro" => TokenKind::Reserved(Reserved::Macro),
                "type" => TokenKind::Reserved(Reserved::Type),
                "struct" => TokenKind::Reserved(Reserved::Struct),
                "trait" => TokenKind::Reserved(Reserved::Trait),
                "if" => TokenKind::Reserved(Reserved::If),
                "else" => TokenKind::Reserved(Reserved::Else),
                "loop" => TokenKind::Reserved(Reserved::Loop),
                "while" => TokenKind::Reserved(Reserved::While),
                "for" => TokenKind::Reserved(Reserved::For),
                _ => TokenKind::Identifier(buffer),
            },
            len,
        ))
    } else {
        None
    }
}

///Checks if the current char is numeric, if so, gets all the numeric ones until none is found anymore. returns the corresponding token and the amount of chars walked by
fn check_numeric(c: &char, chars: &[char], idx: usize) -> Option<(TokenKind, usize)> {
    if matches!(c, '0') {
        if let Some(c) = chars.get(idx + 1) {
            let mut i = 2;
            match c {
                'x' | 'X' => {
                    let mut buffer = String::from("0x");
                    while let Some(c) = chars.get(idx + i) {
                        if c.is_ascii_hexdigit() {
                            i += 1;
                            buffer.push(*c);
                        } else {
                            break;
                        }
                    }
                    return Some((TokenKind::Int(buffer), i));
                }
                'b' => {
                    let mut buffer = String::from("0b");
                    while let Some(c) = chars.get(idx + i) {
                        if matches!(c, '1' | '0') {
                            i += 1;
                            buffer.push(*c);
                        } else {
                            break;
                        }
                    }
                    return Some((TokenKind::Int(buffer), i));
                }
                _ => panic!("Invalid Number initializing with 0"), //must implement octa digits 'o' => {}
            }
        } else {
            return Some((TokenKind::Int(String::from("0")), 1));
        }
    }
    if c.is_ascii_digit() || matches!(c, '.') {
        let mut i = 0;
        let mut dot = false;
        let mut buffer = String::new();
        while let Some(c) = chars.get(idx + i) {
            if matches!(c, '.') {
                if dot {
                    panic!("While tokenizing found a number with double dots");
                }
                dot = true;
                buffer.push(*c);
                i += 1;
            } else if c.is_ascii_digit() {
                i += 1;
                buffer.push(*c);
            } else {
                break;
            }
        }
        Some((
            if dot {
                TokenKind::Float(buffer)
            } else {
                TokenKind::Int(buffer)
            },
            i,
        ))
    } else {
        None
    }
}

pub fn tokenize(content: &str) -> VecDeque<Token> {
    let mut vec = VecDeque::new();
    let mut i = 0;
    let mut line = 0;
    let mut column = 0;
    let len = content.len();
    let chars = content.chars().collect::<Vec<char>>();
    loop {
        if i >= len {
            break;
        }
        let c = chars.get(i).unwrap();
        match c {
            ' ' => {
                i += 1;
                column += 1;
                continue;
            }
            _ if c.is_whitespace() => {
                i += 1;
                column = 0;
                line += 1;
                continue;
            }
            _ => {}
        }
        let actual = i;
        vec.push_back(token!(
            if let Some((symb, n)) = check_symbol(c, &chars, i) {
                i += n - 1;
                column += n - 1;
                symb //Can be a identifier or reserved keyword
            } else if let Some((num, n)) = check_numeric(c, &chars, i) {
                column += n - 1;
                i += n - 1;
                num
            } else {
                match c {
                    '(' => TokenKind::OpenParen,
                    ')' => TokenKind::CloseParen,
                    '{' => TokenKind::OpenBrace,
                    '}' => TokenKind::CloseBrace,
                    ':' => TokenKind::Colon,
                    ';' => TokenKind::SemiColon,
                    ',' => TokenKind::Comma,
                    _ => {
                        if let Some((op, n)) = check_operator(c, &chars, i) {
                            i += n;
                            column += n;
                            TokenKind::Operator(op)
                        } else {
                            i += 1;
                            continue;
                        }
                    }
                }
            },
            actual,
            line,
            column
        ));
        i += 1;
    }
    vec.push_back(token!(TokenKind::EOF, column, line, column));
    vec
}
