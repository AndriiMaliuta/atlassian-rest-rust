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
    #[serde(rename_all = "camelCase")]
    pub struct CreateIssue {
        pub fields: Fields,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct EditIssue {
        pub fields: Fields,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Fields {
        pub project: Project,
        pub summary: String,
        pub issuetype: Issuetype,
        pub assignee: Assignee,
        pub reporter: Reporter,
        pub priority: Priority,
        pub labels: Vec<String>,
        pub description: String,
        pub duedate: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Project {
        pub id: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Issuetype {
        pub id: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Assignee {
        pub name: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Reporter {
        pub name: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Priority {
        pub id: String,
    }

    // ===================== projects
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ProjectCore {
        pub expand: String,
        #[serde(rename = "self")]
        pub self_field: String,
        pub id: String,
        pub key: String,
        pub name: String,
        pub avatar_urls: AvatarUrls,
        pub project_type_key: String,
        pub archived: bool,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AvatarUrls {
        #[serde(rename = "48x48")]
        pub n48x48: String,
        #[serde(rename = "24x24")]
        pub n24x24: String,
        #[serde(rename = "16x16")]
        pub n16x16: String,
        #[serde(rename = "32x32")]
        pub n32x32: String,
    }

    // workflow
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ProjectWorkflow {
        pub name: String,
        pub description: String,
        pub steps: i16,
        pub default: bool,
        #[serde(rename = "lastModifiedDate")]
        pub last_modified_date: String,
        #[serde(rename = "lastModifiedUser")]
        pub last_modified_user: String,
    }

    use serde_derive::Deserialize;
    use serde_derive::Serialize;

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Attachment {
        #[serde(rename = "self")]
        pub self_field: String,
        pub filename: String,
        pub author: Author,
        pub created: String,
        pub size: i64,
        pub mime_type: String,
        pub content: String,
        pub thumbnail: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Author {
        #[serde(rename = "self")]
        pub self_field: String,
        pub key: String,
        pub name: String,
        pub email_address: String,
        pub avatar_urls: AvatarUrls,
        pub display_name: String,
        pub active: bool,
        pub deleted: bool,
        pub time_zone: String,
        pub locale: String,
    }

    //

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CreateField {
        pub id: String,
        pub name: String,
        pub description: String,
        #[serde(rename = "type")]
        pub type_field: String,
        pub searcher_key: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FieldCore {
        pub id: String,
        pub name: String,
        pub custom: bool,
        pub orderable: bool,
        pub navigable: bool,
        pub searchable: bool,
        pub clause_names: Vec<String>,
        pub schema: Schema,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Schema {
        #[serde(rename = "type")]
        pub type_field: String,
        pub system: String,
    }

}



}