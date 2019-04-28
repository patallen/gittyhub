pub trait Model {}

/// Represents a Github user as returned by the Github API
#[derive(Debug, Clone)]
pub struct User {
    pub id: usize,
    pub login: String,
    pub url: String,
}

/// Represents a pull request as returned by the Github API
#[derive(Debug, Clone)]
pub struct PullRequest {
    pub owner: User,
    pub number: usize,
    pub id: usize,
    pub title: String,
    pub body: String,
    pub state: String,
    pub requested_reviewers: Vec<User>,
}

impl PullRequest {
    pub fn new(id: usize, number: usize, title: String, body: String, owner: User) -> PullRequest {
        PullRequest {
            id,
            owner,
            number,
            title,
            body,
            state: "open".into(),
            requested_reviewers: vec![],
        }
    }
}

impl Model for PullRequest {}
