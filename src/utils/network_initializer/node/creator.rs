use super::Node;
use crate::utils::network_initializer::conn_converter::Connection;
use crate::utils::network_initializer::node::NodeType;
use crossbeam_channel::unbounded;
use std::collections::{HashMap, HashSet};
use std::thread;
use std::thread::JoinHandle;
use wg_2024::controller::{DroneCommand, DroneEvent};
use wg_2024::drone::Drone;
use wg_2024::network::NodeId;
use wg_2024::packet::Packet;

impl Node {
    #[must_use]
    pub fn create_simple_drone<T: Drone + 'static>(id: NodeId, pdr: f32) -> (Self, T) {
        let (controller_send, event_recv) = unbounded::<DroneEvent>();
        let (command_send, controller_recv) = unbounded::<DroneCommand>();
        let (packet_send, packet_rcv) = unbounded::<Packet>();

        let node = Self {
            node_type: NodeType::Drone,
            neighbours: HashSet::default(),
            packet_insert: packet_send,
            event_recv,
            command_send,
        };

        let drone = T::new(
            id,
            controller_send,
            controller_recv,
            packet_rcv,
            HashMap::new(),
            pdr,
        );

        (node, drone)
    }

    #[must_use]
    pub fn create_drone<T: Drone + 'static>(
        id: NodeId,
        conn: Connection,
        pdr: f32,
    ) -> (Self, JoinHandle<()>) {
        let (controller_send, event_recv) = unbounded::<DroneEvent>();
        let (command_send, controller_recv) = unbounded::<DroneCommand>();

        let node = Self {
            node_type: NodeType::Drone,
            neighbours: conn.nexts.keys().copied().collect(),
            packet_insert: conn.sender,
            event_recv,
            command_send,
        };

        let mut drone = T::new(
            id,
            controller_send,
            controller_recv,
            conn.receiver,
            conn.nexts,
            pdr,
        );

        let join = thread::spawn(move || drone.run());

        (node, join)
    }

    #[must_use]
    pub fn create_leaf(conn: Connection) -> Self {
        let (_controller_send, event_recv) = unbounded::<DroneEvent>();
        let (command_send, _controller_recv) = unbounded::<DroneCommand>();

        Self {
            node_type: NodeType::Leaf(conn.receiver),
            neighbours: HashSet::default(),
            packet_insert: conn.sender,
            event_recv,
            command_send,
        }
    }
}
