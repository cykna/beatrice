use std::collections::VecDeque;

use crate::expect;

use super::{AST, AstError, AstErrorKind, KeyTypePair, Parser, Token, TokenKind};

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
        dbg!(self.peek());
        Ok(AST::Struct {
            name: structname,
            fields: params,
        })
    }
}
