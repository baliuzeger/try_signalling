// use std::rc::{Rc};
// use std::cell::RefCell;
// use try_signalling::connections::signal_1::Connection as Connection1;
// use try_signalling::connection_populations::SimplePassiveConnectionPopulation;
// use try_signalling::connections::signal_2::Connection as Connection2;
// use try_signalling::connections::signal_1::Connection as Connection1;
// use try_signalling::supervisor::{Supervisor};
// use try_signalling::agents::agent_a::Model as AAgent;
// use try_signalling::agents::agent_b::Model as BAgent;
// use try_signalling::agent_populations::SimplePopulation;
// use std::sync::Arc;
// use std::collections::HashMap;

fn main() {

    // let mut sp0 = Supervisor {
    //     populations: HashMap::new(),
    // };

    // let name_pp_a = String::from("PPA");
    // let pp_agnt_a = SimplePopulation::new();
    // sp0.add_population(
    //     name_pp_a.clone(),
    //     Arc::clone(&pp_agnt_a)
    // );

    // pp_agnt_a.lock().unwrap().add_agent(AAgent::new(0, 0, Some(2)));
    // pp_agnt_a.lock().unwrap().add_agent(AAgent::new(10, 0, Some(2)));
    // pp_agnt_a.lock().unwrap().add_agent(AAgent::new(100, 0, None));
    
    // let name_pp_b = String::from("PPB");
    // let pp_agnt_b = SimplePopulation::new();
    // sp0.add_population(
    //     name_pp_b.clone(),
    //     Arc::clone(&pp_agnt_b)
    // );
    
    // pp_agnt_b.lock().unwrap().add_agent(BAgent::new(-100, 0, Some(2)));
    // pp_agnt_b.lock().unwrap().add_agent(BAgent::new(-1000, 0, Some(2)));
    // pp_agnt_b.lock().unwrap().add_agent(BAgent::new(-10000, 0, None));
    
    // // A -> A, Conn1
    // let pp_conn_aa1 = SimplePassiveConnectionPopulation::<Connection1<AAgent, AAgent>>::new();
    // let ag1 = pp_agnt_a.lock().unwrap().agent_by_id(0);
    // let ag2 = pp_agnt_a.lock().unwrap().agent_by_id(2);
    // pp_conn_aa1.lock().unwrap().add_connection(Connection1::new(ag1, ag2, 1));

    // // A -> A, Conn2
    // let pp_conn_aa2 = SimplePassiveConnectionPopulation::<Connection2<AAgent, AAgent>>::new();
    // let ag1 = pp_agnt_a.lock().unwrap().agent_by_id(1);
    // let ag2 = pp_agnt_a.lock().unwrap().agent_by_id(2);
    // pp_conn_aa2.lock().unwrap().add_connection(Connection2::new(ag1, ag2, 2));

    // // B -> B, Conn1
    // let pp_conn_bb1 = SimplePassiveConnectionPopulation::<Connection1<BAgent, BAgent>>::new();
    // let ag1 = pp_agnt_b.lock().unwrap().agent_by_id(0);
    // let ag2 = pp_agnt_b.lock().unwrap().agent_by_id(2);
    // pp_conn_bb1.lock().unwrap().add_connection(Connection1::new(ag1, ag2, 11));

    // // B -> B, Conn2
    // let pp_conn_bb2 = SimplePassiveConnectionPopulation::<Connection2<BAgent, BAgent>>::new();
    // let ag1 = pp_agnt_b.lock().unwrap().agent_by_id(1);
    // let ag2 = pp_agnt_b.lock().unwrap().agent_by_id(2);
    // pp_conn_bb2.lock().unwrap().add_connection(Connection2::new(ag1, ag2, 12));


    // // A -> B, Conn1
    // let pp_conn_ab1 = SimplePassiveConnectionPopulation::<Connection1<AAgent, BAgent>>::new();
    // let ag1 = pp_agnt_a.lock().unwrap().agent_by_id(0);
    // let ag2 = pp_agnt_b.lock().unwrap().agent_by_id(2);
    // pp_conn_ab1.lock().unwrap().add_connection(Connection1::new(ag1, ag2, 101));

    // // A -> B, Conn2
    // let pp_conn_ab2 = SimplePassiveConnectionPopulation::<Connection2<AAgent, BAgent>>::new();
    // let ag1 = pp_agnt_a.lock().unwrap().agent_by_id(1);
    // let ag2 = pp_agnt_b.lock().unwrap().agent_by_id(2);
    // pp_conn_ab2.lock().unwrap().add_connection(Connection2::new(ag1, ag2, 102));
    
    // // B -> A, Conn1
    // let pp_conn_ba1 = SimplePassiveConnectionPopulation::<Connection1<BAgent, AAgent>>::new();
    // let ag1 = pp_agnt_b.lock().unwrap().agent_by_id(0);
    // let ag2 = pp_agnt_a.lock().unwrap().agent_by_id(2);
    // pp_conn_ba1.lock().unwrap().add_connection(Connection1::new(ag1, ag2, 1001));

    // // B -> A, Conn2
    // let pp_conn_ba2 = SimplePassiveConnectionPopulation::<Connection2<BAgent, AAgent>>::new();
    // let ag1 = pp_agnt_b.lock().unwrap().agent_by_id(1);
    // let ag2 = pp_agnt_a.lock().unwrap().agent_by_id(2);
    // pp_conn_ba2.lock().unwrap().add_connection(Connection2::new(ag1, ag2, 1002));

    
    // // // 1st Mutex blocks the 2nd
    // // // sp0.add_passive_connection(Connection1::new(
    // // //     pp_agnt_a.lock().unwrap().agent_by_id(0),
    // // //     pp_agnt_a.lock().unwrap().agent_by_id(2),
    // // //     1
    // // // ));
    
    // sp0.run(30);

    // pp_agnt_a.lock().unwrap()
    //     .agent_by_id(2)
    //     .lock().unwrap()
    //     .show_1();

    // pp_agnt_a.lock().unwrap()
    //     .agent_by_id(2)
    //     .lock().unwrap()
    //     .show_2();

    // pp_agnt_b.lock().unwrap()
    //     .agent_by_id(2)
    //     .lock().unwrap()
    //     .show_1();

    // pp_agnt_b.lock().unwrap()
    //     .agent_by_id(2)
    //     .lock().unwrap()
    //     .show_2();
    
}
