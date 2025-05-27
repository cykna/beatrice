use std::{collections::VecDeque, io::Write};

use crate::{
    parser::{AST, AstError},
    transpiler::transpiler::BeatriceTranspiler,
};

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
                            out.push_str(&self.generate_if_expr_assign(varname, body));
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
            AST::If {
                expr,
                block,
                elseblock,
            } => {
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
            AST::Block(asts) => {
                let mut out = String::from("{");
                for ast in asts {
                    out.push_str(&self.generate_expression_content(ast));
                }
                out.push('}');
                out
            }
        }
    }
    fn generate_if_expr_assign(&mut self, assigned: &String, ast: &AST) -> String {
        let AST::If {
            expr,
            block,
            elseblock,
        } = ast
        else {
            unreachable!()
        };
        if let Some(elseblock) = elseblock {
            let mut out = String::new();
            if block.is_blockexpr() {
                let mut content = {
                    let content = self.generate_expression_content(expr);
                    self.indent(format!("if({})", content))
                };
                content.push_str("{\n");
                self.increase_identation_level();
                let AST::Block(exprs) = &**block else {
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
                        let exprassign = self.generate_if_expr_assign(assigned, last);
                        content.push_str(&exprassign);
                        content.push('\n');
                    } else {
                        let exprcontent = self.generate_expression_content(last);
                        content.push_str(&self.indent(format!("{assigned} = {exprcontent};\n")));
                    };
                }
                self.decrease_identation_level();
                content.push_str(&self.indent("}"));
                out.push_str(&content);
            } else {
                let cond_content = self.generate_expression_content(expr);
                let blockexpr = self.generate_expression_content(block);
                out.push_str(&self.indent(format!(
                    "if({cond_content}) {assigned} = {blockexpr};\n{}",
                    " ".repeat(self.indentation_level())
                )));
            }
            if elseblock.is_blockexpr() {
                let mut content = "else {\n".to_string();
                self.increase_identation_level();

                let AST::Block(exprs) = &**elseblock else {
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
                        let exprassign = self.generate_if_expr_assign(assigned, last);
                        content.push_str(&exprassign);
                        content.push('\n');
                    } else {
                        let exprcontent = self.generate_expression_content(last);
                        content.push_str(&self.indent(format!("{assigned} = {exprcontent};\n")));
                    };
                }
                self.decrease_identation_level();
                content.push_str(&self.indent("}"));

                out.push_str(&content);
            } else {
                out.push_str(&format!(
                    "else {assigned} = {};",
                    self.generate_expression_content(elseblock)
                ));
            }
            out
        } else {
            panic!(
                "Bug. Generate if expr assign should receive an AST of type If which contains an else branch!"
            );
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
