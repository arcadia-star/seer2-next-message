use crate::utils::null_terminated_str::NullTerminatedStr;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;

#[derive(Clone, Default)]
pub struct UTFString {
    data: String,
}
impl Display for UTFString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.data, f)
    }
}
impl Debug for UTFString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.data, f)
    }
}
impl Deref for UTFString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl Serialize for UTFString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.data.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for UTFString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = Vec::<u8>::deserialize(deserializer)?;
        Ok(Self::new(&data.null_terminated_string()))
    }
}
impl UTFString {
    pub fn new(data: &str) -> UTFString {
        Self {
            data: data.to_string(),
        }
    }
}
