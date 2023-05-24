use std::collections::{VecDeque, BTreeSet, BTreeMap};
use crate::utils::find_arceos_crate;
use cargo_toml::Manifest;

fn parse_cargo_toml_and_append(
    name: String,
    req_features:BTreeSet<String>, 
    use_default_features:bool,
    result: &mut BTreeMap<String,BTreeSet<String>>,

    visited: &mut BTreeSet<(String,BTreeSet<String>,bool)>, // stop if called with exactly the same parameters.
)
{
    let check_dup = (name.clone(),req_features.clone(),use_default_features);
    if !visited.insert(check_dup){
        return;
    }

    let pathbuf = find_arceos_crate(&name);
    let path = pathbuf.unwrap_or_else(||{
        panic!("Cannot find crate or module {}",name);
    });
    let toml_path = path.join("Cargo.toml");
    let toml = Manifest::from_path(toml_path).unwrap();

    let mut enabled_dependencies = BTreeMap::<String,(BTreeSet<String>,bool)>::new();
    let mut optional_dependencies = BTreeSet::<String>::new();
    
    let deps = toml.dependencies;
    deps.into_iter().for_each(|(name,dependency)|{
        if let Some(_) = find_arceos_crate(&name){
            if dependency.optional(){
                optional_dependencies.insert(name);
            }else{
                let default_features = dependency.detail().unwrap().default_features;

                let mut features_requested_dep = BTreeSet::<String>::new();
                dependency.req_features().into_iter().for_each(|feature|{
                    features_requested_dep.insert(feature.to_owned());
                });

                enabled_dependencies.insert(name,(features_requested_dep,default_features));
            }
        }
    });

    
    let mut features = BTreeMap::<String,Vec<String>>::new();
    let mut default_features = Vec::<String>::new();
    toml.features.into_iter().for_each(|(k,mut v)|{
        if k=="default"{
            default_features.append(&mut v);
        }else{
            features.insert(k, v);
        }
    });
    let mut enabled_features_queue = VecDeque::<String>::new();
    let mut enabled_features = BTreeSet::<String>::new();
    
    req_features.into_iter().for_each(|feature|{
        enabled_features_queue.push_back(feature);
    });
    if use_default_features{
        default_features.into_iter().for_each(|feature|{
            enabled_features_queue.push_back(feature);
        });
    }
    while !enabled_features_queue.is_empty(){
        let feature = enabled_features_queue.pop_front().unwrap();
        if let Some((dependency,feature)) = feature.split_once('/'){
            if let Some(dep_features) = enabled_dependencies.get_mut(dependency){
                (*dep_features).0.insert(feature.to_string());
            }else if optional_dependencies.contains(dependency){
                let mut new_features = BTreeSet::<String>::new();
                new_features.insert(feature.to_string());
            }else{
                panic!("Unknown dependency {}",dependency.to_string());
            }
        }else{
            if enabled_features.insert(feature.clone()){
                enabled_features_queue.push_back(feature);
            }
        }
    }

    let mut dependencies_for_this = BTreeSet::<String>::new();
    enabled_dependencies.into_iter().for_each(|(depname,(dep_features,dep_use_default))|{
        dependencies_for_this.insert(depname.clone());
        parse_cargo_toml_and_append(depname,dep_features,dep_use_default,result,visited);
    });
    result.insert(name, dependencies_for_this);
}
pub fn parse_deps_from_toml(
    name: String,
    req_features:BTreeSet<String>, 
    use_default_features:bool,
)->BTreeMap<String,BTreeSet<String>>
{
    let mut result = BTreeMap::<String,BTreeSet<String>>::new();
    let mut visited_checker = BTreeSet::<(String,BTreeSet<String>,bool)>::new();
    parse_cargo_toml_and_append(name, req_features, use_default_features, &mut result, &mut visited_checker);
    result
}