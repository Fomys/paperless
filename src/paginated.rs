use crate::paperless::Paperless;
use reqwest::{Method, Url};
use serde::de::DeserializeOwned;
use serde::Deserialize;

#[derive(Deserialize)]
struct PaginatedResult<T> {
    #[serde(rename = "count")]
    _count: u64,
    next: Option<String>,
    #[serde(rename = "previous")]
    _previous: Option<String>,
    results: Vec<T>,
}

pub struct Paginated<'p, T> {
    paperless: &'p Paperless,
    url: Url,
    last_result: Option<PaginatedResult<T>>,
    current_index: usize,
}

impl<'p, T> Paginated<'p, T> {
    pub fn new(paperless: &'p Paperless, url: Url) -> Self {
        Self {
            paperless,
            url,
            last_result: None,
            current_index: 0,
        }
    }
}
impl<'p, T> Paginated<'p, T>
where
    T: DeserializeOwned,
{
    fn fetch_next(&mut self) -> Result<(), reqwest::Error> {
        let next_url = if let Some(last) = &self.last_result {
            match &last.next {
                None => None,
                Some(s) => {
                    let mut url = Url::parse(s).unwrap(); // TODO: fix this unwrap
                    url.set_scheme("https").unwrap();
                    Some(url)
                }
            }
        } else {
            Some(self.url.clone())
        };

        match next_url {
            None => {}
            Some(path) => {
                println!("{} ", path.to_string());
                self.last_result = Some(
                    self.paperless
                        .http_client
                        .execute(self.paperless.request(Method::GET, path))?
                        .json()?,
                );
                self.current_index = 0;
            }
        }
        Ok(())
    }
}

impl<'p, T> Iterator for Paginated<'p, T>
where
    T: DeserializeOwned,
{
    type Item = Result<T, reqwest::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.last_result {
            None => match self.fetch_next() {
                Ok(_) => {}
                Err(e) => return Some(Err(e)),
            },
            Some(last) => {
                if last.results.is_empty() {
                    match self.fetch_next() {
                        Ok(_) => {}
                        Err(e) => return Some(Err(e)),
                    }
                }
            }
        }

        match &mut self.last_result {
            None => None,
            Some(last) => {
                if last.results.is_empty() {
                    None
                } else {
                    Some(Ok(last.results.remove(0)))
                }
            }
        }
    }
}
