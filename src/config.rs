use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::path::Path;
use std::io::{Read, Write};

use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;
use prettytable::format;
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
        let source_project =
            unpack("source_project", || data["source_project"].as_str())?.to_string();
        let source_slug = unpack("source_slug", || data["source_slug"].as_str())?.to_string();
        let target_project =
            unpack("target_project", || data["target_project"].as_str())?.to_string();
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
    pub groups: BTreeMap<String, HashSet<String>>,
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
        let open_in_browser = unpack("open_in_browser", || data["open_in_browser"].as_bool())
            .unwrap_or(false);
        let browser_command =
            unpack("browser_command", || data["browser_command"].as_str())?.to_string();

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
        let mut groups = BTreeMap::new();

        let empty_group: HashSet<String> = HashSet::new();
        groups.insert("empty".to_string(), empty_group);

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

    pub fn create_file(path: &Path,
                       server: &str,
                       auth: &str,
                       project_name: &str,
                       source_project: &str,
                       source_slug: &str,
                       target_project: &str,
                       target_slug: &str,
                       target_branch: &str)
                       -> Result<()> {
        let mut file = try!(File::create(&path));
        let content = format!("# configuration for bitbucket-cli
server: \"{server}\"
auth_token: \"{auth}\"

# default value for open in browser
open_in_browser: false

# This is executed as <browser_command> <pull request url>
browser_command: \"google-chrome\"

# You can specify a list of projects here. Projects are detected via the
# basename of the git repo directory or via a .bitbucket-proj file at
# <git repo basename>/.bitbucket-proj. The must contain one line: the project name
projects:
  # The project name should be the same name as the repo basename (directory).
  # This enables the auto-detection. If you'd rather specify the project for
  # a repo explicitly, create a .bitbucket-proj file containing the project
  # name as specified below
  {project_name}:
    # The source project is the project KEY or a tilde-prefixed username for
    # a personal project. This is the project the pull request will be made from.
    source_project: {source_project}

    # The source slug is the repository under the source project from which the
    # pull request will be made.
    source_slug: {source_slug}

    # The target project is the project KEY to which the pull request will be made.
    target_project: {target_project}

    # The target slug is the repo within the target project to which the pull
    # request will be made.
    target_slug: {target_slug}

    # The target branch is the branch to which the pull request will be made.
    # This can be overwritten on the command line.
    target_branch: {target_branch}

# You can always specify reviewers via the command line, but these are here to
# provide convenient sets of frequently-included reviewers. The names here are
# the \"names\" for the desired set of stash users. You can get a (limit 1000)
# list of users by running `stash users`
reviewer_groups:
  default:
    - foo
    - bar.baz

#  core:
#    - foo
#    - herp
#    - derp
",
                              server = server,
                              auth = auth,
                              project_name = project_name,
                              source_project = source_project,
                              source_slug = source_slug,
                              target_project = target_project,
                              target_slug = target_slug,
                              target_branch = target_branch);

        try!(file.write_all(content.as_bytes()));
        Ok(())
    }

    pub fn get_project(&self, project: &str) -> Result<&Project> {
        self.projects
            .get(project)
            .ok_or::<Error>(ErrorKind::ProjectNotFound(project.to_string()).into())
    }

    pub fn get_group(&self, group: &str) -> Result<&HashSet<String>> {
        self.groups
            .get(group)
            .ok_or::<Error>(ErrorKind::GroupNotFound(group.to_string()).into())
    }

    pub fn print_groups(&self, force_colorize: bool) {
        let mut table = Table::new();

        let format = format::FormatBuilder::new()
            .padding(1, 1)
            .build();

        table.set_format(format);

        for (name, group) in &self.groups {
            table.add_row(Row::new(vec![Cell::new(name), Cell::new(&format!("{:?}", group))]));
        }

        table.print_tty(force_colorize);
    }
}
