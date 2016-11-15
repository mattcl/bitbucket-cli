use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::path::Path;
use std::io::Read;

use tera::Tera;
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

impl Project {
    pub fn from_data(data: &Yaml) -> Result<Project> {
        let source_project = unpack("source_project", || data["source_project"].as_str())?.to_string();
        let source_slug = unpack("source_slug", || data["source_slug"].as_str())?.to_string();
        let target_project = unpack("target_project", || data["target_project"].as_str())?.to_string();
        let target_slug = unpack("target_slug", || data["target_slug"].as_str())?.to_string();
        let target_branch = unpack("target_branch", || data["target_branch"].as_str())?.to_string();

        Ok(Project {
            source_project: source_project,
            source_slug: source_slug,
            target_project: target_project,
            target_slug: target_slug,
            target_branch: target_branch,
        })
    }
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

        // Projects
        let projects_raw = unpack("projects", || data["projects"].as_hash())?;
        let mut projects = HashMap::new();

        for (key, value) in projects_raw {
            let name = unpack("this should not be possible", || key.as_str())?.to_string();
            let project = Project::from_data(&value)?;
            projects.insert(name, project);
        }

        // Groups
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
            projects: projects,
            groups: groups,
        })
    }

    pub fn save(&self) -> Result<()> {
        let tera = Tera::new("templates/**/*");
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
