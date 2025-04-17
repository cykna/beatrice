//This files implements the basics features of a parser. Simply to generate AST and bin expr and variable definittions

use super::{AST, AstError, AstErrorKind, Parser};
use crate::{
    expect,
    parser::Operator,
    tokenizer::{Reserved, Token, TokenKind},
};

impl Parser {
    pub fn parse_primary(&mut self, tk: Token) -> Result<AST, AstError> {
        match tk.kind {
            TokenKind::Int(s) => {
                let sslice = s.as_bytes();
                if s.len() > 2 && sslice[0] == b'0' {
                    let n = i64::from_str_radix(
                        &s,
                        match sslice[1] {
                            b'x' | b'X' => 16,
                            b'B' | b'b' => 8,
                            _ => 10,
                        },
                    )
                    .unwrap();
                    Ok(AST::Int(n))
                } else {
                    Ok(AST::Int(s.parse().unwrap()))
                }
            }
            TokenKind::Float(f) => Ok(AST::Float(f.parse().unwrap())),
            TokenKind::Identifier(s) => Ok(AST::Identifier(s)),
            TokenKind::OpenParen => {
                let next = self.eat()?;
                let val = self.parse_expr(next)?;
                expect!(self, TokenKind::CloseParen)?;
                Ok(val)
            }
            _ => Err(AstError {
                kind: AstErrorKind::UnexpectedToken(tk),
                line: 0,
                column: 0,
            }),
        }
    }

    pub fn parse_multiplicative(&mut self, tk: Token) -> Result<AST, AstError> {
        let mut left = self.parse_primary(tk)?;
        while let Some(Token {
            kind: TokenKind::Operator(op @ (Operator::Star(false) | Operator::Slash(false))),
            ..
        }) = self.peek()
        {
            let op = op.clone();
            self.eat()?;
            let curr = self.eat()?;
            let right = self.parse_primary(curr)?;
            left = AST::BinExpr(Box::new(left), Box::new(right), op);
        }
        Ok(left)
    }
    pub fn parse_additive(&mut self, tk: Token) -> Result<AST, AstError> {
        let mut left = self.parse_multiplicative(tk)?;
        while let Some(Token {
            kind: TokenKind::Operator(op @ (Operator::Add(false) | Operator::Sub(false))),
            ..
        }) = self.peek()
        {
            let op = op.clone();
            self.eat()?;
            let curr = self.eat()?;
            let right = self.parse_multiplicative(curr)?;
            left = AST::BinExpr(Box::new(left), Box::new(right), op);
        }
        Ok(left)
    }
    /**
     * Parses the current expression. The given token is the current token
     */
    pub fn parse_expr(&mut self, tk: Token) -> Result<AST, AstError> {
        self.parse_additive(tk)
    }
    pub fn parse_statment(&mut self, tk: Token) -> Result<AST, AstError> {
        let val = match tk.kind {
            TokenKind::Reserved(Reserved::Let) => match self.eat()?.kind {
                TokenKind::Reserved(Reserved::Mut) => {
                    let Token {
                        kind: TokenKind::Identifier(varname),
                        ..
                    } = expect!(self, TokenKind::Identifier(_))?
                    else {
                        unreachable!();
                    };
                    expect!(self, TokenKind::Operator(Operator::Eq(false)))?; //eats the '='
                    //operator
                    let next = self.eat()?;
                    Ok(AST::VarDecl {
                        varname,
                        mutable: true,
                        body: Box::new(self.parse_expr(next)?),
                    })
                }
                TokenKind::Identifier(varname) => {
                    expect!(self, TokenKind::Operator(Operator::Eq(false)))?; //eats the '=' operator
                    let next = self.eat()?;
                    Ok(AST::VarDecl {
                        varname,
                        mutable: false,
                        body: Box::new(self.parse_expr(next)?),
                    })
                }
                _ => Err(AstError {
                    kind: AstErrorKind::UnexpectedToken(tk),
                    line: 0,
                    column: 0,
                }),
            },
            TokenKind::OpenParen => {
                let next = self.eat()?;
                let val = self.parse_expr(next)?;
                expect!(self, TokenKind::CloseParen)?;
                Ok(val)
            }
            TokenKind::Identifier(ref s) => {
                if let Some(Token {
                    kind: TokenKind::OpenParen,
                    ..
                }) = self.peek()
                {
                    self.eat()?;
                    self.parse_function_call(s.clone())
                } else {
                    self.parse_expr(tk)
                }
            }
            TokenKind::Int(_) | TokenKind::Float(_) => self.parse_expr(tk),
            _ => Err(AstError {
                kind: AstErrorKind::UnexpectedToken(tk),
                line: 0,
                column: 0,
            }),
        }?;
        Ok(val)
    }

    pub fn parse_global_scope(&mut self, token: Token) -> Result<AST, AstError> {
        match &token.kind {
            TokenKind::Reserved(Reserved::Struct) => self.parse_struct_decl(),
            TokenKind::Reserved(Reserved::Function) => self.parse_function(), //does not neet to give the token because the current is 'function' keyword
            _ => Err(AstError {
                kind: AstErrorKind::InvalidScopeExpr(token.kind),
                line: token.line,
                column: token.start,
            }),
        }
    }
}
