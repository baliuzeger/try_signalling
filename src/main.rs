// use std::rc::{Rc};
// use std::cell::RefCell;
// use try_signalling::connections::signal_1::Connection as Connection1;
// use try_signalling::connection_populations::SimplePassiveConnectionPopulation;
// use try_signalling::connections::signal_2::Connection as Connection2;
// use try_signalling::connections::signal_1::Connection as Connection1;
use try_signalling::supervisor::{Supervisor, RunMode};
// use try_signalling::agents::agent_a::Model as AAgent;
// use try_signalling::agents::agent_b::Model as BAgent;
use try_signalling::agents::agent_c::Model as CAgent;
use try_signalling::agent_populations::SimplePopulation;
use try_signalling::connection_populations::SimplePassiveConnectionPopulation;
use try_signalling::connections::signal_1::connection_1x::Model as Connection1x;
use try_signalling::connections::signal_1::{FwdPreS1, FwdPostS1};
use std::sync::Arc;
use std::collections::HashMap;

fn main() {

    let mut sp0 = Supervisor {
        agent_populations: HashMap::new(),
        connection_populations: HashMap::new(),
    };

    let name_ppa_a = String::from("Agent PP A");
    let pp_agnt_a = SimplePopulation::<CAgent>::new();
    sp0.add_agent_population(
        name_ppa_a.clone(),
        Arc::clone(&pp_agnt_a)
    );

    pp_agnt_a.lock().unwrap().add_agent(CAgent::new(0, 0, Some(2)));
    pp_agnt_a.lock().unwrap().add_agent(CAgent::new(10, 0, Some(2)));
    pp_agnt_a.lock().unwrap().add_agent(CAgent::new(100, 0, None));

    let name_ppc_x = String::from("Connection PP X");
    let pp_conn_x = SimplePassiveConnectionPopulation::<Connection1x<CAgent, CAgent>, FwdPreS1, FwdPostS1>::new();
    sp0.add_connection_population(
        name_ppc_x.clone(),
        Arc::clone(&pp_conn_x)
    );

    let ag1 = Arc::downgrade(&pp_agnt_a.lock().unwrap().agent_by_id(0));
    let ag2 = Arc::downgrade(&pp_agnt_a.lock().unwrap().agent_by_id(2));
    pp_conn_x.lock().unwrap().add_connection(Connection1x::new(ag1, ag2, 1));

    sp0.run(RunMode::Feedforward, 10);

    pp_agnt_a.lock().unwrap()
        .agent_by_id(2)
        .lock().unwrap()
        .show();

    pp_agnt_a.lock().unwrap()
        .agent_by_id(0)
        .lock().unwrap()
        .show();

    pp_agnt_a.lock().unwrap()
        .agent_by_id(1)
        .lock().unwrap()
        .show();
    
}
