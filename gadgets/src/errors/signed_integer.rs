use snarkos_errors::gadgets::SynthesisError;

#[derive(Debug, Error)]
pub enum SignedIntegerError {
    #[error("Division by zero")]
    DivisionByZero,

    #[error("Integer overflow")]
    Overflow,

    #[error("{}", _0)]
    SynthesisError(#[from] SynthesisError),
}
