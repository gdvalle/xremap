pub mod action;
mod actions;
pub mod application;
mod key;
pub mod key_action;
pub mod key_press;
mod keymap;
mod modmap;

extern crate serde_yaml;

use keymap::Keymap;
use modmap::Modmap;
use serde::Deserialize;
use std::{error, fs};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default = "Vec::new")]
    pub modmap: Vec<Modmap>,
    #[serde(default = "Vec::new")]
    pub keymap: Vec<Keymap>,
}

pub fn load_config(filename: &str) -> Result<Config, Box<dyn error::Error>> {
    let yaml = fs::read_to_string(&filename)?;
    let config: Config = serde_yaml::from_str(&yaml)?;
    return Ok(config);
}
