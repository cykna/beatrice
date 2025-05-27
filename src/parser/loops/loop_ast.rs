use crate::parser::{AST, AstResult, Parser, TokenKind};

impl Parser {
    pub fn parse_loop_statment(&mut self) -> AstResult {
        let current = self.eat()?;
        let loop_body = self.parse_expr(current, crate::parser::ParsingCondition::NoStruct)?;
        Ok(AST::Loop(Box::new(loop_body)))
    }
}
