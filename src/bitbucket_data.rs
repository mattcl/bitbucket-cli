use std::collections::HashMap;

use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;
use prettytable::format;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Reference {
    id: String,
    pub repository: Repository,
}

impl Reference {
    pub fn new(branch: String, slug: String, project: String) -> Reference {
        Reference {
            id: format!("refs/heads/{}", branch),
            repository: Repository {
                slug: slug,
                project: Project { key: project },
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Repository {
    pub slug: String,
    pub project: Project,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Project {
    pub key: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct User {
    name: String,
    #[serde(skip_serializing)]
    displayName: Option<String>,
    #[serde(skip_serializing)]
    slug: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Reviewer {
    user: User,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Author {
    user: User,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct UserSearchResult {
    values: Vec<User>,
}

impl UserSearchResult {
    pub fn print_tty(&self, force_colorize: bool) {
        let mut table = Table::new();

        let format = format::FormatBuilder::new()
            .padding(1, 1)
            .separator(
                format::LinePosition::Title,
                format::LineSeparator::new('-', '-', '-', '-'),
            )
            .build();

        table.set_format(format);
        table.set_titles(Row::new(vec![Cell::new("name"), Cell::new("slug")]));

        for user in &self.values {
            let display_name = user.displayName
                .clone()
                .unwrap_or("missing display name".to_string());
            let slug = user.slug.clone().unwrap_or("missing slug".to_string());
            table.add_row(Row::new(vec![Cell::new(&display_name), Cell::new(&slug)]));
        }

        table.print_tty(force_colorize);
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Link {
    href: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PullRequest {
    title: String,
    fromRef: Option<Reference>,
    toRef: Option<Reference>,
    reviewers: Vec<Reviewer>,
    #[serde(default = "missing_description")]
    description: String,
    #[serde(skip_serializing)]
    links: HashMap<String, Vec<Link>>,
    #[serde(skip_serializing)]
    author: Option<Author>,
}

fn missing_description() -> String {
    "missing description".to_string()
}

impl PullRequest {
    pub fn new(title: &str) -> PullRequest {
        PullRequest {
            title: title.to_string(),
            fromRef: None,
            toRef: None,
            reviewers: Vec::new(),
            description: String::new(),
            links: HashMap::new(),
            author: None,
        }
    }

    pub fn from_ref<'a>(
        &'a mut self,
        branch: &str,
        slug: &str,
        project: &str,
    ) -> &'a mut PullRequest {
        self.fromRef = Some(Reference::new(
            branch.to_string(),
            slug.to_string(),
            project.to_string(),
        ));
        self
    }

    pub fn to_ref<'a>(
        &'a mut self,
        branch: &str,
        slug: &str,
        project: &str,
    ) -> &'a mut PullRequest {
        self.toRef = Some(Reference::new(
            branch.to_string(),
            slug.to_string(),
            project.to_string(),
        ));
        self
    }

    pub fn reviewers<'a, I>(&'a mut self, reviewers: I) -> &'a mut PullRequest
    where
        I: Iterator<Item = &'a String>,
    {
        for reviewer in reviewers {
            let reviewer = Reviewer {
                user: User {
                    name: reviewer.to_string(),
                    displayName: None,
                    slug: None,
                },
            };
            self.reviewers.push(reviewer);
        }
        self
    }

    pub fn description<'a>(&'a mut self, description: &str) -> &'a mut PullRequest {
        self.description = description.to_string();
        self
    }

    pub fn project(&self) -> Option<String> {
        if let Some(ref r) = self.toRef {
            return Some(r.repository.project.key.clone());
        }
        None
    }

    pub fn slug(&self) -> Option<String> {
        if let Some(ref r) = self.toRef {
            return Some(r.repository.slug.clone());
        }
        None
    }

    pub fn self_link(&self) -> Option<String> {
        if let Some(links) = self.links.get("self") {
            for l in links {
                return Some(l.href.clone());
            }
        }
        None
    }

    pub fn author_name(&self) -> Option<String> {
        if let Some(ref author) = self.author {
            if let Some(ref display_name) = author.user.displayName {
                return Some(display_name.clone());
            }
        }
        None
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PullRequestList {
    values: Vec<PullRequest>,
}

impl PullRequestList {
    pub fn print_tty(&self, force_colorize: bool) {
        let mut table = Table::new();

        let format = format::FormatBuilder::new()
            .padding(1, 1)
            .separator(
                format::LinePosition::Title,
                format::LineSeparator::new('-', '-', '-', '-'),
            )
            .build();

        table.set_format(format);
        table.set_titles(Row::new(vec![
            Cell::new("title"),
            Cell::new("author"),
            Cell::new("link"),
        ]));

        for pr in &self.values {
            let mut display_name = pr.title.clone();
            display_name.truncate(50);
            let author = pr.author_name().unwrap_or("missing author".to_string());
            let link = match pr.self_link() {
                Some(l) => l.clone(),
                None => "missing link".to_string(),
            };
            table.add_row(Row::new(vec![
                Cell::new(&display_name),
                Cell::new(&author),
                Cell::new(&link),
            ]));
        }

        table.print_tty(force_colorize);
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pull_request_construction() {
        let pull_request = PullRequest::new("derp");
        assert_eq!("derp", pull_request.title);

        let reviewers: Vec<Reviewer> = Vec::new();
        assert_eq!(reviewers, pull_request.reviewers);

        assert_eq!(String::new(), pull_request.description);
        assert!(pull_request.fromRef.is_none());
        assert!(pull_request.toRef.is_none());
    }

    #[test]
    fn setting_from_ref() {
        let mut pull_request = PullRequest::new("derp");
        let reference = Reference::new(
            "branch".to_string(),
            "slug".to_string(),
            "project".to_string(),
        );

        pull_request.from_ref("branch", "slug", "project");

        assert_eq!(reference, pull_request.fromRef.unwrap());
    }

    #[test]
    fn setting_to_ref() {
        let mut pull_request = PullRequest::new("derp");
        let reference = Reference::new(
            "branch".to_string(),
            "slug".to_string(),
            "project".to_string(),
        );

        pull_request.to_ref("branch", "slug", "project");

        assert_eq!(reference, pull_request.toRef.unwrap());
    }

    #[test]
    fn setting_reviewrs() {
        let mut pull_request = PullRequest::new("derp");
        let names = vec!["foo".to_string(), "bar".to_string(), "baz".to_string()];
        let mut reviewers = Vec::new();
        for name in &names {
            let reviewer = Reviewer {
                user: User {
                    name: name.to_string(),
                    displayName: None,
                    slug: None,
                },
            };
            reviewers.push(reviewer);
        }
        pull_request.reviewers(names.iter());
    }

    #[test]
    fn setting_description() {
        let mut pull_request = PullRequest::new("derp");
        pull_request.description("my description");
        assert_eq!("my description".to_string(), pull_request.description);
    }
}
