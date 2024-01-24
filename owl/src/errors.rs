use crate::ox;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OwlError {
    info: String,
    internal: Option<ox::OxError>
}
impl OwlError {
    pub(crate) fn custom(message: &str) -> Self {
        Self { info: message.to_owned(), internal: None }
    }
    pub(crate) fn with_context(self, context: &str) -> Self {
        Self { info: format!("{context}, {}", self.info), ..self }
    }
}

impl std::fmt::Display for OwlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.internal {
            Some(ref internal) => write!(f,"{}: {}", self.info, internal),
            None => write!(f, "{}: Other", self.info),
        }
        
    }
}

impl std::error::Error for OwlError {}

#[allow(clippy::redundant_pub_crate)] // emphasise should not be used externally, despite currently private mod
pub(crate) trait UnmessagedError {
    type MessagedError;
    fn with_message(self, message: &str) -> Self::MessagedError;
}

impl UnmessagedError for ox::OxError {
    type MessagedError = OwlError;
    fn with_message(self, message: &str) -> OwlError {
        OwlError { internal: Some(self), info: message.to_owned() }
    }
}

impl<T> UnmessagedError for Result<T, ox::OxError> {
    type MessagedError = Result<T, OwlError>;
    fn with_message(self, message: &str) -> Result<T, OwlError> {
        self.map_err(|e| e.with_message(message))
    }
}
