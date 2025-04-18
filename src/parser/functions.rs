use std::collections::VecDeque;

use super::{
    AST, AstError, AstErrorKind, KeyTypePair, Operator, Program, Token, TokenKind, TypeAst,
};
use crate::{expect, parser::Parser};

#[derive(PartialEq, Eq)]
enum FunctionBodyType {
    Block,
    Expression,
}

impl Parser {
    ///Parses the current function parameter, eating only its necessary data to create a FunctionParameter.
    fn parse_fparameter(&mut self) -> Result<KeyTypePair, AstError> {
        let Token {
            kind: TokenKind::Identifier(key),
            ..
        } = expect!(self, TokenKind::Identifier(_))?
        else {
            unreachable!();
        };
        expect!(self, TokenKind::Colon)?;
        let kindof = self.get_type()?;
        Ok(KeyTypePair { key, kindof })
    }

    fn parse_params(&mut self) -> Result<VecDeque<KeyTypePair>, AstError> {
        let mut params = VecDeque::new();
        loop {
            if let Some(Token {
                kind: TokenKind::CloseParen,
                ..
            }) = self.peek()
            {
                self.eat()?;
                break;
            }
            params.push_back(self.parse_fparameter()?);
            let kind = expect!(self, TokenKind::CloseParen | TokenKind::Comma)?.kind;
            if let TokenKind::CloseParen = kind {
                break;
            }
        }
        Ok(params)
    }
    pub fn parse_fbody(&mut self) -> Result<VecDeque<AST>, AstError> {
        let mut vec = VecDeque::new();
        loop {
            let current = self.eat()?;
            if let Token {
                kind: TokenKind::CloseBrace,
                ..
            } = current
            {
                break;
            }
            let statment = self.parse_statment(current)?;
            if let Some(Token {
                kind: TokenKind::CloseBrace,
                ..
            }) = self.peek()
            {
                vec.push_back(AST::Return(Box::new(statment)))
            } else {
                vec.push_back(statment);
                expect!(self, TokenKind::SemiColon)?;
            }
        }
        Ok(vec)
    }

    pub fn parse_function(&mut self) -> Result<AST, AstError> {
        let Token {
            kind: TokenKind::Identifier(name),
            ..
        } = expect!(self, TokenKind::Identifier(_))?
        else {
            unreachable!();
        };
        expect!(self, TokenKind::OpenParen)?;
        let params = self.parse_params()?;
        let mut body_type = FunctionBodyType::Block;
        let returntype = {
            let curr = self.eat()?;
            if let Token {
                kind: tk @ (TokenKind::Operator(Operator::Eq(false)) | TokenKind::OpenBrace),
                ..
            } = curr
            {
                match tk {
                    TokenKind::OpenBrace => {}
                    _ => body_type = FunctionBodyType::Expression,
                };
                TypeAst::Primitive("void".to_string())
            } else if let Token {
                kind: TokenKind::Colon,
                ..
            } = curr
            {
                let type_tokens = self.get_type()?;
                let curr = self.eat()?;
                match curr.kind {
                    TokenKind::Operator(Operator::Eq(false)) => {
                        body_type = FunctionBodyType::Expression
                    }
                    TokenKind::OpenBrace => {}
                    _ => {
                        return Err(AstError {
                            line: curr.line,
                            column: curr.column,
                            kind: AstErrorKind::InvalidReturnType(curr.kind),
                        });
                    }
                };
                type_tokens
            } else {
                return Err(AstError {
                    line: curr.line,
                    column: curr.column,
                    kind: AstErrorKind::InvalidReturnType(curr.kind),
                });
            }
        };
        let body = if body_type == FunctionBodyType::Block {
            Program {
                body: self.parse_fbody()?,
            }
        } else {
            let current = self.eat()?;
            let p = Program {
                body: VecDeque::from(vec![AST::Return(Box::new(self.parse_expr(current)?))]),
            };
            expect!(self, TokenKind::SemiColon)?;
            p
        };
        Ok(AST::Function {
            name,
            returntype,
            body,
            params,
        })
    }
    pub fn parse_function_call(&mut self, fname: String) -> Result<AST, AstError> {
        if let Some(Token {
            kind: TokenKind::CloseParen,
            ..
        }) = self.peek()
        {
            self.eat()?;
            return Ok(AST::FunctionCall {
                name: fname,
                args: VecDeque::new(),
            });
        }
        let mut params = VecDeque::new();
        loop {
            let curr = self.eat()?;
            params.push_back(self.parse_expr(curr)?);
            if let Some(Token {
                kind: TokenKind::CloseParen,
                ..
            }) = self.peek()
            {
                self.eat()?;
                break;
            }
            expect!(self, TokenKind::Comma)?;
        }
        Ok(AST::FunctionCall {
            name: fname,
            args: params,
        })
    }
}
