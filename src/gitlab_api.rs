use reqwest;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitLab {
    pub id: i64,
    pub iid: i64,
    #[serde(rename = "project_id")]
    pub project_id: i64,
    pub title: String,
    pub description: String,
    pub state: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "closed_at")]
    pub closed_at: Value,
    #[serde(rename = "closed_by")]
    pub closed_by: Value,
    pub labels: Vec<Value>,
    pub milestone: Value,
    pub assignees: Vec<Assignee>,
    pub author: Author,
    #[serde(rename = "type")]
    pub type_field: String,
    pub assignee: Assignee2,
    #[serde(rename = "user_notes_count")]
    pub user_notes_count: i64,
    #[serde(rename = "merge_requests_count")]
    pub merge_requests_count: i64,
    pub upvotes: i64,
    pub downvotes: i64,
    #[serde(rename = "due_date")]
    pub due_date: Value,
    pub confidential: bool,
    #[serde(rename = "discussion_locked")]
    pub discussion_locked: Value,
    #[serde(rename = "issue_type")]
    pub issue_type: String,
    #[serde(rename = "web_url")]
    pub web_url: String,
    #[serde(rename = "time_stats")]
    pub time_stats: TimeStats,
    #[serde(rename = "task_completion_status")]
    pub task_completion_status: TaskCompletionStatus,
    pub weight: Value,
    #[serde(rename = "blocking_issues_count")]
    pub blocking_issues_count: i64,
    #[serde(rename = "has_tasks")]
    pub has_tasks: bool,
    #[serde(rename = "_links")]
    pub links: Links,
    pub references: References,
    pub subscribed: bool,
    #[serde(rename = "moved_to_id")]
    pub moved_to_id: Value,
    #[serde(rename = "service_desk_reply_to")]
    pub service_desk_reply_to: Value,
    #[serde(rename = "epic_iid")]
    pub epic_iid: Value,
    pub epic: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Assignee {
    pub id: i64,
    pub username: String,
    pub name: String,
    pub state: String,
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    #[serde(rename = "web_url")]
    pub web_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub id: i64,
    pub username: String,
    pub name: String,
    pub state: String,
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    #[serde(rename = "web_url")]
    pub web_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Assignee2 {
    pub id: i64,
    pub username: String,
    pub name: String,
    pub state: String,
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    #[serde(rename = "web_url")]
    pub web_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeStats {
    #[serde(rename = "time_estimate")]
    pub time_estimate: i64,
    #[serde(rename = "total_time_spent")]
    pub total_time_spent: i64,
    #[serde(rename = "human_time_estimate")]
    pub human_time_estimate: Value,
    #[serde(rename = "human_total_time_spent")]
    pub human_total_time_spent: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskCompletionStatus {
    pub count: i64,
    #[serde(rename = "completed_count")]
    pub completed_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    #[serde(rename = "self")]
    pub self_field: String,
    pub notes: String,
    #[serde(rename = "award_emoji")]
    pub award_emoji: String,
    pub project: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct References {
    pub short: String,
    pub relative: String,
    pub full: String,
}

impl GitLab {
    pub async fn get(
        project_id: &str,
        issue_id: &str,
        token: &str,
    ) -> Result<Self, reqwest::Error> {
        let url = format!(
            "https://gitlab.com/api/v4/projects/{}/issues/{}/",
            project_id, issue_id
        );
        //let url = reqwest::Url::parse(&*url)?;
        let client = reqwest::Client::builder().build()?;
        let res = client
            .get(url)
            .header("PRIVATE-TOKEN", token)
            .send()
            .await?
            .json::<GitLab>()
            .await?;
        Ok(res)
    }
}
