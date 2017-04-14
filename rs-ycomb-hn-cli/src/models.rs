use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde_json;
use utils::*;

#[derive(Serialize)]
pub struct HnTopStories {
    pub values: Vec<i32>,
}

impl Deserialize for HnTopStories {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        Deserialize::deserialize(deserializer).map(|arr: Vec<i32>| HnTopStories { values: arr })
    }
}

#[derive(Serialize, Deserialize)]
pub struct HnItem {
    pub by: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub descendants: Option<i32>,
    pub id: i32,
    #[serde(skip_serializing_if="Option::is_none")]
    pub kids: Option<Vec<i32>>,
    pub title: String,
    pub score: i32,
    pub time: f64,
    #[serde(rename(deserialize = "type"))]
    pub type_str: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct HnUser {
    pub about: String,
    pub created: f64,
    pub id: String,
    pub karma: i32,
    pub submitted: Vec<i32>,
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hn_item_serde_test() {
        use std::fs::File;
        use std::io::prelude::*;
        let mut contents = String::new();
        File::open("res/test/item.json")
            .and_then(|mut file| file.read_to_string(&mut contents))
            .unwrap();
        let deserialized: HnItem = serde_json::from_str(&contents).unwrap();
        assert_eq!(71, deserialized.descendants.unwrap());
        assert_eq!("dhouston", deserialized.by);
        assert_eq!(8863, deserialized.id);
        assert_eq!(111, deserialized.score);
        assert_eq!(1175714200.0f64, deserialized.time);
        assert_eq!("My YC app: Dropbox - Throw away your USB drive",
                   deserialized.title);
        assert_eq!("story", deserialized.type_str);
        assert_eq!("http://www.getdropbox.com/u/2/screencast.html",
                   deserialized.url.unwrap());
    }
    #[test]
    fn hn_top_stories_serde_test() {
        use std::fs::File;
        use std::io::prelude::*;
        let mut contents = String::new();
        File::open("res/test/top-stories.json")
            .and_then(|mut file| file.read_to_string(&mut contents))
            .unwrap();
        let deserialized: HnTopStories = serde_json::from_str(&contents).unwrap();
        assert!(deserialized.values.len() > 3);
    }
    #[test]
    fn hn_user_serde_test() {
        use std::fs::File;
        use std::io::prelude::*;
        let mut contents = String::new();
        File::open("res/test/user.json")
            .and_then(|mut file| file.read_to_string(&mut contents))
            .unwrap();
        let deserialized: HnUser = serde_json::from_str(&contents).unwrap();
        assert_eq!("This is a test", deserialized.about);
        assert_eq!(1173923446.0f64, deserialized.created);
        assert_eq!("jl", deserialized.id);
        assert_eq!(3496, deserialized.karma);
        assert!(deserialized.submitted.len() > 3);
    }
}