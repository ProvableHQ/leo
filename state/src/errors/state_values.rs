#[derive(Debug, Error)]
pub enum StateValuesError {
    #[error("state parameter `{}` not found in state file", _0)]
    MissingParameter(String),
}
