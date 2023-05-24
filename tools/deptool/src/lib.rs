#[macro_use]
extern crate lazy_static;

use std::collections::{BTreeSet, BTreeMap};
mod parse_from_toml;
mod mermaid_generator;
mod config;
mod utils;
use config::{GeneratorType, ParserType};
pub use config::{Config, parse_args};
use mermaid_generator::generate_mermaid;
use parse_from_toml::parse_deps_from_toml;

fn parse_deps(config: &Config)->Result<BTreeMap<String,BTreeSet<String>>,String>{
    match config.parser{
        ParserType::ParseFromToml => 
            Ok(parse_deps_from_toml(config.crate_name.clone(),BTreeSet::<String>::new(),true)),
        ParserType::ParseFromCargoTree =>
            Err("Parse from cargo tree not implemented yet".to_string()),
        // _ =>
        //     Err("Unknown parser".to_string()),
    }
}
fn generate_graph(config: &Config, links: BTreeMap<String,BTreeSet<String>>)->Result<String,String>{
    match config.generator{
        GeneratorType::GenerateMermaid => Ok(generate_mermaid(links)),
        // _ => Err("Unknown generator".to_string()),
    }
    
}

pub fn parse_and_generate(config:&Config)->Result<String, String>{
    let parsed = parse_deps(config)?;
    generate_graph(config, parsed)
}