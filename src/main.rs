// use std::rc::{Rc};
// use std::cell::RefCell;
use try_signalling::agents::agent_a;
use try_signalling::signals::signal_1::Connection1;
use try_signalling::supervisor::{Supervisor};
// use try_signalling::signals::signal_2::Channel2;
// use std::thread;
// use std::time::Duration;
// extern crate crossbeam_channel;
// use std::time::Duration;
use std::sync::{Arc};

fn main() {

    let mut super_1 = Supervisor {
        agents: Vec::new(),
        passive_connections: Vec::new(),
    };

    super_1.add_agent(agent_a::Model::new(0, 0, Some(2)));
    super_1.add_agent(agent_a::Model::new(10, 0, Some(2)));
    super_1.add_agent(agent_a::Model::new(100, 0, None));
    let cn = Connection1::new(
        // Arc::clone(&super_1.agents[0]),
        // Arc::clone(&super_1.agents[2]),
        Arc::clone(&(super_1.agent_by_id(0))),
        Arc::clone(&(super_1.agent_by_id(2))),
        1
    );

    // this part is OK
    
    // let agnt_x = agent_a::Model::new(0, 0, Some(2));
    // let agnt_y = agent_a::Model::new(10, 0, Some(2));
    // let cn1 = Connection1::new(
    //     Arc::clone(&agnt_x),
    //     Arc::clone(&agnt_y),
    //     1
    // );

    
    // super_1.add_passive_connection(Connection1::new(
    //     Arc::clone(&super_1.agents[0]),
    //     Arc::clone(&super_1.agents[2]),
    //     1
    // ));
    // super_1.add_passive_connection(Connection1::new(
    //     Arc::clone(&super_1.agents[1]),
    //     Arc::clone(&super_1.agents[2]),
    //     2
    // ));
    
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
