use std::collections::VecDeque;

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
}
