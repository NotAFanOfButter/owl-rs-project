use crate::ox;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OwlError {
    info: String,
    internal: ox::OxError
}

impl std::fmt::Display for OwlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}: {}", self.info, self.internal)
    }
}
impl std::error::Error for OwlError {}
pub(crate) trait UnmessagedError {
    fn with_message(self, message: &str) -> OwlError;
}
impl UnmessagedError for ox::OxError {
    fn with_message(self, message: &str) -> OwlError {
        OwlError { internal: self, info: message.to_owned() }
    }
}
