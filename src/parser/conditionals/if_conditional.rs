use crate::{
    expect,
    parser::{
        AST, AstError, AstErrorKind, AstResult, Operator, Parser, ParsingCondition, Token,
        TokenKind,
    },
    tokenizer::Reserved,
};

impl Parser {
    ///Parsers an if expr -> expr else -> expr; or if expr {...} else {...}
    pub fn parse_if_expr(&mut self) -> AstResult {
        let expr = {
            let current = self.eat()?;
            self.parse_expr(current, ParsingCondition::NoStruct)?
        };
        let block = if let Some(Token {
            kind: TokenKind::Operator(Operator::Arrow),
            ..
        }) = self.peek()
        {
            self.eat()?; //eats ->
            let current = self.eat()?;
            self.parse_expr(current, ParsingCondition::PrimitiveExpr)?
        } else {
            expect!(self, TokenKind::OpenBrace)?;
            self.parse_block_expr()?
        };
        let elseblock = if let Some(Token {
            kind: TokenKind::Reserved(Reserved::Else),
            ..
        }) = self.peek()
        {
            self.eat()?;
            let tk = self.eat()?;
            Some(match tk.kind {
                TokenKind::Operator(Operator::Arrow) => {
                    let tk = self.eat()?;
                    Box::new(self.parse_expr(tk, ParsingCondition::PrimitiveExpr)?)
                }
                TokenKind::OpenBrace => {
                    //self.eat()?;
                    Box::new(self.parse_block_expr()?)
                }
                _ => {
                    return Err(AstError {
                        line: tk.line,
                        column: tk.column,
                        kind: AstErrorKind::UnexpectedToken(tk),
                    });
                }
            })
        } else {
            None
        };
        Ok(AST::If {
            expr: Box::new(expr),
            block: Box::new(block),
            elseblock,
        })
    }
    pub fn parse_if_assign_expr(&mut self) -> AstResult {
        let expr = {
            let current = self.eat()?;
            self.parse_expr(current, ParsingCondition::NoStruct)?
        };
        let block = if let Some(Token {
            kind: TokenKind::Operator(Operator::Arrow),
            ..
        }) = self.peek()
        {
            self.eat()?; //eats ->
            let current = self.eat()?;
            self.parse_expr(current, ParsingCondition::PrimitiveExpr)?
        } else {
            expect!(self, TokenKind::OpenBrace)?;
            self.parse_block_expr()?
        };
        let elseblock = if let Some(Token {
            kind: TokenKind::Reserved(Reserved::Else),
            ..
        }) = self.peek()
        {
            self.eat()?;
            let tk = self.eat()?;
            Some(match tk.kind {
                TokenKind::Operator(Operator::Arrow) => {
                    let tk = self.eat()?;
                    Box::new(self.parse_expr(tk, ParsingCondition::PrimitiveExpr)?)
                }
                TokenKind::OpenBrace => {
                    //self.eat()?;
                    Box::new(self.parse_block_expr()?)
                }
                _ => {
                    return Err(AstError {
                        line: tk.line,
                        column: tk.column,
                        kind: AstErrorKind::UnexpectedToken(tk),
                    });
                }
            })
        } else {
            let current = self.peek().expect("Reached EOF");
            return Err(AstError {
                line: current.line,
                column: current.column,
                kind: AstErrorKind::ExpectedElseBranch,
            });
        };
        Ok(AST::If {
            expr: Box::new(expr),
            block: Box::new(block),
            elseblock,
        })
    }
}
