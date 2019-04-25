use std::sync::{Arc, Mutex};
extern crate crossbeam_channel;
use crossbeam_channel::Receiver as CCReceiver;
use crossbeam_channel::Sender as CCSender;
use std::thread;
use std::thread::JoinHandle;
use crate::operation::firing_device::FiringDevice;
use crate::operation::passive_device::PassiveDevice;
use crate::operation::firing_population::FiringPopulation;

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

#[derive(Copy, Clone, Debug)]
pub enum RunMode {
    Idle,
    Feedforward,
}

impl RunMode {
    pub fn mode_from_device<I, F>(m: &DeviceMode<I, F>) -> RunMode {
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

impl<I, F> DeviceMode<I, F> {
    pub fn eq_mode<I1, F1, I2, F2>(m1: DeviceMode<I1, F1>, m2: DeviceMode<I2, F2>) -> RunMode {
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
    pub fn new_neuron<T>(device: Arc<Mutex<T>>) -> RunningSet<C, R>
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

    pub fn new_passive<T>(device: Arc<Mutex<T>>) -> RunningSet<C, R>
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

    pub fn new_neuron_population<T>(device: Arc<Mutex<T>>) -> RunningSet<C, R>
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

    // pub fn new<F, C, R>(f: Box<dyn FnMut(CCReceiver<C>, CCSender<R>)>) -> RunningSet<C, R> {
    //     // for strict ordering of agent-connection_prop, bounded(1) is chosen.
    //     let (tx_confirm, rx_confirm) = crossbeam_channel::bounded(1);
    //     let (tx_report, rx_report) = crossbeam_channel::bounded(1);
    //     RunningSet {
    //         instance: thread::spawn(move || {*f(rx_confirm, tx_report)}),
    //         confirm: tx_confirm,
    //         report: rx_report,
    //     }
    // }
}
