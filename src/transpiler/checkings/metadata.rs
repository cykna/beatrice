use std::collections::{HashMap, VecDeque};

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
            AST::FunctionCall { name, .. } => {
                //i dont think is needed to do the args checking again, so im not gonna do. If
                //finding some bugs, imma implement it
                let BeatriceType::Function { return_type, .. } = self.typeof_function(name)? else {
                    panic!(
                        "This is a bug. typeof_function method expected to return a function or error, but it returned anything else"
                    );
                };
                *return_type.clone()
            }
            AST::Struct { .. } => self.ast_typeof_struct(expr)?,
            AST::StructExpr { name, fields } => {
                let field_values = fields;

                let BeatriceType::Struct { fields, order } = self.typeof_struct(name)? else {
                    panic!("This is a bug. Expected typeof struct to return a struct type");
                };
                let mut flags = Vec::with_capacity(order.len());
                for (idx, field) in field_values.iter().enumerate() {
                    if let Some(field_type) = fields.get(&field.key) {
                        flags.push(order[idx].clone());
                        let expr_type = self.ast_typeof_expression(&field.value)?;
                        if *field_type != expr_type {
                            return Err(TypeError::InvalidFieldValue {
                                target: name.clone(),
                                field: field.key.clone(),
                                received: expr_type,
                                expected: field_type.clone(),
                            });
                        }
                    } else {
                        return Err(TypeError::InvalidFieldName {
                            field: field.key.clone(),
                            target_struct: name.clone(),
                        });
                    }
                }
                if flags.len() != order.len() {
                    return Err(TypeError::NotCorrectFields {
                        fields: order
                            .iter()
                            .filter_map(|f| {
                                if !flags.contains(f) {
                                    Some(f.clone())
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<String>>(),
                        target: name.clone(),
                    });
                } else {
                    BeatriceType::Struct { fields, order }
                }
            }
        };
        Ok(v)
    }

    ///Generates a beatrice type based on a struct declaration
    pub(crate) fn ast_typeof_struct(&mut self, s: &AST) -> Result<BeatriceType, TypeError> {
        let AST::Struct { fields, .. } = s else {
            panic!("This ia a bug. Expected to receive a struct");
        };
        let (fields, order) = {
            let mut order = Vec::with_capacity(fields.len());
            let mut mapfields = HashMap::with_capacity(fields.len());
            for field in fields {
                order.push(field.key.clone());
                mapfields.insert(
                    field.key.clone(),
                    Self::t_abstract_from_primitive(&field.kindof)?,
                );
            }
            (mapfields, order)
        };
        Ok(BeatriceType::Struct { fields, order })
    }
    ///Generates a beatrice type based on a function declaration
    pub(crate) fn ast_typeof_function(&mut self, f: &AST) -> Result<BeatriceType, TypeError> {
        let AST::Function {
            params, returntype, ..
        } = f
        else {
            panic!("This is a bug. Expected to receive a function");
        };
        let mut fparams = VecDeque::with_capacity(params.len());
        for param in params {
            fparams.push_back(Self::t_abstract_from_primitive(&param.kindof)?);
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
                return Ok(scope.kindof(identifier)?.clone());
            }
        }
        Err(TypeError::NotRecognizedVar(identifier.to_string()))
    }

    pub(crate) fn typeof_function(&self, identifier: &str) -> Result<BeatriceType, TypeError> {
        for scope in self.scopes() {
            if scope.has_function(identifier) {
                return Ok(scope.kindof(identifier)?.clone());
            }
        }
        Err(TypeError::NotRecognizedVar(identifier.to_string()))
    }

    pub(crate) fn typeof_struct(&self, identifier: &str) -> Result<BeatriceType, TypeError> {
        for scope in self.scopes() {
            if scope.has_struct(identifier) {
                return Ok(scope.kindof(identifier)?.clone());
            }
        }
        Err(TypeError::NotRecognizedType(identifier.to_string()))
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
                    let param_type = Self::t_abstract_from_primitive(&param.kindof)?;
                    self.current_scope_mut()
                        .define_variable(param.key.clone(), param_type);
                }
                let mut n = 0;
                for ast in body.body() {
                    if let AST::Return(ast) = ast {
                        n += 1;
                        let rtype = self.ast_typeof_expression(ast)?;
                        let BeatriceType::Function { return_type, .. } = &ftype else {
                            unreachable!();
                        };
                        dbg!(&rtype, &return_type);
                        if **return_type != rtype {
                            return Err(TypeError::UnexpectedType {
                                expected: *return_type.clone(),
                                received: rtype,
                            }); //change to unexpected type.
                        }
                    } else {
                        self.generate_metadata(ast)?;
                    }
                }
                if n == 0 {
                    if let BeatriceType::Function { return_type, .. } = &ftype {
                        if BeatriceType::Void != **return_type {
                            return Err(TypeError::UnexpectedType {
                                expected: *return_type.clone(),
                                received: BeatriceType::Void,
                            });
                        }
                    }
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
            AST::FunctionCall { name, args } => {
                let BeatriceType::Function { params, .. } = self.typeof_function(name)? else {
                    panic!("What in the fuck? This isn't a function");
                };
                for (idx, param) in params.iter().enumerate() {
                    let arg_type = self.ast_typeof_expression(&args[idx])?;
                    if arg_type != *param {
                        return Err(TypeError::UnexpectedType {
                            expected: param.clone(),
                            received: arg_type,
                        });
                    }
                }
            }
            AST::Struct { name, .. } => {
                let stype = self.ast_typeof_struct(ast)?;
                self.current_scope_mut().define_struct(name.clone(), stype);
            }
            AST::StructExpr { .. } => {
                self.ast_typeof_expression(ast)?;
            }
        };
        Ok(())
    }
}
