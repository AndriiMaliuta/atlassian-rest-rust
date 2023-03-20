pub mod models {
    use serde_json::{json, Value};
    use serde::{Deserialize, Serialize};

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[allow(non_snake_case)]
    pub struct Extentions {
        // #[serde(skip_serializing_if = "String::is_empty")]
        position: i32,
    }

// #[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
// #[repr(u8)]
// enum PositionEnum {
//     Str = 2,
//     Numm = 3,
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
        extensions: Extentions,
        #[serde(rename(deserialize = "_expandable"))]
        _expandable: Expandable,
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
        history: String,
        ancestors: String,
        body: String,
        version: String,
        descendants: String,
        space: String,
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
    pub struct ContentResponse {
        results: Vec<Content>,
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

    // One content page
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CntPage {
        pub id: String,
        #[serde(rename = "type")]
        pub type_field: String,
        pub status: String,
        pub title: String,
        pub space: CntSpace,
        pub history: CntHistory,
        pub version: Version,
        pub extensions: Extentions,
        #[serde(rename = "_links")]
        #[serde(skip)]
        pub links: Links,
        #[serde(skip)]
        // #[serde(rename = "_expandable")]
        pub expandable: Expandable,
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
        pub created_by: CreatedBy,
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