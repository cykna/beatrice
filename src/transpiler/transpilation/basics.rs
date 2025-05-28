use crate::{parser::AST, transpiler::transpiler::BeatriceTranspiler};

impl BeatriceTranspiler {
    pub(crate) fn generate_binexpr_content(&mut self, ast: &AST) -> String {
        let AST::BinExpr(lhs, rhs, operator) = ast else {
            panic!("This is a bug. Expected receiving a BinExpr");
        };
        let rhs = if let AST::BinExpr(_, _, op) = &**rhs {
            if op.precedence() > operator.precedence() {
                format!("({})", self.generate_binexpr_content(rhs))
            } else {
                self.generate_binexpr_content(rhs)
            }
        } else {
            self.generate_expression_content(rhs)
        };
        let lhs = self.generate_expression_content(lhs);
        format!("{lhs} {operator} {rhs}")
    }
}
