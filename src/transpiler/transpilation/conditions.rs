use crate::{parser::AST, transpiler::transpiler::BeatriceTranspiler};

use super::TranspileCondition;

impl BeatriceTranspiler {
    fn generate_if_branch(
        &mut self,
        cond: &AST,
        ast: &AST,
        condition: &TranspileCondition,
    ) -> String {
        let cond = self.generate_expression_content(cond);
        let cond_value = match condition {
            TranspileCondition::Assign(s) => format!("{s} ="),
            TranspileCondition::Return => "return".to_string(),
            _ => panic!("If Expression should have a condition"),
        };
        if !ast.is_blockexpr() {
            let assign = self.generate_expression_content(ast);
            let mut out = self.indent(format!("if({cond}) {cond_value} {assign};\n"));
            out.push_str(&self.indent(""));
            out
        } else {
            let mut content = self.indent(format!("if({cond}){{\n"));
            self.increase_identation_level();
            let AST::Block(exprs) = ast else {
                unreachable!()
            };
            if let Some(last) = exprs.back() {
                for (idx, expr) in exprs.iter().enumerate() {
                    if idx == exprs.len() - 1 {
                        break;
                    };
                    let exprcontent = format!("{}\n", self.generate_expression_content(expr));
                    content.push_str(&self.indent(exprcontent));
                }
                if matches!(last, AST::If { .. }) {
                    let exprassign =
                        format!("{};\n", self.generate_if_expr_assign(last, condition));
                    content.push_str(&exprassign);
                } else {
                    let exprcontent = self.generate_expression_content(last);
                    content.push_str(&self.indent(format!("{cond_value} {exprcontent};\n")));
                };
            }
            content
        }
    }
    fn generate_else_branch(&mut self, elseblock: &AST, condition: &TranspileCondition) -> String {
        let cond_value = match condition {
            TranspileCondition::Assign(s) => format!("{s} ="),
            TranspileCondition::Return => "return".to_string(),
            _ => panic!("If Expression should have a condition"),
        };

        if elseblock.is_blockexpr() {
            let mut content = "else {\n".to_string();
            self.increase_identation_level();

            let AST::Block(exprs) = elseblock else {
                unreachable!()
            };
            if let Some(last) = exprs.back() {
                for (idx, expr) in exprs.iter().enumerate() {
                    if idx == exprs.len() - 1 {
                        break;
                    };
                    let exprcontent = self.generate_expression_content(expr);
                    content.push_str(&self.indent(exprcontent));
                    content.push('\n');
                }
                if matches!(last, AST::If { .. }) {
                    let exprassign = self.generate_if_expr_assign(last, condition);
                    content.push_str(&exprassign);
                    content.push('\n');
                } else {
                    let exprcontent = self.generate_expression_content(last);
                    content.push_str(&self.indent(format!("{cond_value} {exprcontent};\n")));
                };
            }
            self.decrease_identation_level();
            content.push_str(&self.indent("}"));
            content
        } else {
            format!(
                "else {cond_value} {};",
                self.generate_expression_content(elseblock)
            )
        }
    }
    pub(crate) fn generate_if_expr_assign(
        &mut self,
        ast: &AST,
        condition: &TranspileCondition,
    ) -> String {
        let AST::If {
            expr,
            block,
            elseblock,
        } = ast
        else {
            unreachable!()
        };
        if let Some(elseblock) = elseblock {
            let ifbranch = self.generate_if_branch(expr, block, condition);
            let elsebranch = self.generate_else_branch(elseblock, condition);
            format!("{ifbranch}{elsebranch}")
        } else {
            panic!(
                "Bug. Generate if expr assign should receive an AST of type If which contains an else branch!"
            );
        }
    }
    pub(crate) fn generate_if_expr(&mut self, ast: &AST) -> String {
        let AST::If {
            expr,
            block,
            elseblock,
        } = ast
        else {
            unreachable!()
        };

        let mut out = format!(
            "if({}){};",
            self.generate_expression_content(expr),
            self.generate_expression_content(block)
        );
        if let Some(elsebranch) = elseblock {
            out.push_str(" else ");
            out.push_str(&self.generate_expression_content(elsebranch));
        }
        out
    }
}
