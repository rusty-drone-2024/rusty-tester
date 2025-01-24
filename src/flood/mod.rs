pub mod extra_flood;
pub mod normal_flood;

use crate::utils::data::new_flood_request;
use crate::utils::Network;
use std::collections::HashSet;
use std::time::Duration;
use wg_2024::drone::Drone;
use wg_2024::network::NodeId;
use wg_2024::packet::PacketType;

/// assuming the topology as a client at 0
/// Connected with a drone 1
pub fn assert_topology_of_drones<T: Drone + 'static>(
    amount: u8,
    topology: &[(NodeId, NodeId)],
    timeout: Duration,
) {
    let net = Network::create_and_run::<T>(amount, topology, &[0]);

    let flood = new_flood_request(5, 7, 0, true);
    net.send_to_dest_as_client(0, 1, &flood).unwrap();

    let result = normalize_vec(listen_response_nodes(&net, timeout));
    let expected = normalize_vec(topology.to_vec());
    assert_eq!(expected, result);
}

pub fn listen_response_nodes(network: &Network, timeout: Duration) -> Vec<(NodeId, NodeId)> {
    let mut hash_set = HashSet::new();

    while let Some(packet) = network.recv_as_client(0, timeout) {
        if let PacketType::FloodResponse(ref flood_res) = packet.pack_type {
            let path = flood_res.path_trace.iter().copied().map(|x| x.0);
            let connection = path.clone().skip(1).zip(path);

            connection.for_each(|(a, b)| {
                hash_set.insert((a, b).min((b, a)));
            });
        }
    }

    hash_set.into_iter().collect()
}

pub fn normalize_vec(vec: Vec<(NodeId, NodeId)>) -> Vec<(NodeId, NodeId)> {
    let mut vec = vec
        .into_iter()
        .map(|(a, b)| if a < b { (a, b) } else { (b, a) })
        .collect::<Vec<_>>();

    vec.sort_by(|(a1, b1), (a2, b2)| a1.cmp(a2).then(b1.cmp(b2)));
    vec
}
