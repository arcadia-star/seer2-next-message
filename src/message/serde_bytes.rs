use std::fmt;
use std::fmt::Display;

use bytes::{Buf, BufMut, Bytes, BytesMut};
use serde::de::{DeserializeSeed, SeqAccess, Visitor};
use serde::{de, ser, Deserialize, Serialize};

#[derive(Debug)]
pub struct SerdeError(pub String);

macro_rules! ensure_len {
    ($self:ident, $len:expr) => {
        if $self.bytes.remaining() < $len {
            return Err(SerdeError(format!("no more remaining data, len:{}", $len)));
        }
    };
}

impl std::error::Error for SerdeError {}

impl ser::Error for SerdeError {
    fn custom<T: Display>(msg: T) -> Self {
        SerdeError(msg.to_string())
    }
}

impl de::Error for SerdeError {
    fn custom<T: Display>(msg: T) -> Self {
        SerdeError(msg.to_string())
    }
}

impl Display for SerdeError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

pub struct Serializer {
    bytes: BytesMut,
}

impl<'a> ser::Serializer for &'a mut Serializer {
    // The output type produced by this `Serializer` during successful
    // serialization. Most serializers that produce text or binary output should
    // set `Ok = ()` and serialize into an `io::Write` or buffer contained
    // within the `Serializer` instance, as happens here. Serializers that build
    // in-memory data structures may be simplified by using `Ok` to propagate
    // the data structure around.
    type Ok = ();

    // The error type when some error occurs during serialization.
    type Error = SerdeError;

    // Associated types for keeping track of additional state while serializing
    // compound data structures like sequences and maps. In this case no
    // additional state is required beyond what is already stored in the
    // Serializer struct.
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.serialize_i8(if v { 1 } else { 0 })
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.bytes.put_i8(v);
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.bytes.put_i16_le(v);
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.bytes.put_i32_le(v);
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.bytes.put_i64_le(v);
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.bytes.put_u8(v);
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.bytes.put_u16_le(v);
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.bytes.put_u32_le(v);
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.bytes.put_u64_le(v);
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.bytes.put_f32_le(v);
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.bytes.put_f64_le(v);
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_u32(v as u32)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        let bytes = v.as_bytes();
        self.serialize_u32(bytes.len() as u32)?;
        self.serialize_bytes(bytes)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.bytes.put(v);
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_i32(0)
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        self.serialize_i32(1)?;
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_newtype_variant<T: ?Sized>(self, _: &'static str, _: u32, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        match len {
            None => {}
            Some(v) => {
                self.serialize_u32(v as u32)?;
            }
        }
        Ok(self)
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(None)
    }

    fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        unimplemented!()
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        unimplemented!()
    }

    fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_seq(None)
    }

    fn serialize_struct_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = SerdeError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = SerdeError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = SerdeError;

    fn serialize_field<T: ?Sized>(&mut self, _: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = SerdeError;

    fn serialize_field<T: ?Sized>(&mut self, _: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = SerdeError;

    fn serialize_key<T: ?Sized>(&mut self, _: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_value<T: ?Sized>(&mut self, _: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = SerdeError;

    fn serialize_field<T: ?Sized>(&mut self, _: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = SerdeError;

    fn serialize_field<T: ?Sized>(&mut self, _: &'static str, _: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

pub struct Deserializer<'de> {
    bytes: &'de mut Bytes,
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = SerdeError;

    fn deserialize_any<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        ensure_len!(self, 1);
        visitor.visit_bool(self.bytes.get_i8() > 0)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        ensure_len!(self, 1);
        visitor.visit_i8(self.bytes.get_i8())
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        ensure_len!(self, 2);
        visitor.visit_i16(self.bytes.get_i16_le())
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        ensure_len!(self, 4);
        visitor.visit_i32(self.bytes.get_i32_le())
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        ensure_len!(self, 8);
        visitor.visit_i64(self.bytes.get_i64_le())
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        ensure_len!(self, 1);
        visitor.visit_u8(self.bytes.get_u8())
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        ensure_len!(self, 2);
        visitor.visit_u16(self.bytes.get_u16_le())
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        ensure_len!(self, 4);
        visitor.visit_u32(self.bytes.get_u32_le())
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        ensure_len!(self, 8);
        visitor.visit_u64(self.bytes.get_u64_le())
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        ensure_len!(self, 4);
        visitor.visit_f32(self.bytes.get_f32_le())
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        ensure_len!(self, 8);
        visitor.visit_f64(self.bytes.get_f64_le())
    }

    fn deserialize_char<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        ensure_len!(self, 4);
        let len = self.bytes.get_u32_le() as usize;
        ensure_len!(self, len);
        let bytes = self.bytes.copy_to_bytes(len);
        visitor.visit_string(String::from_utf8(bytes.to_vec()).map_err(|e| SerdeError(e.to_string()))?)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bytes(self.bytes)
    }

    fn deserialize_byte_buf<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        ensure_len!(self, 4);
        match self.bytes.get_i32_le() {
            0 => visitor.visit_none(),
            1 => visitor.visit_some(self),
            _ => Err(SerdeError(String::from("invalid option"))),
        }
    }

    fn deserialize_unit<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(self, _: &'static str, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(self, _: &'static str, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        ensure_len!(self, 4);
        let len = self.bytes.get_u32_le();
        visitor.visit_seq(SeqSeparate { de: &mut self, len: Some(len) })
    }

    fn deserialize_tuple<V>(mut self, _: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(SeqSeparate { de: &mut self, len: None })
    }

    fn deserialize_tuple_struct<V>(self, _: &'static str, _: usize, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_map<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_struct<V>(mut self, _: &'static str, _: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(SeqSeparate { de: &mut self, len: None })
    }

    fn deserialize_enum<V>(self, _: &'static str, _: &'static [&'static str], _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_identifier<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
}

struct SeqSeparate<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    len: Option<u32>,
}

impl<'de, 'a> SeqAccess<'de> for SeqSeparate<'a, 'de> {
    type Error = SerdeError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.len {
            None => {}
            Some(len) => {
                if len == 0 {
                    self.len = None;
                    return Ok(None);
                }
                self.len = Some(len - 1);
            }
        }
        seed.deserialize(&mut *self.de).map(Some)
    }
}

pub fn to_bytes<T>(value: &T) -> Result<Bytes, SerdeError>
where
    T: Serialize,
{
    let mut serializer = Serializer { bytes: BytesMut::new() };
    value.serialize(&mut serializer)?;
    Ok(serializer.bytes.freeze())
}

pub fn from_bytes<'a, T>(bytes: &'a mut Bytes) -> Result<T, SerdeError>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer { bytes };
    let t = T::deserialize(&mut deserializer)?;
    let len = deserializer.bytes.remaining();
    if len == 0 {
        Ok(t)
    } else {
        Err(SerdeError(format!("TrailingCharacters len:{len}")))
    }
}

#[cfg(test)]
mod tests {
    use bytes::{Buf, Bytes};
    use serde::{Deserialize, Serialize};

    use crate::entity::LoginGetSessionReq;

    use super::{from_bytes, to_bytes};

    #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
    struct Test {
        t_i8: i8,
        t_i16: i16,
        t_i32: i32,
        t_i64: i64,
        t_string: String,
        bytes: [u8; 10],
        test2: Test2,
        t_u32: u32,
        vec: Vec<i32>,
    }

    #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
    struct Test2 {
        t_i8: i8,
    }

    #[test]
    fn test() {
        let t = Test {
            t_i8: 1,
            t_i16: 2,
            t_i32: 3,
            t_i64: 4,
            t_string: "5".to_string(),
            bytes: [1, 2, 3, 4, 5, 6, 7, 8, 9, 0],
            test2: Test2 { t_i8: 6 },
            t_u32: 7,
            vec: vec![1, 2, 3, 4, 5],
        };
        println!("t:{:?}", t);
        let mut bytes = to_bytes(&t).unwrap();
        println!("{:?}", bytes);
        assert_eq!(t.t_i8, bytes.get_i8());
        assert_eq!(t.t_i16, bytes.get_i16_le());
        assert_eq!(t.t_i32, bytes.get_i32_le());
        assert_eq!(t.t_i64, bytes.get_i64_le());
        let len = bytes.get_u32_le() as usize;
        assert_eq!(t.t_string.as_bytes().len(), len);
        let s = bytes.copy_to_bytes(len);
        assert_eq!(t.t_string, String::from_utf8(s.to_vec()).unwrap());
        assert_eq!(t.bytes, bytes.copy_to_bytes(t.bytes.len()).chunk());
        assert_eq!(t.test2.t_i8, bytes.get_i8());
        assert_eq!(t.t_u32, bytes.get_u32_le());
        let size = bytes.get_u32_le();
        assert_eq!(t.vec.len(), size as usize);
        for x in &t.vec {
            assert_eq!(*x, bytes.get_i32_le());
        }
        assert_eq!(0, bytes.remaining());

        let mut bytes = to_bytes(&t).unwrap();
        let t2: Test = from_bytes(&mut bytes).unwrap();
        println!("t2:{:?}", t2);
        assert_eq!(t, t2);
    }

    #[test]
    fn test2() {
        let b = b"14e1b600b1fd579f47433b88e8d85291A\0\0\0\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\00\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
        let mut bytes = Bytes::from_static(b);

        let d: LoginGetSessionReq = from_bytes(&mut bytes).unwrap();
        println!("{:?}", d);
    }

    #[test]
    fn test3() {
        let mut bytes = to_bytes(&Some(1)).unwrap();
        println!("{:?}", bytes);
        let x: Option<i32> = from_bytes(&mut bytes).unwrap();
        println!("{:?}", x);

        let mut bytes = to_bytes(&None::<i32>).unwrap();
        println!("{:?}", bytes);
        let x: Option<i32> = from_bytes(&mut bytes).unwrap();
        println!("{:?}", x);
    }
}
