use crate::expect;

use super::{AstError, AstErrorKind, Parser, Token, TokenKind, TypeAst};

impl Parser {
    pub fn get_f_type(&mut self) -> Result<TypeAst, AstError> {
        let mut vec = Vec::new();
        loop {
            let current = self.eat()?;
            if current.kind == TokenKind::CloseParen {
                break;
            } else {
                match current.kind {
                    TokenKind::OpenParen => vec.push(self.get_f_type()?),
                    TokenKind::Identifier(c) => vec.push(TypeAst::Primitive(c)),
                    TokenKind::Comma => {}
                    _ => {
                        return Err(AstError {
                            line: current.line,
                            column: current.column,
                            kind: AstErrorKind::UnexpectedToken(current),
                        });
                    }
                }
            }
        }
        expect!(self, TokenKind::Colon)?;
        Ok(TypeAst::Function {
            params: vec,
            return_type: Box::new(self.get_type()?),
        })
    }
    pub fn get_type(&mut self) -> Result<TypeAst, AstError> {
        let current = self.eat()?;
        match current.kind {
            TokenKind::OpenParen => self.get_f_type(),
            TokenKind::Identifier(t) => Ok(TypeAst::Primitive(t)),
            _ => Err(AstError {
                line: current.line,
                column: current.column,
                kind: AstErrorKind::UnexpectedToken(current),
            }),
        }
    }
}
