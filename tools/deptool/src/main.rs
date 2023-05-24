use deptool::{parse_and_generate,parse_args};
fn main() {
    let conf = parse_args();
    let rst = parse_and_generate(&conf).unwrap();
    println!("{}", rst);
}
