use snarkos_errors::gadgets::SynthesisError;

#[derive(Debug, Error)]
pub enum IntegerError {
    #[error("Negative zero")]
    NegativeZero,

    #[error("Integer overflow")]
    Overflow,

    #[error("{}", _0)]
    SynthesisError(#[from] SynthesisError),
}
