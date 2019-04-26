// use crate::operation::{RunMode};
// use crate::components::InSet;


// pub struct SingleInComponent<G: Send + ?Sized, R: Send> {
//     mode: RunMode,
//     target_set: InSet<G, R>,
// }

// impl<G: Send + ?Sized, R: Send> SingleInComponent<G, R> {
//     pub fn new(target: Weak<Mutex<G>>) -> SingleInComponent<G, R> {
//         SingleInComponent {
//             mode: RunMode::Idle,
//             target_set: InSet::new(target),
//         }
//     }

//     pub fn mode(&self) -> RunMode {
//         self.mode
//     }
    
//     pub fn config_mode(&mut self, mode: RunMode) {
//         match (mode, &self.mode) {
//             (RunMode::Idle, _) => println!("config_mode for mode Idle, no effect."),
//             (_, RunMode::Idle) => {
//                 self.mode = mode;
//                 self.target_set.config_mode(mode);
//             },
//             (_, _) => panic!("call fn config_mode when not RunMode::Idle!"),
//         }
//     }

//     pub fn config_channels(&mut self) {
//         self.target_set.config_channels();
//     }

    
    
//     pub fn ffw_accepted(&self) -> Vec<S> {
//         match &self.mode {
//             RunMode::Feedforward => self.target_set.ffw_accepted_iter().collect(),
//             RunMode::Idle => panic!("PostComponent is Idle when accepted() called!"),
//         }
//     }
// }
