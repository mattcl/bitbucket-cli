#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate eprompt;
extern crate git2;
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

mod config;
mod error;
mod git;

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

    match git::repo_dir() {
        Ok(path) => println!("path: {}", path.to_str().unwrap_or("????")),
        Err(why) => println!("error!: {}", why)
    }

    match git::repo_name() {
        Ok(name) => println!("repo name: {}", name),
        Err(why) => println!("error!: {}", why)
    }

    match git::current_branch() {
        Ok(branch) => println!("branch: {}", branch),
        Err(why) => println!("error!: {}", why)
    }
}
