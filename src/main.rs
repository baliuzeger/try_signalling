use std::sync::{Arc};
use try_signalling::supervisor::Supervisor;
use try_signalling::populations::{SimpleFiringPopulation, SimplePassivePopulation};
use try_signalling::devices::neurons::NeuronC;
use try_signalling::devices::connections::ConnectionS1X;

fn main() {

    let mut sp0 = Supervisor::new();

    // make NeuronC
    let name_pp_neuron_c = String::from("NeuronC Population");
    let pp_neuron_c = SimpleFiringPopulation::<NeuronC>::new();
    sp0.add_firing(
        name_pp_neuron_c.clone(),
        Arc::downgrade(&pp_neuron_c) // should try to avoid Arc::clone.
    );

    pp_neuron_c.lock().unwrap().add(NeuronC::new(0, 0, Some(2)));
    pp_neuron_c.lock().unwrap().add(NeuronC::new(10, 0, Some(2)));
    pp_neuron_c.lock().unwrap().add(NeuronC::new(100, 0, None));

    // make Connection1x
    let name_pp_connection_s1_x = String::from("ConnectionS1 Population");
    let pp_conn_s1_x = SimplePassivePopulation::<ConnectionS1X>::new();
    sp0.add_passive(
        name_pp_connection_s1_x.clone(),
        Arc::downgrade(&pp_conn_s1_x)
    );

    
    
    // pp_agnt_c[2] get 2 S1 from pp_agnt_c[0 & 1].
    //pp_conn_s1_x.lock().unwrap().add(ConnectionS1X::new_on_populations(10, &pp_neuron_c, 0, &pp_neuron_c, 2));
    
    // pp_conn_1cc.lock().unwrap().add_connection(ConnectionS1X::new_on_populations(10, &pp_agnt_c, 0, &pp_agnt_c, 2));
    // pp_conn_1cc.lock().unwrap().add_connection(ConnectionS1X::new_on_populations(11, &pp_agnt_c, 1, &pp_agnt_c, 2));


    // // make Connection1x a -> a
    // let name_ppc_1aa = String::from("Connection PP 1AA");
    // let pp_conn_1aa = SimplePassiveConnectionPopulation::<Connection1x<NeuronC, NeuronC>, FwdPreS1, FwdPostS1>::new();
    // sp0.add_connection_population(
    //     name_ppc_1aa.clone(),
    //     Arc::clone(&pp_conn_1aa)
    // );
    // // pp_neuron_c[2] get 2 S1 from pp_neuron_c[0 & 1].
    // pp_conn_1aa.lock().unwrap().add_connection(Connection1x::new_on_populations(-10, &pp_neuron_c, 0, &pp_neuron_c, 2));
    // pp_conn_1aa.lock().unwrap().add_connection(Connection1x::new_on_populations(-11, &pp_neuron_c, 1, &pp_neuron_c, 2));

    // // // make Connection1x c -> a
    // // let name_ppc_1ca = String::from("Connection PP 1CA");
    // // let pp_conn_1ca = SimplePassiveConnectionPopulation::<Connection1x<CAgent, NeuronC>, FwdPreS1, FwdPostS1>::new();
    // // sp0.add_connection_population(
    // //     name_ppc_1ca.clone(),
    // //     Arc::clone(&pp_conn_1ca)
    // // );
    // // // pp_agnt_c[1] get 1 S1 from pp_neuron_c[0]
    // // pp_conn_1ca.lock().unwrap().add_connection(Connection1x::new_on_populations(110, &pp_agnt_c, 0, &pp_neuron_c, 1));

    // // // make Connection1x a -> c
    // // let name_ppc_1ac = String::from("Connection PP 1AC");
    // // let pp_conn_1ac = SimplePassiveConnectionPopulation::<Connection1x<NeuronC, CAgent>, FwdPreS1, FwdPostS1>::new();
    // // sp0.add_connection_population(
    // //     name_ppc_1ac.clone(),
    // //     Arc::clone(&pp_conn_1ac)
    // // );
    // // // pp_neuron_c[1] get 1 S1 from pp_agnt_c[0]
    // // pp_conn_1ac.lock().unwrap().add_connection(Connection1x::new_on_populations(-110, &pp_neuron_c, 0, &pp_agnt_c, 1));

    // // // make Connection2 a -> a
    // // let name_ppc_2aa = String::from("Connection PP 2AA");
    // // let pp_conn_2aa = SimplePassiveConnectionPopulation::<Connection2x<NeuronC, NeuronC>, FwdPreS2, FwdPostS2>::new();
    // // sp0.add_connection_population(
    // //     name_ppc_2aa.clone(),
    // //     Arc::clone(&pp_conn_2aa)
    // // );
    // // // pp_neuron_c[2] get 1 signal_2 from pp_neuron_c[0 & 1].
    // // pp_conn_2aa.lock().unwrap().add_connection(Connection2x::new_on_populations(20, &pp_neuron_c, 1, &pp_neuron_c, 0));

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
    // pp_neuron_c.lock().unwrap()
    //     .agent_by_id(2)
    //     .lock().unwrap()
    //     .show();

    // // make Connection1x c -> a
    // // series of {_, 110, _}
    // pp_neuron_c.lock().unwrap()
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
    // pp_neuron_c.lock().unwrap()
    //     .agent_by_id(0)
    //     .lock().unwrap()
    //     .show();

    
}
