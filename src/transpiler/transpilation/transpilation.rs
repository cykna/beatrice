use std::{collections::VecDeque, io::Write};

use crate::{parser::AST, transpiler::transpiler::BeatriceTranspiler};

impl BeatriceTranspiler {
    pub(crate) fn generate_expression_content(&mut self, ast: &AST) -> String {
        match ast {
            AST::Function { .. } => self.generate_function_content(ast),
            AST::Identifier(s) => s.clone(),
            AST::Float(f) => f.to_string(),
            AST::Int(i) => i.to_string(),
            AST::VarDecl { .. } => self.generate_var_decl_content(ast),
            AST::BinExpr(..) => self.generate_binexpr_content(ast),
            AST::Return(r) => match &**r {
                AST::Loop(body) => {
                    format!("{};", self.generate_loop_content(body))
                }
                AST::If { .. } => {
                    self.generate_if_expr_assign(r, &super::TranspileCondition::Return)
                }
                r => format!("return {};", self.generate_expression_content(r)),
            },
            AST::FunctionCall { name, args } => self.generate_fcall_content(name, args),
            AST::Struct { .. } => "".to_string(),
            AST::StructExpr { name, fields } => {
                let mut out = format!("/**{name}*/ {{");
                for field in fields {
                    match &field.value {
                        AST::Identifier(fval) if field.key == *fval => {
                            out.push_str(&field.key);
                            out.push(',');
                        }
                        _ => {
                            let content = self.generate_expression_content(&field.value);
                            out.push_str(&format!("{}:{content},", field.key));
                        }
                    }
                }
                out.split_off(out.len() - 1).truncate(0);
                out.push('}');
                out
            }
            AST::If { .. } => self.generate_if_expr(ast),
            AST::Block(asts) => {
                let mut out = String::from("{\n");
                self.increase_identation_level();
                for ast in asts {
                    let content = self.generate_expression_content(ast);
                    out.push_str(&self.indent(content));
                    out.push('\n');
                }
                self.decrease_identation_level();
                out.push_str(&self.indent(""));
                out.push('}');
                out
            }
            AST::Loop(body) => self.generate_loop_content(body),
        }
    }
    fn generate_function_content(&mut self, ast: &AST) -> String {
        self.increase_identation_level();
        let AST::Function {
            name, params, body, ..
        } = ast
        else {
            panic!("This is a bug. Expected to receive a function");
        };
        let mut content = format!("function {name}(");
        {
            let mut param_amount = 0;
            for param in params {
                content.push_str(&param.key);
                content.push(',');
                param_amount += 1;
            }
            if param_amount > 0 {
                content.split_off(content.len() - 1).truncate(0);
            }
        }
        content.push_str("){\n");
        let body = self.generate_transpilation_content(body.body());
        content.push_str(&body);
        self.decrease_identation_level();
        content.push_str(&self.indent("}\n"));
        content
    }
    fn generate_transpilation_content(&mut self, ast: &VecDeque<AST>) -> String {
        let mut content = String::new();
        for ast in ast {
            let exprcontent = self.generate_expression_content(ast);
            content.push_str(&self.indent(exprcontent));
            content.push('\n');
        }
        content
    }
    pub fn transpile(&mut self, ast: &VecDeque<AST>) -> Result<usize, ()> {
        let content = self.generate_transpilation_content(ast);
        let mut f = std::fs::File::create(self.outdir()).unwrap();
        println!("Writing into {:?}:\n\n{}", self.outdir(), content);
        let bytes = f.write(content.as_bytes()).unwrap();
        Ok(bytes)
    }
}
