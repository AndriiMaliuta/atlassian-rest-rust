pub mod spaces {
    use reqwest::Response;
    use crate::model::models::{Content, CreatePage, ContentResponse, SearchResults};
    use crate::model::space::space::{Space, CreateSpace, Spaces};

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

    pub async fn get_spaces(url: &str, token: String) -> Spaces {
        let request_url = format!("{url}/rest/api/space/");
        let client = reqwest::Client::new();
        let resp: Response = client.get(&request_url)
            .header("Authorization", format!("Basic {token}"))
            .header("Accept", "application/json")
            .send().await.unwrap();
        let body = resp.text().await.unwrap();
        return serde_json::from_str(body.as_str()).unwrap();
    }

    pub async fn create_space(conf_url: &str, token: &str, page: CreateSpace) -> Space {
        let request_url = format!("{conf_url}/rest/api/content/");
        let client = reqwest::Client::new();
        let res = client.post(&request_url)
            .json(&page)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Basic {token}"))
            .send()
            .await.unwrap();
        let created: String = res.text().await.unwrap();
        return serde_json::from_str(created.as_str()).unwrap();
    }
}