use std::{fs, path::{PathBuf, Path}, sync::RwLock};
lazy_static!{
    static ref CRATE_ROOT:RwLock<PathBuf> = RwLock::new(PathBuf::from("../../crates/"));
    static ref MODULE_ROOT:RwLock<PathBuf> = RwLock::new(PathBuf::from("../../modules/"));
}
pub fn check_crate_name(name: &String) -> bool {
    let crates = fs::read_dir(CRATE_ROOT.read().as_ref().unwrap().as_path()).unwrap();
    crates.into_iter().map(|p| p.unwrap().file_name()).any(|n| n.to_str().unwrap() == name)
}

pub fn check_module_name(name: &String) -> bool {
    let crates = fs::read_dir(MODULE_ROOT.read().as_ref().unwrap().as_path()).unwrap();
    crates.into_iter().map(|p| p.unwrap().file_name()).any(|n| n.to_str().unwrap() == name)
}
pub fn find_arceos_crate(name: &String)->Option<Box<PathBuf>>{
    if check_crate_name(name){
        Some(Box::new(CRATE_ROOT.read().as_ref().unwrap().as_path().join(name)))
    }else if check_module_name(name){
        Some(Box::new(MODULE_ROOT.read().as_ref().unwrap().as_path().join(name)))
    }else{
        None
    }
}

pub fn change_root(path: &String){
    *CRATE_ROOT.write().unwrap() = Path::new(path).join("crates");
    *MODULE_ROOT.write().unwrap() = Path::new(path).join("modules");
}