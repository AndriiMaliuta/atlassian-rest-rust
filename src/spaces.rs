pub mod spaces {

    use reqwest::Response;
    use crate::model::models::{Content, CreatePage, ContentResponse, SearchResults};

    pub async fn get_space(url: &str, token: String, key: String) -> Space {
        let request_url = format!("{url}/rest/api/space/{key}");
        let client = reqwest::Client::new();
        let resp: Response = client.get(&request_url)
            .header("Authorization", format!("Basic {token}"))
            .header("Accept", "application/json")
            .send().await.unwrap();
        let body = resp.text().await.unwrap();
        return serde_json::from_str(body.as_str()).unwrap();
    }

    pub async fn create_space(conf_url: &str, token: &str, page: CreateSpace) -> String {
        let request_url = format!("{conf_url}/rest/api/content/");
        let client = reqwest::Client::new();
        let res = client.post(&request_url)
            .json(&page)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Basic {token}"))
            .send()
            .await.unwrap();
        let created: String = res.text().await.unwrap();
        return created;
    }
}