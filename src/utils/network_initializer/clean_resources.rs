use crate::utils::network_initializer::NetworkDrone;
use crate::utils::Network;
use wg_2024::controller::DroneCommand;

impl Drop for Network {
    fn drop(&mut self) {
        let mut threads = vec![];

        for node in &mut self.nodes {
            if let Some(handle) = node.thread_handle.take() {
                threads.push(handle);
            }
        }

        self.nodes.clear();

        for t in threads {
            let _ = t.join();
        }
    }
}

impl Drop for NetworkDrone {
    fn drop(&mut self) {
        for neighbour in self.options.packet_send.keys() {
            let res = self
                .options
                .command_send
                .send(DroneCommand::RemoveSender(*neighbour));
            if res.is_err() {
                return;
            }
        }

        let _ = self.options.command_send.send(DroneCommand::Crash);
    }
}