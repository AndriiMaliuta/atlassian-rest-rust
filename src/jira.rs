pub(crate) mod jira_models;

pub mod jira {
    use reqwest::Response;
    use crate::jira::jira_models::jira_models::{CreateIssue, EditIssue, ProjectCore};

    pub struct JiraClient {}

    pub struct IssueService();
    impl IssueService {
        pub async fn get_issue(&self, url: &str, token: String, key: String) -> String {
            let req_url = format!("{url}/rest/api/2/issue/{key}");
            let client = reqwest::Client::new();
            let resp: Response = client.get(&req_url)
                .header("Authorization", format!("Basic {token}"))
                .header("Accept", "application/json")
                .header("Content-Type", "application/json")
                .send().await.unwrap();
            let body = resp.text().await.unwrap();
            return body;
        }

        pub async fn create_issue(&self, url: &str, token: String, issue: CreateIssue) -> String {
            let req_url = format!("{url}/rest/api/2/issue");
            let client = reqwest::Client::new();
            let resp: Response = client.post(&req_url)
                .json(&issue)
                .header("Authorization", format!("Basic {token}"))
                .header("Accept", "application/json")
                .header("Content-Type", "application/json")
                .send().await.unwrap();
            let body = resp.text().await.unwrap();
            return body;
        }

        pub async fn edit_issue(&self, url: &str, token: String, issue: EditIssue) -> String {
            let req_url = format!("{url}/rest/api/2/issue");
            let client = reqwest::Client::new();
            let resp: Response = client.post(&req_url)
                .json(&issue)
                .header("Authorization", format!("Basic {token}"))
                .header("Accept", "application/json")
                .header("Content-Type", "application/json")
                .send().await.unwrap();
            let body = resp.text().await.unwrap();
            return body;
        }

        pub async fn delete_issue(&self, url: &str, token: String, key: &str) {
            let req_url = format!("{url}/rest/api/2/issue/{key}");
            let client = reqwest::Client::new();

            client.delete(&req_url)
                .header("Authorization", format!("Basic {token}"))
                .header("Accept", "application/json")
                .header("Content-Type", "application/json")
                .send().await.unwrap();
        }
    }
    pub struct ProjectService();

    impl ProjectService {
        pub async fn get_projects(&self, url: &str, token: String) -> Vec<ProjectCore> {
            let req_url = format!("{url}/rest/api/2/project");
            let client = reqwest::Client::new();
            let resp: Response = client.get(&req_url)
                .header("Authorization", format!("Basic {token}"))
                .header("Accept", "application/json")
                .send().await.unwrap();
            let body = resp.text().await.unwrap();
            return serde_json::from_str(&body).unwrap();
        }

        pub async fn get_project(&self, url: &str, token: String, key: String) -> ProjectCore {
            let req_url = format!("{url}/rest/api/2/project/{key}");
            let client = reqwest::Client::new();
            let resp: Response = client.get(&req_url)
                .header("Authorization", format!("Basic {token}"))
                .header("Accept", "application/json")
                .send().await.unwrap();
            let body = resp.text().await.unwrap();
            return serde_json::from_str(&body).unwrap();
        }
    }

    // issues Workflows
    // control of the statuses sets
    pub struct WorkflowService();
}