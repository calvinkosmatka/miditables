extern crate miditables;
extern crate alsa;

//use alsa::seq;
//use std::env;
use std::thread;
use std::sync::{Arc,Mutex, mpsc::channel};
use miditables::config::Config;

fn main() {
    let mut c = Config::new("conf/miditables.conf");
    let (table, mut input, output) = c.parse();
    let table = Arc::new(table);
    //let client = Mutex::new(SeqClient::new(c.ins, c.outs));
    //let mut sinput = client.get_input();
    //let thread_client = &client;
    //thread::spawn(move || {
    //    let data = alsa::seq::EvNote::default();
    //    let mut x = alsa::seq::Event::new(alsa::seq::EventType::Noteon, &data);
    //    thread_client.output_event(&mut x);
    //});

    let (tx, rx) = channel();

    thread::spawn(move || {
        //output.check_has_input();
        loop {
            println!("{:?}",rx.recv());

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
