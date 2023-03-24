pub mod workflows {
    use reqwest::Response;
    use crate::jira::jira_models::jira_models::ProjectWorkflow;

    // issues Workflows
    // control of the statuses sets
    pub struct WorkflowService();

    impl WorkflowService {
        pub async fn get_all_workflows(&self, url: &str, token: String) -> Vec<ProjectWorkflow> {
            let req_url = format!("{url}/rest/api/2/workflow");
            let client = reqwest::Client::new();
            let resp: Response = client.get(&req_url)
                .header("Authorization", format!("Basic {token}"))
                .header("Accept", "application/json")
                .send().await.unwrap();
            let body = resp.text().await.unwrap();
            return serde_json::from_str(&body).unwrap();
        }

        pub async fn get_workflow(&self, url: &str, token: String, id: String) -> ProjectWorkflow {
            let req_url = format!("{url}/rest/api/2/workflow/{id}");
            let client = reqwest::Client::new();
            let resp: Response = client.get(&req_url)
                .header("Authorization", format!("Basic {token}"))
                .header("Accept", "application/json")
                .send().await.unwrap();
            let body = resp.text().await.unwrap();
            return serde_json::from_str(&body).unwrap();
        }
    }

    pub struct WorkflowScheme();
}