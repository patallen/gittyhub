use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    pub name: String,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub login: String,
    pub site_admin: bool,
    pub id: usize,
    pub avatar_url: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequest {
    pub url: String,
    pub id: usize,
    pub number: usize,
    pub node_id: String,
    pub diff_url: String,
    pub user: User,
    pub body: String,
    pub state: String,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub merged_at: Option<DateTime<Utc>>,
    pub closed_at: Option<DateTime<Utc>>,
    pub locked: bool,
    pub author_association: String,
    pub merge_commit_sha: String,
    pub assignees: Vec<User>,
    pub requested_reviewers: Vec<User>,
    pub labels: Vec<Label>,
}
