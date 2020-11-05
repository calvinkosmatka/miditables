extern crate miditables;
extern crate alsa;

//use alsa::seq;
//use std::env;
use std::thread;
use std::sync::{Arc,Mutex, mpsc::channel};
use miditables::config::Config;
use miditables::SendEventType::{self,*};

fn main() {
    let mut c = Config::new("conf/miditables.conf");
    let (table, mut input, output) = c.parse();
    let table = Arc::new(table);
    let (tx, rx) = channel();

    thread::spawn(move || {
        loop {
            let e = rx.recv().unwrap();
            match e {
                Output(mut e) => {
                    println!("{:?}", e);
                    output.output_event(&mut e);
                }
            }
        }
    });

    loop {
        let mut ev = input.get_event();
        let clone_table = Arc::clone(&table);
        let tx1 = tx.clone();
        thread::spawn(move || {
            clone_table.process(&mut ev, tx1);
        });
    }
}
