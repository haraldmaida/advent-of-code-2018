//! # Day 8: Memory Maneuver
//!
//! ## Part 1
//!
//! The sleigh is much easier to pull than you'd expect for something its
//! weight. Unfortunately, neither you nor the Elves know which way the North
//! Pole is from here.
//!
//! You check your wrist device for anything that might help. It seems to have
//! some kind of navigation system! Activating the navigation system produces
//! more bad news: "Failed to start navigation system. Could not read software
//! license file."
//!
//! The navigation system's license file consists of a list of numbers (your
//! puzzle input). The numbers define a data structure which, when processed,
//! produces some kind of tree that can be used to calculate the license number.
//!
//! The tree is made up of nodes; a single, outermost node forms the tree's
//! root, and it contains all other nodes in the tree (or contains nodes that
//! contain nodes, and so on).
//!
//! Specifically, a node consists of:
//!
//! * A header, which is always exactly two numbers:
//!     - The quantity of child nodes.
//!     - The quantity of metadata entries.
//! * Zero or more child nodes (as specified in the header).
//! * One or more metadata entries (as specified in the header).
//!
//! Each child node is itself a node that has its own header, child nodes, and
//! metadata. For example:
//!
//! ```text
//! 2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2
//! A----------------------------------
//!     B----------- C-----------
//!                      D-----
//! ```
//!
//! In this example, each node of the tree is also marked with an underline
//! starting with a letter for easier identification. In it, there are four
//! nodes:
//!
//! * A, which has 2 child nodes (B, C) and 3 metadata entries (1, 1, 2).
//! * B, which has 0 child nodes and 3 metadata entries (10, 11, 12).
//! * C, which has 1 child node (D) and 1 metadata entry (2).
//! * D, which has 0 child nodes and 1 metadata entry (99).
//!
//! The first check done on the license file is to simply add up all of the
//! metadata entries. In this example, that sum is 1+1+2+10+11+12+2+99=138.
//!
//! What is the sum of all metadata entries?
//!
//! [Advent of Code 2018 - Day 8](https://adventofcode.com/2018/day/8)

use std::{collections::HashMap, iter::FromIterator};

const ROOT_ID: NodeId = NodeId(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Metadata(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(usize);

impl From<usize> for NodeId {
    fn from(value: usize) -> Self {
        NodeId(value)
    }
}

impl NodeId {
    pub fn val(self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Node {
    parent_id: NodeId,
    id: NodeId,
}

impl Node {
    pub fn new(parent_id: impl Into<NodeId>, id: impl Into<NodeId>) -> Self {
        Node {
            parent_id: parent_id.into(),
            id: id.into(),
        }
    }

    pub fn parent_id(self) -> NodeId {
        self.parent_id
    }

    pub fn id(self) -> NodeId {
        self.id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct License {
    root_id: NodeId,
    nodes: HashMap<NodeId, Node>,
    metadata: HashMap<NodeId, Vec<Metadata>>,
}

impl AsRef<License> for License {
    fn as_ref(&self) -> &License {
        self
    }
}

impl License {
    pub fn new(
        nodes: impl IntoIterator<Item = Node>,
        metadata: impl IntoIterator<Item = (NodeId, Vec<Metadata>)>,
    ) -> Self {
        License {
            root_id: ROOT_ID,
            metadata: HashMap::from_iter(metadata.into_iter()),
            nodes: HashMap::from_iter(nodes.into_iter().map(|node| (node.id, node))),
        }
    }

    pub fn metadata_checksum(&self) -> u32 {
        self.metadata
            .iter()
            .flat_map(|(_, mds)| mds)
            .map(|md| md.0)
            .sum()
    }
}

#[aoc_generator(day8)]
pub fn parse(input: &str) -> License {
    let input_data = input
        .trim()
        .lines()
        .next()
        .unwrap_or_else(|| panic!("there is not a single line in input: {:?}", input));

    let mut digits = input_data.split(' ').map(|data| {
        data.parse::<u32>().unwrap_or_else(|_| {
            panic!("there is other data then digits in input: {:?}", input_data)
        })
    });

    let mut next_digits = || {
        digits
            .next()
            .unwrap_or_else(|| panic!("no more input data"))
    };

    let root_node = Node::new(ROOT_ID, 0);

    let mut node_id_seq = root_node.id.0;
    let mut next_node_id = || {
        node_id_seq += 1;
        node_id_seq
    };

    let mut nodes = HashMap::with_capacity(8);
    nodes.insert(root_node.id, root_node);
    let mut metadata = HashMap::with_capacity(8);
    let mut parent_header = Vec::with_capacity(8);

    let mut current_node = root_node;
    let mut num_children = next_digits();
    let mut num_metadata_entries = next_digits();

    loop {
        if num_children == 0 {
            let current_metadata = metadata
                .entry(current_node.id)
                .or_insert_with(|| Vec::with_capacity(num_metadata_entries as usize));
            for _ in 0..num_metadata_entries {
                let metadata_entry = next_digits();
                current_metadata.push(Metadata(metadata_entry));
            }
            if let Some((parent_node, parent_children, parent_metadata)) = parent_header.pop() {
                current_node = parent_node;
                num_children = parent_children - 1;
                num_metadata_entries = parent_metadata;
            } else {
                break;
            }
        } else {
            parent_header.push((current_node, num_children, num_metadata_entries));
            num_children = next_digits();
            num_metadata_entries = next_digits();
            current_node = Node::new(current_node.id, next_node_id());
            nodes.insert(current_node.id, current_node);
        }
    }

    License {
        root_id: root_node.id,
        nodes,
        metadata,
    }
}

#[aoc(day8, part1)]
pub fn metadata_checksum(license: &License) -> u32 {
    license.metadata_checksum()
}

#[cfg(test)]
mod tests;
