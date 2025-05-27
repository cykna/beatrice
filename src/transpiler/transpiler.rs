use std::{
    collections::VecDeque,
    fmt::Display,
    path::{Path, PathBuf},
};

use super::{TypeError, scope::Scope};
use crate::parser::AST;

pub struct BeatriceTranspiler {
    scopes: VecDeque<Scope>,
    target: PathBuf,
    indent_level: usize,
}

///The transpiler of Beatrice source code.
///The pattern of functions that generate types are  ast_<name> for methods that generate a type
///from an AST,  and t_abstract_<name> for methods that generate a type from abstract types,
///generated from parsing.
impl BeatriceTranspiler {
    pub fn new<T: Into<PathBuf>>(target: T) -> Self {
        Self {
            scopes: VecDeque::from(vec![Scope::new()]),
            target: target.into(),
            indent_level: 0,
        }
    }

    pub fn indent<T: Display>(&self, content: T) -> String {
        format!("{}{content}", " ".repeat(self.indent_level))
    }

    pub fn indentation_level(&self) -> usize {
        self.indent_level
    }
    pub fn increase_identation_level(&mut self) {
        self.indent_level += 4;
    }
    pub fn decrease_identation_level(&mut self) {
        self.indent_level -= 4;
    }
    pub fn outdir(&self) -> &Path {
        &self.target
    }

    pub fn scopes(&self) -> &VecDeque<Scope> {
        &self.scopes
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push_back(Scope::new());
    }
    /**
     * Exits the current scope. If the current scope is global scope, it doesn't exit
     */
    pub fn exit_scope(&mut self) -> Option<Scope> {
        if self.scopes.len() == 1 {
            None
        } else {
            self.scopes.pop_back()
        }
    }

    pub fn current_scope(&self) -> &Scope {
        self.scopes.back().unwrap()
    }

    pub fn current_scope_mut(&mut self) -> &mut Scope {
        self.scopes.back_mut().unwrap()
    }

    pub fn start_transpilation(&mut self, ast: &VecDeque<AST>) -> Result<(), TypeError> {
        for ast in ast {
            self.generate_metadata(ast)?;
        }
        self.transpile(ast).unwrap();
        Ok(())
    }
}
