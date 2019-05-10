use std::sync::{Arc};
use try_signalling::supervisor::Supervisor;
use try_signalling::populations::{SimpleFiringPopulation, SimplePassivePopulation};
use try_signalling::devices::neurons::{NeuronC, NeuronD};
use try_signalling::devices::connections::{ConnectionS1X, ConnectionS1PrePre};
use try_signalling::operation::RunMode;
use try_signalling::populations::HoldDevices;
use try_signalling::connectivity;

fn main() {

    let mut sp0 = Supervisor::new();

    // // make NeuronC
    // let name_pp_neuron_c = String::from("NeuronC Population");
    // let pp_neuron_c = SimpleFiringPopulation::<NeuronC>::new();
    // sp0.add_firing(
    //     name_pp_neuron_c.clone(),
    //     Arc::downgrade(&pp_neuron_c) // should try to avoid Arc::clone.
    // );

    // pp_neuron_c.lock().unwrap().add(NeuronC::new(0, 0, Some(2)));
    // pp_neuron_c.lock().unwrap().add(NeuronC::new(10, 0, Some(2)));
    // pp_neuron_c.lock().unwrap().add(NeuronC::new(100, 0, None));

    // // make Connection1x
    // let name_pp_connection_s1_x = String::from("ConnectionS1 Population");
    // let pp_conn_s1_x = SimplePassivePopulation::<ConnectionS1X>::new();
    // sp0.add_passive(
    //     name_pp_connection_s1_x.clone(),
    //     Arc::downgrade(&pp_conn_s1_x)
    // );

    // // pp_agnt_c[0 & 1] -> S1Pre -> ConnS1X -> S1Post -> pp_agnt_c[2]
    // pp_conn_s1_x.lock().unwrap().add(ConnectionS1X::new_with_active_population(10, &pp_neuron_c, 0, &pp_neuron_c, 2));
    // pp_conn_s1_x.lock().unwrap().add(ConnectionS1X::new_with_active_population(11, &pp_neuron_c, 1, &pp_neuron_c, 2));

    /// make NeuronD
    let name_pp_neuron_d = String::from("NeuronD Population");
    let pp_neuron_d = SimpleFiringPopulation::<NeuronD>::new();
    sp0.add_firing(
        name_pp_neuron_d.clone(),
        Arc::downgrade(&pp_neuron_d) // should try to avoid Arc::clone.
    );

    pp_neuron_d.lock().unwrap().add(NeuronD::new(0, 100, Some(2)));
    // pp_neuron_d.lock().unwrap().add(NeuronD::new(10, 100, Some(2)));
    // pp_neuron_d.lock().unwrap().add(NeuronD::new(100, 100, None));

    /// NeuronD -> S1Pre -> NeuronD; active -> active
    // connectivity::connect_on_population_active(&pp_neuron_d, 0, &pp_neuron_d, 2);
    // connectivity::connect_on_population_active(&pp_neuron_d, 1, &pp_neuron_d, 2);


    /// make ConnectionS1PrePre
    let name_pp_conn_s1_pre_pre = String::from("ConnectionS1PrePre Population");
    let pp_conn_s1_pre_pre = SimplePassivePopulation::<ConnectionS1PrePre>::new();
    sp0.add_passive(
        name_pp_conn_s1_pre_pre.clone(),
        Arc::downgrade(&pp_conn_s1_pre_pre) // should try to avoid Arc::clone.
    );
    pp_conn_s1_pre_pre.lock().unwrap().add(ConnectionS1PrePre::new(0));
    pp_conn_s1_pre_pre.lock().unwrap().add(ConnectionS1PrePre::new(-50));
    pp_conn_s1_pre_pre.lock().unwrap().add(ConnectionS1PrePre::new(-100));
    
    /// active -> passive -> passive
    connectivity::connect_on_population_passive(&pp_neuron_d, 0, &pp_conn_s1_pre_pre, 0);
    connectivity::connect_on_population_passive(&pp_conn_s1_pre_pre, 0, &pp_conn_s1_pre_pre, 1);
    connectivity::connect_on_population_passive(&pp_conn_s1_pre_pre, 1, &pp_conn_s1_pre_pre, 2);
    
    /// active -> passive -> passive -> active

   


    
    println!("start run.");
    sp0.run(RunMode::Feedforward, 10);

    // // pp_agnt_c[0 & 1] -> S1Pre -> ConnS1X -> S1Post -> pp_agnt_c[2]
    // // series of {_, 10, _} , {_, 11, _}
    // pp_neuron_c.lock().unwrap()
    //     .device_by_id(2)
    //     .lock().unwrap()
    //     .show();

    // NeuronD -> S1Pre -> NeuronD; active -> active
    // series of {x, 10x} , {1x, 10x}
    // pp_neuron_d.lock().unwrap()
    //     .device_by_id(2)
    //     .lock().unwrap()
    //     .show();
}
