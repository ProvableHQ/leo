#[derive(Debug, Error)]
pub enum StateLeafValuesError {
    #[error("state parameter `{}` not found in state file", _0)]
    MissingParameter(String),
}
