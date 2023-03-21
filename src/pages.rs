pub mod page_service {
    use reqwest::Response;
    use crate::model::models::{Content, CreatePage, ContentResponse, SearchResults};


    pub async fn get_page(url: &str, token: String, id: String) -> Content {
        let request_url = format!("{url}/rest/api/content/{id}");
        let client = reqwest::Client::new();
        let resp: Response = client.get(&request_url)
            .header("Authorization", format!("Basic {token}"))
            .header("Accept", "application/json")
            .send().await.unwrap();
        let body = resp.text().await.unwrap();
        return serde_json::from_str(body.as_str()).unwrap();
    }

    pub async fn get_children(url: &str, token: String, id: String) -> ContentResponse {
        let request_url = format!("{url}/rest/api/content/{id}/child/page");
        let client = reqwest::Client::new();
        let resp: Response = client.get(&request_url)
            .header("Authorization", format!("Basic {token}"))
            .header("Accept", "application/json")
            .send().await.unwrap();
        let body = resp.text().await.unwrap();
        return serde_json::from_str(body.as_str()).unwrap();
    }

    pub async fn get_descendants(url: &str, token: String, id: String) -> SearchResults {
        let request_url = format!("{url}/rest/api/content/search?cql=parent={id}");
        let client = reqwest::Client::new();
        let resp: Response = client.get(&request_url)
            .header("Authorization", format!("Basic {token}"))
            .header("Accept", "application/json")
            .send().await.unwrap();
        let body = resp.text().await.unwrap();
        return serde_json::from_str(body.as_str()).unwrap();
    }

    pub async fn create_page(conf_url: &str, token: &str, page: CreatePage) -> String {
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

    pub async fn get_comments(conf_url: &str, token: &str) -> SearchResults {
        let request_url = format!("{conf_url}/rest/api/search?cql=type=comment");
        let client = reqwest::Client::new();
        let res = client.get(&request_url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Basic {token}"))
            .send()
            .await.unwrap();
        return serde_json::from_str(res.text().await.unwrap().as_str()).unwrap();
    }

    // todo
    pub async fn copy_page(conf_url: &str, token: &str, page: CreatePage) -> String {
        return String::from("");
    }
    // todo
    pub async fn move_page(conf_url: &str, token: &str, page: CreatePage) -> String {
        return String::from("");
    }


}