// @generated rust packets from test

#![allow(warnings, missing_docs)]

use bytes::{Buf, BufMut, Bytes, BytesMut};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::sync::Arc;
use thiserror::Error;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Packet parsing failed")]
    InvalidPacketError,
    #[error("{field} was {value:x}, which is not known")]
    ConstraintOutOfBounds { field: String, value: u64 },
    #[error("when parsing {obj} needed length of {wanted} but got {got}")]
    InvalidLengthError { obj: String, wanted: usize, got: usize },
    #[error("Due to size restrictions a struct could not be parsed.")]
    ImpossibleStructError,
    #[error("when parsing field {obj}.{field}, {value} is not a valid {type_} value")]
    InvalidEnumValueError { obj: String, field: String, value: u64, type_: String },
}

#[derive(Debug, Error)]
#[error("{0}")]
pub struct TryFromError(&'static str);

pub trait Packet {
    fn to_bytes(self) -> Bytes;
    fn to_vec(self) -> Vec<u8>;
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct FooData {
    a: u8,
    b: u32,
    c: u8,
}
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FooPacket {
    #[cfg_attr(feature = "serde", serde(flatten))]
    foo: Arc<FooData>,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FooBuilder {
    pub a: u8,
    pub b: u32,
    pub c: u8,
}
impl FooData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 4
    }
    fn parse(mut bytes: &[u8]) -> Result<Self> {
        if bytes.remaining() < 4 {
            return Err(Error::InvalidLengthError {
                obj: "Foo".to_string(),
                wanted: 4,
                got: bytes.remaining(),
            });
        }
        let chunk = bytes.get_u32();
        let a = (chunk & 0x3) as u8;
        let b = ((chunk >> 2) & 0xffffff);
        let c = ((chunk >> 26) & 0x3f) as u8;
        Ok(Self { a, b, c })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        if self.a > 0x3 {
            panic!("Invalid value for {}::{}: {} > {}", "Foo", "a", self.a, 0x3);
        }
        if self.b > 0xffffff {
            panic!("Invalid value for {}::{}: {} > {}", "Foo", "b", self.b, 0xffffff);
        }
        if self.c > 0x3f {
            panic!("Invalid value for {}::{}: {} > {}", "Foo", "c", self.c, 0x3f);
        }
        let value = (self.a as u32) | ((self.b as u32) << 2) | ((self.c as u32) << 26);
        buffer.put_u32(value);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        4
    }
}
impl Packet for FooPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.foo.get_total_size());
        self.foo.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<FooPacket> for Bytes {
    fn from(packet: FooPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<FooPacket> for Vec<u8> {
    fn from(packet: FooPacket) -> Self {
        packet.to_vec()
    }
}
impl FooPacket {
    pub fn parse(mut bytes: &[u8]) -> Result<Self> {
        Ok(Self::new(Arc::new(FooData::parse(bytes)?)).unwrap())
    }
    fn new(root: Arc<FooData>) -> std::result::Result<Self, &'static str> {
        let foo = root;
        Ok(Self { foo })
    }
    pub fn get_a(&self) -> u8 {
        self.foo.as_ref().a
    }
    pub fn get_b(&self) -> u32 {
        self.foo.as_ref().b
    }
    pub fn get_c(&self) -> u8 {
        self.foo.as_ref().c
    }
}
impl FooBuilder {
    pub fn build(self) -> FooPacket {
        let foo = Arc::new(FooData { a: self.a, b: self.b, c: self.c });
        FooPacket::new(foo).unwrap()
    }
}
