#![feature(generic_associated_types)]

#[macro_use]
extern crate serde;

#[macro_use]
extern crate sval_derive;

#[macro_use]
extern crate valuable;

use miniserde::Serialize as MiniSerialize;

#[cfg(test)]
fn input_json() -> String {
    std::fs::read_to_string("./mini-twitter.json").unwrap()
}

#[cfg(test)]
fn input_struct() -> Twitter {
    let j = input_json();
    serde_json::from_str(&j).unwrap()
}

#[test]
fn sval_consistency() {
    let s = input_struct();

    assert_eq!(
        serde_json::to_string(&s).unwrap(),
        sval_json::to_string(&s).unwrap()
    );
}

#[test]
fn sval_erased_consistency() {
    use sval_erased as erased;

    let s = input_struct();

    assert_eq!(
        sval_json::to_string(&s).unwrap(),
        sval_json::to_string(&s as &dyn erased::Value).unwrap()
    );
}

struct TestReceiver;

impl<'a> sval::Receiver<'a> for TestReceiver {
    fn null(&mut self) -> sval::Result {
        println!(" null");
        Ok(())
    }

    fn u128(&mut self, value: u128) -> sval::Result {
        println!(" uint: {}", value);
        Ok(())
    }

    fn i128(&mut self, value: i128) -> sval::Result {
        println!(" int: {}", value);
        Ok(())
    }

    fn f64(&mut self, value: f64) -> sval::Result {
        println!(" num: {}", value);
        Ok(())
    }

    fn bool(&mut self, value: bool) -> sval::Result {
        println!(" bool: {}", value);
        Ok(())
    }

    fn text_begin(&mut self, num_bytes_hint: Option<usize>) -> sval::Result {
        println!(" text begin");
        Ok(())
    }

    fn text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
        println!(" text: {:?}", fragment);
        Ok(())
    }

    fn text_end(&mut self) -> sval::Result {
        println!(" text end");
        Ok(())
    }

    fn binary_begin(&mut self, num_bytes_hint: Option<usize>) -> sval::Result {
        println!(" binary begin");
        Ok(())
    }

    fn binary_fragment_computed(&mut self, fragment: &[u8]) -> sval::Result {
        println!(" binary: {:?}", fragment);
        Ok(())
    }

    fn binary_end(&mut self) -> sval::Result {
        println!(" binary end");
        Ok(())
    }

    fn map_begin(&mut self, num_entries_hint: Option<usize>) -> sval::Result {
        println!(" map begin");
        Ok(())
    }

    fn map_key_begin(&mut self) -> sval::Result {
        println!(" map key begin");
        Ok(())
    }

    fn map_key_end(&mut self) -> sval::Result {
        println!(" map key end");
        Ok(())
    }

    fn map_value_begin(&mut self) -> sval::Result {
        println!(" map value begin");
        Ok(())
    }

    fn map_value_end(&mut self) -> sval::Result {
        println!(" map value end");
        Ok(())
    }

    fn map_end(&mut self) -> sval::Result {
        println!(" map end");
        Ok(())
    }

    fn seq_begin(&mut self, num_elems_hint: Option<usize>) -> sval::Result {
        println!(" seq begin");
        Ok(())
    }

    fn seq_elem_begin(&mut self) -> sval::Result {
        println!(" seq elem begin");
        Ok(())
    }

    fn seq_elem_end(&mut self) -> sval::Result {
        println!(" seq elem end");
        Ok(())
    }

    fn seq_end(&mut self) -> sval::Result {
        println!(" seq end");
        Ok(())
    }
}

#[test]
fn sval_parse_simple() {
    use sval::Source;

    let mut json = sval_json::JsonBufReader::new(
        "{\"a\": true, \"b\": [true, false, null, {}], \"c\": {}, \"d\": []}",
    );

    while json
        .stream_resume(&mut TestReceiver)
        .expect("failed to stream")
        .is_continue()
    {
        println!("tick");
    }
}

#[derive(Debug, Serialize, Deserialize, MiniSerialize, Value, Valuable)]
pub struct Twitter {
    statuses: Vec<Status>,
    search_metadata: SearchMetadata,
}

#[derive(Debug, Serialize, Deserialize, MiniSerialize, Value, Valuable)]
pub struct Status {
    metadata: Metadata,
    created_at: String,
    id: u64,
    id_str: String,
    text: String,
    source: String,
    truncated: bool,
    in_reply_to_status_id: Option<u64>,
    in_reply_to_status_id_str: Option<String>,
    in_reply_to_user_id: Option<u32>,
    in_reply_to_user_id_str: Option<String>,
    in_reply_to_screen_name: Option<String>,
    user: User,
    geo: (),
    coordinates: (),
    place: (),
    contributors: (),
    retweeted_status: Option<Box<Status>>,
    retweet_count: u32,
    favorite_count: u32,
    entities: StatusEntities,
    favorited: bool,
    retweeted: bool,
    possibly_sensitive: Option<bool>,
    lang: String,
}

#[derive(Debug, Serialize, Deserialize, MiniSerialize, Value, Valuable)]
pub struct Metadata {
    result_type: String,
    iso_language_code: String,
}

#[derive(Debug, Serialize, Deserialize, MiniSerialize, Value, Valuable)]
pub struct User {
    id: u32,
    id_str: String,
    name: String,
    screen_name: String,
    location: String,
    description: String,
    url: Option<String>,
    entities: UserEntities,
    protected: bool,
    followers_count: u32,
    friends_count: u32,
    listed_count: u32,
    created_at: String,
    favourites_count: u32,
    utc_offset: Option<i32>,
    time_zone: Option<String>,
    geo_enabled: bool,
    verified: bool,
    statuses_count: u32,
    lang: String,
    contributors_enabled: bool,
    is_translator: bool,
    is_translation_enabled: bool,
    profile_background_color: String,
    profile_background_image_url: String,
    profile_background_image_url_https: String,
    profile_background_tile: bool,
    profile_image_url: String,
    profile_image_url_https: String,
    profile_banner_url: Option<String>,
    profile_link_color: String,
    profile_sidebar_border_color: String,
    profile_sidebar_fill_color: String,
    profile_text_color: String,
    profile_use_background_image: bool,
    default_profile: bool,
    default_profile_image: bool,
    following: bool,
    follow_request_sent: bool,
    notifications: bool,
}

#[derive(Debug, Serialize, Deserialize, MiniSerialize, Value, Valuable)]
pub struct UserEntities {
    url: Option<UserUrl>,
    description: UserEntitiesDescription,
}

#[derive(Debug, Serialize, Deserialize, MiniSerialize, Value, Valuable)]
pub struct UserUrl {
    urls: Vec<Url>,
}

#[derive(Debug, Serialize, Deserialize, MiniSerialize, Value, Valuable)]
pub struct Url {
    url: String,
    expanded_url: String,
    display_url: String,
    indices: Indices,
}

#[derive(Debug, Serialize, Deserialize, MiniSerialize, Value, Valuable)]
pub struct UserEntitiesDescription {
    urls: Vec<Url>,
}

#[derive(Debug, Serialize, Deserialize, MiniSerialize, Value, Valuable)]
pub struct StatusEntities {
    hashtags: Vec<Hashtag>,
    symbols: Vec<()>,
    urls: Vec<Url>,
    user_mentions: Vec<UserMention>,
    media: Option<Vec<Media>>,
}

#[derive(Debug, Serialize, Deserialize, MiniSerialize, Value, Valuable)]
pub struct Hashtag {
    text: String,
    indices: Indices,
}

#[derive(Debug, Serialize, Deserialize, MiniSerialize, Value, Valuable)]
pub struct UserMention {
    screen_name: String,
    name: String,
    id: u32,
    id_str: String,
    indices: Indices,
}

#[derive(Debug, Serialize, Deserialize, MiniSerialize, Value, Valuable)]
pub struct Media {
    id: u64,
    id_str: String,
    indices: Indices,
    media_url: String,
    media_url_https: String,
    url: String,
    display_url: String,
    expanded_url: String,
    #[serde(rename = "type")]
    #[sval(rename = "type")]
    media_type: String,
    sizes: Sizes,
    source_status_id: Option<u64>,
    source_status_id_str: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, MiniSerialize, Value, Valuable)]
pub struct Sizes {
    medium: Size,
    small: Size,
    thumb: Size,
    large: Size,
}

#[derive(Debug, Serialize, Deserialize, MiniSerialize, Value, Valuable)]
pub struct Size {
    w: u16,
    h: u16,
    resize: String,
}

pub type Indices = (u8, u8);

#[derive(Debug, Serialize, Deserialize, MiniSerialize, Value, Valuable)]
pub struct SearchMetadata {
    completed_in: f32,
    max_id: u64,
    max_id_str: String,
    next_results: String,
    query: String,
    refresh_url: String,
    count: u8,
    since_id: u64,
    since_id_str: String,
}
