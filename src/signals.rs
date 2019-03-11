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

pub struct ImportPair<T> {
    sgnl: crossbeam_channel::Receiver<T>,
    sync: crossbeam_channel::Sender<bool>,
}
