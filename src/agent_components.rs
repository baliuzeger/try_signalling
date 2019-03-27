use std::sync::{Arc, Mutex, Weak};
use std::marker;
use std::marker::PhantomData;
extern crate crossbeam_channel;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use crate::supervisor::{RunMode};
use crate::connections::{PassiveConnection, RunningPassiveConnection};

pub mod pre_component;
pub mod post_component;

pub struct ComponentIdle<C, S0, S1>
where C: PassiveConnection<S0, S1> + Send + ?Sized,
      S0: Send,
      S1: Send,
{
    connections: Vec<Weak<Mutex<C>>>,
    phantom: PhantomData<(S0, S1)>,
}

impl<C, S0, S1> ComponentIdle<C, S0, S1>
where C: PassiveConnection<S0, S1> + Send + ?Sized,
      S0: Send,
      S1: Send,
{
    fn new() -> ComponentIdle<C, S0, S1> {
        ComponentIdle {
            connections: Vec::new(),
            phantom: PhantomData {},
        }
    }

    fn add_connection(&mut self, connection: Weak<Mutex<C>>) {
        self.connections.push(connection);
    }

    fn make_ffw_pre<S: Send>(&self) -> PreComponentFFW<C, S0, S1> {
        PreComponentFFW {
            connections: self.connections.iter().map(|conn| {
                let unlocked_conn = conn.upgrade().unwrap().lock().unwrap();
                OutSetFFW {
                    connection: conn.clone(),
                    channel: match unlocked_conn.mode() {
                        RunMode::Idle => None,
                        RunMode::Feedforward => {
                            let (tx, rx) = crossbeam_channel::bounded(1);
                            unlocked_conn.set_pre_channel_ffw(Some(rx));
                            Some(tx)
                        },
                    }
                }
            }).collect(),
            phantom: PhantomData {},       
        }
    }

    fn make_ffw_post<S: Send>(&self) -> PostComponentFFW<C, S> {
        PostComponentFFW {
            connections: self.connections.iter().map(|conn| InSetFFW {
                connection: conn.clone(),
                channel: match conn.upgrade().lock().unwrap().mode() {
                    RunMode::Feedforward => None,
                    RunMode::Feedforward => {
                        let (tx, rx) = crossbeam_channel::bounded(1);
                        conn.set_post_channel(Some(tx));
                        Some(rx)
                    },
                }
            }).collect(),
            phantom: PhantomData {},
        }
    }
}

pub struct PreComponentFFW<C, S0, S1>
where C: PassiveConnection<S0, S1> + Send + ?Sized,
      S0: Send,
      S1: Send,
{
    connections: Vec<OutSetFFW<C, S0>>,
    phantom: PhantomData<S1>,
}

impl<C, S0, S1> PreComponentFFW<C, S0, S1>
where C: PassiveConnection<S0, S1> + Send + ?Sized,
      S0: Send,
      S1: Send,
{
    pub fn make_idle(&self) -> ComponentIdle<C, S0, S1> {
        ComponentIdle {
            connections: self.connections.iter()
                .map(|set| Arc::downgrade(set.connection.upgrade().expect("no object in Weak<conection>!")))
                .collect(),
            phantom: PhantomData {},
        }
    }

    pub fn running_connections(&self) -> Vec<RunningPassiveConnection> {
        self.connections.iter().filter_map(|set| {
            match &set.channel {
                None => None,
                Some(_) => RunningPassiveConnection::new(set.connection.upgrade().unwrap()),
            }
        }).collect();
    }
    
    pub fn feedforward(&self, s: S0) {
        for set in &self.connections {
            match &set.channel {
                None => (),
                Some(tx) => tx.send(s),
            }
        }
    }
}

pub struct PostComponentFFW<C, S0, S1>
where C: PassiveConnection<S0, S1> + Send + ?Sized,
      S0: Send,
      S1: Send,
{
    connections: Vec<InSetFFW<C, S1>>,
    phantom: PhantomData<S0>,
}

impl<C, S0, S1> PostComponentFFW<C, S0, S1>
where C: PassiveConnection<S0, S1> + Send + ?Sized,
      S0: Send,
      S1: Send,
{
    pub fn make_idle(&self) -> ComponentIdle<C, S0, S1> {
        ComponentIdle {
            connections: self.connections.iter()
                .map(|set| Arc::downgrade(set.connection.upgrade().expect("no object in Weak<conection>!")))
                .collect(),
            phantom: PhantomData {},
        }
    }

    fn accepted(&self) -> Vec<S1> {
        self.connections.iter()
            .filter_map(|conn| {
                match conn.channel {
                    None => None,
                    Some(rx) => Some(rx.try_iter()),
                }
            }).flatten().collect();
    }

    fn store(&mut self) {
        for conn in &self.connections {
            match &conn.channel {
                None => (),
                Some(rx) => self.buffer.append(rx.try_iter().collect()),
            }
        }
    }
}

struct OutSetFFW<C: Send + ?Sized, S: Send> {
    pub connection: Weak<Mutex<C>>,
    pub channel: Option<CCSender<S>>,
}

struct InSetFFW<C: Send + ?Sized, R: Send> {
    pub connection: Weak<Mutex<C>>,
    pub channel: Option<CCReceiver<R>>,
}
