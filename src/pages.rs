pub mod page_service {
    use reqwest::Response;
    use crate::model::models::{Content, CreatePage, ContentResponse, SearchResults, CreatePageSpace, PageBody, Storage};

    pub async fn get_page(url: &str, token: String, id: String) -> Content {
        let request_url = format!("{url}/rest/api/content/{id}?expand=body.storage.value,ancestors");
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
    pub async fn copy_page(conf_url: &str, token: &str, page_id: String) -> String {
        let req_url = format!("{conf_url}/rest/api/content/");
        let page = get_page(conf_url, token.to_string(), page_id).await;
        let client = reqwest::Client::new();
        let res = client.post(&req_url)
            .json(&CreatePage {
                title: page.id,
                ctype: String::from("page"),
                space: CreatePageSpace {
                    key: page.space.key,
                },
                body: PageBody {
                    storage: Storage {
                        representation: String::from("storage"),
                        value: page.body.storage.value,
                    },
                },
                ancestors: page.a,
            })
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Basic {token}"))
            .send()
            .await.unwrap();
        return res.text().await.unwrap();
    }

    // todo
    pub async fn move_page(conf_url: &str, token: &str, page: CreatePage) -> String {
        return String::from("");
    }

    // todo
    // {url}/rest/api/content/{id}/child/attachment
    pub async fn get_page_attachments() {
        // /download/attachments/{page_id}/{file]?version=1&modificationDate=1679505520993&api=v2

    }
}


}