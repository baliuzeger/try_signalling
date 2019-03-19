// currently not in use due to the problem of impl Device on Agent/AgentPopulation/Connections
extern crate crossbeam_channel;
use std::sync::{Mutex, Arc};
use std::thread;
use std::thread::JoinHandle;

pub struct RunningSet<R, C> {
    pub instance: JoinHandle<()>,
    pub report: CCReceiver<R>,
    pub confirm: CCSender<C>,
}

impl<R, C> RunningSet<R, C> {
    fn new<T>(device: Arc<Mutex<T>>) -> RunningSet<R, C>
    where T: 'static + Device + Send + ?Sized
    {
        // for strict ordering of agent-connection_prop, bounded(1) is chosen.
        let (tx_conn_report, rx_conn_report) = crossbeam_channel::bounded::<R>(1);
        let (tx_conn_confirm, rx_conn_confirm) = crossbeam_channel::bounded<C>(1);
        RunningSet {
            instance: thread::spawn(move || {device.lock().unwrap().run(rx_conn_confirm, tx_conn_report)}),
            report: rx_conn_report,
            confirm: tx_conn_confirm,
        }
    }    
}

pub trait Device {
    fn run(&mut self);
}
