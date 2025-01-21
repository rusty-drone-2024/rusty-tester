use crate::utils::{DroneOptions, Network};
use crossbeam_channel::Receiver;
use std::time::Duration;
use wg_2024::controller::{DroneCommand, DroneEvent};
use wg_2024::network::NodeId;
use wg_2024::packet::Packet;

impl Network{
    fn get(&self, id: NodeId) -> Option<&DroneOptions>{
        Some(&self.nodes.get(usize::from(id))?.options)
    }
    
    pub fn crash_node(&mut self, id: NodeId) -> Option<()>{
        let node_opt = self.get(id)?;
        
        for next in node_opt.packet_send.keys() {
            self.get(*next)?
                .command_send
                .send(DroneCommand::RemoveSender(id)).ok()?;
        }
        node_opt.command_send.send(DroneCommand::Crash).ok()
    }
    
    pub fn add_connections(&mut self, start: NodeId, end: NodeId) -> Option<()>{
        self.send_as_simulation_controller_to(
            start,
            DroneCommand::AddSender(end, self.get(start)?.packet_drone_in.clone()),
        )?;
        self.send_as_simulation_controller_to(
            end,
            DroneCommand::AddSender(start, self.get(end)?.packet_drone_in.clone()),
        )
    }

    pub fn simulation_controller_event_receiver(&self) -> Receiver<DroneEvent> {
        self.sc_event_rcv.clone()
    }
    
    pub fn send_as_simulation_controller_to(&self, node_id: NodeId, command: DroneCommand) -> Option<()>{
        self.get(node_id)?
            .command_send
            .send(command).ok()
    }

    pub fn send_as_client(&self, node_id: NodeId, packet: &Packet) -> Option<()> {
        let to = packet.routing_header.current_hop();
        self.send_to_dest_as_client(node_id, to?, packet)
    }

    pub fn send_to_dest_as_client(&self, from: NodeId, to: NodeId, packet: &Packet) -> Option<()> {
        let neighbour = self.get(from)?.packet_send.get(&to);
        neighbour?.send(packet.clone()).ok()
    }

    pub fn recv_as_client(&self, node_id: NodeId, timeout: Duration) -> Option<Packet> {
        self.get(node_id)?.packet_recv.recv_timeout(timeout).ok()
    }
}