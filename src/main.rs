// use std::sync::Arc;
// use std::collections::HashMap;

fn main() {

    // let mut sp0 = Supervisor {
    //     agent_populations: HashMap::new(),
    //     connection_populations: HashMap::new(),
    // };

    // // make CAgents
    // let name_ppa_a = String::from("Agent PP A");
    // let pp_agnt_a = SimplePopulation::<AAgent>::new();
    // sp0.add_agent_population(
    //     name_ppa_a.clone(),
    //     Arc::clone(&pp_agnt_a) // should try to avoid Arc::clone.
    // );

    // pp_agnt_a.lock().unwrap().add_agent(AAgent::new(0, 0, Some(2)));
    // pp_agnt_a.lock().unwrap().add_agent(AAgent::new(10, 0, Some(2)));
    // pp_agnt_a.lock().unwrap().add_agent(AAgent::new(100, 0, None));

    // // make CAgents
    // let name_ppa_c = String::from("Agent PP C");
    // let pp_agnt_c = SimplePopulation::<CAgent>::new();
    // sp0.add_agent_population(
    //     name_ppa_c.clone(),
    //     Arc::clone(&pp_agnt_c)
    // );

    // pp_agnt_c.lock().unwrap().add_agent(CAgent::new(0, 10, Some(2)));
    // pp_agnt_c.lock().unwrap().add_agent(CAgent::new(10, 10, Some(2)));
    // pp_agnt_c.lock().unwrap().add_agent(CAgent::new(100, 10, None));

    // // // make Connection1x c -> c
    // // let name_ppc_1cc = String::from("Connection PP 1CC");
    // // let pp_conn_1cc = SimplePassiveConnectionPopulation::<Connection1x<CAgent, CAgent>, FwdPreS1, FwdPostS1>::new(); // how to reduce type signature?
    // // sp0.add_connection_population(
    // //     name_ppc_1cc.clone(),
    // //     Arc::clone(&pp_conn_1cc)
    // // );

    // // // pp_agnt_c[2] get 2 S1 from pp_agnt_c[0 & 1].
    // // pp_conn_1cc.lock().unwrap().add_connection(Connection1x::new_on_populations(10, &pp_agnt_c, 0, &pp_agnt_c, 2));
    // // pp_conn_1cc.lock().unwrap().add_connection(Connection1x::new_on_populations(11, &pp_agnt_c, 1, &pp_agnt_c, 2));


    // // make Connection1x a -> a
    // let name_ppc_1aa = String::from("Connection PP 1AA");
    // let pp_conn_1aa = SimplePassiveConnectionPopulation::<Connection1x<AAgent, AAgent>, FwdPreS1, FwdPostS1>::new();
    // sp0.add_connection_population(
    //     name_ppc_1aa.clone(),
    //     Arc::clone(&pp_conn_1aa)
    // );
    // // pp_agnt_a[2] get 2 S1 from pp_agnt_a[0 & 1].
    // pp_conn_1aa.lock().unwrap().add_connection(Connection1x::new_on_populations(-10, &pp_agnt_a, 0, &pp_agnt_a, 2));
    // pp_conn_1aa.lock().unwrap().add_connection(Connection1x::new_on_populations(-11, &pp_agnt_a, 1, &pp_agnt_a, 2));

    // // // make Connection1x c -> a
    // // let name_ppc_1ca = String::from("Connection PP 1CA");
    // // let pp_conn_1ca = SimplePassiveConnectionPopulation::<Connection1x<CAgent, AAgent>, FwdPreS1, FwdPostS1>::new();
    // // sp0.add_connection_population(
    // //     name_ppc_1ca.clone(),
    // //     Arc::clone(&pp_conn_1ca)
    // // );
    // // // pp_agnt_c[1] get 1 S1 from pp_agnt_a[0]
    // // pp_conn_1ca.lock().unwrap().add_connection(Connection1x::new_on_populations(110, &pp_agnt_c, 0, &pp_agnt_a, 1));

    // // // make Connection1x a -> c
    // // let name_ppc_1ac = String::from("Connection PP 1AC");
    // // let pp_conn_1ac = SimplePassiveConnectionPopulation::<Connection1x<AAgent, CAgent>, FwdPreS1, FwdPostS1>::new();
    // // sp0.add_connection_population(
    // //     name_ppc_1ac.clone(),
    // //     Arc::clone(&pp_conn_1ac)
    // // );
    // // // pp_agnt_a[1] get 1 S1 from pp_agnt_c[0]
    // // pp_conn_1ac.lock().unwrap().add_connection(Connection1x::new_on_populations(-110, &pp_agnt_a, 0, &pp_agnt_c, 1));

    // // // make Connection2 a -> a
    // // let name_ppc_2aa = String::from("Connection PP 2AA");
    // // let pp_conn_2aa = SimplePassiveConnectionPopulation::<Connection2x<AAgent, AAgent>, FwdPreS2, FwdPostS2>::new();
    // // sp0.add_connection_population(
    // //     name_ppc_2aa.clone(),
    // //     Arc::clone(&pp_conn_2aa)
    // // );
    // // // pp_agnt_a[2] get 1 signal_2 from pp_agnt_a[0 & 1].
    // // pp_conn_2aa.lock().unwrap().add_connection(Connection2x::new_on_populations(20, &pp_agnt_a, 1, &pp_agnt_a, 0));

    // println!("start run.");
    // sp0.run(RunMode::Feedforward, 10);

    // // make Connection1x c -> c
    // // series of {_, 10, _} , {_, 11, _}
    // pp_agnt_c.lock().unwrap()
    //     .agent_by_id(2)
    //     .lock().unwrap()
    //     .show();

    // // make Connection1x a -> a
    // // series of {_, -10, _} , {_, -11, _}
    // pp_agnt_a.lock().unwrap()
    //     .agent_by_id(2)
    //     .lock().unwrap()
    //     .show();

    // // make Connection1x c -> a
    // // series of {_, 110, _}
    // pp_agnt_a.lock().unwrap()
    //     .agent_by_id(1)
    //     .lock().unwrap()
    //     .show();

    // // make Connection1x a -> c
    // // series of {_, -110, _}
    // pp_agnt_c.lock().unwrap()
    //     .agent_by_id(1)
    //     .lock().unwrap()
    //     .show();

    // // make Connection2 a -> a
    // // series of {_, 20, _}
    // pp_agnt_a.lock().unwrap()
    //     .agent_by_id(0)
    //     .lock().unwrap()
    //     .show();

    
}
