pub mod config;
extern crate clap;
use crate::config::traits::ToTmux;
use crate::config::Config;
use clap::{App, Arg, SubCommand};
use dirs::home_dir;

fn tmux(config: Config, session_name: String) {
    let session = &config[&session_name];
    session.to_tmux(&session_name);
}

fn main() {
    let matches = App::new("tmux-manager")
        .version("1.0")
        .author("Quentin Guilloteau")
        .about("Recreate Tmux sessions")
        .subcommand(SubCommand::with_name("list").about("List session"))
        .subcommand(
            SubCommand::with_name("start")
                .about("Start a Tmux session")
                .arg(Arg::with_name("profil").help("Name of the profil")),
        )
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("CONF_FILE")
                .help("Path to the configuration file (default: ~/.tmux-manager.yml)"),
        )
        .get_matches();
    let default_config_file = format!(
        // "{}/.tmux_manager.yml",
        "{}/dev/tmux-manger/map.yml",
        home_dir().unwrap().to_str().unwrap()
    );
    let filename = matches.value_of("config").unwrap_or(&default_config_file);
    let config =
        std::fs::read_to_string(filename).expect("Something went wrong reading the config file");

    let sessions: Config = serde_yaml::from_str(&config).expect("Could not read config file");

    if let Some(_matches) = matches.subcommand_matches("list") {
        println!("Available Session Profiles:");
        sessions.iter().for_each(|(k, _v)| println!("\t- {}", k));
        return;
    }

    if let Some(matches) = matches.subcommand_matches("start") {
        if let Some(session_name) = matches.value_of("profil") {
            tmux(sessions, session_name.to_string());
        }
    }
}
