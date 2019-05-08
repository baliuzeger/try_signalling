use std::sync::{Arc, Mutex};
extern crate crossbeam_channel;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use std::thread;
use std::thread::JoinHandle;

pub use self::op_population::{ConsecutiveActivePopulation, FiringActivePopulation, SilentActivePopulation, PassivePopulation};
pub use self::op_device::{SilentPassiveDevice, FiringPassiveDevice, ConsecutiveActiveDevice, ConsecutivePassiveDevice, FiringActiveDevice, SilentActiveDevice};

pub mod population;
pub mod device;

pub enum Broadcast {
    Evolve,
    Respond,
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

pub trait Configurable {
    fn config_mode(&mut self, mode: RunMode);
    fn config_channels(&mut self);
}

pub trait Runnable {
    type Report;
    fn run(&mut self);
}

pub trait ActiveDevice {}

/// for PassivePopulation & connectivity / OutComponents
pub trait PassiveDevice: Runnable {}

pub struct RunningSet<C: Send, R: Send> {
    pub instance: JoinHandle<()>,
    pub confirm: CCSender<C>,
    pub report: CCReceiver<R>,
}

impl<C: Send, R: Send> RunningSet<C, R> {
    pub fn new<T>(device: Arc<Mutex<T>>) -> RunningSet<Broadcast, <T as Runnable>::Report>
    where T: 'static + Runnable + Send + ?Sized
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
}
