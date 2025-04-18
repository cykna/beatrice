use std::collections::VecDeque;

use crate::expect;

use super::{AST, AstError, AstErrorKind, KeyExprPair, KeyTypePair, Parser, Token, TokenKind};

impl Parser {
    pub fn parse_struct_fields(&mut self) -> Result<VecDeque<KeyTypePair>, AstError> {
        if let Some(Token {
            kind: TokenKind::CloseBrace,
            ..
        }) = self.peek()
        {
            self.eat()?;
            return Ok(VecDeque::new());
        }
        let mut out = VecDeque::new();
        loop {
            let Token {
                kind: TokenKind::Identifier(field_name),
                ..
            } = expect!(self, TokenKind::Identifier(_))?
            else {
                unreachable!();
            };
            expect!(self, TokenKind::Colon)?;
            let field_type = self.get_type()?;
            out.push_back(KeyTypePair {
                key: field_name,
                kindof: field_type,
            });
            expect!(self, TokenKind::SemiColon)?;
            if let Some(Token {
                kind: TokenKind::CloseBrace,
                ..
            }) = self.peek()
            {
                self.eat()?;
                break;
            }
        }
        Ok(out)
    }

    pub fn parse_struct_decl(&mut self) -> Result<AST, AstError> {
        let Token {
            kind: TokenKind::Identifier(structname),
            ..
        } = expect!(self, TokenKind::Identifier(_))?
        else {
            unreachable!()
        };
        expect!(self, TokenKind::OpenBrace)?;
        let params = self.parse_struct_fields()?;
        Ok(AST::Struct {
            name: structname,
            fields: params,
        })
    }
    pub fn parse_struct_expr(&mut self, structname: String) -> Result<AST, AstError> {
        if let Some(Token {
            kind: TokenKind::CloseBrace,
            ..
        }) = self.peek()
        {
            self.eat()?;
            return Ok(AST::StructExpr {
                name: structname,
                fields: VecDeque::new(),
            });
        };
        let mut fields = VecDeque::new();
        loop {
            let Token {
                kind: TokenKind::Identifier(field_name),
                ..
            } = expect!(self, TokenKind::Identifier(_))?
            else {
                unreachable!();
            };
            if let Some(Token {
                kind: TokenKind::Comma,
                ..
            }) = self.peek()
            {
                self.eat()?;
                fields.push_back(KeyExprPair {
                    key: field_name.clone(),
                    value: AST::Identifier(field_name),
                });
                continue;
            }
            expect!(self, TokenKind::Colon)?;
            let current = self.eat()?;
            let value = self.parse_expr(current)?;
            fields.push_back(KeyExprPair {
                key: field_name,
                value,
            });
            if let Some(Token {
                kind: TokenKind::CloseBrace,
                ..
            }) = self.peek()
            {
                self.eat()?;
                break;
            } else {
                expect!(self, TokenKind::Comma)?;
            }
        }
        Ok(AST::StructExpr {
            name: structname,
            fields,
        })
    }
}
