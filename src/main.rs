mod parser;
mod tokenizer;

fn main() {
    let file_content = std::fs::read_to_string("./templates/func.bt").unwrap();
    let mut parser = parser::Parser::from_content(&file_content);
    println!("{:#?}", parser.gen_ast());
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vardecl() {
        let vardecl = "let a = 52 * 8.5";
        let tokens = tokenize(vardecl);
        assert_eq!(tokens[0].kind, TokenKind::Reserved(Reserved::Let));
        assert_eq!(tokens[1].kind, TokenKind::Identifier(String::from("a")));
        assert_eq!(tokens[2].kind, TokenKind::Operator(Operator::Eq(false)));
        assert_eq!(tokens[3].kind, TokenKind::Int(String::from("52")));
        assert_eq!(tokens[4].kind, TokenKind::Operator(Operator::Star(false)));
        assert_eq!(tokens[5].kind, TokenKind::Float(String::from("8.5")));
    }
}
