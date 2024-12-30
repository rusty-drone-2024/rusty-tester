# Rusty Tester
## Importing it
Add to your cargo file in the dev dependencies this:
```toml
[dev-dependencies]
rusty_tester = { git = "https://github.com/rusty-drone-2024/rusty-tester" }
```

## Using it
Create a file with the following content (only change the Drone to test):
```rust
#[cfg(test)]
use crate::drone::RustyDrone;
use rusty_tester::*;
use std::time::Duration;

type Tested = /* TODO insert drone type to be tested (eg. RustyDrone) */;
const TIMEOUT: Duration = Duration::from_millis(20);
const FLOOD_TIMEOUT: Duration = Duration::from_millis(50);
const HARD_TEST_TIMEOUT: Duration = Duration::from_millis(500);

#[test]
fn drone_destination_is_drone() {
    test_drone_destination_is_drone::<Tested>(TIMEOUT);
}

#[test]
fn drone_error_in_routing() {
    test_drone_error_in_routing::<Tested>(TIMEOUT);
}

#[test]
fn drone_packet_1_hop() {
    test_drone_packet_1_hop::<Tested>(TIMEOUT);
}

#[test]
fn drone_packet_3_hop() {
    test_drone_packet_3_hop::<Tested>(TIMEOUT);
}

#[test]
fn drone_packet_3_hop_crash() {
    test_drone_packet_3_hop_crash::<Tested>(TIMEOUT);
}

#[test]
fn drone_packet_255_hop() {
    test_drone_packet_255_hop::<Tested>(HARD_TEST_TIMEOUT);
}

#[test]
fn easiest_flood() {
    test_easiest_flood::<Tested>(FLOOD_TIMEOUT);
}

#[test]
fn loop_flood() {
    test_loop_flood::<Tested>(FLOOD_TIMEOUT);
}

#[test]
fn hard_loop_flood() {
    test_hard_loop_flood::<Tested>(FLOOD_TIMEOUT);
}

#[test]
fn matrix_loop_flood() {
    test_matrix_loop_flood::<Tested>(FLOOD_TIMEOUT);
}

#[test]
fn star_loop_flood() {
    test_star_loop_flood::<Tested>(FLOOD_TIMEOUT);
}

#[test]
fn butterfly_loop_flood() {
    test_butterfly_loop_flood::<Tested>(FLOOD_TIMEOUT);
}

#[test]
fn tree_loop_flood() {
    test_tree_loop_flood::<Tested>(FLOOD_TIMEOUT);
}

```
