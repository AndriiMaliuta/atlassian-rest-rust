mod space;

pub mod models {
    use serde_json::{json, Value};
    use serde::{Deserialize, Serialize};

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[allow(non_snake_case)]
    pub struct Extentions<T> {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub position: Option<T>,
    }

    // impl Extentions {
    //     pub fn is_empty(option: Option<Extentions>) -> bool {
    //         if option == None { true} else { false }
    //     }
    // }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[allow(non_snake_case)]
    pub struct Content {
        id: String,
        #[serde(rename(serialize = "type"))]
        #[serde(rename(deserialize = "type"))]
        Type: String,
        status: String,
        title: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        extensions: Option<Extentions<String>>,
        #[serde(rename(deserialize = "_expandable"))]
        _expandable: Expandable,
        version: Version,
        #[serde(rename(deserialize = "history"))]
        history: Option<CntHistory>,
        _links: ContentLinks,
    }
    
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[allow(non_snake_case)]
    pub struct Expandable {
        pub container: String,
        pub metadata: String,
        pub operations: String,
        pub children: String,
        pub restrictions: String,
        pub ancestors: String,
        pub body: String,
        pub descendants: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[allow(non_snake_case)]
    pub struct ContentLinks {
        #[serde(rename(serialize = "self"))]
        #[serde(rename(deserialize = "self"))]
        pub sself: String,
        pub webui: String,
        pub edit: String,
        pub tinyui: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[allow(non_snake_case)]
    pub struct Links {
        #[serde(rename(serialize = "self"))]
        #[serde(rename(deserialize = "self"))]
        #[serde(skip_serializing_if = "String::is_empty")]
        pub sself: String,
        #[serde(skip_serializing_if = "String::is_empty")]
        // #[serde(skip_deserializing_if = "String::is_empty")]
        pub next: String,
        pub base: String,
        pub context: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[allow(non_snake_case)]
    pub struct SearchResultsLinks {
        pub base: String,
        pub context: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[allow(non_snake_case)]
    pub struct ContentArrPage {
        pub id: String,
        #[serde(rename(serialize = "type"))]
        #[serde(rename(deserialize = "type"))]
        pub Type: String,
        pub status: String,
        pub title: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub extensions: Option<Extentions<String>>,
        #[serde(rename(deserialize = "_expandable"))]
        pub _expandable: Expandable,
        pub _links: ContentLinks,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[allow(non_snake_case)]
    pub struct ContentResponse {
        pub results: Vec<ContentArrPage>,
        pub start: i8,
        pub limit: i8,
        pub size: i8,
        // #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(skip)]
        pub _links: Links,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct User {
        pub login: String,
        pub id: u32,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct CreatePage {
        pub(crate) title: String,
        #[serde(rename(serialize = "type"))]
        #[serde(rename(deserialize = "type"))]
        pub(crate) ctype: String,
        pub(crate) space: CreatePageSpace,
        pub(crate) body: PageBody,
        pub(crate) ancestors: Vec<Ancestor>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct PageBody {
        pub(crate) storage: Storage,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Storage {
        pub representation: String,
        pub value: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct CreatePageSpace {
        pub(crate) key: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Ancestor {
        pub(crate) id: i32,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CntSpace {
        pub id: i64,
        pub key: String,
        pub name: String,
        #[serde(rename = "type")]
        pub type_field: String,
        #[serde(rename = "_links")]
        #[serde(skip)]
        pub links: Links,
        // #[serde(rename = "_expandable")]
        #[serde(skip)]
        pub expandable: CntExpandable,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CntHistory {
        pub latest: bool,
        #[serde(rename = "createdBy")]
        pub created_by: CreatedBy,
        #[serde(rename = "createdDate")]
        pub created_date: String,
        #[serde(rename = "_links")]
        #[serde(skip)]
        pub links: Links,
        // #[serde(rename = "_expandable")]
        #[serde(skip)]
        pub expandable: CntExpandable,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CreatedBy {
        #[serde(rename = "type")]
        pub type_field: String,
        pub profile_picture: ProfilePicture,
        pub display_name: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ProfilePicture {
        pub path: String,
        pub width: i64,
        pub height: i64,
        pub is_default: bool,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Version {
        pub by: By,
        pub when: String,
        pub message: String,
        pub number: i64,
        pub minor_edit: bool,
        pub hidden: bool,
        #[serde(rename = "_links")]
        #[serde(skip)]
        pub links: Links,
        // #[serde(rename = "_expandable")]
        #[serde(skip)]
        pub expandable: CntExpandable,
    }

    #[serde(rename_all = "camelCase")]
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct By {
        #[serde(rename = "type")]
        pub type_field: String,
        pub profile_picture: ProfilePicture,
        pub display_name: String,
    }


    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CntExpandable {
        #[serde(skip_serializing_if = "String::is_empty")]
        pub container: String,
        pub metadata: String,
        pub operations: String,
        pub children: String,
        pub restrictions: String,
        pub ancestors: String,
        pub body: String,
        pub descendants: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SearchResults {
        pub results: Vec<ContentArrPage>,
        pub start: i8,
        pub limit: i8,
        pub size: i8,
        #[serde(rename = "totalSize")]
        pub total_size: i8,
        #[serde(rename = "cqlQuery")]
        pub cql_query: String,
        #[serde(skip)]
        pub _links: Links,
    }
}
