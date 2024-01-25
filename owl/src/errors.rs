use crate::ox;
#[allow(unused_imports)] // for use in other mods only
pub(crate) use crate::safe_bindings::Error as OriginalError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OwlError {
    context: String,
    error_message: String,
}
impl OwlError {
    pub(crate) fn custom(message: &str) -> Self {
        Self { error_message: message.to_owned(), context: String::new() }
    }
}

impl std::fmt::Display for OwlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.context, self.error_message)        
    }
}

impl std::error::Error for OwlError {}

pub(crate) trait ToOwlError {
    type OwlErrorType;
    fn with_context(self, context: &str) -> Self::OwlErrorType;
    fn with_message(self, message: &str) -> Self::OwlErrorType;
    /// Avoid most of the time, unless the original OpenGL message is actually useful by itself
    fn no_message(self) -> Self::OwlErrorType;
}

impl ToOwlError for ox::OxError {
    type OwlErrorType = OwlError;
    fn with_context(self, context: &str) -> OwlError {
        OwlError { context: context.to_owned(), error_message: format!("{self}") }
    }
    fn with_message(self, message: &str) -> Self::OwlErrorType {
        OwlError { context: String::new(), error_message: format!("{message}, {self}") }
    }
    fn no_message(self) -> Self::OwlErrorType {
        OwlError { context: String::new(), error_message: String::new() }
    }
}

impl<T> ToOwlError for Result<T, ox::OxError> {
    type OwlErrorType = Result<T, OwlError>;
    fn with_context(self, context: &str) -> Result<T, OwlError> {
        self.map_err(|e| e.with_context(context))
    }
    fn with_message(self, message: &str) -> Result<T, OwlError> {
        self.map_err(|e| e.with_message(message))
    }
    fn no_message(self) -> Self::OwlErrorType {
        Err(OwlError { context: String::new(), error_message: String::new() })
    }
}

impl ToOwlError for OwlError {
    type OwlErrorType = Self;
    fn with_context(self, context: &str) -> Self {
        let context = if self.context.is_empty() {
            context.to_owned()
        } else {
            format!("{context}, {}", self.context)
        };
        Self { context, ..self }
    }
    /// Replaces the previous message
    fn with_message(self, message: &str) -> Self {
        Self { error_message: message.to_owned(), ..self }
    }
    /// why you would use this, I have no idea
    fn no_message(self) -> Self::OwlErrorType {
        Self { context: String::new(), error_message: String::new() }
    }
}

impl<T> ToOwlError for Result<T,OwlError> {
    type OwlErrorType = Self;
    fn with_context(self, context: &str) -> Self {
        self.map_err(|e| e.with_context(context))
    }
    /// Replaces the previous message
    fn with_message(self, message: &str) -> Self {
        self.map_err(|e| e.with_message(message))
    }
    /// why you would use this, I have no idea
    fn no_message(self) -> Self::OwlErrorType {
        self.map_err(OwlError::no_message)
    }
}
