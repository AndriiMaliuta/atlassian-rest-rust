pub mod models {
    use serde_json::{json, Value};
    use serde::{Deserialize, Serialize};

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[allow(non_snake_case)]
    pub struct Extentions<T> {
        #[serde(skip_serializing_if = "Option::is_none")]
        position: Option<T>,
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
        container: String,
        metadata: String,
        operations: String,
        children: String,
        restrictions: String,
        ancestors: String,
        body: String,
        descendants: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[allow(non_snake_case)]
    pub struct ContentLinks {
        #[serde(rename(serialize = "self"))]
        #[serde(rename(deserialize = "self"))]
        sself: String,
        webui: String,
        edit: String,
        tinyui: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[allow(non_snake_case)]
    pub struct Links {
        #[serde(rename(serialize = "self"))]
        #[serde(rename(deserialize = "self"))]
        sself: String,
        #[serde(skip_serializing_if = "String::is_empty")]
        // #[serde(skip_deserializing_if = "String::is_empty")]
        next: String,
        base: String,
        context: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[allow(non_snake_case)]
    pub struct ContentArrPage {
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
        _links: ContentLinks,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[allow(non_snake_case)]
    pub struct ContentResponse {
        results: Vec<ContentArrPage>,
        start: i8,
        limit: i8,
        size: i8,
        // #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(skip)]
        _links: Links,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct User {
        login: String,
        id: u32,
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
}
