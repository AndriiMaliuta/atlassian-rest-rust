pub mod jira_models {
    pub struct Issue {
        pub id: String,
        pub key: String,
        pub proj_key: String,
        pub summary: String,
        pub description: String,
    }
}