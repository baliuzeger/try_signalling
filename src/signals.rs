/// functionality needed:
/// 1. a channel should not connect from/to an identical agent.
pub mod signal_1;
// pub mod signal_2;

pub trait PassiveConnection {
    fn standby(&self);
}

pub trait ActiveConnection {
    fn evolve(&self);
}

pub struct InAgentSet<S: Generate1, T: Send> {
    agent: Arc<Mutex<S>>,
    sgnl: crossbeam_channel::Receiver<T>,
//    sync: crossbeam_channel::Sender<bool>,
}

pub struct OutAgnetSet<R: Process1, T: Send> {
    agent: Arc<Mutex<R>>,
    sgnl: crossbeam_channel::Sender<T>,
}
