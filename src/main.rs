use transpiler::{TypeError, transpiler::BeatriceTranspiler};

mod parser;
mod tokenizer;
mod transpiler;

fn transpile_file(name: &str) -> Result<(), TypeError> {
    let file_content = std::fs::read_to_string(format!("./templates/{name}.bt")).unwrap();
    let mut parser = parser::Parser::from_content(&file_content);
    let ast = parser.gen_ast();
    let mut transpiler = BeatriceTranspiler::new(format!("./out/{name}.js"));
    transpiler.start_transpilation(ast.unwrap().body())?;
    Ok(())
}

fn main() {
    println!("{:?}", transpile_file("struct"));
}
