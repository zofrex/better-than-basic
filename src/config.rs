extern crate toml;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

pub struct Config {
    pub listen: String,
    pub socket_mode: Option<u32>,
}

#[derive(Deserialize)]
struct ConfigFile {
    listen: Option<String>,
    socket_mode: Option<String>,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Config {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let config: ConfigFile = toml::from_str(&contents).unwrap();
        let socket_mode = config.socket_mode.map(|m| u32::from_str_radix(&m, 8).unwrap());
        Config {
            listen: config.listen.unwrap(),
            socket_mode: socket_mode,
        }
    }
}
