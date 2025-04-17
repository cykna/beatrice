use std::collections::HashMap;

use crate::transpiler::BeatriceType;

#[derive(Default)]
pub struct TypeChecker {
    var_types: HashMap<String, BeatriceType>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            var_types: HashMap::new(),
        }
    }
    pub fn define(&mut self, varname: String, kind: BeatriceType) {
        self.var_types.insert(varname, kind);
    }
    pub fn get(&self, name: &str) -> Option<&BeatriceType> {
        self.var_types.get(name)
    }
}
