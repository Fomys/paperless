use crate::{correspondent, document, document_type, saved_view, tag, Paginated};

use reqwest::blocking::{Client, Request};
use reqwest::header::HeaderValue;
use reqwest::{Method, Url};

pub struct Paperless {
    pub(crate) http_client: Client,
    root: Url,
    token: HeaderValue,
}

impl Paperless {
    /// Create a new instance of paperless API
    ///
    /// # Arguments
    ///
    /// * `root` - Root URL of the api, for example `https://paperless.com/api/`
    /// * `token` - A token to access this instance
    ///
    /// # Examples
    ///
    /// You can create a new instance like this:
    /// ```rust
    /// let paperless = Paperless::new("https://example.com/paperless/api/", "thisIsAToken");
    /// ```
    pub fn new(root: &str, token: &str) -> Self {
        Self {
            http_client: Client::new(),
            root: Url::parse(root).unwrap(),
            token: HeaderValue::from_str(&format!("Token {token}")).unwrap(),
        }
    }

    /// Generate a request object with authorization tokens.
    ///
    /// Caution: this will take any url and can leak token to wrong destination
    pub(crate) fn request(&self, method: Method, path: Url) -> Request {
        let mut request = Request::new(method, path);
        request
            .headers_mut()
            .append("Authorization", self.token.clone());
        request.headers_mut().append(
            "Accept",
            HeaderValue::from_str("application/json; version=2").unwrap(),
        );
        request
    }

    /// Generate a request object for an endpoint
    ///
    /// The caller must ensure that path is valid, otherwise it will panic
    fn url_api(&self, path: &str) -> Url {
        self.root.join(path).unwrap()
    }

    /// List all the correspondents, in form of iterator to avoid loading everything
    ///
    /// # Arguments
    ///
    /// * `filters` - Filter to apply during the listing of all document types
    pub fn correspondents(
        &self,
        filter: correspondent::Filter,
    ) -> Paginated<correspondent::Correspondent> {
        let mut url = self.url_api("correspondents/");
        filter.insert_query(&mut url);
        Paginated::new(self, url)
    }

    /// List all the document types, in form of iterator to avoid loading everything
    ///
    /// # Arguments
    ///
    /// * `filter` - Filter to apply during the listing of all document types
    pub fn document_types(
        &self,
        filter: document_type::Filter,
    ) -> Paginated<document_type::DocumentType> {
        let mut url = self.url_api("document_types/");
        filter.insert_query(&mut url);
        Paginated::new(self, url)
    }

    /// List all documents, in for of iterator to avoid loading everything
    ///
    /// # Arguments
    ///
    /// * `filter` - Filter to apply during the listing of all documents
    pub fn documents(&self, filter: document::Filter) -> Paginated<document::Document> {
        let mut url = self.url_api("documents/");
        filter.insert_query(&mut url);
        Paginated::new(self, url)
    }

    /// List all tags, in form of iterator to avoid loading everything
    ///
    /// # Arguments
    ///
    /// * `filters` - Filter to apply during the listing of all tags
    pub fn tags(&self, filters: tag::Filter) -> Paginated<tag::Tag> {
        let mut url = self.url_api("tags/");
        filters.insert_query(&mut url);
        Paginated::new(self, url)
    }

    /// List all saved views, in form of an iterator to avoid load everything
    pub fn saved_views(&self) -> Paginated<saved_view::SaveView> {
        Paginated::new(self, self.url_api("saved_views/"))
    }

    /// Get information about a correspondent
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the correspondent from which you are trying to retrieve information
    pub fn correspondent(
        &self,
        id: correspondent::Id,
    ) -> Result<correspondent::Correspondent, reqwest::Error> {
        let request = self.request(
            Method::GET,
            self.url_api(&format!("correspondents/{}/", u64::from(id))),
        );
        self.http_client.execute(request)?.json()
    }

    /// Get information about a document_type
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the document_type from which you are trying to retrieve information
    pub fn document_type(
        &self,
        id: document_type::Id,
    ) -> Result<document_type::DocumentType, reqwest::Error> {
        let request = self.request(
            Method::GET,
            self.url_api(&format!("document_types/{}/", u64::from(id))),
        );
        self.http_client.execute(request)?.json()
    }

    /// Get information about a document
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the document from which you are trying to retrieve information
    pub fn document(&self, id: document::Id) -> Result<document::Document, reqwest::Error> {
        let request = self.request(
            Method::GET,
            self.url_api(&format!("documents/{}/", u64::from(id))),
        );
        self.http_client.execute(request)?.json()
    }

    /// Get information about a tag
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the tag from which you are trying to retrieve information
    pub fn tag(&self, id: tag::Id) -> Result<tag::Tag, reqwest::Error> {
        let request = self.request(
            Method::GET,
            self.url_api(&format!("tags/{}/", u64::from(id))),
        );
        self.http_client.execute(request)?.json()
    }

    /// Get information about a view
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the view from which you are trying to retrieve information
    pub fn saved_view(&self, id: saved_view::Id) -> Result<saved_view::SaveView, reqwest::Error> {
        let request = self.request(
            Method::GET,
            self.url_api(&format!("saved_views/{}/", u64::from(id))),
        );
        self.http_client.execute(request)?.json()
    }

    pub fn document_size(&self, id: document::Id) -> usize {
        let request = self.request(Method::HEAD, self.url_api(&format!("/documents/{}/download/", id.to_string())));
        let r = self.http_client.execute(request).unwrap();

        r.headers()
            .get("content-length")
            .unwrap()
            .to_str()
            .unwrap()
            .parse()
            .unwrap()
    }

    pub fn document_download(&self, id: document::Id) -> Vec<u8> {
        let request = self.request(Method::GET, self.url_api(&format!("/documents/{}/download/", id.to_string())));
        self.http_client
            .execute(request)
            .unwrap()
            .bytes()
            .unwrap()
            .to_vec()
    }
}
