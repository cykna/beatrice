use std::collections::VecDeque;

use crate::{
    parser::{AST, TypeAst},
    transpiler::{BeatriceType, TypeError, transpiler::BeatriceTranspiler},
};

impl BeatriceTranspiler {
    fn ast_typeof_expression(&mut self, expr: &AST) -> Result<BeatriceType, TypeError> {
        let v = match expr {
            AST::Float(_) => BeatriceType::Float,
            AST::Int(_) => BeatriceType::Int,
            AST::VarDecl { body, .. } => self.ast_typeof_expression(body)?,
            AST::Identifier(s) => self.typeof_var(s)?,
            AST::BinExpr(lhs, ..) => self.ast_typeof_expression(lhs)?,
            AST::Function { .. } => self.ast_typeof_function(expr)?,
            AST::Return(r) => self.ast_typeof_expression(r)?,
        };
        Ok(v)
    }

    pub(crate) fn ast_typeof_function(&mut self, f: &AST) -> Result<BeatriceType, TypeError> {
        let AST::Function {
            params, returntype, ..
        } = f
        else {
            panic!("This is a bug. Expected to receive a function");
        };
        let mut fparams = VecDeque::with_capacity(params.len());
        for param in params {
            fparams.push_back(Self::t_abstract_from_primitive(&param.paramtype)?);
        }
        let rtype = Self::t_abstract_from_primitive(returntype)?;
        Ok(BeatriceType::Function {
            params: fparams,
            return_type: Box::new(rtype),
        })
    }

    ///Following the pattern of t_abstract<name> this is the function that generates an
    ///BeatriceType based on a primitive TypeAst type generate on parsing
    pub(crate) fn t_abstract_from_primitive(datatype: &TypeAst) -> Result<BeatriceType, TypeError> {
        let v = match datatype {
            TypeAst::Primitive(s) => match s.as_ref() {
                "void" => BeatriceType::Void,
                "int" => BeatriceType::Int,
                "float" => BeatriceType::Float,
                _ => return Err(TypeError::NotRecognizedType(s.clone())),
            },
            TypeAst::Function {
                params,
                return_type,
            } => {
                let mut fparams = VecDeque::with_capacity(params.len());
                for param in params {
                    fparams.push_back(Self::t_abstract_from_primitive(param)?);
                }
                let rtype = Self::t_abstract_from_primitive(return_type)?;
                BeatriceType::Function {
                    params: fparams,
                    return_type: Box::new(rtype),
                }
            }
        };
        Ok(v)
    }

    pub(crate) fn typeof_var(&self, identifier: &str) -> Result<BeatriceType, TypeError> {
        for scope in self.scopes() {
            if scope.has_variable_or_function(identifier) {
                return Ok(scope.kindof(identifier).clone());
            }
        }
        Err(TypeError::NotRecognizedVar(identifier.to_string()))
    }

    pub(crate) fn generate_metadata(&mut self, ast: &AST) -> Result<(), TypeError> {
        match ast {
            AST::Function {
                name, body, params, ..
            } => {
                let ftype = self.ast_typeof_function(ast)?;
                self.current_scope_mut()
                    .define_function(name.clone(), ftype.clone());
                self.enter_scope();

                for param in params {
                    let param_type = Self::t_abstract_from_primitive(&param.paramtype)?;
                    self.current_scope_mut()
                        .define_variable(param.paramname.clone(), param_type);
                }
                for ast in body.body() {
                    if let AST::Return(ast) = ast {
                        let rtype = self.ast_typeof_expression(ast)?;
                        let BeatriceType::Function { return_type, .. } = &ftype else {
                            unreachable!();
                        };
                        if **return_type != rtype {
                            return Err(TypeError::UnexpectedType {
                                expected: *return_type.clone(),
                                received: rtype,
                            }); //change to unexpected type.
                        }
                    }
                    self.generate_metadata(ast)?;
                }
            }
            AST::Identifier(s) => {
                self.typeof_var(s)?;
            }
            AST::Int(_) | AST::Float(_) => {}
            AST::VarDecl { varname, body, .. } => {
                let expr_kind = self.ast_typeof_expression(body)?;
                self.current_scope_mut()
                    .define_variable(varname.clone(), expr_kind);
            }
            AST::BinExpr(lhs, rhs, _) => {
                //checks if the bin expr is valid
                self.generate_metadata(lhs)?;
                self.generate_metadata(rhs)?;
            }
            AST::Return(r) => self.generate_metadata(r)?,
        };
        Ok(())
    }
}
