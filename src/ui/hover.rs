use crate::{Node, NodeId};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Hover {
    None,
    Node(NodeId),
    NodeSocket {
        node: NodeId,
        socket: usize,
        input: bool,
    },
}

impl Hover {
    pub fn should_highlight_node(&self, node: &Node) -> bool {
        match self {
            Hover::Node(id) => id == &node.id,
            _ => false,
        }
    }

    pub fn get_highlight_socket(&self, node: &Node) -> Option<(usize, bool)> {
        match self {
            Hover::NodeSocket {
                node: id,
                socket,
                input,
            } if id == &node.id => Some((*socket, *input)),
            _ => None,
        }
    }
}
