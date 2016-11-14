use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::path::Path;
use std::io::Read;

use yaml_rust::{Yaml, YamlLoader};

use error::{Error, ErrorKind, Result};

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
    pub open_in_browser: bool,
    pub browser_command: String,
    pub projects: HashMap<String, Project>,
    pub groups: HashMap<String, HashSet<String>>,
}

fn unpack<F, T>(key: &str, f: F) -> Result<T>
    where F: Fn() -> Option<T>
{
    f().ok_or::<Error>(ErrorKind::InvalidConfig(key.to_string()).into())
}

impl Config {
    pub fn from_file(path: &Path) -> Result<Config> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let docs = YamlLoader::load_from_str(&content)?;
        let data = &docs[0];

        let server = unpack("server", || data["server"].as_str())?.to_string();
        let auth = unpack("auth_token", || data["auth_token"].as_str())?.to_string();
        let open_in_browser = unpack("open_in_browser", || data["open_in_browser"].as_bool())?;
        let browser_command = unpack("browser_command", || data["browser_command"].as_str())?.to_string();
        let groups_raw = unpack("reviewer_groups", || data["reviewer_groups"].as_hash())?;

        let mut groups = HashMap::new();

        for (key, value) in groups_raw {
            let name = unpack("this should not be possible", || key.as_str())?.to_string();

            let mut group = HashSet::new();

            let users = unpack("", || value.as_vec())?;
            for user in users {
                let u = unpack("", || user.as_str())?.to_string();
                group.insert(u);
            }
            groups.insert(name, group);
        }

        Ok(Config {
            server: server,
            auth: auth,
            open_in_browser: open_in_browser,
            browser_command: browser_command,
            projects: HashMap::new(),
            groups: groups,
        })
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
