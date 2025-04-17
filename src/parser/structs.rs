use std::collections::VecDeque;

use crate::expect;

use super::{AST, AstError, AstErrorKind, FunctionParameter, Parser, Token, TokenKind};

impl Parser {
    pub fn parse_struct_params(&mut self) -> Result<VecDeque<FunctionParameter>, AstError> {
        if let Some(Token {
            kind: TokenKind::CloseBrace,
            ..
        }) = self.peek()
        {
            self.eat();
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
            expect!(self, TokenKind::Colon);
            let field_type = self.get_type()?;
            out.push_back(FunctionParameter {
                paramname: field_name,
                paramtype: field_type,
            });
            if let Some(Token {
                kind: TokenKind::CloseBrace,
                ..
            }) = self.peek()
            {
                self.eat();
                break;
            } else {
                expect!(self, TokenKind::SemiColon)?;
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
        let params = self.parse_struct_params()?;
    }
}
