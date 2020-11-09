//use std::collections::HashMap;
use super::chain::Chain;
use super::client::{InputClient, OutputClient};
use super::matchers::get_matcher;
use super::rules::{Jump, Rule};
use super::tables::Table;
use super::transformers::get_transformer;
use crate::matcher::MatcherType::{self, *};
use crate::plugin::{PluginGetMatcherFn, PluginGetTransformerFn};
use crate::transformer::TransformerType::{self, *};
use crate::{Matcher, Transformer};
use libloading::{Library, Symbol};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::sync::Mutex;

fn validate_plugin(lib: &Library) {
    // use built or rustc_version to validate compiler version
    // used to build plugin and miditables version
    // panic if it doesn't match
}

pub struct Config {
    pub conf_file: String,
    loaded_libs: HashMap<String, Library>,
}

impl Config {
    pub fn new(file: &str) -> Config {
        Config {
            conf_file: String::from(file),
            loaded_libs: HashMap::new(),
        }
    }
    fn get_lib_matcher(&self, lib_ns: &str, matcher_name: &str) -> Box<dyn Matcher> {
        //panic if requested matcher does not exist
        let lib = self.loaded_libs.get(lib_ns).expect("library not found");
        let matcher_fn: Symbol<PluginGetMatcherFn> = unsafe {
            lib.get(b"get_matcher")
                .expect("plugin does not declare any matchers")
        };
        matcher_fn(matcher_name).expect("matcher not found in plugin")
    }
    fn get_lib_transformer(&self, lib_ns: &str, transformer_name: &str) -> Box<dyn Transformer> {
        //panic if requested transformer does not exist
        let lib = self.loaded_libs.get(lib_ns).expect("library not found");
        let transformer_fn: Symbol<PluginGetTransformerFn> = unsafe {
            lib.get(b"get_transformer")
                .expect("plugin does not declare any transformers")
        };
        transformer_fn(transformer_name).expect("transformer not found in plugin")
    }
    pub fn parse(&mut self) -> (Table, InputClient, OutputClient) {
        let mut ins = 0;
        let mut outs = 0;
        let mut table = Table::new();
        let conf_file = File::open(&self.conf_file).expect("Failed to open conf file");
        let conf_file = BufReader::new(conf_file);
        for line in conf_file.lines() {
            let line = line.unwrap();
            let mut split = line.split_whitespace().peekable();
            match split.next() {
                Some("#inputs:") => {
                    ins = u8::from_str(
                        split
                            .next()
                            .expect("#inputs requrires argument (none given)"),
                    )
                    .expect("not a u8");
                }
                Some("#outputs:") => {
                    outs = u8::from_str(
                        split
                            .next()
                            .expect("#outputs requires argument (none given)"),
                    )
                    .expect("not a u8");
                }
                Some("#plugin:") => {
                    let lib_path = split.next().expect("#plugin requires plugin (none given)");
                    let lib =
                        Library::new(&lib_path).expect(&format!("failed to load {}", lib_path));
                    let plug_ns = split
                        .next()
                        .expect("#plugin requires namespace (none given)");
                    self.loaded_libs.insert(plug_ns.to_string(), lib);
                }
                Some("-A") => {
                    let target_chain = match split.peek() {
                        Some(x) => {
                            if x.starts_with("-") {
                                "default"
                            } else {
                                split.next().unwrap()
                            }
                        }
                        None => panic!("empty rule"),
                    };
                    //println!("target_chain {}", target_chain);
                    let mut matchers: Vec<MatcherType> = Vec::new();
                    let mut transformers: Vec<TransformerType> = Vec::new();
                    let mut jump = Jump::Continue;
                    while let Some(chunk) = split.next() {
                        match chunk {
                            // will need to clean this up to enforce matcher* transformer* jump?
                            // format
                            "-m" | "-!m" => {
                                let matcher_name = split.next().expect("no matcher");
                                //println!("matcher name {}", matcher_name);
                                let mut match_args: Vec<String> = Vec::new();
                                loop {
                                    match split.peek() {
                                        Some(arg) => {
                                            if arg.starts_with("-") {
                                                break;
                                            } else {
                                                //println!("\targ: {}", arg);
                                                match_args.push(split.next().unwrap().to_string());
                                            }
                                        }
                                        None => break,
                                    }
                                }
                                let mut m = if matcher_name.contains(":") {
                                    let mut mspl = matcher_name.split(":");
                                    let ns = mspl.next().expect("something is wrong");
                                    let matcher_name = mspl.next().expect("need a matcher name");
                                    // let this stabilize first, ignre for now
                                    // mspl.next().expect_none("too many colons");
                                    self.get_lib_matcher(ns, matcher_name)
                                } else {
                                    get_matcher(matcher_name).unwrap()
                                };
                                m.parse_args(match_args);
                                if chunk == "-m" {
                                    matchers.push(LocalMatcher(Mutex::new(m)));
                                } else if chunk == "-!m" {
                                    matchers.push(LocalMatcherNeg(Mutex::new(m)));
                                }
                            }
                            "-t" => {
                                let transformer_name = split.next().expect("no transformer");
                                //println!("transformer name {}", transformer_name);
                                let mut transform_args: Vec<String> = Vec::new();
                                loop {
                                    match split.peek() {
                                        Some(arg) => {
                                            if arg.starts_with("-") {
                                                break;
                                            } else {
                                                //println!("\targ: {}", arg);
                                                transform_args
                                                    .push(split.next().unwrap().to_string());
                                            }
                                        }
                                        None => break,
                                    }
                                }
                                let mut t = if transformer_name.contains(":") {
                                    let mut tspl = transformer_name.split(":");
                                    let ns = tspl.next().expect("something is wrong");
                                    let transformer_name =
                                        tspl.next().expect("need a transformer name");
                                    // let this stabilize first, ignre for now
                                    // tspl.next().expect_none("too many colons");
                                    self.get_lib_transformer(ns, transformer_name)
                                } else {
                                    get_transformer(transformer_name).unwrap()
                                };
                                t.parse_args(transform_args);
                                transformers.push(LocalTransformer(Mutex::new(t)));
                            }
                            "-j" => {
                                let jump_name = split.next().expect("no jump");
                                jump = match jump_name.to_ascii_lowercase().as_str() {
                                    "end" => Jump::End,
                                    "continue" => Jump::Continue,
                                    x => Jump::Chain(x.to_string()),
                                };
                                //println!("jump {:?}", jump);
                            }
                            _ => println!("handler not implemented {}", chunk),
                        }
                    }
                    let rule = Rule::new(matchers, transformers, jump);
                    table
                        .get_chain_mut(target_chain)
                        .expect("could not find chain")
                        .rules
                        .push(rule);
                    //println!("{:?}",split.next());
                }
                Some("-N") => {
                    let chain_name = split.next().expect("no chain name");
                    table.add_chain(chain_name, Chain::new());
                    //println!("new chain {}", chain_name);
                }
                Some(x) => {
                    println!("Some({})", x);
                }
                None => println!("blank"),
            }
        }
        (table, InputClient::new(ins), OutputClient::new(outs))
    }
}
