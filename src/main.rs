mod pages;
mod helpers;
mod models;

use std::fmt::format;
use std::future::Future;
use std::iter::Map;
use reqwest::{Body, Error};
use serde_json::{json, Value};
use tokio::time::Instant;
use crate::models::models::{Ancestor, create_page, CreatePage, CreatePageSpace, PageBody, Storage};
use crate::pages::page_service::create_page;


#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut start = Instant::now();
    println!("{}", "[ *** ] Starting");

    // data
    let token = base64::encode(b"admin:admin");
    let conf_url = "http://localhost:8110";

    // CREATE PAGE
    let req = CreatePage {
        title: "Rust page 2".to_string(),
        ctype: "page".to_string(),
        space: CreatePageSpace {
            key: "dev16".to_string(),
        },
        body: PageBody {
            storage: Storage {
                representation: "storage".to_string(),
                value: "lorem...".to_string(),
            },
        },
        ancestors: vec![Ancestor {
            id: 1213317,
        }],
    };
    let resp = create_page(&conf_url, &token, req).await;
    println!("{:?}", resp);


    // create several pages
    // for a in 21..=40 {
    //     let mut to_create: CreatePage = CreatePage {
    //         title: format!("RUST page {a}"),
    //         ctype: "page".to_string(),
    //         space: CreatePageSpace {
    //             key: "DEV12".to_string(),
    //         },
    //         body: PageBody {
    //             storage: Storage {
    //                 representation: "storage".to_string(),
    //                 value: rand_string(100),
    //             }
    //         },
    //         ancestors: vec![Ancestor {
    //             id: 1048683
    //         }]
    //     };
    //
    //     println!("{:?}", serde_json::to_string(&to_create));
    //
    //     let created = create_page(&conf_url.to_string(), &token,to_create);
    //     let fin = created.await;
    // }

    let mut end: u128 = start.elapsed().as_millis();
    println!("{:?}", println!(">>> Action took :: {end} millis"));

    Ok(())
}


