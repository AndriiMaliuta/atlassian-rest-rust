pub(crate) mod jira_models;

pub mod jira {
    use reqwest::Response;
    use crate::jira::jira_models::jira_models::CreateIssue;

    pub struct JiraClient {}

    pub struct IssueService();

    impl IssueService {
        pub async fn create_issue(&self, url: &str, token: String, issue: CreateIssue) -> String {
            let req_url = format!("{url}/rest/api/2/issue");
            let client = reqwest::Client::new();
            let body = r#"{
                      "fields": {
                        "project": {
                            "id": "10000"
                        },
                        "summary": "Test Rust",
                        "issuetype": {
                            "id": "10006"
                        },
                        "assignee": {
                            "name": "admin"
                        },
                        "reporter": {
                            "name": "admin"
                        },
                        "priority": {
                            "id": "3"
                        },
                        "labels": [
                            "a",
                            "b"
                        ],
                        "description": "Test",
                        "duedate": "2023-04-11"
                    }
                }
            "#;
            let resp: Response = client.post(&req_url)
                .body(body)
                .header("Authorization", format!("Basic {token}"))
                .header("Accept", "application/json")
                .header("Content-Type", "application/json")
                .send().await.unwrap();
            let body = resp.text().await.unwrap();
            return body;
        }
    }
}