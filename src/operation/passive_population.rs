use crate::operation::RunMode;

pub trait PassivePopulation {
    fn config_run(&mut self, mode: RunMode);
    fn config_channels(&mut self);
    fn config_idle(&mut self);
}
