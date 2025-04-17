use beatrice::parser::{Parser, TypeAst};

fn create_parser(input: &str) -> Parser {
    Parser::from_content(input)
}

#[test]
fn test_parse_primitive_types() {
    // Test parsing int type
    let mut parser = create_parser("int");
    let result = parser.get_type().unwrap();
    assert!(matches!(result, TypeAst::Primitive(t) if t == "int"));

    // Test parsing float type
    let mut parser = create_parser("float");
    let result = parser.get_type().unwrap();
    assert!(matches!(result, TypeAst::Primitive(t) if t == "float"));

    // Test parsing void type
    let mut parser = create_parser("void");
    let result = parser.get_type().unwrap();
    assert!(matches!(result, TypeAst::Primitive(t) if t == "void"));

    // Test parsing custom type
    let mut parser = create_parser("MyCustomType");
    let result = parser.get_type().unwrap();
    assert!(matches!(result, TypeAst::Primitive(t) if t == "MyCustomType"));
}

#[test]
fn test_parse_function_types() {
    // Test parsing simple function type with no parameters
    let mut parser = create_parser("(): int");
    let result = parser.get_type().unwrap();
    match result {
        TypeAst::Function {
            params,
            return_type,
        } => {
            assert_eq!(params.len(), 0);
            assert!(matches!(*return_type, TypeAst::Primitive(t) if t == "int"));
        }
        _ => panic!("Expected function type, got {:?}", result),
    }

    // Test parsing function type with single parameter
    let mut parser = create_parser("(int): void");
    let result = parser.get_type().unwrap();
    match result {
        TypeAst::Function {
            params,
            return_type,
        } => {
            assert_eq!(params.len(), 1);
            assert!(matches!(&params[0], TypeAst::Primitive(t) if t == "int"));
            assert!(matches!(*return_type, TypeAst::Primitive(t) if t == "void"));
        }
        _ => panic!("Expected function type, got {:?}", result),
    }

    // Test parsing function type with multiple parameters
    let mut parser = create_parser("(int, float, void): int");
    let result = parser.get_type().unwrap();
    match result {
        TypeAst::Function {
            params,
            return_type,
        } => {
            assert_eq!(params.len(), 3);
            assert!(matches!(&params[0], TypeAst::Primitive(t) if t == "int"));
            assert!(matches!(&params[1], TypeAst::Primitive(t) if t == "float"));
            assert!(matches!(&params[2], TypeAst::Primitive(t) if t == "void"));
            assert!(matches!(*return_type, TypeAst::Primitive(t) if t == "int"));
        }
        _ => panic!("Expected function type, got {:?}", result),
    }

    // Test parsing nested function type
    let mut parser = create_parser("((int): float): void");
    let result = parser.get_type().unwrap();
    match result {
        TypeAst::Function {
            params,
            return_type,
        } => {
            assert_eq!(params.len(), 1);
            match &params[0] {
                TypeAst::Function {
                    params: inner_params,
                    return_type: inner_return,
                } => {
                    assert_eq!(inner_params.len(), 1);
                    assert!(matches!(&inner_params[0], TypeAst::Primitive(t) if t == "int"));
                    assert!(matches!(&**inner_return, TypeAst::Primitive(t) if t == "float"));
                }
                _ => panic!("Expected inner function type, got {:?}", params[0]),
            }
            assert!(matches!(*return_type, TypeAst::Primitive(t) if t == "void"));
        }
        _ => panic!("Expected function type, got {:?}", result),
    }
}

#[test]
fn test_parse_invalid_types() {
    // Test invalid syntax - missing colon in function type
    let mut parser = create_parser("() int");
    assert!(parser.get_type().is_err());

    // Test invalid syntax - missing closing parenthesis
    let mut parser = create_parser("(int: void");
    assert!(parser.get_type().is_err());

    // Test invalid token - using operators or unexpected tokens
    let mut parser = create_parser("+");
    assert!(parser.get_type().is_err());

    // Test empty input
    let mut parser = create_parser("");
    assert!(parser.get_type().is_err());
}
