use std::collections::BTreeSet;
use argparse::{ArgumentParser,Store, StoreOption, StoreTrue};

use crate::utils::change_root;
#[derive(Debug, Clone, Copy)]
pub enum ParserType{
    ParseFromToml,
    ParseFromCargoTree,
}
#[derive(Debug, Clone, Copy)]
pub enum GeneratorType{
    GenerateMermaid,
}
pub struct Config {
    pub crate_name: String,
    pub features: BTreeSet<String>,
    pub no_default: bool,

    pub parser:ParserType,
    pub generator:GeneratorType,
}

pub fn parse_args()->Config{
    let mut ret = Config { 
        crate_name: String::new(),
        features: BTreeSet::<String>::new(), 
        no_default: false,
        parser: ParserType::ParseFromToml, 
        generator: GeneratorType::GenerateMermaid, 
    };

    let mut arceos_root:Option<String> = None;

    let mut feature_str :Option<String> = None;
    let mut parser = ArgumentParser::new();
    parser.refer(&mut ret.crate_name)
        .add_argument("name",Store,"crate or module name")
        .required()
        ;
    
    parser.refer(&mut feature_str)
        .add_option(&["--features"],StoreOption,"comma-separated list of features");

    parser.refer(&mut ret.no_default)
        .add_option(&["--no-default-features"],StoreTrue,"do not use default features");

    parser.refer(&mut arceos_root)
        .add_option(&["--root"], StoreOption, "root path of arceos (default: \"../../\")");
    // parser.refer(&mut ret.parser)
    //     .add_option(&["from-toml"], StoreConst(ParserType::ParseFromToml), "parse information from Cargo.toml");
    // parser.refer(&mut ret.parser)
    //     .add_option(&["from-cargo-tree"], StoreConst(ParserType::ParseFromCargoTree), "parse information from `cargo tree` output");
    // parser.refer(&mut ret.generator)
    //     .add_option(&["output-mermaid"], StoreConst(GeneratorType::GenerateMermaid), "output in mermaid format");
    
    parser.parse_args_or_exit();
    
    drop(parser);

    if let Some(root) = arceos_root{
        change_root(&root);
    }
    if let Some(features) = feature_str{
        features.split(',').into_iter().for_each(|feature|{
            ret.features.insert(feature.to_string());
        });
    }
    
    ret

}