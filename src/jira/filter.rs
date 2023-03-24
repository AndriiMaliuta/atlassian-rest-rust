pub mod filter {
    use reqwest::Response;
    use crate::jira::jira_models::jira_models::{CreateField, FieldCore};

    pub struct FilterService();

    impl FilterService {
        // GET /rest/api/2/field
        pub async fn get_fields(&self, url: String, token: String) -> Vec<FieldCore> {
            let req_url = format!("{url}/rest/api/2/field");
            let client = reqwest::Client::new();
            let resp: Response = client.get(&req_url)
                .header("Authorization", format!("Basic {token}"))
                .header("Accept", "application/json")
                .header("Content-Type", "application/json")
                .send().await.unwrap();
            let body = resp.text().await.unwrap();
            return serde_json::from_str(&body).unwrap();
        }

        pub async fn create_field(&self, url: String, token: String, data: CreateField) -> String {
            let req_url = format!("{url}/rest/api/2/field");
            let client = reqwest::Client::new();
            let resp: Response = client.post(&req_url)
                .json(&data)
                .header("Authorization", format!("Basic {token}"))
                .header("Accept", "application/json")
                .header("Content-Type", "application/json")
                .send().await.unwrap();
            let body = resp.text().await.unwrap();
            return body;
        }
    }
}