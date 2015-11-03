extern crate byteorder;

use std::io::Cursor;
use std::io::Read;
use std::string::FromUtf8Error;
use self::byteorder::{WriteBytesExt, ReadBytesExt};

type LE = self::byteorder::LittleEndian;

pub struct Node {
  pub id: u16,
  pub attributes: Vec<Attribute>,
  pub children: Vec<Node>,
}

pub struct Attribute {
  pub id: u16,
  pub value: Vec<u8>,
}

impl Attribute {
  pub fn from_bytes(id: u16, val: &[u8]) -> Attribute {
    Attribute {
      id: id,
      value: val.to_vec(),
    }
  }

  pub fn from_u8(id: u16, val: u8) -> Attribute {
    Attribute {
      id: id,
      value: vec![val],
    }
  }

  pub fn from_u16(id: u16, val: u16) -> Attribute {
    let mut result = Attribute {
      id: id,
      value: vec!(),
    };
    result.value.write_u16::<LE>(val).ok().expect("Error writing u16 value to attribute.");
    result
  }

  pub fn from_str(id: u16, val: &str) -> Attribute {
    Attribute {
      id: id,
      value: val.bytes().collect(),
    }
  }

  pub fn as_u16(&self) -> byteorder::Result<u16> {
    Cursor::new(&self.value[..]).read_u16::<LE>()
  }

  pub fn as_f32(&self) -> byteorder::Result<f32> {
    Cursor::new(&self.value[..]).read_f32::<LE>()
  }

  pub fn to_str(&self) -> Result<String, FromUtf8Error> {
    String::from_utf8(self.value.clone())
  }
}
