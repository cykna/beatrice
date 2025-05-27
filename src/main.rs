use transpiler::{TypeError, transpiler::BeatriceTranspiler};

mod parser;
mod tokenizer;
mod transpiler;

fn transpile_file() -> Result<(), TypeError> {
    println!("What's the name of the file at 'templates' folder? ");
    let mut file_name = String::new();

    std::io::stdin().read_line(&mut file_name).unwrap();
    println!("Trying to read file ./templates/{file_name}");

    file_name.split_off(file_name.len() - 1).truncate(0);

    let file_content = std::fs::read_to_string(format!("./templates/{file_name}",)).unwrap();

    let mut parser = parser::Parser::from_content(&file_content);
    let ast = parser.gen_ast();
    let mut transpiler = BeatriceTranspiler::new(format!("./out/{file_name}.out.js"));
    transpiler.start_transpilation(ast.unwrap().body())?;
    Ok(())
}

fn main() {
    println!("{:?}", transpile_file());
}
