use std::fmt::Display;

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
    pub fn precedence(&self) -> u8 {
        match self {
            Operator::Increment | Operator::Decrement | Operator::Bang(false) => 0,
            Operator::Pow(false) => 1,
            Operator::Star(false) | Operator::Slash(false) | Operator::Mod(false) => 2,
            Operator::Add(false) | Operator::Sub(false) => 3,
            Operator::BitLeft(false) | Operator::BitRight(false) => 4,
            Operator::BitAnd(false) => 5,
            Operator::BitOr(false) => 6,
            Operator::Xor(false) => 7,
            Operator::Eq(true) | Operator::Bang(true) | Operator::Lt(_) | Operator::Gt(_) => 8,
            Operator::And(false) => 9,
            Operator::Or(false) => 10,
            Operator::Arrow => 11,
            Operator::Add(true)
            | Operator::Sub(true)
            | Operator::Star(true)
            | Operator::Slash(true)
            | Operator::Pow(true)
            | Operator::Eq(false)
            | Operator::Mod(true)
            | Operator::And(true)
            | Operator::Or(true)
            | Operator::BitAnd(true)
            | Operator::BitOr(true)
            | Operator::Xor(true)
            | Operator::BitRight(true)
            | Operator::BitLeft(true) => 12,
        }
    }
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
