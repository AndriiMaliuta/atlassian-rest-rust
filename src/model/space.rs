use serde::{Deserialize, Serialize};
use crate::model::models::{Links};

#[derive(Deserialize, Serialize, Debug)]
pub struct Space {
    id: i16,
    key: String,
    name: String,
    #[serde(rename(serialize = "type"))]
    #[serde(rename(deserialize = "type"))]
    ttype: String,
    _links: Links,
    _expandable: SpaceExpandable
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
    key: String,
    name: String,
}
