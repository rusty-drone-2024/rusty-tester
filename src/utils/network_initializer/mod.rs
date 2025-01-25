mod clean_resources;
mod conn_converter;
mod leaf;
pub mod node;
mod operations;

use crate::utils::network_initializer::conn_converter::convert_connection;
use crate::utils::Node;
use std::collections::{HashMap, HashSet};
use std::thread::JoinHandle;
use wg_2024::drone::Drone;
use wg_2024::network::NodeId;

pub struct Network {
    threads: Vec<JoinHandle<()>>,
    nodes: HashMap<NodeId, Node>,
}

impl Network {
    /// Create vector of drone with ID from 0 to amount
    /// With the given connections
    /// Duplicated connection are ignored and the graph is not directed
    ///
    /// It also run the given network
    /// # panic
    /// If the given network is invalid in some way.
    pub fn create_and_run<T: Drone + 'static>(
        amount: u8,
        connections: &[(NodeId, NodeId)],
        clients: &[NodeId],
    ) -> Self {
        let clients = clients.iter().copied().collect::<HashSet<_>>();
        let connections = convert_connection(amount, connections);

        let mut nodes = HashMap::default();
        let mut threads = vec![];

        for (id, conn) in connections {
            if clients.contains(&id) {
                nodes.insert(id, Node::create_leaf(conn));
            } else {
                let (node, join) = Node::create_drone::<T>(id, conn, 0.0);
                nodes.insert(id, node);
                threads.push(join);
            }
        }

        Self { threads, nodes }
    }
}
