use std::{collections::VecDeque, io::Write};

use crate::{parser::AST, transpiler::transpiler::BeatriceTranspiler};

impl BeatriceTranspiler {
    fn recursive_bin_expr(&mut self, ast: &AST) -> String {
        let AST::BinExpr(lhs, rhs, operator) = ast else {
            panic!("This is a bug. Expected receiving a BinExpr");
        };
        let lhs = self.generate_expression_content(lhs);
        let rhs = self.generate_expression_content(rhs);
        format!("({lhs}{operator}{rhs})")
    }

    fn generate_fcall_content(&mut self, name: &str, args: &VecDeque<AST>) -> String {
        let mut params = Vec::with_capacity(args.len());
        for arg in args {
            params.push(self.generate_expression_content(arg));
        }
        format!("{name}({})", params.join(","))
    }

    fn generate_expression_content(&mut self, ast: &AST) -> String {
        match ast {
            AST::Function { .. } => self.generate_function_content(ast),
            AST::Identifier(s) => s.clone(),
            AST::Float(f) => f.to_string(),
            AST::Int(i) => i.to_string(),
            AST::VarDecl {
                varname,
                mutable,
                body,
            } => {
                let content = self.generate_expression_content(body);
                if *mutable {
                    format!("let {varname} = {content};")
                } else {
                    format!("const {varname} = {content};")
                }
            }
            AST::BinExpr(lhs, rhs, operator) => {
                let lhs = if lhs.is_binexpr() {
                    self.recursive_bin_expr(lhs)
                } else {
                    self.generate_expression_content(lhs)
                };
                let rhs = if rhs.is_binexpr() {
                    self.recursive_bin_expr(rhs)
                } else {
                    self.generate_expression_content(rhs)
                };
                let content = format!("{}{}{}", lhs, operator, rhs);
                content
            }
            AST::Return(r) => format!("return {};", self.generate_expression_content(r)),
            AST::FunctionCall { name, args } => self.generate_fcall_content(name, args),
            AST::Struct { .. } => "".to_string(),
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
        content.push_str(&format!("{}}}\n", " ".repeat(self.indentation_level())));
        content
    }
    fn generate_transpilation_content(&mut self, ast: &VecDeque<AST>) -> String {
        let split = " ".repeat(self.indentation_level());
        let mut content = String::new();
        for ast in ast {
            content.push_str(&split);
            content.push_str(&self.generate_expression_content(ast));
            content.push('\n');
        }
        content
    }
    pub fn transpile(&mut self, ast: &VecDeque<AST>) -> Result<usize, ()> {
        let content = self.generate_transpilation_content(ast);
        let mut f = std::fs::File::create(self.outdir()).unwrap();
        let bytes = f.write(content.as_bytes()).unwrap();
        Ok(bytes)
    }
}
