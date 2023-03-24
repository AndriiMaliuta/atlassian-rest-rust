pub mod filter {
    use reqwest::Response;
    use crate::jira::jira_models::jira_models::{CreateField, FieldCore};

    pub struct FilterService();

    impl FilterService {
        pub async fn get_filters(&self, url: &str, token: &str) -> Vec<FieldCore> {
            let req_url = format!("{url}/rest/api/2/filter");
            let client = reqwest::Client::new();
            let resp: Response = client.get(&req_url)
                .header("Authorization", format!("Basic {token}"))
                .header("Accept", "application/json")
                .header("Content-Type", "application/json")
                .send().await.unwrap();
            let body = resp.text().await.unwrap();
            return serde_json::from_str(&body).unwrap();
        }


    }
}