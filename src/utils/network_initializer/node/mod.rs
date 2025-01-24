mod creator;
mod tester;

use crate::utils::network_initializer::NodeId;
use crossbeam_channel::{Receiver, Sender};
use std::collections::HashSet;
use wg_2024::controller::{DroneCommand, DroneEvent};
use wg_2024::packet::Packet;

pub enum NodeType {
    Drone,
    //TODO actual leaf
    Leaf(Receiver<Packet>),
}

pub struct Node {
    pub node_type: NodeType,
    pub neighbours: HashSet<NodeId>,
    pub packet_insert: Sender<Packet>,
    pub event_recv: Receiver<DroneEvent>,
    pub command_send: Sender<DroneCommand>,
}
