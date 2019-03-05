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

impl Node {
  /// Returns a node with the specified ID path, if present.
  ///
  /// Returns the first (with respect to a depth-first search) node
  /// for which the function `cond` returns `true` and
  /// that satisfies the following condition: Its ID is equal to the last
  /// value of `ids`, its parent node's id is equal to the second-to-last
  /// value of `ids` etc., and its `n`th parent node is a child of `self`,
  /// where `n` equals `ids.len() - 1`.
  pub fn find_node_excl_cond<F>(&self, ids: &[u16], cond: F) -> Option<&Node>
  where F: Fn(&Node) -> bool {
    if ids.len() == 0 { if cond(&self) { Some(&self) } else { None } }
    else {
      for c in self.children.iter().filter(|c| c.id == ids[0]) {
        let res: Option<&Node> =
          if ids.len() == 1 { if cond(&c) { Some(c) } else { None } }
          else { c.find_node_excl(&ids[1..]) };
        if res.is_some() { return res; }
      }
      None
    }
  }

  /// Returns a node with the specified ID path, if present.
  ///
  /// Returns the first (with respect to a depth-first search) node
  /// that satisfies the following condition: Its ID is equal to the last
  /// value of `ids`, its parent node's id is equal to the second-to-last
  /// value of `ids` etc., and its `n`th parent node is a child of `self`,
  /// where `n` equals `ids.len() - 1`.
  pub fn find_node_excl(&self, ids: &[u16]) -> Option<&Node> {
    self.find_node_excl_cond(ids, |_: &Node| true)
  }

  /// Returns a node with ID path `ids[1..]` if it exists and
  /// the first element of the ID path matches this node's ID.
  /// Additionally, the function `cond` applied to that node must
  /// return `true`.
  ///
  /// The ID path must contain at least one element.
  pub fn find_node_cond<F>(&self, ids: &[u16], cond: F) -> Option<&Node>
  where F: Fn(&Node) -> bool {
    assert!(ids.len() >= 1);
    if self.id != ids[0] { None }
    else if ids.len() == 1 { if cond(&self) { Some(&self) } else { None } }
    else { self.find_node_excl_cond(&ids[1..], cond) }
  }

  /// Returns a node with ID path `ids[1..]` if it exists and
  /// the first element of the ID path matches this node's ID.
  ///
  /// The ID path must contain at least one element.
  pub fn find_node(&self, ids: &[u16]) -> Option<&Node> {
    self.find_node_cond(ids, |_: &Node| true)
  }

  /// Returns an attribute with the specified ID path, if present.
  ///
  /// Returns the first (with respect to a depth-first search) attribute
  /// that satisfies the following condition: Its ID is equal to the last
  /// value of `ids`, its parent node's id is equal to the second-to-last
  /// value of `ids` etc., and its `n`th parent node is a child of `self`,
  /// where `n` equals `ids.len() - 1`.
  ///
  /// The ID path must contain at least one element.
  pub fn find_attribute_excl(&self, ids: &[u16]) -> Option<&Attribute> {
    assert!(ids.len() >= 1);
    if ids.len() > 1 {
      for c in self.children.iter().filter(|c| c.id == ids[0]) {
        let res: Option<&Attribute> = c.find_attribute_excl(&ids[1..]);
        if res.is_some() { return res; }
      }
      None
    } else {
      self.attributes.iter().filter(|a| a.id == ids[0]).next()
    }
  }

  /// Returns an attribute with ID path `ids[1..]` if it exists and
  /// the first element of the ID path matches this node's ID.
  ///
  /// The ID path must contain at least two elements.
  pub fn find_attribute(&self, ids: &[u16]) -> Option<&Attribute> {
    assert!(ids.len() >= 2);
    if self.id != ids[0] {
      None
    } else {
      self.find_attribute_excl(&ids[1..])
    }
  }
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

  pub fn from_u32(id: u16, val: u32) -> Attribute {
    let mut result = Attribute {
      id: id,
      value: vec!(),
    };
    result.value.write_u32::<LE>(val).ok().expect("Error writing u32 value to attribute.");
    result
  }

  pub fn from_i16(id: u16, val: i16) -> Attribute {
    let mut result = Attribute {
      id: id,
      value: vec!(),
    };
    result.value.write_i16::<LE>(val).ok().expect("Error writing i16 value to attribute.");
    result
  }

  pub fn from_f32(id: u16, val: f32) -> Attribute {
    let mut result = Attribute {
      id: id,
      value: vec!(),
    };
    result.value.write_f32::<LE>(val).ok().expect("Error writing f32 value to attribute.");
    result
  }

  pub fn from_str(id: u16, val: &str) -> Attribute {
    Attribute {
      id: id,
      value: val.bytes().collect(),
    }
  }

  pub fn as_u8(&self) -> byteorder::Result<u8> {
    Cursor::new(&self.value[..]).read_u8()
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
