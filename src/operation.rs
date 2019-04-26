use std::sync::{Arc, Mutex};
extern crate crossbeam_channel;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use std::thread;
use std::thread::JoinHandle;
pub use crate::operation::firing_device::FiringDevice;
pub use crate::operation::passive_device::PassiveDevice;
pub use crate::operation::firing_population::FiringPopulation;
pub use crate::operation::passive_population::PassivePopulation;

pub mod firing_population;
// pub mod active_population;
pub mod passive_population;
pub mod firing_device;
// pub mod active_device;
pub mod passive_device;

pub enum Broadcast {
    NewCycle,
    FinishCycle,
    Exit,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RunMode {
    Idle,
    Feedforward,
}

impl RunMode {
    pub fn mode_from_device<F>(m: &DeviceMode<F>) -> RunMode {
        match m {
            DeviceMode::Idle => RunMode::Idle,
            DeviceMode::Feedforward(_) => RunMode::Feedforward,
        }
    }

    pub fn eq_mode(m1: RunMode, m2: RunMode) -> RunMode {
        match (m1, m2) {
            (RunMode::Idle, RunMode::Idle) => RunMode::Idle,
            (RunMode::Feedforward, RunMode::Feedforward) => RunMode::Feedforward,
            _ => panic!("Runmode mismatch at check!"),
        }
    }
}

pub enum DeviceMode<F> {
    Idle,
    Feedforward(F),
}

impl<F> DeviceMode<F> {
    pub fn eq_mode<F1, F2>(m1: DeviceMode<F1>, m2: DeviceMode<F2>) -> RunMode {
        match (m1, m2) {
            (DeviceMode::Idle, DeviceMode::Idle) => RunMode::Idle,
            (DeviceMode::Feedforward(_), DeviceMode::Feedforward(_)) => RunMode::Feedforward,
            _ => panic!("Runmode mismatch at check!"),
        }
    }
}

pub enum Fired {
    Y,
    N,
}

pub trait Runnable {
    fn config_run(&mut self, mode: RunMode);
    fn config_channels(&mut self);
    fn config_idle(&mut self);
}

pub struct RunningSet<C: Send, R: Send> {
    pub instance: JoinHandle<()>,
    pub confirm: CCSender<C>,
    pub report: CCReceiver<R>,
}

impl<C: Send, R: Send> RunningSet<C, R> {
    pub fn new_firing_device<T>(device: Arc<Mutex<T>>) -> RunningSet<Broadcast, Fired>
    where T: 'static + FiringDevice + Send + ?Sized
    {
        // for strict ordering of agent-connection_prop, bounded(1) is chosen.
        let (tx_confirm, rx_confirm) = crossbeam_channel::bounded(1);
        let (tx_report, rx_report) = crossbeam_channel::bounded(1);
        RunningSet {
            instance: thread::spawn(move || {device.lock().unwrap().run(rx_confirm, tx_report)}),
            confirm: tx_confirm,
            report: rx_report,
        }
    }

    pub fn new_passive_device<T>(device: Arc<Mutex<T>>) -> RunningSet<Broadcast, ()>
    where T: 'static + PassiveDevice + Send + ?Sized
    {
        // for strict ordering of agent-connection_prop, bounded(1) is chosen.
        let (tx_confirm, rx_confirm) = crossbeam_channel::bounded(1);
        let (tx_report, rx_report) = crossbeam_channel::bounded(1);
        RunningSet {
            instance: thread::spawn(move || {device.lock().unwrap().run(rx_confirm, tx_report)}),
            report: rx_report,
            confirm: tx_confirm,
        }
    }

    pub fn new_firing_population<T>(device: Arc<Mutex<T>>) -> RunningSet<Broadcast, Fired>
    where T: 'static + FiringPopulation + Send + ?Sized
    {
        // for strict ordering of agent-connection_prop, bounded(1) is chosen.
        let (tx_confirm, rx_confirm) = crossbeam_channel::bounded(1);
        let (tx_report, rx_report) = crossbeam_channel::bounded(1);
        RunningSet {
            instance: thread::spawn(move || {device.lock().unwrap().run(rx_confirm, tx_report)}),
            confirm: tx_confirm,
            report: rx_report,
        }
    }

    // pub fn new(f: Box<dyn FnMut(CCReceiver<C>, CCSender<R>) + Send>) -> RunningSet<C, R> {
    //     // for strict ordering of agent-connection_prop, bounded(1) is chosen.
    //     let (tx_confirm, rx_confirm) = crossbeam_channel::bounded(1);
    //     let (tx_report, rx_report) = crossbeam_channel::bounded(1);
    //     RunningSet {
    //         instance: thread::spawn(move || {(*f)(rx_confirm, tx_report)}),
    //         confirm: tx_confirm,
    //         report: rx_report,
    //     }
    // }
}
