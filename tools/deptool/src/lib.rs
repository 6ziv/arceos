#[macro_use]
extern crate lazy_static;

use std::{collections::{BTreeSet, BTreeMap}};
mod parse_from_toml;

mod utils;

mod config;
use config::{GeneratorType, ParserType};
pub use config::{Config, parse_args};

mod mermaid_generator;
use mermaid_generator::generate_mermaid;

mod d2_generator;
use d2_generator::generate_d2;

use parse_from_toml::parse_deps_from_toml;
//use parse_from_cargo_tree::parse_deps_from_cargo_tree;


fn parse_deps(config: &Config)->Result<BTreeMap<String,BTreeSet<String>>,String>{
    match config.parser{
        ParserType::ParseFromToml => 
            Ok(parse_deps_from_toml(config.crate_name.clone(), &config.path, config.features.clone(),!config.no_default)),
        ParserType::ParseFromCargoTree =>
            //Ok(parse_deps_from_cargo_tree(config.crate_name.clone(),BTreeSet::<String>::new(),true)),
            Err("Parse from cargo tree not implemented yet".to_string()),

        // _ =>
        //     Err("Unknown parser".to_string()),
    }
}
fn generate_graph(config: &Config, links: BTreeMap<String,BTreeSet<String>>)->Result<String,String>{
    match config.generator{
        GeneratorType::GenerateMermaid => Ok(generate_mermaid(links)),
        GeneratorType::GenerateD2 => Ok(generate_d2(links)),
        // _ => Err("Unknown generator".to_string()),
    }
    
}

pub fn parse_and_generate(config:&Config)->Result<String, String>{
    let parsed = parse_deps(config)?;
    generate_graph(config, parsed)
}