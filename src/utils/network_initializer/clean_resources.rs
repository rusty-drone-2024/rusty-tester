use std::collections::HashSet;
use crate::utils::network_initializer::node::{DroneData, NodeType};
use crate::utils::network_initializer::Node;
use crate::utils::Network;
use wg_2024::controller::DroneCommand;
use wg_2024::network::NodeId;

impl Drop for Network {
    fn drop(&mut self) {
        // Keep it around so that drone don't misbehaves
        let mut event_receivers = vec!();
        
        // Clear so that it run the drop of node
        for (_, node) in self.nodes.drain() {
            if let Node{ node_type: NodeType::Drone(drone), neighbours, .. } = node {
                kill_drone(&drone, &neighbours);
                event_receivers.push(drone);
            }
            // Here node is dropped
        }

        for t in self.threads.drain(..) {
            t.join().ok();
        }
        
        // to make it live long enough
        drop(event_receivers);
    }
}

fn kill_drone(drone: &DroneData, neighbours: &HashSet<NodeId>) {
    for neighbour in neighbours {
        let _ = drone
            .command_send
            .send(DroneCommand::RemoveSender(*neighbour));
    }

    let _ = drone.command_send.send(DroneCommand::Crash);
}
