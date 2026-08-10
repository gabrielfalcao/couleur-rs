use winnow::error::ContextError;
pub type Result<T> = std::result::Result<T, ContextError>;
pub type Stream<'i> = &'i str;
