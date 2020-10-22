extern crate miditables;
extern crate alsa;

//use alsa::seq;
//use std::env;
use miditables::config::Config;
use miditables::client::SeqClient;

fn main() {
    let mut c = Config::new("conf/miditables.conf");
    println!("ins: {} outs: {}", c.ins, c.outs);
    let client = SeqClient::new(c.ins, c.outs);
    let mut sinput = client.get_input();

    loop {
        let ev = sinput.event_input();
        let mut ev = match ev {
            Ok(x) => x,
            _ => continue,
        };
        c.table.process(&mut ev, &client);
    }
}
