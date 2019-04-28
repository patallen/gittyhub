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

pub fn pull_requests<'a>(user: User) -> Result<Vec<PullRequest<'a>>, io::Error> {
    let prs: Vec<PullRequest> = vec![
        PullRequest::new(
            0,
            123,
            "Fix that stupid bug that's been plaguing us",
            "...",
            user.clone(),
        ),
        PullRequest::new(
            1,
            124,
            "Revert previous changes made in 0.4.2b release",
            "These changes had many many breaking changes that we didn't anticipate.",
            user.clone(),
        ),
        PullRequest::new(
            4,
            128,
            "Quick fix: Don't panic on read Err.",
            "'nuff said",
            user.clone(),
        ),
        PullRequest::new(
            3,
            109,
            "Refactor the backend buff-related structures",
            "...",
            user.clone(),
        ),
    ];
    Ok(prs)
}
