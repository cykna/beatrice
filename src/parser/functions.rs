use std::collections::VecDeque;

use super::{AST, AstError, AstErrorKind, Operator, Program, Token, TokenKind, TypeAst};
use crate::{expect, parser::Parser};

#[derive(Debug)]
pub struct FunctionParameter {
    pub paramname: String,
    pub paramtype: TypeAst,
}
#[derive(PartialEq, Eq)]
enum FunctionBodyType {
    Block,
    Expression,
}

impl Parser {
    ///Parses the current function parameter, eating only its necessary data to create a FunctionParameter.
    fn parse_fparameter(&mut self) -> Result<FunctionParameter, AstError> {
        let Token {
            kind: TokenKind::Identifier(paramname),
            ..
        } = expect!(self, TokenKind::Identifier(_))?
        else {
            unreachable!();
        };
        expect!(self, TokenKind::Colon)?;
        let paramtype = self.get_type()?;
        Ok(FunctionParameter {
            paramname,
            paramtype,
        })
    }

    fn parse_params(&mut self) -> Result<VecDeque<FunctionParameter>, AstError> {
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
            } = &curr
            {
                body_type = match tk {
                    TokenKind::OpenBrace => FunctionBodyType::Block,
                    _ => FunctionBodyType::Expression,
                };
                TypeAst::Primitive("void".to_string())
            } else if let Token {
                kind: TokenKind::Colon,
                ..
            } = &curr
            {
                let type_tokens = self.get_type()?;
                if let Some(Token {
                    kind: TokenKind::Operator(Operator::Eq(false)),
                    ..
                }) = self.peek()
                {
                    body_type = FunctionBodyType::Expression
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
            expect!(self, TokenKind::OpenBrace)?;
            Program {
                body: self.parse_fbody()?,
            }
        } else {
            expect!(self, TokenKind::Operator(Operator::Eq(false)))?;
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
}
