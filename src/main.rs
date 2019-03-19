// use std::rc::{Rc};
// use std::cell::RefCell;
// use try_signalling::connections::signal_1::Connection as Connection1;
use try_signalling::connections::signal_2::Connection as Connection2;
use try_signalling::supervisor::{Supervisor};
// use try_signalling::agents::agent_a::Model as AAgent;
use try_signalling::agents::agent_b::Model as BAgent;
use std::sync::Arc;
use std::collections::HashMap;

fn main() {

    // let mut sp0 = Supervisor {
    //     populations: HashMap::new(),
    //     passive_connections: Vec::new(),
    // };

    // let name_pp_a = String::from("PPA");
    // let pp_a = APopulation::new();
    // sp0.add_population(
    //     name_pp_a.clone(),
    //     Arc::clone(&pp_a)
    // );

    // pp_a.lock().unwrap().add_agent(AAgent::new(0, 0, Some(2)));
    // pp_a.lock().unwrap().add_agent(AAgent::new(10, 0, Some(2)));
    // pp_a.lock().unwrap().add_agent(AAgent::new(100, 0, None));
    
    // let name_pp_b = String::from("PPB");
    // let pp_b = BPopulation::new();
    // sp0.add_population(
    //     name_pp_b.clone(),
    //     Arc::clone(&pp_b)
    // );

    // pp_b.lock().unwrap().add_agent(BAgent::new(-100, 0, Some(2)));
    // pp_b.lock().unwrap().add_agent(BAgent::new(-1000, 0, Some(2)));
    // pp_b.lock().unwrap().add_agent(BAgent::new(-10000, 0, None));

    // // // A -> A, Conn1
    // // let ag1 = pp_a.lock().unwrap().agent_by_id(0);
    // // let ag2 = pp_a.lock().unwrap().agent_by_id(2);
    // // sp0.add_passive_connection(Connection1::new(ag1, ag2, 1));

    // // A -> A, Conn2
    // let ag1 = pp_a.lock().unwrap().agent_by_id(1);
    // let ag2 = pp_a.lock().unwrap().agent_by_id(2);
    // sp0.add_passive_connection(Connection2::new(ag1, ag2, 2));

    // // // b -> B, Conn1
    // // let ag1 = pp_b.lock().unwrap().agent_by_id(0);
    // // let ag2 = pp_b.lock().unwrap().agent_by_id(2);
    // // sp0.add_passive_connection(Connection1::new(ag1, ag2, 11));

    // // // B -> B, Conn2
    // // let ag1 = pp_b.lock().unwrap().agent_by_id(1);
    // // let ag2 = pp_b.lock().unwrap().agent_by_id(2);
    // // sp0.add_passive_connection(Connection2::new(ag1, ag2, 12));

    // // // A -> B, Conn1
    // // let ag1 = pp_a.lock().unwrap().agent_by_id(0);
    // // let ag2 = pp_b.lock().unwrap().agent_by_id(2);
    // // sp0.add_passive_connection(Connection1::new(ag1, ag2, 101));

    // // // A -> B, Conn2
    // // let ag1 = pp_a.lock().unwrap().agent_by_id(1);
    // // let ag2 = pp_b.lock().unwrap().agent_by_id(2);
    // // sp0.add_passive_connection(Connection2::new(ag1, ag2, 102));
    
    // // B -> A, Conn1
    // let ag1 = pp_b.lock().unwrap().agent_by_id(0);
    // let ag2 = pp_a.lock().unwrap().agent_by_id(2);
    // sp0.add_passive_connection(Connection1::new(ag1, ag2, 1001));

    // // B -> A, Conn2
    // let ag1 = pp_b.lock().unwrap().agent_by_id(1);
    // let ag2 = pp_a.lock().unwrap().agent_by_id(2);
    // sp0.add_passive_connection(Connection2::new(ag1, ag2, 1002));

    
    // // 1st Mutex blocks the 2nd
    // // sp0.add_passive_connection(Connection1::new(
    // //     pp_a.lock().unwrap().agent_by_id(0),
    // //     pp_a.lock().unwrap().agent_by_id(2),
    // //     1
    // // ));
    
    // sp0.run(30);

    // pp_a.lock().unwrap()
    //     .agent_by_id(2)
    //     .lock().unwrap()
    //     .show_1();

    // pp_a.lock().unwrap()
    //     .agent_by_id(2)
    //     .lock().unwrap()
    //     .show_2();

    // pp_b.lock().unwrap()
    //     .agent_by_id(2)
    //     .lock().unwrap()
    //     .show_1();

    // pp_b.lock().unwrap()
    //     .agent_by_id(2)
    //     .lock().unwrap()
    //     .show_2();
    
}
