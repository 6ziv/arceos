use std::{collections::BTreeSet, path::PathBuf};
use argparse::{ArgumentParser,Store, StoreOption, StoreTrue};
use std::path::Path;
use crate::utils::{change_root, find_arceos_crate};
#[derive(Debug, Clone, Copy)]
pub enum ParserType{
    ParseFromToml,
    ParseFromCargoTree,
}
#[derive(Debug, Clone, Copy)]
pub enum GeneratorType{
    GenerateMermaid,
    GenerateD2,
}
pub struct Config {
    pub crate_name: String,
    pub path: Box<PathBuf>,
    pub features: BTreeSet<String>,
    pub no_default: bool,

    pub parser:ParserType,
    pub generator:GeneratorType,
}

pub fn parse_args()->Result<Config,String>{
    let mut ret = Config { 
        crate_name: String::new(),
        path: Box::new(PathBuf::new()),
        features: BTreeSet::<String>::new(), 
        no_default: false,
        parser: ParserType::ParseFromToml, 
        generator: GeneratorType::GenerateMermaid, 
    };

    let mut arceos_root:Option<String> = None;

    let mut feature_str :Option<String> = None;
    let mut path_string : Option<String> = None;
    let mut parser = ArgumentParser::new();
    parser.refer(&mut ret.crate_name)
        .add_argument("name",Store,"crate or module name")
        .required()
        ;

    parser.refer(&mut path_string)
        .add_option(&["--path"], StoreOption, "path containing the Cargo.toml to analyze");

    parser.refer(&mut feature_str)
        .add_option(&["--features"],StoreOption,"comma-separated list of features");

    parser.refer(&mut ret.no_default)
        .add_option(&["--no-default-features"],StoreTrue,"do not use default features");

    parser.refer(&mut arceos_root)
        .add_option(&["--root"], StoreOption, "root path of arceos (default: \"../../\")");

    let (mut parse_toml,mut parse_tree) = (false,false);
    parser.refer(&mut parse_toml)
         .add_option(&["--from-toml"], StoreTrue, "parse information from Cargo.toml");
    parser.refer(&mut parse_tree)
         .add_option(&["--from-cargo-tree"], StoreTrue, "parse information from `cargo tree` output");
    
    let mut generate_mermaid = false;
    parser.refer(&mut generate_mermaid)
        .add_option(&["--output-mermaid"], StoreTrue, "output in mermaid format");
    let mut generate_d2 = false;
    parser.refer(&mut generate_d2)
        .add_option(&["--output-d2"], StoreTrue, "output in D2 format");

    parser.parse_args_or_exit();
    
    drop(parser);
    if let Some(root) = arceos_root{
        change_root(&root);
    }


    if let Some(path_str) = path_string
    {
        if !Path::new(&path_str).join("Cargo.toml").exists(){
            return Err(format!("Given path {} does not exist", path_str));
        }else{
            ret.path = Box::new(Path::new(&path_str).to_path_buf());
        }
    }else{
        if let Some(path_str_2) = find_arceos_crate(&ret.crate_name){
            ret.path = path_str_2;
        }else{
            return Err(format!("{} is not an arceos crate or module", ret.crate_name));
        }
    }

    if parse_toml && parse_tree{
        return Err("multiple parsers given".to_string());
    }else if parse_tree{
        ret.parser = ParserType::ParseFromCargoTree;
    }else{
        ret.parser = ParserType::ParseFromToml;
    }
    
    if generate_mermaid && generate_d2{
        return Err("multiple generators given".to_string());
    }else if generate_d2{
        ret.generator = GeneratorType::GenerateD2;
    }else{
        ret.generator = GeneratorType::GenerateMermaid;
    }
    
    if let Some(features) = feature_str{
        features.split(',').into_iter().for_each(|feature|{
            ret.features.insert(feature.to_string());
        });
    }
    
    Ok(ret)

}