use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::io::Read;

use yaml_rust::YamlLoader;

use error::{ErrorKind, Result};

#[derive(Debug)]
pub struct Project {
    pub source_project: String,
    pub source_slug: String,
    pub target_project: String,
    pub target_slug: String,
    pub target_branch: String,
}

#[derive(Debug)]
pub struct Config {
    pub server: String,
    pub auth: String,
    pub browser_command: String,
    pub open_in_browser: bool,
    pub projects: HashMap<String, Project>,
}

impl Config {
    pub fn from_file(path: &Path) -> Result<Config> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let docs = YamlLoader::load_from_str(&content)?;
        let data = &docs[0];

        let server = data["server"].as_str()
            .ok_or(ErrorKind::InvalidConfig("server".to_string()).into())?;
        Err(ErrorKind::InvalidConfig("derp".to_string()).into())
    }
}


// fn create_config_file(path: &Path, config: &Config) -> Result<()> {
//     let mut file = try!(File::create(&path));
//     let content = format!("# configuration for oh-bother
// config_version: 1
// config:
//   # connectivity settings
//   jira: \"{jira}\"
//   username: \"{username}\"
//   auth: \"{auth}\"

//   # controls whether or not manipulated issues are opened in the web browser
//   open_in_browser: true
//   browser_command: google-chrome

//   # These projects are used to find issues for commands like 'list' and 'next'
//   project_keys:
//     - \"{project_key}\"

//   # these users are users for whom a ticket is considered 'fair game' or 'unassigned'
//   npc_users:
//     - Unassigned
//     - \"{npc}\"

//   new_issue_defaults:
//     project_key: \"{project_key}\"
//     assignee: \"{npc}\"
//     labels:
//       - interrupt
// ",
//                           jira = jira,
//                           username = username,
//                           auth = auth,
//                           npc = npc,
//                           project_key = project_key);

//     try!(file.write_all(content.as_bytes()));
//     Ok(())
// }
