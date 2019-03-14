// use std::rc::{Rc};
// use std::cell::RefCell;
use try_signalling::signals::signal_1::Connection1;
use try_signalling::supervisor::{Supervisor};
use try_signalling::agents::agent_a::Model as AAgent;
use try_signalling::agents::agent_a::Population as APopulation;
// use try_signalling::signals::signal_2::Channel2;
// use std::thread;
// use std::time::Duration;
// extern crate crossbeam_channel;
// use std::time::Duration;
use std::sync::Arc;
use std::collections::HashMap;

fn main() {

    let mut sp0 = Supervisor {
        populations: HashMap::new(),
        passive_connections: Vec::new(),
    };

    let name_pp_a1 = String::from("a#1");

    let pp_a1 = APopulation::new(); // why I don't use mut and passed?

    sp0.add_population(
        name_pp_a1.clone(),
        Arc::clone(&pp_a1)
    );

    pp_a1.lock().unwrap().add_agent(AAgent::new(0, 0, Some(2)));
    pp_a1.lock().unwrap().add_agent(AAgent::new(10, 0, Some(2)));
    pp_a1.lock().unwrap().add_agent(AAgent::new(100, 0, None));
    // pp_a1.lock().unwrap().agent_by_id(0).lock().unwrap().print_values(); // confirm agent created

    let ag1 = pp_a1.lock().unwrap().agent_by_id(0);
    let ag2 = pp_a1.lock().unwrap().agent_by_id(2);
    sp0.add_passive_connection(Connection1::new(ag1, ag2, 1));

    let ag1 = pp_a1.lock().unwrap().agent_by_id(1);
    let ag2 = pp_a1.lock().unwrap().agent_by_id(2);
    sp0.add_passive_connection(Connection1::new(ag1, ag2, 2));

    // 1st Mutex blocks the 2nd
    // sp0.add_passive_connection(Connection1::new(
    //     pp_a1.lock().unwrap().agent_by_id(0),
    //     pp_a1.lock().unwrap().agent_by_id(2),
    //     1
    // ));
    
    sp0.run(10);
    pp_a1.lock().unwrap()
        .agent_by_id(2)
        .lock().unwrap()
        .show_1();
}
