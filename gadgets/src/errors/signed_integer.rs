use snarkos_errors::gadgets::SynthesisError;

#[derive(Debug, Error)]
pub enum SignedIntegerError {
    #[error("Integer overflow")]
    Overflow,

    #[error("Division by zero")]
    DivisionByZero,

    #[error("{}", _0)]
    SynthesisError(#[from] SynthesisError),
}
