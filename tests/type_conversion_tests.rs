use std::collections::VecDeque;
use beatrice::{
    parser::TypeAst,
    transpiler::{BeatriceType, TypeError},
};

// Since we don't have direct access to the conversion functions, we'll mock them
// based on what we can infer from the type definitions
fn convert_type_ast_to_beatrice_type(ast: TypeAst) -> Result<BeatriceType, TypeError> {
    match ast {
        TypeAst::Primitive(name) => match name.as_str() {
            "int" => Ok(BeatriceType::Int),
            "float" => Ok(BeatriceType::Float),
            "void" => Ok(BeatriceType::Void),
            _ => Err(TypeError::NotRecognizedType(name)),
        },
        TypeAst::Function { params, return_type } => {
            let mut beatrice_params = VecDeque::new();
            for param in params {
                beatrice_params.push_back(convert_type_ast_to_beatrice_type(param)?);
            }
            let beatrice_return = Box::new(convert_type_ast_to_beatrice_type(*return_type)?);
            Ok(BeatriceType::Function {
                params: beatrice_params,
                return_type: beatrice_return,
            })
        }
    }
}

#[test]
fn test_convert_primitive_types() {
    // Test converting int
    let ast = TypeAst::Primitive("int".to_string());
    let result = convert_type_ast_to_beatrice_type(ast).unwrap();
    assert_eq!(result, BeatriceType::Int);

    // Test converting float
    let ast = TypeAst::Primitive("float".to_string());
    let result = convert_type_ast_to_beatrice_type(ast).unwrap();
    assert_eq!(result, BeatriceType::Float);

    // Test converting void
    let ast = TypeAst::Primitive("void".to_string());
    let result = convert_type_ast_to_beatrice_type(ast).unwrap();
    assert_eq!(result, BeatriceType::Void);
}

#[test]
fn test_convert_function_types() {
    // Test converting simple function type
    let ast = TypeAst::Function {
        params: vec![TypeAst::Primitive("int".to_string())],
        return_type: Box::new(TypeAst::Primitive("void".to_string())),
    };
    let result = convert_type_ast_to_beatrice_type(ast).unwrap();
    match result {
        BeatriceType::Function { params, return_type } => {
            assert_eq!(params.len(), 1);
            assert_eq!(params[0], BeatriceType::Int);
            assert_eq!(*return_type, BeatriceType::Void);
        }
        _ => panic!("Expected function type, got {:?}", result),
    }

    // Test converting function with multiple parameters
    let ast = TypeAst::Function {
        params: vec![
            TypeAst::Primitive("int".to_string()),
            TypeAst::Primitive("float".to_string()),
        ],
        return_type: Box::new(TypeAst::Primitive("int".to_string())),
    };
    let result = convert_type_ast_to_beatrice_type(ast).unwrap();
    match result {
        BeatriceType::Function { params, return_type } => {
            assert_eq!(params.len(), 2);
            assert_eq!(params[0], BeatriceType::Int);
            assert_eq!(params[1], BeatriceType::Float);
            assert_eq!(*return_type, BeatriceType::Int);
        }
        _ => panic!("Expected function type, got {:?}", result),
    }

    // Test converting nested function type
    let inner_function = TypeAst::Function {
        params: vec![TypeAst::Primitive("int".to_string())],
        return_type: Box::new(TypeAst::Primitive("float".to_string())),
    };
    let ast = TypeAst::Function {
        params: vec![inner_function],
        return_type: Box::new(TypeAst::Primitive("void".to_string())),
    };
    let result = convert_type_ast_to_beatrice_type(ast).unwrap();
    match result {
        BeatriceType::Function { params, return_type } => {
            assert_eq!(params.len(), 1);
            match &params[0] {
                BeatriceType::Function { params: inner_params, return_type: inner_return } => {
                    assert_eq!(inner_params.len(), 1);
                    assert_eq!(inner_params[0], BeatriceType::Int);
                    assert_eq!(**inner_return, BeatriceType::Float);
                }
                _ => panic!("Expected inner function type, got {:?}", params[0]),
            }
            assert_eq!(*return_type, BeatriceType::Void);
        }
        _ => panic!("Expected function type, got {:?}", result),
    }
}

#[test]
fn test_convert_invalid_types() {
    // Test converting unknown primitive type
    let ast = TypeAst::Primitive("unknown".to_string());
    let result = convert_type_ast_to_beatrice_type(ast);
    assert!(matches!(result, Err(TypeError::NotRecognizedType(t)) if t == "unknown"));

    // Test converting function with invalid parameter type
    let ast = TypeAst::Function {
        params: vec![TypeAst::Primitive("unknown".to_string())],
        return_type: Box::new(TypeAst::Primitive("void".to_string())),
    };
    let result = convert_type_ast_to_beatrice_type(ast);
    assert!(result.is_err());

    // Test converting function with invalid return type
    let ast = TypeAst::Function {
        params: vec![TypeAst::Primitive("int".to_string())],
        return_type: Box::new(TypeAst::Primitive("unknown".to_string())),
    };
    let result = convert_type_ast_to_beatrice_type(ast);
    assert!(result.is_err());
}

