mod basics;
mod functions;
mod structs;
mod types;

pub use crate::tokenizer::{Operator, Token, TokenKind, tokenize};
use std::collections::VecDeque;

#[derive(Debug)]
pub enum AstErrorKind {
    InvalidScopeExpr(TokenKind),
    InvalidReturnType(TokenKind),
    UnexpectedToken(Token),
    EatingEOF,
}

#[derive(Debug)]
pub struct AstError {
    line: usize,
    column: usize,
    kind: AstErrorKind,
}

#[derive(Debug)]
pub enum TypeAst {
    Primitive(String),
    Function {
        params: Vec<TypeAst>,
        return_type: Box<TypeAst>,
    },
}
///A pair of key and type. The key is the name and kindof the type of it. Used for function
///parameters and struct fields
#[derive(Debug)]
pub struct KeyTypePair {
    pub key: String,
    pub kindof: TypeAst,
}
#[derive(Debug)]
pub enum AST {
    Identifier(String),
    Int(i64),
    Float(f64),
    BinExpr(Box<AST>, Box<AST>, Operator),
    VarDecl {
        varname: String,
        mutable: bool,
        body: Box<AST>,
    },
    Function {
        name: String,
        params: VecDeque<KeyTypePair>,
        returntype: TypeAst,
        body: Program,
    },
    FunctionCall {
        name: String,
        args: VecDeque<AST>,
    },
    Return(Box<AST>),
    Struct {
        name: String,
        fields: VecDeque<KeyTypePair>,
    },
}
#[derive(Debug, Default)]
pub struct Program {
    body: VecDeque<AST>,
}
pub struct Parser {
    tokens: VecDeque<Token>,
}
#[macro_export()]
macro_rules! expect {
    ($parser:expr, $pattern:pat) => {{
        let tk = $parser.tokens.pop_front();
        match tk {
            Some(t @ Token { kind: $pattern, .. }) => Ok(t),
            Some(_) => {
                let tk = tk.unwrap();
                Err(AstError {
                    line: tk.line,
                    column: tk.column,
                    kind: AstErrorKind::UnexpectedToken(tk),
                })
            }
            None => Err(AstError {
                kind: AstErrorKind::EatingEOF,
                line: 0,
                column: 0,
            }),
        }
    }};
}
impl Parser {
    pub fn from_content(content: &str) -> Self {
        Self {
            tokens: tokenize(content),
        }
    }

    pub fn eat(&mut self) -> Result<Token, AstError> {
        self.tokens.pop_front().ok_or(AstError {
            kind: AstErrorKind::EatingEOF,
            line: 0,
            column: 0,
        })
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.front()
    }

    pub fn gen_ast(&mut self) -> Result<Program, Vec<AstError>> {
        let mut body = VecDeque::with_capacity(self.tokens.len() >> 1); //At least half of the len to not reallocate a lot
        let mut errs = Vec::new();
        loop {
            if matches!(
                self.peek(),
                Some(Token {
                    kind: TokenKind::EOF,
                    ..
                }) | None
            ) {
                break;
            }
            let tk = self.eat().unwrap();
            match self.start_gen(tk) {
                Err(e) => errs.push(e),
                Ok(ast) => body.push_back(ast),
            }
        }
        if errs.is_empty() {
            Ok(Program { body })
        } else {
            Err(errs)
        }
    }

    #[inline]
    pub fn start_gen(&mut self, token: Token) -> Result<AST, AstError> {
        self.parse_global_scope(token)
    }
}

impl AST {
    #[inline]
    pub fn is_binexpr(&self) -> bool {
        matches!(self, Self::BinExpr(_, _, _))
    }
}

impl Program {
    pub fn body(&self) -> &VecDeque<AST> {
        &self.body
    }
}
