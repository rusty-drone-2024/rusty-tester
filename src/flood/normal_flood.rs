use crate::flood::assert_topology_of_drones;
use crate::utils::data::{new_flood_request, new_flood_request_with_path};
use crate::utils::Network;
use std::time::Duration;
use wg_2024::drone::Drone;
use wg_2024::packet::NodeType;

/// # Panics
pub fn test_easiest_flood<T: Drone + 'static>(timeout: Duration) {
    let net = Network::create_and_run::<T>(4, &[(0, 1), (1, 2), (1, 3)], &[0, 2, 3]);

    let flood = new_flood_request(5, 7, 0, true);
    net.send_to_dest_as_client(0, 1, &flood).unwrap();

    let expected =
        new_flood_request_with_path(5, 7, 0, &[(0, NodeType::Client), (1, NodeType::Drone)]);
    assert_eq!(
        expected.pack_type,
        net.recv_as_client(2, timeout).unwrap().pack_type
    );
    assert_eq!(
        expected.pack_type,
        net.recv_as_client(3, timeout).unwrap().pack_type
    );
}

pub fn test_loop_flood<T: Drone + 'static>(timeout: Duration) {
    assert_topology_of_drones::<T>(4, &[(0, 1), (1, 2), (1, 3), (2, 3)], timeout);
}

pub fn test_hard_loop_flood<T: Drone + 'static>(timeout: Duration) {
    assert_topology_of_drones::<T>(
        6,
        &[
            (0, 1),
            (2, 1),
            (3, 1),
            (3, 2),
            (4, 1),
            (4, 2),
            (4, 3),
            (5, 1),
            (5, 2),
            (5, 3),
            (5, 4),
        ],
        timeout,
    );
}
