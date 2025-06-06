use std::collections::{HashMap, VecDeque};

pub mod checkings;
pub mod scope;
pub mod transpilation;
pub mod transpiler;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BeatriceType {
    Void,
    Int,
    Float,
    Function {
        params: VecDeque<BeatriceType>,
        return_type: Box<BeatriceType>,
    },
    Struct {
        fields: HashMap<String, BeatriceType>,
        order: Vec<String>,
    },
}
#[derive(Debug)]
pub enum TypeError {
    NotRecognizedVar(String),
    NotRecognizedType(String),
    ExpectedValue, //must implement track of line and column
    UnexpectedType {
        expected: BeatriceType,
        received: BeatriceType,
    },
    InvalidFieldName {
        field: String,
        target_struct: String,
    },
    InvalidFieldValue {
        target: String,
        field: String,
        expected: BeatriceType,
        received: BeatriceType,
    },
    NotCorrectFields {
        //for when not passing a field that is required. I must think in a better
        //name
        fields: Vec<String>,
        target: String,
    },
    IfElseWrong {
        ifbranch: BeatriceType,
        elsebranch: BeatriceType,
    },
}
