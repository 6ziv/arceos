
// use std::{collections::{BTreeMap, BTreeSet}, process::Command, fs};

// use crate::utils::find_arceos_crate;

// pub fn get_deps_by_crate_name(
//     crate_path: String,
//     req_features:BTreeSet<String>,
//     use_default_features:bool,
// ) -> String {
//     let binding = fs::canonicalize(&crate_path).unwrap();
//     let abs_path = binding.to_str().unwrap();

//     let mut cmdline = String::from("cd ") + abs_path + " && " + "cargo tree -e normal --prefix depth --format {lib}";
//     if !use_default_features{
//         cmdline += " --no-default-features";
//     }
//     cmdline += " --features ";
//     cmdline += req_features.into_iter().fold(String::new(), |s1,s2|{s1+" "+s2.as_str()}).as_str();

//     let cmd1 = cmdline.as_str();
//     let cmds = ["-c", cmd1];
//     let output = if cfg!(target_os = "windows") {
//         Command::new("cmd")
//                 .args(cmds)
//                 .output()
//                 .expect("failed to execute process")
//     } else {
//         Command::new("sh")
//                 .args(cmds)
//                 .output()
//                 .expect("failed to execute process")
//     };

//     let hello = output.stdout;
//     String::from_utf8(hello).unwrap()
// }

// fn parse_deps(deps: &String) -> Vec<(i32, String)> {
//     let mut rst = vec!();
//     for line in deps.lines() {
//         let level_name = line.split_whitespace().next().unwrap();
//         let level = level_name.get(0..1).unwrap().parse().unwrap();
//         let name = level_name.get(1..).unwrap();
//         rst.push((level, name.to_string()));
//     }
//     rst
// }

// pub fn generate_deps_path(cfg: &Config, parsed_crates: &mut Vec<String>, result: &mut String) {
//     let deps = get_deps_by_crate_name(cfg);
//     let deps_parsed = parse_deps(&deps);
//     let dep_root = &deps_parsed[0].1;
//     let root_level = deps_parsed[0].0;
//     for (level, crate_name) in deps_parsed.iter().skip(1) {
//         if !is_arceos_crate(&crate_name) {
//             continue;
//         } else {
//             if *level != root_level + 1 {
//                 continue;
//             }
//             // println!("{}-->{}", dep_root, crate_name);
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
// }

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