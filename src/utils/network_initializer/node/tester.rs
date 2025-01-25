use super::Node;
use super::NodeType::Drone;
use wg_2024::controller::DroneEvent;

impl Node {
    /// # Panics
    pub fn assert_expect_drone_event(&self, expected_event: &DroneEvent) {
        let Drone(data) = &self.node_type else {
            panic!("Trying to call on a not drone");
        };
        assert_eq!(Ok(expected_event), data.event_recv.try_recv().as_ref());
    }

    /// # Panics
    pub fn assert_expect_drone_event_fail(&self) {
        let Drone(data) = &self.node_type else {
            panic!("Trying to call on a not drone");
        };
        assert!(data.event_recv.try_recv().is_err());
    }
}
