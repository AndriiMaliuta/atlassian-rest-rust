
async fn run() {
    // =============== get page
    // let pages = get_descendants(conf_url, token, "1213317".to_string()).await;
    // pages.results.iter().for_each(|p| println!("{:?}", p.title));

    // =============== get space
    // let space = get_space(conf_url, token, "dev16".to_string()).await;
    // println!("{:?}", space);

    // =============== get spaces
    // let mut space_service = SpaceService { spaces: vec![] };
    // let vec1 = space_service.get_spaces(&conf_url, &token).await;
    // println!("{:?}", vec1);

    // =============== CREATE PAGEs
    // let space_key = "dev3";
    // let parent = 1212664;
    //
    // for a in 1..20 {
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

    // ========= add labels
    // let ls = LabelService{labels: vec![]};
    // let label = String::from("bbb");
    // ls.add_label(conf_url, token, "2523141".to_string(), vec![label]).await;
}