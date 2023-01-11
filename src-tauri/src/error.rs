use anyhow::Error;
use core::fmt::{Debug, Display};
use serde::{Serialize, Serializer};
use std::convert::From;

pub struct SerializeError(Error);
pub type Result<T, E = SerializeError> = std::result::Result<T, E>;

impl Serialize for SerializeError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.0.to_string().as_str())
    }
}

impl<M> From<M> for SerializeError
where
    M: Display + Debug + Send + Sync + 'static,
{
    fn from(value: M) -> Self {
        Self(Error::msg(value))
    }
}
