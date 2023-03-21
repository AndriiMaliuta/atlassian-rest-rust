pub mod space {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Space {
        pub id: i64,
        pub key: String,
        pub name: String,
        #[serde(rename(serialize = "type"))]
        #[serde(rename(deserialize = "type"))]
        pub ttype: String,
        pub _links: SpaceLinks,
        pub _expandable: SpaceExpandable
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Spaces {
        pub results: Vec<SpaceResult>,
        pub start: i8,
        pub limit: i8,
        pub size: i8,
        pub _links: SpacesLinks,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct SpaceResult {
        pub id: i64,
        pub key: String,
        pub name: String,
        #[serde(rename(serialize = "type"))]
        #[serde(rename(deserialize = "type"))]
        pub ttype: String,
        pub _links: SpaceResultLinks,
        pub _expandable: SpaceExpandable
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct SpaceLinks {
        pub webui: String,
        pub collection: String,
        pub base: String,
        #[serde(skip_serializing_if = "String::is_empty")]
        pub context: String,
        #[serde(rename(serialize = "self"))]
        #[serde(rename(deserialize = "self"))]
        #[serde(skip_serializing_if = "String::is_empty")]
        pub sself: String,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct SpaceResultLinks {
        pub webui: String,
        #[serde(rename(serialize = "self"))]
        #[serde(rename(deserialize = "self"))]
        #[serde(skip_serializing_if = "String::is_empty")]
        pub sself: String,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct SpacesLinks {
        #[serde(rename(serialize = "self"))]
        #[serde(rename(deserialize = "self"))]
        #[serde(skip_serializing_if = "String::is_empty")]
        pub sself: String,
        pub next: String,
        pub base: String,
        #[serde(skip_serializing_if = "String::is_empty")]
        pub context: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[allow(non_snake_case)]
    pub struct SpaceExpandable {
        pub metadata: String,
        pub icon: String,
        pub description: String,
        #[serde(rename(serialize = "retentionPolicy"))]
        #[serde(rename(deserialize = "retentionPolicy"))]
        pub retention_policy: String,
        pub homepage: String,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct CreateSpace {
        pub key: String,
        pub name: String,
    }

}