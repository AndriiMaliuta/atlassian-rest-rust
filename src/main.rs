mod helpers;
mod model;
mod jira;
mod confluence;

use std::fmt::format;
use std::future::Future;
use std::iter::Map;
use reqwest::{Body, Error};
use serde_json::{json, Value};
use tokio::time::Instant;
use crate::labels::labels::LabelService;
use crate::model::models::{Ancestor, CreatePage, CreatePageSpace, PageBody, Storage};
use crate::jira::jira_models::jira_models::{Assignee, CreateIssue, Fields, Issuetype, Priority, Project, Reporter};
use crate::pages::page_service::{create_page, get_children, get_descendants, get_page};
use crate::spaces::spaces::{SpaceService};


#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut start = Instant::now();
    println!("{}", "[ *** ] Starting");

    // data
    let token = base64::encode(b"admin:admin");
    let conf_url = "http://localhost:9500";

    let is = jira::jira::IssueService();

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

    let ps = jira::jira::ProjectService();
    let projects = ps.get_projects(conf_url, token).await;
    for proj in projects {
        println!("{:?}", proj);
    }


    // end
    let mut end: u128 = start.elapsed().as_millis();
    println!("{:?}", println!(">>> Action took :: {end} millis"));

    Ok(())
}
