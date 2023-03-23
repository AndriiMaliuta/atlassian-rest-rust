pub mod jira_models {
    use serde::{Deserialize, Serialize};

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[allow(non_snake_case)]
    pub struct Issue {
        pub id: String,
        pub key: String,
        pub proj_key: String,
        pub summary: String,
        pub description: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[allow(non_snake_case)]
    pub struct CreateIssue {
        #[serde(rename(serialize = "type"))]
        #[serde(rename(deserialize = "type"))]
        pub Type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub fields: Option<Fields>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[allow(non_snake_case)]
    pub struct Fields {

    }



}