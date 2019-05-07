use crate::operation::RunMode;

pub trait PassivePopulation {
    fn config_mode(&mut self, mode: RunMode);
    fn config_channels(&mut self);
}
