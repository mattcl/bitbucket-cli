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

#[allow(non_snake_case)]
#[derive(RustcEncodable)]
pub struct PullRequest {
    title: String,
    fromRef: Option<Reference>,
    toRef: Option<Reference>,
    reviewers: Vec<Reviewer>,
    description: String,
}

impl PullRequest {
    pub fn new(title: &str) -> PullRequest {
        PullRequest {
            title: title.to_string(),
            fromRef: None,
            toRef: None,
            reviewers: Vec::new(),
            description: String::new(),
        }
    }

    pub fn from_ref<'a>(&'a mut self,
                        branch: &str,
                        slug: &str,
                        project: &str)
                        -> &'a mut PullRequest {
        self.fromRef =
            Some(Reference::new(branch.to_string(), slug.to_string(), project.to_string()));
        self
    }

    pub fn to_ref<'a>(&'a mut self,
                      branch: &str,
                      slug: &str,
                      project: &str)
                      -> &'a mut PullRequest {
        self.toRef =
            Some(Reference::new(branch.to_string(), slug.to_string(), project.to_string()));
        self
    }

    pub fn reviewers<'a, I>(&'a mut self, reviewers: I) -> &'a mut PullRequest
        where I: Iterator<Item = &'a String>
    {
        for reviewer in reviewers {
            let reviewer = Reviewer { user: User { name: reviewer.to_string() } };
            self.reviewers.push(reviewer);
        }
        self
    }

    pub fn description<'a>(&'a mut self, description: &str) -> &'a mut PullRequest {
        self.description = description.to_string();
        self
    }
}
