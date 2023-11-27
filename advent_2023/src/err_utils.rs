use thiserror::Error;

#[derive(Error, Debug)]
#[error("Option Empty")]
pub struct OptionEmptyError;

pub trait OptionUtils {
    type Target;
    fn ok_or_err(self) -> Result<Self::Target, OptionEmptyError>;
}

impl<T> OptionUtils for Option<T> {
    type Target = T;
    fn ok_or_err(self) -> Result<Self::Target, OptionEmptyError> {
        match self {
            Some(e) => Ok(e),
            None => Err(OptionEmptyError {}),
        }
    }
}
