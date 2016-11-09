use std::collections::HashMap;
use error::Result;

#[derive(Debug)]
pub struct Config {
    pub server: String,
    pub auth: String,
    pub browser_command: String,
    pub open_in_browser: bool,
    pub projects: HashMap<String, Project>,
}

#[derive(Debug)]
pub struct Project {
    pub source_project: String,
    pub source_slug: String,
    pub target_project: String,
    pub target_slug: String,
    pub target_branch: String,
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
