use std::rc::{Rc};
use std::cell::RefCell;
use try_signalling::agents::agent_a;
use try_signalling::signals::signal_1::Channel1;
use try_signalling::signals::signal_2::Channel2;
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let x = agent_a::Agent::new(0, 0);
    let y = agent_a::Agent::new(10, 0);
    let z = agent_a::Agent::new(100, 0);
    let cn1 = Channel1::new(Rc::clone(&x), Rc::clone(&z));
    let cn1 = Channel1::new(Rc::clone(&y), Rc::clone(&z));
    
    let (tx_report_x, rx_report_x) = mpsc::channel();
    let (tx_report_y, rx_report_y) = mpsc::channel();
    let (tx_report_z, rx_report_z) = mpsc::channel();
    let (tx_confirm_x, rx_confirm_x) = mpsc::channel();
    let (tx_confirm_y, rx_confirm_y) = mpsc::channel();
    let (tx_confirm_z, rx_confirm_z) = mpsc::channel();

    let synchronizer = thread::spawn(move || {
        let mut counter = 0;
        loop {
            if counter < 10 {
                tx_confirm_x.send(Some(counter)).unwrap();
                tx_confirm_y.send(Some(counter)).unwrap();
                tx_confirm_z.send(Some(counter)).unwrap();
            } else {
                tx_confirm_x.send(None).unwrap();
                tx_confirm_y.send(None).unwrap();
                tx_confirm_z.send(None).unwrap();
            }

            rx_report_x.recv(None).unwrap();
            rx_report_y.recv(None).unwrap();
            rx_report_z.recv(None).unwrap();
        }
    });
    
    let run x = thread::spawn(move || {
        if let cf_r = rx_confirm_x.recv().unwrap().expext() == None {
            
        }
        let cf_r = rx_confirm_x.recv().unwrap().expext();
        if cf_r == 2 || cf_r == 3 || cf_r == 4 {
            x.borrow_mut().send_count();
        }
        x.borrow_mut().evolve();
        tx_report_x.send(True).unwrap();
    });
        
    let run y = thread::spawn(move || {
        let cf_r = rx_confirm_y.recv().unwrap().expext();
        if cf_r == 2 || cf_r == 3 || cf_r == 4 {
            y.borrow_mut().send_count();
        }
        y.borrow_mut().evolve();
        tx_report_y.send(True).unwrap();
    });

    let run z = thread::spawn(move || {
        rx_confirm_x.recv().unwrap();
        x.borrow_mut().evolve();
        tx_report_x.send(True).unwrap();
    });
    
    for i in (0..7) {
        if i == 2 || i == 3 || i == 4 {
            x.borrow_mut().send_count();
            y.borrow_mut().send_count();            
        }
        x.borrow_mut().evolve();
        y.borrow_mut().evolve();
        z.borrow_mut().evolve();
    }
    println!("{:?}", z.borrow().show_1());
}
