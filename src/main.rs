extern crate alsa;
extern crate miditables;

//use alsa::seq;
//use std::env;
use miditables::config::Config;
use miditables::SendEventType::{self, *};
use std::sync::{mpsc::channel, Arc, Mutex};
use std::thread;

fn main() {
    let mut c = Config::new("conf/miditables.conf");
    let (table, mut input, output) = c.parse();
    let table = Arc::new(table);
    let (tx, rx) = channel();

    thread::spawn(move || loop {
        let e = rx.recv().unwrap();
        match e {
            Output(mut e) => {
                output.output_event(&mut e);
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
