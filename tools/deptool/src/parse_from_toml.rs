use std::{collections::{VecDeque, BTreeSet, BTreeMap}, path::Path, fmt};
use crate::utils::find_arceos_crate;
use cargo_toml::Manifest;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Node
{
    Component(String),
    Feature(String,String),
    Condition((String,String),String),
}
impl Node{
    fn get_component(&self)->String{
        match self{
            Node::Component(v) => v.to_string(),
            Node::Feature(v,_)=>v.to_string(),
            Node::Condition((v,_),_)=>v.to_string(),
        }
    }
}
struct Graph{
    pub content: BTreeMap<Node,BTreeSet<Node>>,
    pub conditions: BTreeMap<Node, usize>,
}
impl Graph{
    fn new()->Self{
        Self { 
            content: BTreeMap::<Node,BTreeSet<Node>>::new(),
            conditions: BTreeMap::<Node, usize>::new(),
        }
    }
    fn make_link(&mut self, lhs: Node, rhs: Node){
        if !self.content.contains_key(&lhs){
            self.content.insert(lhs.clone(), BTreeSet::<Node>::new());
        }
        self.content.get_mut(&lhs).unwrap().insert(rhs);
    }
    fn insert_condition(&mut self, cond1: Node, cond2: Node)->Node{
        let Node::Feature(cur,feat) = cond1.clone() else {panic!("unexpected node type!");};
        let Node::Component(dep) = cond2.clone() else {panic!("unexpected node type!");};
        let cond = Node::Condition((cur,feat), dep);
        self.make_link(cond1.clone(), cond.clone());
        self.make_link(cond2.clone(), cond.clone());
        self.conditions.insert(cond.clone(), 2);
        cond
    }
}
fn parse_cargo_toml_and_append(
    name: String,
    path: &Path, //override crate/module lookup. so we can support looking into apps.
    
    graph: &mut Graph,
    visited: &mut BTreeSet<String>,
)
{
    if !visited.insert(name.clone()){
        return;
    }

    let toml_path = path.join("Cargo.toml");
    let toml = Manifest::from_path(toml_path).unwrap();

    let mut dependencies = BTreeMap::<String,bool>::new();
    let mut weakname = BTreeSet::<String>::new();
    let deps = toml.dependencies;
    deps.into_iter().for_each(|(depname,dependency)|{
        if let Some(p) = find_arceos_crate(&depname){
            parse_cargo_toml_and_append(depname.clone(),p.as_path(),graph,visited);

            weakname.insert(depname.clone());
            let mut use_default = true;
            if !dependency.optional(){
                graph.make_link(Node::Component(name.clone()), Node::Component(depname.clone()));
                if let Some(detail) = dependency.detail(){
                    if !detail.default_features{
                        use_default = false;
                        //dependencies.insert(depname.clone(), dependency.detail().def);
                    }
                }
                dependency.req_features().iter().for_each(|req_feature|{
                    graph.make_link(Node::Component(name.clone()), Node::Feature(depname.clone(), req_feature.clone()));
                });
            }

            dependencies.insert(depname.clone(), use_default);
        }
    });

    
    //let mut features = BTreeMap::<String,Vec<String>>::new();
    //let mut default_features = Vec::<String>::new();
    toml.features.iter().for_each(|(feature_name,v)|{
        let current_feature = Node::Feature(name.clone(), feature_name.clone());
        graph.make_link(current_feature.clone(), Node::Component(name.clone()));

        v.into_iter().for_each(|feature_str|{
            if feature_str.starts_with("dep:"){
                let (_, dep_name) = feature_str.split_at(4);
                weakname.remove(dep_name);
                if find_arceos_crate(&(dep_name.to_string())).is_some(){
                    if !dependencies.contains_key(dep_name){
                        panic!("Unknown dependency {} when parsing feature {} in {}", dep_name,feature_name,name);
                    }
                    //println!("CONNECT DEP: {} -> {}",current_feature,Node::Component(dep_name.to_string()));
                    graph.make_link(current_feature.clone(), Node::Component(dep_name.to_string()));
                    if *dependencies.get(dep_name).unwrap(){
                        graph.make_link(current_feature.clone(), Node::Feature(dep_name.to_string(),"default".to_string()));
                    }
                }
            }else if let Some((dep_name,dep_feat)) = feature_str.split_once('/'){
                let only_when_enabled = dep_name.ends_with('?');
                let dependency_name = if only_when_enabled{
                    dep_name.trim_end_matches('?')
                }else{
                    dep_name
                }.to_string();
                if find_arceos_crate(&dependency_name).is_some(){
                    if !dependencies.contains_key(&dependency_name){
                        panic!("Unknown dependency {} when parsing feature {} in {}", dependency_name, feature_name,name);
                    }

                    let dependency_feature = Node::Feature(dependency_name.clone(), dep_feat.to_string());
                    if only_when_enabled{
                        let dependency_component = Node::Component(dependency_name.clone());
                        let cond = graph.insert_condition(current_feature.clone(), dependency_component);
                        graph.make_link(cond, dependency_feature);
                    }else{
                        graph.make_link(current_feature.clone(), dependency_feature);
                    }
                }
            }else if toml.features.contains_key(feature_str) || weakname.contains(feature_str){
                let linked_feature = Node::Feature(name.clone(), feature_str.to_owned());
                graph.make_link(current_feature.clone(), linked_feature);
            }else{
                //ignore it: may be build dependencies.

                //panic!("unknown feature {} / {}",name,feature_str);
            }
        });
    });

    weakname.iter().for_each(|dep_name|{
        graph.make_link(Node::Feature(name.clone(), dep_name.to_owned()), Node::Component(dep_name.to_owned()));
        if *dependencies.get(dep_name).unwrap(){
            graph.make_link(Node::Feature(name.clone(), dep_name.to_owned()), Node::Feature(dep_name.to_string(),"default".to_string()));
        }
    })

}

impl fmt::Display for Node{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.clone(){
            Node::Component(str)=>write!(f,"Component({})",str),
            Node::Feature(c,fea )=>write!(f,"Feature({}/{})",c,fea),
            Node::Condition((c1,fea), c2)=>write!(f,"Feature({c1}/{fea}) + Component({c2})"),
        }
        
    }
}

pub fn parse_deps_from_toml(
    name: String,
    app_path: &Path,
    req_features:BTreeSet<String>, 
    use_default_features:bool,
)->BTreeMap<String,BTreeSet<String>>
{
    let mut result = BTreeMap::<String,BTreeSet<String>>::new();
    let mut queue= VecDeque::<Node>::new();
    let mut taken= BTreeSet::<Node>::new();
    let mut visited = BTreeSet::<String>::new();
    let mut graph = Graph::new();
    parse_cargo_toml_and_append(name.clone(), app_path, &mut graph, &mut visited);

    graph.content.iter().for_each(|(node,nodes)|{
        print!("{node}: [");
        nodes.iter().for_each(|v|{print!("{v}, ")});
        println!("]");
    });

    
    queue.push_back(Node::Component(name.clone()));
    req_features.into_iter().for_each(|feature|{
        let feat = Node::Feature(name.clone(), feature.clone());
        if taken.insert(feat.clone()){
            queue.push_back(feat.clone());
        };
    });
    if use_default_features{
        let feat = Node::Feature(name.clone(), "default".to_string());
        if taken.insert(feat.clone()){
            queue.push_back(feat.clone());
        };
    }

    while !queue.is_empty(){
        let current_node = queue.pop_front().unwrap();
        if let Some(neighbor_nodes) = graph.content.get(&current_node){
            neighbor_nodes.into_iter().for_each(|nnode|{
                if let Node::Condition(..) = nnode{
                    if let Some(v) = graph.conditions.get_mut(nnode){
                        (*v) -= 1;
                        if *v == 0usize{
                            if taken.insert(nnode.clone()){
                                queue.push_back(nnode.clone());
                            };
                        }
                    }
                }else{
                    if taken.insert(nnode.clone()){
                        queue.push_back(nnode.clone());
                    };
                    if current_node.get_component() != nnode.get_component(){
                        let cur_com = current_node.get_component();
                        let dep_com = nnode.get_component();
                        if !result.contains_key(&cur_com){
                            result.insert(cur_com.clone(), BTreeSet::<String>::new());
                        }
                        result.get_mut(&cur_com).unwrap().insert(dep_com);
                    }
                }
            });
        }
    }
    result
}