use crate::components::{MultiInComponent, MultiOutComponent, SingleInComponent, SingleOutComponent};
use crate::connectivity::{Generator, PassiveAcceptor};

#[derive(Copy, Clone)]
pub struct FwdPostS1 {
    pub msg_gen: i32,
    pub msg_prop: i32,
}

// pub struct BkwdPostS1 {
//     pub msg_gen: i32,
//     pub msg_prop: i32,
// }

// Active_Multi_Out to Active_Multi_Out not implemented yet.
pub type MultiOutComponentS1Post = MultiOutComponent<dyn PassiveAcceptor<FwdPostS1>, FwdPostS1>;

pub type MultiInComponentS1Post = MultiInComponent<dyn Generator<FwdPostS1>, FwdPostS1>;

// SingleOut to PassiveSingleOut not implemented yet.
pub type SingleOutComponentS1Post<A> = SingleOutComponent<A, FwdPostS1>;

pub type SingleInComponentS1Post<G> = SingleInComponent<G, FwdPostS1>;
