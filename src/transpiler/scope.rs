use std::collections::HashSet;

use super::{BeatriceType, checkings::checker::TypeChecker};

pub struct Scope {
    var_names: HashSet<String>,
    function_names: HashSet<String>,
    types: TypeChecker,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            var_names: HashSet::new(),
            function_names: HashSet::new(),
            types: TypeChecker::new(),
        }
    }

    pub fn define_variable(&mut self, varname: String, kind: BeatriceType) {
        self.var_names.insert(varname.clone());
        self.types.define(varname, kind);
    }

    pub fn define_function(&mut self, name: String, kind: BeatriceType) {
        assert!(matches!(kind, BeatriceType::Function { .. }));
        self.function_names.insert(name.clone());
        self.types.define(name, kind);
    }

    pub fn has_variable_or_function(&self, name: &str) -> bool {
        self.var_names.contains(name) || self.function_names.contains(name)
    }

    /// Gets the typeof a variable. Throws if it does not exist
    pub fn kindof(&self, name: &str) -> &BeatriceType {
        self.types.get(name).unwrap()
    }
}
