pub mod debug;
pub mod node;
pub mod protocol;
pub mod tcp;

#[cfg(test)]
mod tests {
  use super::node;

  #[test]
  fn test_find_node() {
    let a1 = node::Attribute::from_u16(0x0042, 0x0001);
    let a2 = node::Attribute::from_u16(0x0042, 0x0002);

    let n = node::Node {
      id: 0x0001,
      attributes: vec![a1],
      children: vec![
        node::Node {
          id: 0x0002,
          attributes: vec![a2],
          children: vec![],
        }
      ],
    };

    assert_eq!(n.find_node(&[0x0001, 0x0002]).unwrap().id, 0x0002);
    assert_eq!(n.find_node_excl(&[0x0002]).unwrap().id, 0x0002);

    assert!(n.find_node_excl(&[0x0001, 0x0002]).is_none());
    assert!(n.find_node(&[0x0002]).is_none());

    assert_eq!(n.find_node(&[0x0001]).unwrap().id, 0x0001);
    assert_eq!(n.find_node_excl(&[]).unwrap().id, 0x0001);

    assert!(n.find_node_excl(&[0x0001]).is_none());

    assert!(n.find_node(&[0x0001, 0x0002, 0x0042]).is_none());
    assert!(n.find_node_excl(&[0x0002, 0x0042]).is_none());
    assert!(n.find_node(&[0x0002, 0x0042]).is_none());
    assert!(n.find_node_excl(&[0x0001, 0x0002, 0x0042]).is_none());
  }

  #[test]
  fn test_find_node_cond() {
    let a1 = node::Attribute::from_u16(0x0042, 0x0001);
    let a2 = node::Attribute::from_u16(0x0042, 0x0002);

    let n = node::Node {
      id: 0x0001,
      attributes: vec![],
      children: vec![
        node::Node {
          id: 0x0002,
          attributes: vec![],
          children: vec![],
        },
        node::Node {
          id: 0x0002,
          attributes: vec![a1, a2],
          children: vec![],
        }
      ],
    };

    assert!(n.find_node_cond(&[0x0001, 0x0002], |n: &node::Node| n.attributes.len() > 0).unwrap().attributes.len() > 0);
    assert!(n.find_node_excl_cond(&[0x0002], |n: &node::Node| n.attributes.len() > 0).unwrap().attributes.len() > 0);
  }

  #[test]
  fn test_find_attribute() {
    let a1 = node::Attribute::from_u16(0x0042, 0x0001);
    let a2 = node::Attribute::from_u16(0x0042, 0x0002);
    let a3 = node::Attribute::from_u16(0x0042, 0x0003);
    let a4 = node::Attribute::from_u16(0x0042, 0x0004);

    let n = node::Node {
      id: 0x0001,
      attributes: vec![a1, a2],
      children: vec![
        node::Node {
          id: 0x0001,
          attributes: vec![a3, a4],
          children: vec![],
        }
      ],
    };

    assert!(n.find_attribute(&[0x0001, 0x0042]).unwrap().as_u16().unwrap() == 0x0001);
    assert!(n.find_attribute_excl(&[0x0042]).unwrap().as_u16().unwrap() == 0x0001);

    assert!(n.find_attribute(&[0x0001, 0x0001, 0x0042]).unwrap().as_u16().unwrap() == 0x0003);
    assert!(n.find_attribute_excl(&[0x0001, 0x0042]).unwrap().as_u16().unwrap() == 0x0003);

    assert!(n.find_attribute(&[0x0001, 0x0001]).is_none());
    assert!(n.find_attribute_excl(&[0x0001]).is_none());
    assert!(n.find_attribute(&[0x0001, 0x0024]).is_none());
    assert!(n.find_attribute_excl(&[0x0024]).is_none());
  }
}
