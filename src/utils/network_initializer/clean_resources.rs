use crate::utils::network_initializer::node::NodeType::Leaf;
use crate::utils::network_initializer::Node;
use crate::utils::Network;
use wg_2024::controller::DroneCommand;

impl Drop for Network {
    fn drop(&mut self) {
        // Clear so that it run the drop of node
        for (_, node) in self.nodes.drain() {
            drop(node);
        }

        for t in self.threads.drain(..) {
            t.join().ok();
        }
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        if let Leaf(_) = self.node_type {
            return;
        }

        for neighbour in &self.neighbours {
            let _ = self
                .command_send
                .send(DroneCommand::RemoveSender(*neighbour));
        }

        let _ = self.command_send.send(DroneCommand::Crash);
    }
}
