use crossbeam_channel::{unbounded, Receiver, Sender};
use std::collections::HashMap;
use wg_2024::network::NodeId;
use wg_2024::packet::Packet;

pub struct Connection {
    pub sender: Sender<Packet>,
    pub receiver: Receiver<Packet>,
    pub nexts: HashMap<NodeId, Sender<Packet>>,
}

impl Default for Connection {
    fn default() -> Self {
        let (sender, receiver) = unbounded();
        Self {
            sender,
            receiver,
            nexts: HashMap::default(),
        }
    }
}

impl Connection {
    pub fn new(
        sender: Sender<Packet>,
        receiver: Receiver<Packet>,
        nexts: HashMap<NodeId, Sender<Packet>>,
    ) -> Self {
        Self {
            sender,
            receiver,
            nexts,
        }
    }
}

/// # panics
/// If connection are invalid
pub(super) fn convert_connection(
    amount: u8,
    connections: &[(NodeId, NodeId)],
) -> HashMap<NodeId, Connection> {
    let mut res = HashMap::<NodeId, Connection>::default();

    for id in 0..amount {
        let (send, recv) = unbounded();
        let conn = Connection::new(send, recv, HashMap::default());
        res.insert(id, conn);
    }

    for (id_a, id_b) in connections {
        let mut a = res.remove(id_a).unwrap();
        let mut b = res.remove(id_b).unwrap();

        a.nexts.insert(*id_b, b.sender.clone());
        b.nexts.insert(*id_a, a.sender.clone());

        res.insert(*id_a, a);
        res.insert(*id_b, b);
    }

    res
}
