use std::fmt;

#[derive(Debug)]
pub(crate) struct NoError;

impl fmt::Display for NoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}
