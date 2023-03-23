pub mod labels {
    use reqwest::Response;
    use serde_json::{json, Value};
    use serde::{Deserialize, Serialize};

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[allow(non_snake_case)]
    pub struct Label {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[allow(non_snake_case)]
    pub struct AddLabel {
        pub prefix: String,
        pub name: String,
    }

    pub struct LabelService {
        pub labels: Vec<Label>,
    }

    impl LabelService {
        pub async fn add_label(&self, url: &str, token: String, id: String, labels: Vec<String>) -> String {
            let mut labels_vec: Vec<AddLabel> = vec![];
            for label in labels {
                labels_vec.push(AddLabel {
                    prefix: String::from("global"),
                    name: label,
                });
            }
            let request_url = format!("{url}/rest/api/content/{id}/label/");
            let client = reqwest::Client::new();
            let resp: Response = client.post(&request_url)
                .json(&labels_vec)
                .header("Authorization", format!("Basic {token}"))
                .header("Accept", "application/json")
                .header("Content-Type", "application/json")
                .send().await.unwrap();
            let body = resp.text().await.unwrap();
            return body;
        }
    }
}