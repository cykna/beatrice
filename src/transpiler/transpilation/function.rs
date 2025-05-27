use std::collections::VecDeque;

use crate::{parser::AST, transpiler::transpiler::BeatriceTranspiler};

impl BeatriceTranspiler {
    pub(crate) fn generate_fcall_content(&mut self, name: &str, args: &VecDeque<AST>) -> String {
        let mut params = Vec::with_capacity(args.len());
        for arg in args {
            params.push(self.generate_expression_content(arg));
        }
        format!("{name}({})", params.join(","))
    }
}
