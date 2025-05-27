use crate::{parser::AST, transpiler::transpiler::BeatriceTranspiler};

impl BeatriceTranspiler {
    pub(crate) fn generate_loop_content(&mut self, body: &AST) -> String {
        let content = self.generate_expression_content(body);
        self.indent(format!("for(;;){}", content))
    }
}
