/// Helper functions for mocking data and calls to the Github API
use super::models::{PullRequest, User};
use std::io;

pub fn user(login: &str) -> Result<User, io::Error> {
    let user = User {
        id: 0,
        login: login.into(),
        url: format!("https://github.com/{}", login).clone(),
    };
    Ok(user)
}

pub fn pull_requests(user: User) -> Result<Vec<PullRequest>, io::Error> {
    let prs: Vec<PullRequest> = vec![
        PullRequest::new(
            0,
            123,
            "Fix that stupid bug that's been plaguing us".into(),
            "...".into(),
            user.clone(),
        ),
        PullRequest::new(
            1,
            124,
            "Revert previous changes made in 0.4.2b release".into(),
            "These changes had many many breaking changes that we didn't anticipate.".into(),
            user.clone(),
        ),
        PullRequest::new(
            4,
            128,
            "Quick fix: Don't panic on read Err.".into(),
            "'nuff said".into(),
            user.clone(),
        ),
        PullRequest::new(
            3,
            109,
            "Refactor the backend buff-related structures".into(),
            "...".into(),
            user.clone(),
        ),
    ];
    Ok(prs)
}
