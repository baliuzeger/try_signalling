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
    pp_a1.lock().unwrap().add_agent(AAgent::new(100, 10, None));
    // pp_a1.lock().unwrap().agent_by_id(0).lock().unwrap().print_values(); // confirm agent created
    
    sp0.add_passive_connection(Connection1::new(
        pp_a1.lock().unwrap().agent_by_id(0),
        pp_a1.lock().unwrap().agent_by_id(2),
        1
    ));

    sp0.add_passive_connection(Connection1::new(
        pp_a1.lock().unwrap().agent_by_id(1),
        pp_a1.lock().unwrap().agent_by_id(2),
        1
    ));

    sp0.run(20);
    
    // let (tx_report_x, rx_report_x) = mpsc::channel();
    // let (tx_report_y, rx_report_y) = mpsc::channel();
    // let (tx_report_z, rx_report_z) = mpsc::channel();
    // let (tx_confirm_x, rx_confirm_x) = mpsc::channel();
    // let (tx_confirm_y, rx_confirm_y) = mpsc::channel();
    // let (tx_confirm_z, rx_confirm_z) = mpsc::channel();

    // let synchronizer = thread::spawn(move || {
    //     let mut counter = 0;
    //     loop {
    //         if counter < 10 {
    //             tx_confirm_x.send(Some(counter)).unwrap();
    //             tx_confirm_y.send(Some(counter)).unwrap();
    //             tx_confirm_z.send(Some(counter)).unwrap();
    // //            println!("syn: {}.", counter);
    //             counter += 1;
    //         } else {
    //             tx_confirm_x.send(None).unwrap();
    //             tx_confirm_y.send(None).unwrap();
    //             tx_confirm_z.send(None).unwrap();
    //             break;
    //         }
    //         rx_report_x.recv().unwrap();
    //         rx_report_y.recv().unwrap();
    //         rx_report_z.recv().unwrap();
    //     }
    // });
    
    // let run_x = thread::spawn(move || {
    //     loop {
    //         let cf_r = rx_confirm_x.recv().unwrap();
    //         if let None = cf_r {
    //             break;
    //         } else {
    //             let cf_r = cf_r.expect("got no signal.");
    //             if cf_r == 2 || cf_r == 3 || cf_r == 4 {
    //                 x.lock().unwrap().send_count();
    //             }
    //             x.lock().unwrap().evolve();
    //             tx_report_x.send(true).unwrap();
    //         }
    //     }
    // });

    // let run_y = thread::spawn(move || {
    //     loop {
    //         let cf_r = rx_confirm_y.recv().unwrap();
    //         if let None = cf_r {
    //             break;
    //         } else {
    //             let cf_r = cf_r.expect("got no signal.");
    //             if cf_r == 2 || cf_r == 3 || cf_r == 4 {
    //                 y.lock().unwrap().send_count();
    //             }
    //             y.lock().unwrap().evolve();
    //             tx_report_y.send(true).unwrap();
    //         }
    //     }
    // });

    // let z1 = Arc::clone(&z);
    //     let run_z = thread::spawn(move || {
    //     loop {
    //         let cf_r = rx_confirm_z.recv().unwrap();
    //         if let None = cf_r {
    //             break;
    //         } else {
    //             z1.lock().unwrap().evolve();
    //             tx_report_z.send(true).unwrap();
    //         }
    //     }
    // });

    // synchronizer.join().expect("thread syn panicked.");
    // run_x.join().expect("thread x panicked.");
    // run_y.join().expect("thread y panicked.");
    // run_z.join().expect("thread z panicked.");
    // println!("{:?}", z.lock().unwrap().show_1());
}
