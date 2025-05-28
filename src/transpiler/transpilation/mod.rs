mod basics;
mod conditions;
mod function;
mod loops;
mod transpilation;
mod vars;

pub enum TranspileCondition {
    Assign(String),
    Return,
    None,
}
