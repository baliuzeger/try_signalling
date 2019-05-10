use std::sync::{Arc};
use try_signalling::supervisor::Supervisor;
use try_signalling::populations::{SimpleFiringPopulation, SimplePassivePopulation};
use try_signalling::devices::neurons::NeuronC;
use try_signalling::devices::connections::ConnectionS1X;
use try_signalling::operation::RunMode;
use try_signalling::populations::HoldDevices;

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
    pp_conn_s1_x.lock().unwrap().add(ConnectionS1X::new_with_active_population(10, &pp_neuron_c, 0, &pp_neuron_c, 2));
    pp_conn_s1_x.lock().unwrap().add(ConnectionS1X::new_with_active_population(11, &pp_neuron_c, 1, &pp_neuron_c, 2));

    println!("start run.");
    sp0.run(RunMode::Feedforward, 10);

    // make ConnectionS1X c -> c
    // series of {_, 10, _} , {_, 11, _}
    pp_neuron_c.lock().unwrap()
        .device_by_id(2)
        .lock().unwrap()
        .show();

    // // make Connection1x a -> a
    // // series of {_, -10, _} , {_, -11, _}
    // pp_neuron_c.lock().unwrap()
    //     .device_by_id(2)
    //     .lock().unwrap()
    //     .show();

    // // make Connection1x c -> a
    // // series of {_, 110, _}
    // pp_neuron_c.lock().unwrap()
    //     .device_by_id(1)
    //     .lock().unwrap()
    //     .show();

    // // make Connection1x a -> c
    // // series of {_, -110, _}
    // pp_neuron_c.lock().unwrap()
    //     .device_by_id(1)
    //     .lock().unwrap()
    //     .show();

    // // make Connection2 a -> a
    // // series of {_, 20, _}
    // pp_neuron_c.lock().unwrap()
    //     .device_by_id(0)
    //     .lock().unwrap()
    //     .show();

    
}
