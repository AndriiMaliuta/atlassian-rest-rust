Work in pregress...

Implementation of Atlassian Confluence and Jira client using RUST.

```rust
let token = base64::encode(b"admin:admin");
let conf_url = "http://localhost:8110";

// =============== get page
let pages = get_descendants(conf_url, token, "1213317".to_string()).await;
pages.results.iter().for_each(|p| println!("{:?}", p.title));

// =============== CREATE 20 pages
let space_key = "dev3";
let parent = 1212664;

for a in 1..20 {
    let title = format!("Rust page {a}");

    let req = CreatePage {
        title: title.to_string(),
        ctype: "page".to_string(),
        space: CreatePageSpace {
            key: space_key.to_string(),
        },
        body: PageBody {
            storage: Storage {
                representation: "storage".to_string(),
                value: helpers::helpers::rand_string(30).to_string(),
            },
        },
        ancestors: vec![Ancestor {
            id: parent,
        }],
    };
    let resp = create_page(&conf_url, &token, req).await;
    println!("{:?}", resp);
}
```