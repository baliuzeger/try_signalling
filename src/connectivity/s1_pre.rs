use crate::components::{MultiInComponent, MultiOutComponent, SingleInComponent, SingleOutComponent};
use crate::connectivity::{Generator, PassiveAcceptor};

#[derive(Copy, Clone)]
pub struct FwdPreS1 {
    pub msg_gen: i32
}

// pub struct BkwdPreS1 {
//     pub msg_gen: i32
// }

// Active_Multi_Out to Active_Multi_Out not implemented yet.
pub type MultiOutComponentS1Pre = MultiOutComponent<dyn PassiveAcceptor<FwdPreS1>, FwdPreS1>;

pub type MultiInComponentS1Pre = MultiInComponent<dyn Generator<FwdPreS1>, FwdPreS1>;

// SingleOut to PassiveSingleOut not implemented yet.
pub type SingleOutComponentS1Pre<A> = SingleOutComponent<A, FwdPreS1>;

pub type SingleInComponentS1Pre<G> = SingleInComponent<G, FwdPreS1>;
