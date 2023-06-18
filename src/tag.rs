use hex_color::HexColor;
use reqwest::Url;
use serde::Deserialize;

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct Id(u64);

impl From<u64> for Id {
    fn from(value: u64) -> Self {
        Self(value)
    }
}
impl From<Id> for u64 {
    fn from(value: Id) -> Self {
        value.0
    }
}
impl ToString for Id {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Debug, Deserialize)]
pub struct Tag {
    pub id: Id,
    pub slug: String,
    pub name: String,
    pub color: HexColor,
    pub text_color: HexColor,
    #[serde(rename = "match")]
    pub match_: String,
    pub matching_algorithm: u64,
    pub is_insensitive: bool,
    pub is_inbox_tag: bool,
    pub document_count: u64,
}

#[derive(Debug, Default)]
pub struct Filter {
    name_starts_with: Option<String>,
    name_ends_with: Option<String>,
    name_contains: Option<String>,
    name_is: Option<String>,
}

impl Filter {
    pub fn insert_query(self, url: &mut Url) {
        url.query_pairs_mut()
            .append_pair(
                "name__istartswith",
                &self.name_starts_with.unwrap_or_default(),
            )
            .append_pair("name__iendswith", &self.name_ends_with.unwrap_or_default())
            .append_pair("name__icontains", &self.name_contains.unwrap_or_default())
            .append_pair("name__iexact", &self.name_is.unwrap_or_default());
    }
}
