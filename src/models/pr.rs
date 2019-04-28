use super::user::User;

#[derive(Debug, Clone)]
pub struct PullRequest<'a> {
    pub owner: User,
    pub number: usize,
    pub id: usize,
    pub title: &'a str,
    pub body: &'a str,
    pub state: &'a str,
    pub requested_reviewers: Vec<User>,
}

impl<'a> PullRequest<'a> {
    pub fn new(
        id: usize,
        number: usize,
        title: &'a str,
        body: &'a str,
        owner: User,
    ) -> PullRequest<'a> {
        PullRequest {
            id,
            owner,
            number,
            title,
            body,
            state: "open",
            requested_reviewers: vec![],
        }
    }
}
