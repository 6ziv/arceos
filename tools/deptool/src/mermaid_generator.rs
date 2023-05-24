use std::collections::{BTreeSet, BTreeMap};
pub fn generate_mermaid(links: BTreeMap<String,BTreeSet<String>>) -> String {
    links.iter().fold("graph TD;\n".to_string(), 
        |prev, (name,dep)|{
            prev + dep.into_iter().fold(String::new(), |old_str,cur|{
                old_str + format!("{}-->{}\n", name, cur).as_str()
            }).as_str()
        }
    )
}