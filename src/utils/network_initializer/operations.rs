use crate::utils::network_initializer::node::NodeType::LeafEdge;
use crate::utils::network_initializer::node::{DroneData, NodeType};
use crate::utils::network_initializer::Node;
use crate::utils::Network;
use crossbeam_channel::Receiver;
use std::time::Duration;
use wg_2024::controller::{DroneCommand, DroneEvent};
use wg_2024::network::NodeId;
use wg_2024::packet::Packet;

impl Network {
    fn get(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(&id)
    }

    fn get_drone(&self, id: NodeId) -> Option<&DroneData> {
        let NodeType::Drone(drone) = &self.get(id)?.node_type else {
            return None;
        };
        Some(drone)
    }

    pub fn crash_node(&mut self, id: NodeId) -> Option<()> {
        let node_drone = self.get(id)?;
        let drone = self.get_drone(id)?;

        for neighbour in &node_drone.neighbours {
            self.get_drone(*neighbour)?
                .command_send
                .send(DroneCommand::RemoveSender(id))
                .ok()?;
        }
        drone.command_send.send(DroneCommand::Crash).ok()
    }

    pub fn add_connections(&mut self, start: NodeId, end: NodeId) -> Option<()> {
        self.send_as_simulation_controller_to(
            start,
            DroneCommand::AddSender(end, self.get(start)?.packet_insert.clone()),
        )?;
        self.send_as_simulation_controller_to(
            end,
            DroneCommand::AddSender(start, self.get(end)?.packet_insert.clone()),
        )
    }

    pub fn simulation_controller_event_receiver(
        &self,
        node_id: NodeId,
    ) -> Option<Receiver<DroneEvent>> {
        Some(self.get_drone(node_id)?.event_recv.clone())
    }

    pub fn send_as_simulation_controller_to(
        &self,
        node_id: NodeId,
        command: DroneCommand,
    ) -> Option<()> {
        self.get_drone(node_id)?.command_send.send(command).ok()
    }

    pub fn send_as_client(&self, node_id: NodeId, packet: &Packet) -> Option<()> {
        let to = packet.routing_header.current_hop();
        self.send_to_dest_as_client(node_id, to?, packet)
    }

    pub fn send_to_dest_as_client(&self, from: NodeId, to: NodeId, packet: &Packet) -> Option<()> {
        if !self.get(from)?.neighbours.contains(&to) {
            return None; // Not connected
        }

        let next_hop = self.get(to)?;
        next_hop.packet_insert.send(packet.clone()).ok()
    }

    #[cfg(not(feature = "leaf"))]
    pub fn recv_as_client(&self, node_id: NodeId, timeout: Duration) -> Option<Packet> {
        let LeafEdge(packet_remove) = &self.get(node_id)?.node_type else {
            return None;
        };
        packet_remove.recv_timeout(timeout).ok()
    }

    #[cfg(feature = "leaf")]
    pub fn recv_as_client(&self, node_id: NodeId, timeout: Duration) -> Option<Packet> {
        let LeafEdge(data) = &self.get(node_id)?.node_type else {
            return None;
        };
        data.data_to_run
            .as_ref()?
            .packet_remove
            .recv_timeout(timeout)
            .ok()
    }
}
