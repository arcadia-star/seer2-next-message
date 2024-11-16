use bytes::Bytes;
use serde::de::{Error, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;

#[derive(Clone, Default)]
pub struct Hex<const N: usize> {
    data: Bytes,
}
impl<const N: usize> Display for Hex<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&hex::encode(&self.data))
    }
}
impl<const N: usize> Debug for Hex<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&hex::encode(&self.data))
    }
}
impl<const N: usize> Deref for Hex<N> {
    type Target = Bytes;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<const N: usize> Serialize for Hex<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut bytes = [0u8; N];
        for i in 0..self.data.len().min(N) {
            bytes[i] = self.data[i];
        }
        serializer.serialize_bytes(&bytes)
    }
}
impl<'de, const N: usize> Deserialize<'de> for Hex<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct V<const N: usize>;
        impl<'de, const N: usize> Visitor<'de> for V<N> {
            type Value = Hex<N>;
            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                write!(formatter, "not valid, expect:{}", N)
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut bytes = [0u8; N];
                for byte in &mut bytes {
                    match seq.next_element()? {
                        Some(b) => *byte = b,
                        None => Err(Error::custom("not valid"))?,
                    }
                }
                Ok(Hex { data: Bytes::copy_from_slice(&bytes) })
            }
        }
        deserializer.deserialize_tuple(N, V)
    }
}
impl<const N: usize> Hex<N> {
    pub fn new(data: Bytes) -> Hex<N> {
        Self { data }
    }
    pub fn new_copy(data: &[u8]) -> Hex<N> {
        Self { data: Bytes::copy_from_slice(data) }
    }
}