use crate::components::{MultiInComponent, MultiOutComponent, SingleInComponent, SingleOutComponent};
// use crate::components::multi_out_components::MultiOutComponent;
// use crate::components::multi_in_components::MultiInComponent;
// use crate::components::multi_out_components::MultiOutComponent;
// use crate::components::multi_in_components::MultiInComponent;
use crate::connectivity::{Generator, PassiveAcceptor, Acceptor};

pub mod connection_1x;

#[derive(Copy, Clone)]
pub struct FwdPreS1 {
    pub msg_gen: i32
}

// pub struct BkwdPreS1 {
//     pub msg_gen: i32
// }

// Active_Multi_Out to Active_Multi_Out not implemented yet.
pub type MultiOutComponentS1 = MultiOutComponent<dyn PassiveAcceptor<FwdPreS1>, FwdPreS1>;

pub type MultiInComponentS1 = MultiInComponent<dyn Generator<FwdPreS1>, FwdPreS1>;

// SingleOut to PassiveSingleOut not implemented yet.
pub type SingleOutComponentS1<A> = SingleOutComponent<A: Acceptor<FwdPreS1>, FwdPreS1>;

pub type SingleInComponentS1<G> = SingleInComponent<G: Generator<FwdPreS1>, FwdPreS1>;
