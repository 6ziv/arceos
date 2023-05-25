use std::collections::{BTreeSet, BTreeMap};
pub fn generate_d2(links: BTreeMap<String,BTreeSet<String>>) -> String {
    links.iter().fold(String::new(), 
        |prev, (name,dep)|{
            prev + dep.into_iter().fold(String::new(), |old_str,cur|{
                old_str + format!("{} -> {}\n", name, cur).as_str()
            }).as_str()
        }
    )
}