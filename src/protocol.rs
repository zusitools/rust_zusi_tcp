use node::{Node, Attribute};
use tcp::{TcpSendable, receive};
use std::io;
use std::io::{ErrorKind, Error};
use std::io::Read;
use std::io::Write;

pub fn send_hello<T: Read + Write>(client_name: &str, client_version: &str, stream: &mut T) -> io::Result<()> {
  try!(Node {
    id: 0x0001, // establishing connection
    attributes: vec![],
    children: vec![
      Node {
        id: 0x0001, // HELLO
        attributes: vec![
          Attribute::from_u16(0x0001, 0x0002), // protocol version
          Attribute::from_u16(0x0002, 0x0002), // client type
          Attribute::from_str(0x0003, client_name), // client identification
          Attribute::from_str(0x0004, client_version), // client version number
        ],
        children: vec![],
      },
    ],
  }.send(stream));
  try!(stream.flush());

  let result = match receive(stream) {
    Ok(node) => node,
    Err(e) => return Err(e),
  };
  if result.id != 0x0001 {
    return Err(Error::new(ErrorKind::InvalidData, "Invalid root node ID, expected 0x0001."));
  }
  if result.children.len() != 1 {
    return Err(Error::new(ErrorKind::InvalidData, "Root node does not have exactly one child."));
  }

  let ack_hello_node = &result.children[0];
  if ack_hello_node.id != 0x0002 {
    return Err(Error::new(ErrorKind::InvalidData, "Invalid command ID, expected ACK_HELLO (0x0002)."));
  }

  for attr in &ack_hello_node.attributes {
    if attr.id == 0x0003 {
      if attr.value[0] == 0 {
        return Ok(());
      } else {
        return Err(Error::new(ErrorKind::Other, "Zusi did not accept the client"));
      }
    }
  }

  Err(Error::new(ErrorKind::InvalidData, "Zusi did not accept the client"))
}

pub fn send_needed_data<T: Read + Write>(cab_display_ids: &[u16], program_data_ids: &[u16], cab_operation: bool, stream: &mut T) -> io::Result<()> {
  let mut needed_data = vec![];
  if cab_display_ids.len() > 0 {
    needed_data.push(Node {
      id: 0x000A, // Cab displays
      attributes: cab_display_ids.iter().map(|id| Attribute::from_u16(0x0001, *id)).collect(),
      children: vec![],
    });
  }

  if cab_operation {
    needed_data.push(Node {
      id: 0x000B, // Cab operation
      attributes: vec![],
      children: vec![],
    });
  }

  if program_data_ids.len() > 0 {
    needed_data.push(Node {
      id: 0x000C, // Program data
      attributes: program_data_ids.iter().map(|id| Attribute::from_u16(0x0001, *id)).collect(),
      children: vec![],
    });
  }

  try!(Node {
    id: 0x0002, // client application 02
    attributes: vec![],
    children: vec![
      Node {
        id: 0x0003, // NEEDED_DATA
        attributes: vec![],
        children: needed_data,
      },
    ],
  }.send(stream));
  try!(stream.flush());

  let result = match receive(stream) {
    Ok(node) => node,
    Err(e) => return Err(e),
  };
  if result.id != 0x0002 {
    return Err(Error::new(ErrorKind::InvalidData, "Invalid root node ID, expected 0x0002."));
  }
  if result.children.len() != 1 {
    return Err(Error::new(ErrorKind::InvalidData, "Root node does not have exactly one child."));
  }

  let ack_data_node = &result.children[0];
  if ack_data_node.id != 0x0004 {
    return Err(Error::new(ErrorKind::InvalidData, "Invalid command ID, expected ACK_NEEDED_DATA (0x0004)."));
  }

  for attr in &ack_data_node.attributes {
    if attr.id == 0x0001 {
      if attr.value[0] == 0 {
        return Ok(());
      } else {
        return Err(Error::new(ErrorKind::Other, "Zusi did not accept the NEEDED_DATA command."));
      }
    }
  }

  Err(Error::new(ErrorKind::InvalidData, "Zusi did not accept the NEEDED_DATA cmmand."))
}
