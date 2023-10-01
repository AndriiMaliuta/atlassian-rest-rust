mod helpers;
mod model;
mod jira;
mod confluence;

use std::fmt::format;
use std::future::Future;
use std::iter::Map;
use base64::Engine;
use reqwest::{Body, Error};
use serde_json::{json, Value};
use tokio::time::Instant;
use crate::model::models::{Ancestor, CreatePage, CreatePageSpace, PageBody, Storage};
use crate::jira::jira_models::jira_models::{Assignee, CreateIssue, Fields, Issuetype, Priority, Project, Reporter};


#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut start = Instant::now();
    println!("{}", "[ *** ] Starting");

    let host = std::env::var("ATLAS_URL").unwrap();
    let token = std::env::var("ATLAS_TOKEN").unwrap();

    let page =
        confluence::pages::page_service::get_page(host.as_str(), token, "519276711".to_string()).await;

    println!("{:?}", page);

    // end
    let mut end: u128 = start.elapsed().as_millis();
    println!("{:?}", println!(">>> Action took :: {end} millis"));

    Ok(())
}

async fn run_jira(token: String, conf_url: &str) {
// let result = is.create_issue(conf_url, token, CreateIssue {
    //     fields: Fields {
    //         project: Project { id: "10000".to_string() },
    //         summary: "test".to_string(),
    //         issuetype: Issuetype{ id: "10006".to_string() },
    //         assignee: Assignee{ name: "admin".to_string() },
    //         reporter: Reporter{ name: "admin".to_string() },
    //         priority: Priority{ id: "3".to_string() },
    //         labels: vec![],
    //         description: "test".to_string(),
    //         duedate: chrono::NaiveDate::from_yo_opt(2023, 91).unwrap().to_string(),
    //     },
    // });
    // println!("{:?}", result.await);

    // ============ GET issue
    // let resp = is.get_issue(conf_url, token, "PROC-1".to_string()).await;
    // println!("{:?}", resp);

    let is = jira::jira::IssueService();
    let ps = jira::jira::ProjectService();

    let projects = ps.get_projects(conf_url, token.as_str()).await;
    for proj in projects {
        let result = is.create_issue(conf_url, token.as_str(), CreateIssue {
            fields: Fields {
                project: Project { id: proj.id },
                summary: "test".to_string(),
                issuetype: Issuetype { id: "10006".to_string() },
                assignee: Assignee { name: "admin".to_string() },
                reporter: Reporter { name: "admin".to_string() },
                priority: Priority { id: "3".to_string() },
                labels: vec![],
                description: "test".to_string(),
                duedate: chrono::NaiveDate::from_yo_opt(2023, 91).unwrap().to_string(),
            },
        });
        println!("{:?}", result.await);
    }
}
