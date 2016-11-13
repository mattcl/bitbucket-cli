use std::collections::HashMap;
use error::{ErrorKind, Result};

#[derive(RustcEncodable)]
pub struct Reference {
    id: String,
    repository: Repository,
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

#[derive(RustcEncodable)]
pub struct Repository {
    slug: String,
    project: Project,
}

#[derive(RustcEncodable)]
pub struct Project {
    key: String,
}

#[derive(RustcEncodable)]
pub struct Reviewer {
    user: User,
}

#[derive(RustcEncodable)]
pub struct User {
    name: String,
}

#[derive(RustcEncodable)]
pub struct PullRequest {
    title: String,
    fromRef: Option<Reference>,
    toRef: Option<Reference>,
    reviewers: Vec<Reviewer>,
    description: String,
}

impl PullRequest {
    pub fn new(title: String) -> PullRequest {
        PullRequest {
            title: title,
            fromRef: None,
            toRef: None,
            reviewers: Vec::new(),
            description: String::new(),
        }
    }

    pub fn from_ref<'a>(&'a mut self,
                        branch: String,
                        slug: String,
                        project: String)
                        -> &'a mut PullRequest {
        self.fromRef = Some(Reference::new(branch, slug, project));
        self
    }

    pub fn to_ref<'a>(&'a mut self,
                      branch: String,
                      slug: String,
                      project: String)
                      -> &'a mut PullRequest {
        self.toRef = Some(Reference::new(branch, slug, project));
        self
    }

    pub fn reviewer<'a>(&'a mut self, reviewer: String) -> &'a mut PullRequest {
        let reviewer = Reviewer { user: User { name: reviewer } };
        self.reviewers.push(reviewer);
        self
    }

    pub fn description<'a>(&'a mut self, description: String) -> &'a mut PullRequest {
        self.description = description;
        self
    }
}
