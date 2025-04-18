use std::collections::{HashSet, VecDeque};

use crate::parser::KeyTypePair;

use super::{BeatriceType, TypeError, checkings::checker::TypeChecker};

#[derive(Default)]
pub struct Scope {
    var_names: HashSet<String>,
    function_names: HashSet<String>,
    struct_names: HashSet<String>,
    types: TypeChecker,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            var_names: HashSet::new(),
            function_names: HashSet::new(),
            struct_names: HashSet::new(),
            types: TypeChecker::new(),
        }
    }

    #[inline]
    pub fn define_variable(&mut self, varname: String, kind: BeatriceType) {
        self.var_names.insert(varname.clone());
        self.types.define(varname, kind);
    }

    #[inline]
    pub fn define_function(&mut self, name: String, kind: BeatriceType) {
        assert!(matches!(kind, BeatriceType::Function { .. }));
        self.function_names.insert(name.clone());
        self.types.define(name, kind);
    }
    #[inline]
    pub fn define_struct(&mut self, name: String, kind: BeatriceType) {
        assert!(matches!(kind, BeatriceType::Struct { .. }));
        self.struct_names.insert(name.to_string());
        self.types.define(name, kind);
    }

    #[inline]
    pub fn has_function(&self, name: &str) -> bool {
        self.function_names.contains(name)
    }

    #[inline]
    pub fn has_struct(&self, name: &str) -> bool {
        self.struct_names.contains(name)
    }

    #[inline]
    pub fn has_variable_or_function(&self, name: &str) -> bool {
        self.var_names.contains(name) || self.function_names.contains(name)
    }

    #[inline]
    /// Gets the typeof a variable. Throws if it does not exist
    pub fn kindof(&self, name: &str) -> Result<&BeatriceType, TypeError> {
        self.types
            .get(name)
            .ok_or(TypeError::NotRecognizedVar(name.to_string()))
    }
}
