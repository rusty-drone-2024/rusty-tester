use super::{DroneData, Node};
use crate::utils::network_initializer::conn_converter::Connection;
use crate::utils::network_initializer::node::NodeType;
use crossbeam_channel::unbounded;
use std::collections::{HashMap, HashSet};
use std::thread;
use std::thread::JoinHandle;

#[cfg(feature = "leaf")]
use super::{LeafData, LeafToRunData};
#[cfg(feature = "leaf")]
use common_structs::leaf::{LeafCommand, LeafEvent};
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

        let drone_data = DroneData {
            event_recv,
            command_send,
        };

        let node = Self {
            node_type: NodeType::Drone(drone_data),
            neighbours: HashSet::default(),
            packet_insert: packet_send,
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

        let drone_data = DroneData {
            event_recv,
            command_send,
        };

        let node = Self {
            node_type: NodeType::Drone(drone_data),
            neighbours: conn.nexts.keys().copied().collect(),
            packet_insert: conn.sender,
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
    #[cfg(feature = "leaf")]
    pub fn create_leaf(conn: Connection) -> Self {
        let (controller_send, event_recv) = unbounded::<LeafEvent>();
        let (command_send, controller_recv) = unbounded::<LeafCommand>();

        let data_to_run = LeafToRunData {
            controller_send,
            controller_recv,
            packet_remove: conn.receiver,
            packet_send: conn.nexts.clone(),
        };

        let node_type = NodeType::LeafEdge(LeafData {
            event_recv,
            command_send,
            data_to_run: Some(data_to_run),
        });

        Self {
            node_type,
            neighbours: conn.nexts.into_keys().collect(),
            packet_insert: conn.sender,
        }
    }

    #[must_use]
    #[cfg(not(feature = "leaf"))]
    pub fn create_leaf(conn: Connection) -> Self {
        let node_type = NodeType::LeafEdge(conn.receiver);

        Self {
            node_type,
            neighbours: conn.nexts.into_keys().collect(),
            packet_insert: conn.sender,
        }
    }
}
