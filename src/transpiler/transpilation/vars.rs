use crate::{parser::AST, transpiler::transpiler::BeatriceTranspiler};

impl BeatriceTranspiler {
    pub(crate) fn generate_var_decl_content(&mut self, ast: &AST) -> String {
        let AST::VarDecl {
            varname,
            mutable,
            body,
        } = ast
        else {
            unreachable!();
        };
        if let AST::StructExpr { .. } = **body {
            let content = self.generate_expression_content(body);
            if !*mutable {
                format!("const {varname} = Object.seal({content})")
            } else {
                format!("const {varname} = {content}")
            }
        } else if let AST::If {
            elseblock,
            block,
            expr,
        } = &**body
        {
            if let Some(ast) = elseblock {
                if !(block.is_blockexpr() || ast.is_blockexpr()) {
                    let cond_content = self.generate_expression_content(expr);
                    let ifcontent = self.generate_expression_content(block);
                    let elsecontent = self.generate_expression_content(ast);
                    format!(
                        "{} {varname} = {} ? {} : {}",
                        if *mutable { "let" } else { "const" },
                        cond_content,
                        ifcontent,
                        elsecontent
                    )
                } else {
                    let mut out = format!("let {varname};\n");
                    out.push_str(&self.generate_if_expr_assign(
                        body,
                        &super::TranspileCondition::Assign(varname.clone()),
                    ));
                    out
                }
            } else {
                unreachable!();
            }
        } else {
            let content = self.generate_expression_content(body);
            if *mutable {
                format!("let {varname} = {content};")
            } else {
                format!("const {varname} = {content};")
            }
        }
    }
}
