use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::FromStr;
use super::tables::Table;
use super::matchers::{get_plugin_matchers,get_matcher};
use super::transformers::{get_plugin_transformers,get_transformer};
use super::rules::{Jump, Rule};
use super::chain::Chain;
use crate::{Transformer,Matcher};

pub struct Config {
    pub ins: u8,
    pub outs: u8,
    pub table: Table,
    pub conf_file: String
    //keep track of plugins
    //keep track of which matchers come from which plugins
    //keep track of which transformers come from which plugins
}

impl Config {
    pub fn new(file: &str) -> Config {
        let mut config = Config {
            ins: 0,
            outs: 0,
            table: Table::new(),
            conf_file: String::from(file),
        };
        config.parse_conf();
        config
    }
    fn parse_conf(&mut self) {
        let conf_file = File::open(&self.conf_file).expect("Failed to open conf file");
        let conf_file = BufReader::new(conf_file);
        for line in conf_file.lines() {
            let line = line.unwrap();
            if line.starts_with("#") {
                let line = &line[1..];
                if line.starts_with("inputs: ") {
                    self.ins = u8::from_str(line.trim_start_matches("inputs: ")).expect("not a u8");
                }
                if line.starts_with("outputs: ") {
                    self.outs = u8::from_str(line.trim_start_matches("outputs: ")).expect("not a u8");
                }
                /*if line.starts_with("plugin: ") {
                 * add a plugin
                 * }*/
            }
            else {
                let mut split = line.split_whitespace().peekable();
                match split.next() {
                    Some("-A") => {
                        let target_chain = match split.peek() {
                            Some(x) => {
                                if x.starts_with("-") {
                                    "default"
                                }
                                else {
                                    split.next().unwrap()
                                }
                            },
                            None => panic!("empty rule"),
                        };
                        println!("target_chain {}",target_chain);
                        let mut matchers: Vec<Box<dyn Matcher>> = Vec::new();
                        let mut transformers: Vec<Box<dyn Transformer>> = Vec::new();
                        let mut jump = Jump::Continue;
                        while let Some(chunk) = split.next() {
                            match chunk {
                                // will need to clean this up so that the format is matcher*
                                // transformer* jump?
                                "-m" => {
                                    let matcher_name = split.next()
                                        .expect("no matcher");
                                    println!("matcher name {}", matcher_name);
                                    let mut match_args: Vec<String> = Vec::new();
                                    loop {
                                        match split.peek() {
                                            Some(arg) => {
                                                if arg.starts_with("-") {
                                                    break;
                                                }
                                                else {
                                                    println!("\targ: {}",arg);
                                                    match_args.push(split.next().unwrap().to_string());
                                                }
                                            },
                                            None => break,
                                        }
                                    }
                                    let mut m = get_matcher(matcher_name)
                                        .unwrap();
                                    m.parse_args(match_args);
                                    matchers.push(m);
                                },
                                "-t" => {
                                    let transformer_name = split.next()
                                        .expect("no transformer");
                                    println!("transformer name {}", transformer_name);
                                    let mut transform_args: Vec<String> = Vec::new();
                                    loop {
                                        match split.peek() {
                                            Some(arg) => {
                                                if arg.starts_with("-") {
                                                    break;
                                                }
                                                else {
                                                    println!("\targ: {}",arg);
                                                    transform_args.push(split.next()
                                                                        .unwrap()
                                                                        .to_string());
                                                }
                                            },
                                            None => break,
                                        }
                                    }
                                    // find plugin holding transformer_name here
                                    let mut t = get_transformer(transformer_name)
                                        .unwrap();
                                    t.parse_args(transform_args);
                                    transformers.push(t);
                                },
                                "-j" => {
                                    let jump_name = split.next().expect("no jump");
                                    jump = match jump_name.to_ascii_lowercase().as_str() {
                                        "end" => Jump::End,
                                        "continue" => Jump::Continue,
                                        x => Jump::Chain(x.to_string()),
                                    };
                                    println!("jump {:?}", jump);
                                }
                                _ => println!("handler not implemented {}", chunk),
                            }
                        }
                        let mut rule = Rule::new(matchers, transformers, jump);
                        self.table.get_chain_mut(target_chain)
                            .expect("could not find chain")
                            .rules.push(rule);
                        //println!("{:?}",split.next());
                    },
                    Some("-N") => {
                        let chain_name = split.next().expect("no chain name");
                        self.table.add_chain(chain_name, Chain::new());
                        println!("new chain {}", chain_name);
                    },
                    Some(x) => {
                        println!("Some({})", x);
                    },
                    None => println!("blank"),
                }
            }
        }
    }
}
