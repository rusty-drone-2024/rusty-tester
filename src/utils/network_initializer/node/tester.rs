use super::Node;
use wg_2024::controller::DroneEvent;

impl Node {
    /// # Panics
    pub fn assert_expect_drone_event(&self, expected_event: &DroneEvent) {
        assert_eq!(Ok(expected_event), self.event_recv.try_recv().as_ref());
    }

    /// # Panics
    pub fn assert_expect_drone_event_fail(&self) {
        assert!(self.event_recv.try_recv().is_err());
    }
}
