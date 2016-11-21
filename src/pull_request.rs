#[derive(Debug, Eq, PartialEq, RustcEncodable)]
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

#[derive(Debug, Eq, PartialEq, RustcEncodable)]
pub struct Repository {
    pub slug: String,
    pub project: Project,
}

#[derive(Debug, Eq, PartialEq, RustcEncodable)]
pub struct Project {
    pub key: String,
}

#[derive(Debug, Eq, PartialEq, RustcEncodable)]
pub struct Reviewer {
    user: User,
}

#[derive(Debug, Eq, PartialEq, RustcEncodable)]
pub struct User {
    name: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Eq, PartialEq, RustcEncodable)]
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
        let reference = Reference::new("branch".to_string(),
                                       "slug".to_string(),
                                       "project".to_string());

        pull_request.from_ref("branch", "slug", "project");

        assert_eq!(reference, pull_request.fromRef.unwrap());
    }

    #[test]
    fn setting_to_ref() {
        let mut pull_request = PullRequest::new("derp");
        let reference = Reference::new("branch".to_string(),
                                       "slug".to_string(),
                                       "project".to_string());

        pull_request.to_ref("branch", "slug", "project");

        assert_eq!(reference, pull_request.toRef.unwrap());
    }

    #[test]
    fn setting_reviewrs() {
        let mut pull_request = PullRequest::new("derp");
        let names = vec!["foo".to_string(), "bar".to_string(), "baz".to_string()];
        let mut reviewers = Vec::new();
        for name in &names {
            let reviewer = Reviewer { user: User { name: name.to_string() } };
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
