//! `paperless` is a create to communicate with paperless api (<https://docs.paperless-ngx.com/api/>)
//!
//! This create was created for a fuse driver for paperless, so some functions and endpoints are not present

pub mod asn;
pub mod correspondent;
pub mod document;
pub mod document_type;
mod paginated;
mod paperless;
pub mod saved_view;
pub mod storage_path;
pub mod tag;

pub use paginated::Paginated;
pub use paperless::Paperless;
