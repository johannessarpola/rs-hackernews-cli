use utils;

pub struct HnNewsEndpoint {
    base_url: String,
    top_news_suffix: String,
    item_suffix: String,
    max_item_suffix: String,
    new_items_suffix: String,
    best_items_suffix: String,
    json_suffix: String,
}

impl HnNewsEndpoint {
    pub fn build_default() -> HnNewsEndpoint {
        let e = HnNewsEndpoint {
            base_url: String::from("https://hacker-news.firebaseio.com/v0/"),
            top_news_suffix: String::from("topstories"),
            item_suffix: String::from("item/"),
            max_item_suffix: String::from("maxitem"),
            new_items_suffix: String::from("newstories"),
            best_items_suffix: String::from("beststories"),
            json_suffix: String::from(".json"),
        };
        e
    }

    pub fn get_top_stories_path(&self) -> String {
        utils::combine_strings(vec![&self.base_url, &self.top_news_suffix, &self.json_suffix])
    }
    pub fn get_max_item_path(&self) -> String {
        utils::combine_strings(vec![&self.base_url, &self.max_item_suffix, &self.json_suffix])
    }
    pub fn get_item_path(&self, id: &str) -> String {
        utils::combine_strings(vec![&self.base_url, &self.item_suffix, id, &self.json_suffix])
    }
    pub fn get_new_stories_path(&self, id: &str) -> String {
        utils::combine_strings(vec![&self.base_url, &self.new_items_suffix, id, &self.json_suffix])
    }
    pub fn get_best_stories_path(&self) -> String {
        utils::combine_strings(vec![&self.base_url, &self.best_items_suffix, &self.json_suffix])
    }
}
