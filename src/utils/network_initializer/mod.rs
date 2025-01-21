mod clean_resources;
mod operations;

use crate::utils::DroneOptions;
use crossbeam_channel::{unbounded, Receiver};
use std::thread;
use std::thread::JoinHandle;
use wg_2024::controller::DroneEvent;
use wg_2024::drone::Drone;
use wg_2024::network::NodeId;

pub struct Network {
    sc_event_rcv: Receiver<DroneEvent>,
    nodes: Vec<NetworkDrone>,
}

pub struct NetworkDrone {
    thread_handle: Option<JoinHandle<()>>,
    options: DroneOptions,
}

impl Network {
    pub fn create_and_run<T: Drone + 'static>(
        amount: usize,
        connections: &[(NodeId, NodeId)],
        client: &[NodeId],
    ) -> Self {
        Network::new::<T>(amount, connections, client)
    }

    /// Create vector of drone with ID from 0 to amount
    /// With the given connections
    /// Duplicated connection are ignored and the graph is not directional
    fn new<T: Drone + 'static>(
        amount: usize,
        connections: &[(NodeId, NodeId)],
        client: &[NodeId],
    ) -> Self {
        let (sc_event_send, sc_event_rcv) = unbounded::<DroneEvent>();
        let mut options = (0..amount)
            .map(|_| DroneOptions::new_with_sc(sc_event_send.clone(), sc_event_rcv.clone()))
            .collect::<Vec<_>>();

        for (start, end) in connections {
            let start_input = options[*start as usize].packet_drone_in.clone();
            let end_input = options[*end as usize].packet_drone_in.clone();

            options[*start as usize].packet_send.insert(*end, end_input);
            options[*end as usize]
                .packet_send
                .insert(*start, start_input);
        }

        let nodes = options
            .into_iter()
            .enumerate()
            .map(|(i, options)| {
                let node_id = NodeId::try_from(i).unwrap();

                let is_drone = !client.contains(&node_id);
                let mut thread_handle = None;

                if is_drone {
                    let mut drone: T = options.create_drone(node_id, 0.0);

                    thread_handle = Some(thread::spawn(move || {
                        drone.run();
                    }));
                };

                NetworkDrone {
                    thread_handle,
                    options,
                }
            })
            .collect();

        Self {
            sc_event_rcv,
            nodes,
        }
    }

    

    //TODO fn start_fake_clients_async
}
