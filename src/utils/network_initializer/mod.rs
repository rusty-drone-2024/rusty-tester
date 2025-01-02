use crate::utils::DroneOptions;
use crossbeam_channel::{unbounded, Receiver};
use std::thread;
use std::time::Duration;
use wg_2024::controller::{DroneCommand, DroneEvent};
use wg_2024::drone::Drone;
use wg_2024::network::NodeId;
use wg_2024::packet::Packet;

pub struct Network {
    sc_event_rcv: Receiver<DroneEvent>,
    nodes: Vec<NetworkDrone>,
}

pub struct NetworkDrone {
    running: bool,
    options: DroneOptions,
}

impl Drop for NetworkDrone {
    fn drop(&mut self) {
        if !self.running {
            return;
        }

        // Partially fix performance when running multiple integration test
        let _ = self.options.command_send.send(DroneCommand::Crash);

        // Removing connection cause instability and does't increase performance
    }
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
                let is_drone = !client.contains(&(i as NodeId));
                if is_drone {
                    let mut drone: T = options.create_drone(i as NodeId, 0.0);

                    thread::spawn(move || {
                        drone.run();
                    });
                };

                NetworkDrone {
                    running: is_drone,
                    options,
                }
            })
            .collect();

        Self {
            nodes,
            sc_event_rcv,
        }
    }

    #[allow(dead_code)]
    pub fn add_connections(&mut self, start: NodeId, end: NodeId) {
        let options_start = &self.nodes[start as usize].options;
        let options_end = &self.nodes[end as usize].options;

        self.send_as_simulation_controller_to(
            start,
            DroneCommand::AddSender(end, options_end.packet_drone_in.clone()),
        );

        self.send_as_simulation_controller_to(
            end,
            DroneCommand::AddSender(start, options_start.packet_drone_in.clone()),
        );
    }

    #[allow(dead_code)]
    pub fn simulation_controller_event_receiver(&self) -> Receiver<DroneEvent> {
        self.sc_event_rcv.clone()
    }

    pub fn send_as_simulation_controller_to(&self, node_id: NodeId, command: DroneCommand) {
        self.nodes[node_id as usize]
            .options
            .command_send
            .send(command)
            .unwrap()
    }

    pub fn send_as_client(&self, node_id: NodeId, packet: &Packet) -> Option<()> {
        let to = packet.routing_header.current_hop();
        self.send_to_dest_as_client(node_id, to?, packet)
    }

    pub fn send_to_dest_as_client(&self, from: NodeId, to: NodeId, packet: &Packet) -> Option<()> {
        let neighbour = self.nodes[from as usize].options.packet_send.get(&to);
        neighbour?.send(packet.clone()).ok()
    }

    pub fn recv_as_client(&self, node_id: NodeId, timeout: Duration) -> Option<Packet> {
        let receiver = &self.nodes[node_id as usize].options.packet_recv;
        receiver.recv_timeout(timeout).ok()
    }
    /// Start some drone as fake client
    /// They only respond to FloodRequest
    #[allow(dead_code)]
    fn start_fake_clients_async(&mut self, fake_clients: &[NodeId]) {
        todo!("{:?}", fake_clients)
    }
}
