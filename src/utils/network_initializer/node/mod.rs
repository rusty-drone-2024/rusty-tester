mod creator;
mod tester;

use crate::utils::network_initializer::NodeId;

#[cfg(feature = "leaf")]
use common_structs::leaf::{LeafCommand, LeafEvent};
use crossbeam_channel::{Receiver, Sender};
#[cfg(feature = "leaf")]
use std::collections::HashMap;
use std::collections::HashSet;
use wg_2024::controller::{DroneCommand, DroneEvent};
use wg_2024::packet::Packet;

pub struct Node {
    pub node_type: NodeType,
    pub neighbours: HashSet<NodeId>,
    pub packet_insert: Sender<Packet>,
}

pub enum NodeType {
    Drone(DroneData),
    #[cfg(feature = "leaf")]
    LeafEdge(LeafData),
    #[cfg(not(feature = "leaf"))]
    LeafEdge(Receiver<Packet>),
}

pub struct DroneData {
    pub event_recv: Receiver<DroneEvent>,
    pub command_send: Sender<DroneCommand>,
}

#[cfg(feature = "leaf")]
pub struct LeafData {
    pub event_recv: Receiver<LeafEvent>,
    pub command_send: Sender<LeafCommand>,
    pub data_to_run: Option<LeafToRunData>,
}

#[cfg(feature = "leaf")]
pub struct LeafToRunData {
    pub controller_send: Sender<LeafEvent>,
    pub controller_recv: Receiver<LeafCommand>,
    pub packet_remove: Receiver<Packet>,
    pub packet_send: HashMap<NodeId, Sender<Packet>>,
}
