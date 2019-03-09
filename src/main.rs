// use std::rc::{Rc};
// use std::cell::RefCell;
use try_signalling::agents::agent_a;
use try_signalling::signals::signal_1::Connection1;
// use try_signalling::signals::signal_2::Channel2;
use std::thread;
use std::sync::mpsc;
// use std::time::Duration;
// extern crate crossbeam_channel;
// use std::time::Duration;
use std::sync::{Mutex, Arc, Weak};

fn main() {
    // let x = agent_a::Agent::new(0, 0);
    // let y = agent_a::Agent::new(10, 0);
    // let z = agent_a::Agent::new(100, 0);
    // let cn_xz = Connection1::new(Arc::clone(&x), Arc::clone(&z));
    // let cn_yz = Connection1::new(Arc::clone(&y), Arc::clone(&z));
    
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
