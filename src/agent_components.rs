use std::sync::{Arc, Mutex, Weak};
extern crate crossbeam_channel;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use crate::supervisor::{RunMode, DeviceMode};

pub mod {pre_component, post_component}

pub struct ComponentIdle<C: Send> {
    connections: Vec<Weak<Mutex<C>>>
}

impl<C: Send> ComponentIdle<C> {
    fn new() -> ComponentIdle<C> {
        ComponentIdle {
            connections: Vec::new(),
        }
    }

    fn add_connection(&mut self, connection: Weak<Mutex<C>>) {
        self.connections.push(connection);
    }

    fn make_ffw_pre(&self) -> PreComponentFFW<C: Send, S: Send> {
        PreComponentFFW {
            connections: self.connections.iter().map(|conn| OutSetFFW {
                connection: Arc::downgrade(conn.upgrade().expect("no object in Weak<connection>!")),
                channel: match conn.mode() {
                    DeviceMode::Idle -> None,
                    DeviceMode::Feedforward -> {
                        let (tx, rx) = crossbeam_channel::bounded(1);
                        conn.set_pre_channel(Some(rx));
                        Some(tx)
                    },
                }
            }).collect(),
        }
    }

    fn make_ffw_post(&self) -> PostComponentFFW<C: Send, S: Send> {
        PostComponentFFW {
            connections: self.connections.iter().map(|conn| InSetFFW {
                connection: Arc::downgrade(conn.upgrade().expect("no object in Weak<connection>!")),
                channel: match conn.mode() {
                    DeviceMode::Feedforward -> None,
                    DeviceMode::Feedforward -> {
                        let (tx, rx) = crossbeam_channel::bounded(1);
                        conn.set_post_channel(Some(tx));
                        Some(rx)
                    },
                }
            }).collect(),
        }
    }
}

pub struct PreComponentFFW<C: Send, S: Send> {
    connections: Vec<OutSetFFW<C, S>>,
}

impl<C: Send, S: Send> PreComponentFFW<C, S> {
    pub fn make_idle(&self) -> ComponentIdle<C> {
        ComponentIdle {
            connections: self.connections.iter()
                .map(|conn| Arc::downgrade(conn.connection.upgrade().expect("no object in Weak<conection>!")))
                .collect(),
        }
    }

    pub fn feedforward(&self, s: S) {
        for conn in self.connections {
            match &conn.channel {
                None => (),
                Some(tx) => tx.send(s),
            }
        }
    }
}

pub struct PostComponentFFW<C: Send> {
    connections: Vec<InSetFFW<C, R>>,
}

impl<C: Send, R: Send> PostComponentFFW<C, R> {
    fn accepted(&self) -> Vec<R> {
        connections.iter()
            .filter_map(|conn| {
                match conn.channel {
                    None => None,
                    Some => Some(rx.try_iter()),
                }
            }).flatten().collect();
    }

    fn store(&mut self) {
        for conn in connections {
            match &conn.channel {
                None => (),
                Some(rx) => self.buffer.append(rx.try_iter().collect()),
            }
        }
    }
}

struct OutSetFFW<C: Send, S: Send> {
    pub connection: Weak<Mutex<C>>,
    pub channel: Option<CCSender<S>>,
}

struct InSetFFW<C: Send, R: Send> {
    pub connection: Weak<Mutex<C>>,
    pub channel: Option<CCReceiver<R>>,
}
