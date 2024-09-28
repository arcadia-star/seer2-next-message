use crate::utils::n_bytes::ToNBytes;
use crate::utils::null_terminated_str::NullTerminatedStr;
use serde::de::{Error, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Debug, Formatter};
use std::ops::Deref;

#[derive(Clone, Default)]
pub struct CString<const N: usize> {
    data: String,
}
impl<const N: usize> Debug for CString<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.data, f)
    }
}
impl<const N: usize> Deref for CString<N> {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<const N: usize> Serialize for CString<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bytes: [_; N] = self.data.n_bytes();
        serializer.serialize_bytes(&bytes)
    }
}
impl<'de, const N: usize> Deserialize<'de> for CString<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct V<const N: usize>;
        impl<'de, const N: usize> Visitor<'de> for V<N> {
            type Value = CString<N>;
            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                write!(formatter, "not a valid c-string, expect:{}", N)
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut bytes = [0u8; N];
                for byte in &mut bytes {
                    match seq.next_element()? {
                        Some(b) => *byte = b,
                        None => Err(Error::custom("not a valid c-string"))?,
                    }
                }
                Ok(CString { data: bytes.null_terminated_string() })
            }
        }
        deserializer.deserialize_tuple(N, V)
    }
}
impl<const N: usize> CString<N> {
    pub fn new(data: &str) -> CString<N> {
        Self { data: data.to_string() }
    }
}