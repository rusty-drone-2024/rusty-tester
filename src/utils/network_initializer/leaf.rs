#![cfg(feature = "leaf")]

use crate::utils::network_initializer::node::LeafToRunData;
use crate::utils::network_initializer::node::NodeType::LeafEdge;
use crate::utils::Network;
use common_structs::leaf::Leaf;
use std::thread;
use wg_2024::network::NodeId;

impl Network {
    pub fn create_and_run_leaf<T: Leaf + 'static>(&mut self, node_id: NodeId) -> Option<()> {
        let node = self.nodes.get_mut(&node_id)?;
        let LeafEdge(data) = &mut node.node_type else {
            return None;
        };

        let LeafToRunData {
            controller_send,
            controller_recv,
            packet_remove,
            packet_send,
        } = data.data_to_run.take()?;

        let mut leaf = T::new(
            node_id,
            controller_send,
            controller_recv,
            packet_remove,
            packet_send,
        );

        let handle = thread::spawn(move || leaf.run());
        self.threads.push(handle);
        Some(())
    }
}
