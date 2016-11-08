#[macro_use]
extern crate clap;
extern crate eprompt;
extern crate hyper;
extern crate prettytable;
extern crate rpassword;
extern crate rustc_serialize;
extern crate url;
extern crate yaml_rust;

use clap::{App, Arg, ArgMatches};
use std::env;
use std::path::Path;

use eprompt::Prompt;

fn main() {
    let default_config_path = env::home_dir().unwrap().join(".bb.yml");
    let yml = load_yaml!("app.yml");
    let matches = App::from_yaml(yml)
        .arg(Arg::with_name("config")
            .help("sets the config file to use")
            .takes_value(true)
            .default_value(default_config_path.to_str().unwrap())
            .short("c")
            .long("config")
            .global(true))
        .get_matches();

    let config_file = matches.value_of("config").unwrap();
}
