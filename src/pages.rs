pub mod page_service {
    use serde_json::{json, Value};
    use serde::{Deserialize, Serialize};
    use crate::models::models::CreatePage;

    pub async fn create_page(conf_url: &str, token: &str, page: CreatePage) -> String {
        let request_url = format!("{conf_url}/rest/api/content/");
        let client = reqwest::Client::new();
        let res = client.post(&request_url)
            .json(&page)
            .header("Authorization", format!("Basic {token}"))
            .send()
            .await.unwrap();
        let created: String = res.text().await.unwrap();
        return created;
    }




}