use std::fs::File;
use std::io::Read;
use std::process::Command;

use hyper::Url;

use config::Config;
use error::Result;
use git;

pub fn get_project_name() -> Result<String> {
    let repo_name = git::repo_name()?;

    let mut project_file = git::repo_dir()?;
    project_file.push(".bitbucket-proj");

    match File::open(project_file) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            return Ok(content);
        }
        Err(_) => return Ok(repo_name),
    };
}

pub fn open_in_browser(config: &Config, url: &Url) -> Result<()> {
    match Command::new(config.browser_command.as_str())
        .arg(url.as_str())
        .output()
    {
        Err(why) => Err(why.into()),
        Ok(_) => Ok(()),
    }
}
