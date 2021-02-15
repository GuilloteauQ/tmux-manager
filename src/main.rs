use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Window {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Session {
    name: String,
    windows: Vec<Window>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    sessions: Vec<Session>,
}

fn main() {
    let filename = "test.yml";
    let config =
        std::fs::read_to_string(filename).expect("Something went wrong reading the config file");

    let sessions: Config = serde_yaml::from_str(&config).expect("oops");
    println!("{:?}", sessions);

    println!("Hello, world!");
}
