mod pages;
mod helpers;
mod model;
mod spaces;

use std::fmt::format;
use std::future::Future;
use std::iter::Map;
use reqwest::{Body, Error};
use serde_json::{json, Value};
use tokio::time::Instant;
use crate::model::models::{Ancestor, CreatePage, CreatePageSpace, PageBody, Storage};
use crate::pages::page_service::{create_page, get_children, get_descendants, get_page};
use crate::spaces::spaces::get_space;


#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut start = Instant::now();
    println!("{}", "[ *** ] Starting");

    // data
    let token = base64::encode(b"admin:admin");
    let conf_url = "http://localhost:8110";

    // get page
    // let pages = get_descendants(conf_url, token, "1213317".to_string()).await;
    // pages.results.iter().for_each(|p| println!("{:?}", p.title));

    // get space
    let space = get_space(conf_url, token, "dev16".to_string()).await;

    // CREATE PAGE
    // let space_key = "dev16";
    // let parent = 1213317;
    //
    // for a in 40..42 {
    //     let title = format!("Rust page {a}");
    //
    //     let req = CreatePage {
    //         title: title.to_string(),
    //         ctype: "page".to_string(),
    //         space: CreatePageSpace {
    //             key: space_key.to_string(),
    //         },
    //         body: PageBody {
    //             storage: Storage {
    //                 representation: "storage".to_string(),
    //                 value: helpers::helpers::rand_string(30).to_string(),
    //             },
    //         },
    //         ancestors: vec![Ancestor {
    //             id: parent,
    //         }],
    //     };
    //     let resp = create_page(&conf_url, &token, req).await;
    //     println!("{:?}", resp);
    // }



    let mut end: u128 = start.elapsed().as_millis();
    println!("{:?}", println!(">>> Action took :: {end} millis"));

    Ok(())
}


