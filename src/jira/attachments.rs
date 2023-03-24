pub mod attach {
    use reqwest::Response;

    pub struct IssueAttachments();

    impl IssueAttachments {
        // GET /rest/api/2/attachment/{id}/expand/human
        // GET /rest/api/2/attachment/{id}/expand/raw

        pub async fn get_attachment(&self, url: &str, token: String, id: String) -> String {
            let req_url = format!("{url}/rest/api/2/attachment/{id}");
            let client = reqwest::Client::new();
            let resp: Response = client.get(&req_url)
                .header("Authorization", format!("Basic {token}"))
                .header("Accept", "application/json")
                .header("Content-Type", "application/json")
                .send().await.unwrap();
            let body = resp.text().await.unwrap();
            return body;
        }

        pub async fn delete_attachment(&self, url: &str, token: String, id: String) -> String {
            let req_url = format!("{url}/rest/api/2/attachment/{id}");
            let client = reqwest::Client::new();
            let resp: Response = client.delete(&req_url)
                .header("Authorization", format!("Basic {token}"))
                .header("Accept", "application/json")
                .header("Content-Type", "application/json")
                .send().await.unwrap();
            let body = resp.text().await.unwrap();
            return body;
        }

        // Get attachment meta
        // GET /rest/api/2/attachment/meta
    }
}