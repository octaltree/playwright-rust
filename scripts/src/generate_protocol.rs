#[macro_use]
extern crate serde;

fn main() {
    let protocol: Protocol = serde_yaml::from_reader(std::io::stdin()).unwrap();
    let t = "";
    println!("{}\n// vim: foldnestmax=0 ft=rust", t);
}

#[derive(Debug, Deserialize)]
struct Protocol(Vec<Interface>);

#[derive(Debug, Deserialize)]
struct Interface {}
