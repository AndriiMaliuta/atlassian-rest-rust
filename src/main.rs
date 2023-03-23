mod pages;
mod helpers;
mod model;
mod spaces;
mod labels;
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
use crate::jira::jira_models::jira_models::CreateIssue;
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
    let result = is.create_issue(conf_url, token, CreateIssue {
        Type: "".to_string(),
        fields: None,
    });
    println!("{:?}", result.await);


    // end
    let mut end: u128 = start.elapsed().as_millis();
    println!("{:?}", println!(">>> Action took :: {end} millis"));

    Ok(())
}


