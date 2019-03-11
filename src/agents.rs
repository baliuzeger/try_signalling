extern crate crossbeam_channel;
pub mod agent_a;

pub trait Agent {
    fn evolve();
}

pub struct ExportPair<T> {
    sgnl: crossbeam_channel::Sender<T>,
    sync: crossbeam_channel::Receiver<bool>,
}
