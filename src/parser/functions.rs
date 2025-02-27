use std::collections::VecDeque;

use crate::{
    expect,
    parser::{Token, TokenKind},
};

use super::{AST, AstError, AstErrorKind, Operator, Parser, Program};

#[derive(Debug)]
pub struct FunctionParameter {
    varname: Token,
    vartype: Token,
}

impl Parser {
    pub fn parse_body(&mut self) -> Result<Program, AstError> {
        let mut body = VecDeque::new();
        while !matches!(
            self.peek(),
            Some(Token {
                kind: TokenKind::CloseBrace,
                ..
            })
        ) {
            let next = self.eat()?;
            let ast = self.parse_statment(next)?;

            body.push_back(ast);
            match self.peek() {
                Some(Token {
                    kind: TokenKind::CloseBrace,
                    ..
                }) => {}
                _ => {
                    expect!(self, TokenKind::SemiColon)?;
                }
            }
        }
        Ok(Program { body })
    }
    pub fn parse_function(&mut self) -> Result<AST, AstError> {
        let Token {
            kind: TokenKind::Identifier(fname),
            ..
        } = expect!(self, TokenKind::Identifier(_))?
        else {
            unreachable!();
        };
        expect!(self, TokenKind::OpenParen)?;
        let mut params = Vec::new();
        while !matches!(
            self.peek(),
            Some(Token {
                kind: TokenKind::CloseParen,
                ..
            })
        ) {
            let varname = expect!(self, TokenKind::Identifier(_))?;
            expect!(self, TokenKind::Colon)?;
            let vartype = expect!(self, TokenKind::Identifier(_))?;
            params.push(FunctionParameter { varname, vartype });
            if let Some(Token {
                kind: TokenKind::CloseParen,
                ..
            }) = self.peek()
            {
                break;
            } else {
                expect!(self, TokenKind::Comma)?
            };
        }
        self.eat()?; //Eat close paren
        let mut hascope = false;
        let returntype = {
            let curr = self.peek();
            match curr {
                Some(Token {
                    kind: TokenKind::Operator(Operator::Arrow),
                    ..
                }) => {
                    self.eat()?;
                    let Token {
                        kind: TokenKind::Identifier(rtype),
                        ..
                    } = expect!(self, TokenKind::Identifier(_))?
                    else {
                        unreachable!();
                    };
                    rtype
                }
                Some(Token {
                    kind: kind @ (TokenKind::Operator(Operator::Eq(false)) | TokenKind::OpenBrace),
                    ..
                }) => {
                    hascope = matches!(kind, TokenKind::OpenBrace);
                    String::from("void")
                }
                None => {
                    return Err(AstError {
                        kind: AstErrorKind::EatingEOF,
                        line: 0,
                        column: 0,
                    });
                }
                _ => {
                    let tk = self.eat().unwrap();
                    return Err(AstError {
                        line: tk.line,
                        column: tk.column,
                        kind: AstErrorKind::UnexpectedToken(tk),
                    });
                }
            }
        };
        let body = if !hascope {
            let curr = self.eat()?;
            if let Token {
                kind: TokenKind::Operator(Operator::Eq(false)),
                ..
            } = curr
            {
                let curr = self.eat()?;
                let mut deque = VecDeque::with_capacity(1);
                deque.push_back(self.parse_expr(curr)?);
                expect!(self, TokenKind::SemiColon)?;
                Ok(Program { body: deque })
            } else {
                let val = self.parse_body()?;
                expect!(self, TokenKind::CloseBrace)?;
                Ok(val)
            }
        } else {
            let val = self.parse_body()?;
            expect!(self, TokenKind::CloseBrace)?;
            Ok(val)
        }?;
        Ok(AST::Function {
            name: fname,
            params,
            returntype,
            body,
        })
    }
}
