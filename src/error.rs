use thiserror::Error;

#[derive(Error, Debug)]
pub enum CalcError {
    #[error("Division by zero")]
    DivByZero,
    
    #[error("Invalid expression")]
    InvalidExpr,
    
    #[error("Unexpected character: {0}")]
    UnexpectedChar(String),
    
    #[error("Mismatched parentheses")]
    MismatchedParen,
    
    #[error("Empty expression")]
    EmptyExpr,
}

pub type CalcResult<T> = Result<T, CalcError>;
