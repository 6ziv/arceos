
use std::{collections::{BTreeMap, BTreeSet}, process::Command, path::Path, env::{set_current_dir, current_dir}, };

use crate::utils::find_arceos_crate;

pub fn get_deps_by_crate_name(
    crate_path: &Path,
    req_features:BTreeSet<String>,
    use_default_features:bool,
) -> String {
    let working_dir = current_dir().unwrap();
    if set_current_dir(crate_path).is_err(){panic!("cannot change working directory to {}",crate_path.display());};

    // let binding = fs::canonicalize(&crate_path).unwrap();
    // let abs_path = binding.to_str().unwrap();

    // let mut cmdline = String::from("cd ") + abs_path + " && " + "cargo tree -e normal --prefix depth --format {lib}";
    let mut cmdline = String::from("cargo tree -e normal --prefix depth --format {lib} --no-dedupe");
    if !use_default_features{
        cmdline += " --no-default-features";
    }
    if !req_features.is_empty(){
        cmdline += " --features ";
        cmdline += req_features.into_iter().fold(String::new(), |s1,s2|{s1+","+s2.as_str()}).as_str();
    }
    let cmd1 = cmdline.as_str();
    let cmds = ["-c", cmd1];
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(cmds)
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .args(cmds)
                .output()
                .expect("failed to execute process")
    };
    
    if set_current_dir(working_dir).is_err(){/*pass*/}

    let hello = output.stdout;
    String::from_utf8(hello).unwrap()

}

fn parse_deps(deps: &String) -> Vec<(i32, String)> {
    let mut rst = vec!();
    for line in deps.lines() {
        let level_name = line.split_whitespace().next().unwrap();
        let level = level_name.get(0..1).unwrap().parse().unwrap();
        let name = level_name.get(1..).unwrap();
        rst.push((level, name.to_string()));
    }
    rst
}

pub fn parse_deps_from_cargo_tree(
    name: String,
    app_path: &Path,
    req_features:BTreeSet<String>, 
    use_default_features:bool,
) -> BTreeMap<String,BTreeSet<String>>
{
    let deps = get_deps_by_crate_name(app_path,req_features,use_default_features);
    let deps_parsed = parse_deps(&deps);
    let mut ret = BTreeMap::<String,BTreeSet<String>>::new();
    let mut stk = Vec::<String>::new();
    stk.push(name);

    for (level, crate_name) in deps_parsed.iter().skip(1){
        if find_arceos_crate(&crate_name).is_none(){
            continue;
        } else {
            stk.resize((*level)as usize, String::new());
            if let Some(element) = stk.last(){
                if !ret.contains_key(element){
                    ret.insert(element.clone(), BTreeSet::<String>::new());
                }
                ret.get_mut(element).unwrap().insert(crate_name.clone());
            }
            stk.push(crate_name.to_owned());
        }
    }
    ret
}

// pub fn parse_deps_from_cargo_tree(
//     name: String,
//     req_features:BTreeSet<String>, 
//     use_default_features:bool,
// )->BTreeMap<String,BTreeSet<String>>
// {

//     if find_arceos_crate(&name).is_none(){
//         panic!("{} is not an arceos crate/module.",name);
//     }
//     let mut ret = BTreeMap::<String,BTreeSet<String>>::new()

//     let deps = get_deps_by_crate_name(name, req_features, use_default_features);
//     let deps_parsed = parse_deps(&deps);
//     let dep_root = &deps_parsed[0].1;
//     let root_level = deps_parsed[0].0;
//     for (level, crate_name) in deps_parsed.iter().skip(1) {
//         if find_arceos_crate(&crate_name).is_none() {
//             continue;
//         } else {
//             if *level != root_level + 1 {
//                 continue;
//             }
//             // println!("{}-->{}", dep_root, crate_name);
//             if !ret.contains_key(dep_root){

//             }
//             *result += &format!("{}-->{}\n", dep_root, crate_name);
//             if parsed_crates.contains(&crate_name) {
//                 continue;
//             }
//             parsed_crates.push(crate_name.clone());
//             let loc;
//             if check_crate_name(&crate_name) {
//                 loc = CRATE_ROOT;
//             } else {
//                 loc = MODULE_ROOT;
//             }
//             let new_cfg = Config {crate_name: (*crate_name).clone(), loc};
//             generate_deps_path(&new_cfg, parsed_crates, result);
//         }
//     }



//     let mut result = BTreeMap::<String,BTreeSet<String>>::new();
//     let mut visited_checker = BTreeSet::<(String,BTreeSet<String>,bool)>::new();
//     parse_cargo_toml_and_append(name, req_features, use_default_features, &mut result, &mut visited_checker);
//     result
// }