use alsa::seq;
use std::ffi::CString;

pub struct SeqClient {
    client: seq::Seq,
    pub ins: u8,
    pub outs: u8,
}

impl SeqClient {
    pub fn new(ins: u8, outs: u8) -> SeqClient {    
        let s = alsa::Seq::open(None, None, false).expect("Failed to open sequencer");
        let seqname = CString::new("miditables").unwrap();
        s.set_client_name(&seqname).expect("Failed to set client name");
        for i in 1..ins+1 {     
            let mut inport = seq::PortInfo::empty().unwrap();
            inport.set_capability(seq::PortCap::WRITE | seq::PortCap::SUBS_WRITE);
            inport.set_type(seq::PortType::MIDI_GENERIC | seq::PortType::APPLICATION);
            inport.set_name(&CString::new(format!("input {}", i)).unwrap());
            s.create_port(&inport)
                .expect("failed to create port");
        }
        for i in 1..outs+1 {
            let mut outport = seq::PortInfo::empty().unwrap();
            outport.set_capability(seq::PortCap::READ | seq::PortCap::SUBS_READ);
            outport.set_type(seq::PortType::MIDI_GENERIC | seq::PortType::APPLICATION);
            outport.set_name(&CString::new(format!("output {}", i)).unwrap());
            s.create_port(&outport)
                .expect("failed to create port");
        }
        SeqClient {
            client: s,
            ins,
            outs,
        }
    }
    pub fn get_input<'a>(&'a self) -> seq::Input<'a> {
        self.client.input()
    }

}
