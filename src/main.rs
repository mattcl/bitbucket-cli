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
use std::io;
use std::io::Write;
use std::path::Path;

use eprompt::Prompt;
use rustc_serialize::base64::{ToBase64, STANDARD};

use client::Bitbucket;
use config::Config;
use error::{Result, UnwrapOrExit};

mod client;
mod config;
mod error;
mod git;
mod pull_request;

pub fn exit(message: &str) -> ! {
    let err = clap::Error::with_description(message, clap::ErrorKind::InvalidValue);
    err.exit();
}

fn prompt(label: &str) -> Result<String> {
    print!("{}", label);
    io::stdout().flush()?; // need to do this since print! won't flush
    let mut res = String::new();
    io::stdin().read_line(&mut res)?;
    Ok(res.trim().to_string())
}

fn setup(path: &Path) -> Result<()> {
    let server = prompt("bitbucket server url: ")?;
    let username = prompt("username: ")?;
    let password = rpassword::prompt_password_stdout("password: ")?;

    println!("
The project name should be the same name as the repo basename (directory).
This enables the auto-detection. If you'd rather specify the project for
a repo explicitly, create a .bitbucket-proj file containing the project
name as specified below");
    let project_name = prompt("primary project name: ")?;

    println!("
The source project is the project KEY or a tilde-prefixed username for
a personal project. This is the project the pull request will be made from.");
    let source_project = prompt("source project: ")?;

    println!("
The source slug is the repository under the source project from which the
pull request will be made.");
    let source_slug = prompt("source slug: ")?;

    println!("
The target project is the project KEY to which the pull request will be made.");
    let target_project = prompt("target project: ")?;

    println!("
The target slug is the repo within the target project to which the pull
request will be made.");
    let target_slug = prompt("target slug: ")?;

    println!("
The target branch is the branch to which the pull request will be made.
This can be overwritten on the command line.");
    let target_branch = prompt("target branch: ")?;

    let auth = format!("{}:{}", username.trim(), password.trim());
    let base64auth = auth.as_bytes().to_base64(STANDARD);

    Config::create_file(path,
                        &server,
                        &base64auth,
                        &project_name,
                        &source_project,
                        &source_slug,
                        &target_project,
                        &target_slug,
                        &target_branch)?;

    println!("
Please edit {} to have your desired configuration (particularly user groups)",
             path.display());

    Ok(())
}

fn groups(config: &Config) -> Result<()> {
    for (name, group) in &config.groups {
        println!("{}: []", name);
    }
    Ok(())
}

fn pr(config: &Config, client: &Bitbucket, matches: &ArgMatches) -> Result<()> {
    Ok(())
}

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
    let config_path = Path::new(config_file);

    if matches.is_present("setup") {
        match setup(&config_path) {
            Err(why) => exit(&format!("error: {}", why)),
            Ok(_) => return,
        }
    }

    let config = Config::from_file(&config_path).unwrap_or_exit("Invalid config file");
    let client = client::Bitbucket::new(config.auth.clone(), config.server.clone())
        .unwrap_or_exit("Could not create client");

    let res = match matches.subcommand_name() {
        Some("groups") => groups(&config),
        Some("pr") => pr(&config, &client, &matches),
        _ => unreachable!(),
    };

    match res {
        Err(why) => exit(&format!("error: {}", why)),
        Ok(_) => {}
    }
    // match git::repo_dir() {
    //     Ok(path) => println!("path: {}", path.to_str().unwrap_or_exit("????")),
    //     Err(why) => println!("error!: {}", why),
    // }

    // match git::repo_name() {
    //     Ok(name) => println!("repo name: {}", name),
    //     Err(why) => println!("error!: {}", why),
    // }

    // match git::current_branch() {
    //     Ok(branch) => println!("branch: {}", branch),
    //     Err(why) => println!("error!: {}", why),
    // }
}
