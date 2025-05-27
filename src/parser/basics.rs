//This files implements the basics features of a parser. Simply to generate AST and bin expr and variable definittions

use std::collections::VecDeque;

use super::{AST, AstError, AstErrorKind, AstResult, Parser, ParsingCondition};
use crate::{
    expect,
    parser::Operator,
    tokenizer::{Reserved, Token, TokenKind},
};

impl Parser {
    pub fn parse_int(integer: String) -> AST {
        let integer_bytes = integer.as_bytes();
        if integer.len() > 2 && integer_bytes[0] == b'0' {
            let n = i64::from_str_radix(
                &integer,
                match integer_bytes[1] {
                    b'x' | b'X' => 16,
                    b'B' | b'b' => 8,
                    _ => 10,
                },
            )
            .unwrap();
            AST::Int(n)
        } else {
            AST::Int(integer.parse().unwrap())
        }
    }
    pub fn parse_identifier(
        &mut self,
        identifier: String,
        condition: ParsingCondition,
    ) -> AstResult {
        if let Some(Token {
            kind: TokenKind::OpenParen,
            ..
        }) = self.peek()
        {
            self.eat()?;
            return self.parse_function_call(identifier);
        } else if let Some(Token {
            kind: TokenKind::OpenBrace,
            ..
        }) = self.peek()
        {
            if condition != ParsingCondition::NoStruct {
                self.eat()?;
                return self.parse_struct_expr(identifier);
            }
        };
        Ok(AST::Identifier(identifier))
    }
    pub fn parse_primary(&mut self, tk: Token, condition: ParsingCondition) -> AstResult {
        match tk.kind {
            TokenKind::Int(s) => Ok(Self::parse_int(s)),
            TokenKind::Float(f) => Ok(AST::Float(f.parse().unwrap())),

            TokenKind::Identifier(s) => self.parse_identifier(s, condition),
            TokenKind::OpenParen => {
                let next = self.eat()?;
                let val = self.parse_expr(next, ParsingCondition::None)?;
                expect!(self, TokenKind::CloseParen)?;
                Ok(val)
            }
            _ => Err(AstError {
                line: tk.line,
                column: tk.column,
                kind: AstErrorKind::UnexpectedToken(tk),
            }),
        }
    }

    pub fn parse_multiplicative(&mut self, tk: Token, condition: ParsingCondition) -> AstResult {
        let mut left = self.parse_primary(tk, condition)?;
        while let Some(Token {
            kind: TokenKind::Operator(op @ (Operator::Star(false) | Operator::Slash(false))),
            ..
        }) = self.peek()
        {
            let op = op.clone();
            self.eat()?;
            let curr = self.eat()?;
            let right = self.parse_primary(curr, condition)?;
            left = AST::BinExpr(Box::new(left), Box::new(right), op);
        }
        Ok(left)
    }
    pub fn parse_additive(&mut self, tk: Token, condition: ParsingCondition) -> AstResult {
        let mut left = self.parse_multiplicative(tk, condition)?;
        while let Some(Token {
            kind: TokenKind::Operator(op @ (Operator::Add(false) | Operator::Sub(false))),
            ..
        }) = self.peek()
        {
            let op = op.clone();
            self.eat()?;
            let curr = self.eat()?;
            let right = self.parse_multiplicative(curr, condition)?;
            left = AST::BinExpr(Box::new(left), Box::new(right), op);
        }
        Ok(left)
    }
    /// Parses the current expression. The given token is the current token
    pub fn parse_expr(&mut self, tk: Token, condition: ParsingCondition) -> AstResult {
        if let TokenKind::OpenBrace = tk.kind {
            self.parse_block_expr()
        } else if let TokenKind::Reserved(Reserved::If) = tk.kind {
            self.parse_if_expr()
        } else {
            self.parse_additive(tk, condition)
        }
    }
    pub fn parse_block_expr(&mut self) -> AstResult {
        let mut asts = VecDeque::new();
        loop {
            if let Some(Token {
                kind: TokenKind::CloseBrace,
                ..
            }) = self.peek()
            {
                self.eat()?;
                break;
            }
            let tk = self.eat()?;
            asts.push_back(self.parse_statment(tk)?);
            if let Some(Token {
                kind: TokenKind::CloseBrace,
                ..
            }) = self.peek()
            {
                self.eat()?;
                break;
            } else {
                expect!(self, TokenKind::SemiColon)?;
            }
        }
        Ok(AST::Block(asts))
    }
    /**
     * Parses a basic statment.
     */
    pub fn parse_statment(&mut self, tk: Token) -> AstResult {
        let val = match tk.kind {
            TokenKind::Reserved(Reserved::Let) => match self.eat()?.kind {
                //let mut name = ...
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
                        body: Box::new(if let TokenKind::Reserved(Reserved::If) = next.kind {
                            self.parse_if_assign_expr()?
                        } else {
                            self.parse_expr(next, ParsingCondition::PrimitiveExpr)?
                        }),
                    })
                }
                //let name = ...
                TokenKind::Identifier(varname) => {
                    expect!(self, TokenKind::Operator(Operator::Eq(false)))?; //eats the '=' operator
                    let next = self.eat()?;

                    Ok(AST::VarDecl {
                        varname,
                        mutable: false,
                        body: Box::new(if let TokenKind::Reserved(Reserved::If) = next.kind {
                            self.parse_if_assign_expr()?
                        } else {
                            self.parse_expr(next, ParsingCondition::PrimitiveExpr)?
                        }),
                    })
                }
                _ => Err(AstError {
                    line: tk.line,
                    column: tk.column,
                    kind: AstErrorKind::UnexpectedToken(tk),
                }),
            },
            TokenKind::OpenParen => {
                let next = self.eat()?;
                let val = self.parse_expr(next, ParsingCondition::PrimitiveExpr)?;
                expect!(self, TokenKind::CloseParen)?;
                Ok(val)
            }
            TokenKind::Identifier(_) => self.parse_expr(tk, ParsingCondition::None),
            TokenKind::Int(_) | TokenKind::Float(_) => self.parse_expr(tk, ParsingCondition::None),
            TokenKind::Reserved(Reserved::If) => self.parse_if_expr(),
            TokenKind::Reserved(Reserved::Loop) => self.parse_loop_statment(),
            _ => Err(AstError {
                line: tk.line,
                column: tk.column,
                kind: AstErrorKind::UnexpectedToken(tk),
            }),
        }?;
        Ok(val)
    }

    pub fn parse_global_scope(&mut self, token: Token) -> AstResult {
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
