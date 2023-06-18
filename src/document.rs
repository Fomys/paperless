//! # Document
//!
//! A document is stored on the server. There are a lot of way to filter documents

use crate::{asn, correspondent, document_type, saved_view, storage_path, tag};
use chrono::{DateTime, NaiveDate, Utc};
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
pub struct Document {
    pub id: Id,
    pub correspondent: Option<correspondent::Id>,
    pub document_type: Option<correspondent::Id>,
    pub storage_path: Option<storage_path::Id>,
    pub title: String,
    pub content: String,
    pub tags: Vec<tag::Id>,
    pub created: DateTime<Utc>,
    pub created_date: NaiveDate,
    pub modified: DateTime<Utc>,
    pub added: DateTime<Utc>,
    pub archive_serial_number: Option<asn::ASN>,
    pub original_file_name: Option<String>,
    pub archived_file_name: Option<String>,
}

/// Filter used when searching for a document
///
/// Multiple values can be defined at the same time if needed
#[derive(Debug, Default)]
pub struct Filter {
    /// Query is equivalent to "advanced search" in the interface
    pub query: Option<String>,
    pub title_content_contains: Option<String>,
    pub is_in_inbox: Option<bool>,
    pub title_starts_with: Option<String>,
    pub title_ends_with: Option<String>,
    pub title_contains: Option<String>,
    pub title_is: Option<String>,
    pub content_starts_with: Option<String>,
    pub content_ends_with: Option<String>,
    pub content_contains: Option<String>,
    pub content_is: Option<String>,
    pub archive_serial_number_is: Option<asn::ASN>,
    pub archive_serial_numer_gt: Option<asn::ASN>,
    pub archive_serial_number_gte: Option<asn::ASN>,
    pub archive_serial_numer_lt: Option<asn::ASN>,
    pub archive_serial_number_lte: Option<asn::ASN>,
    pub archive_serial_number_isnull: Option<bool>,
    pub created_year: Option<usize>,
    pub created_month: Option<usize>,
    pub created_day: Option<usize>,
    pub created_date_gt: Option<NaiveDate>,
    pub created_gt: Option<DateTime<Utc>>,
    pub created_date_lt: Option<NaiveDate>,
    pub created_lt: Option<DateTime<Utc>>,
    pub added_year: Option<usize>,
    pub added_month: Option<usize>,
    pub added_day: Option<usize>,
    pub added_date_gt: Option<NaiveDate>,
    pub added_gt: Option<DateTime<Utc>>,
    pub added_date_lt: Option<NaiveDate>,
    pub added_lt: Option<DateTime<Utc>>,
    pub modified_year: Option<usize>,
    pub modified_month: Option<usize>,
    pub modified_day: Option<usize>,
    pub modified_date_gt: Option<NaiveDate>,
    pub modified_gt: Option<DateTime<Utc>>,
    pub modified_date_lt: Option<NaiveDate>,
    pub modified_lt: Option<DateTime<Utc>>,
    pub correspondent_isnull: Option<bool>,
    pub correspondent_id_in: Option<Vec<correspondent::Id>>,
    pub correspondent_id: Option<correspondent::Id>,
    pub correspondent_name_starts_with: Option<String>,
    pub correspondent_name_ends_with: Option<String>,
    pub correspondent_name_contains: Option<String>,
    pub correspondent_name_is: Option<String>,
    pub is_tagged: Option<bool>,
    /// The document must have all those tags
    pub tag_id_all: Vec<tag::Id>,
    /// The document must have none of those tags
    pub tag_id_none: Vec<tag::Id>,
    /// The document must have any of those tags
    pub tag_id_in: Vec<tag::Id>,
    /// The document must have at least this tag
    pub tag_id: Option<tag::Id>,
    pub tag_name_starts_with: Option<String>,
    pub tag_name_ends_with: Option<String>,
    pub tag_name_contains: Option<String>,
    pub tag_name_is: Option<String>,
    pub document_type_isnull: Option<bool>,
    pub document_type_id_in: Vec<document_type::Id>,
    pub document_type_id: Option<document_type::Id>,
    pub document_type_name_starts_with: Option<String>,
    pub document_type_name_ends_with: Option<String>,
    pub document_type_name_contains: Option<String>,
    pub document_type_name_is: Option<String>,
    pub storage_path_isnull: Option<bool>,
    pub storage_path_id_in: Vec<storage_path::Id>,
    pub storage_path_id: Option<storage_path::Id>,
    pub storage_path_name_starts_with: Option<String>,
    pub storage_path_name_ends_with: Option<String>,
    pub storage_path_name_contains: Option<String>,
    pub storage_path_name_is: Option<String>,
    pub more_like: Option<Id>,
}

impl Filter {
    #[rustfmt::skip]
    /// Insert query parameter in a url
    pub(crate) fn insert_query(self, url: &mut Url) {
        if let Some(more_like) = self.more_like {
            url.query_pairs_mut().append_pair("more_like_id", &more_like.to_string());
        }
        if let Some(query) = self.query {
            url.query_pairs_mut().append_pair("query", &query.to_string());
        }
        if let Some(is_tagged) = self.is_tagged {
            url.query_pairs_mut().append_pair("is_tagged", &is_tagged.to_string());
        }

        url.query_pairs_mut()
            .append_pair("title_content", &self.title_content_contains.unwrap_or_default())
            .append_pair("is_in_inbox", &if let Some(is_in_inbox) = self.is_in_inbox { is_in_inbox.to_string() } else { String::default() })
            .append_pair("title__istartswith", &self.title_starts_with.unwrap_or_default())
            .append_pair("title__iendswith", &self.title_ends_with.unwrap_or_default())
            .append_pair("title__icontains", &self.title_contains.unwrap_or_default())
            .append_pair("title__iexact", &self.title_is.unwrap_or_default())
            .append_pair("content__istartswith", &self.content_starts_with.unwrap_or_default())
            .append_pair("content__iendswith", &self.content_ends_with.unwrap_or_default())
            .append_pair("content__icontains", &self.content_contains.unwrap_or_default())
            .append_pair("content__iexact", &self.content_is.unwrap_or_default())
            .append_pair("archive_serial_number", &self.archive_serial_number_is.map(|asn| asn.to_string()).unwrap_or_default())
            .append_pair("archive_serial_number__gt", &self.archive_serial_numer_gt.map(|asn| asn.to_string()).unwrap_or_default())
            .append_pair("archive_serial_number__gte", &self.archive_serial_number_gte.map(|asn| asn.to_string()).unwrap_or_default())
            .append_pair("archive_serial_number__lt", &self.archive_serial_numer_lt.map(|asn| asn.to_string()).unwrap_or_default())
            .append_pair("archive_serial_number__lte", &self.archive_serial_number_lte.map(|asn| asn.to_string()).unwrap_or_default())
            .append_pair("archive_serial_number__isnull", &if let Some(isnull) = self.archive_serial_number_isnull { isnull.to_string() } else { String::default() })
            .append_pair("created__year", &self.created_year.map(|year| year.to_string()).unwrap_or_default())
            .append_pair("created__month", &self.created_year.map(|month| month.to_string()).unwrap_or_default())
            .append_pair("created__day", &self.created_year.map(|day| day.to_string()).unwrap_or_default())
            .append_pair("created__date__gt", &self.created_date_gt.map(|d| d.format("%Y-%m-%dT%H:%M:%SZ").to_string()).unwrap_or_default())
            .append_pair("created__gt", &self.created_gt.map(|d| d.format("%Y-%m-%dT%H:%M:%SZ").to_string()).unwrap_or_default())
            .append_pair("created__date__lt", &self.created_date_lt.map(|d| d.format("%Y-%m-%dT%H:%M:%SZ").to_string()).unwrap_or_default())
            .append_pair("created__lt", &self.created_lt.map(|d| d.format("%Y-%m-%dT%H:%M:%SZ").to_string()).unwrap_or_default())
            .append_pair("added__year", &self.added_year.map(|year| year.to_string()).unwrap_or_default())
            .append_pair("added__month", &self.added_year.map(|month| month.to_string()).unwrap_or_default())
            .append_pair("added__day", &self.added_year.map(|day| day.to_string()).unwrap_or_default())
            .append_pair("added__date__gt", &self.added_date_gt.map(|d| d.format("%Y-%m-%dT%H:%M:%SZ").to_string()).unwrap_or_default())
            .append_pair("added__gt", &self.added_gt.map(|d| d.format("%Y-%m-%dT%H:%M:%SZ").to_string()).unwrap_or_default())
            .append_pair("added__date__lt", &self.added_date_lt.map(|d| d.format("%Y-%m-%dT%H:%M:%SZ").to_string()).unwrap_or_default())
            .append_pair("added__lt", &self.added_lt.map(|d| d.format("%Y-%m-%dT%H:%M:%SZ").to_string()).unwrap_or_default())
            .append_pair("modified__year", &self.modified_year.map(|year| year.to_string()).unwrap_or_default())
            .append_pair("modified__month", &self.modified_year.map(|month| month.to_string()).unwrap_or_default())
            .append_pair("modified__day", &self.modified_year.map(|day| day.to_string()).unwrap_or_default())
            .append_pair("modified__date__gt", &self.modified_date_gt.map(|d| d.format("%Y-%m-%dT%H:%M:%SZ").to_string()).unwrap_or_default())
            .append_pair("modified__gt", &self.modified_gt.map(|d| d.format("%Y-%m-%dT%H:%M:%SZ").to_string()).unwrap_or_default())
            .append_pair("modified__date__lt", &self.modified_date_lt.map(|d| d.format("%Y-%m-%dT%H:%M:%SZ").to_string()).unwrap_or_default())
            .append_pair("modified__lt", &self.modified_lt.map(|d| d.format("%Y-%m-%dT%H:%M:%SZ").to_string()).unwrap_or_default())
            .append_pair("correspondent__isnull",&if let Some(isnull) = self.correspondent_isnull { isnull.to_string() } else { String::default() })
            .append_pair("correspondent__id__in", &self.correspondent_id_in.map(|ids| ids.iter().map(|id| id.to_string()).collect::<Vec<String>>().join(",")).unwrap_or_default())
            .append_pair("correspondent__id", &self.correspondent_id.map(|id| id.to_string()).unwrap_or_default())
            .append_pair("correspondent__name__istartswith", &self.correspondent_name_starts_with.unwrap_or_default())
            .append_pair("correspondent__name__iendswith", &self.correspondent_name_ends_with.unwrap_or_default())
            .append_pair("correspondent__name__icontains", &self.correspondent_name_contains.unwrap_or_default())
            .append_pair("correspondent__name__iexact", &self.correspondent_name_is.unwrap_or_default())
            .append_pair("tags__id__in", &self.tag_id_in.iter().map(|id| id.to_string()).collect::<Vec<String>>().join(","))
            .append_pair("tags__id__all", &self.tag_id_all.iter().map(|id| id.to_string()).collect::<Vec<String>>().join(","))
            .append_pair("tags__id__none", &self.tag_id_none.iter().map(|id| id.to_string()).collect::<Vec<String>>().join(","))
            .append_pair("tags__id", &self.tag_id.map(|id| id.to_string()).unwrap_or_default())
            .append_pair("tags__name__istartswith", &self.tag_name_starts_with.unwrap_or_default())
            .append_pair("tags__name__iendswith", &self.tag_name_ends_with.unwrap_or_default())
            .append_pair("tags__name__icontains", &self.tag_name_contains.unwrap_or_default())
            .append_pair("tags__name__iexact", &self.tag_name_is.unwrap_or_default())
            .append_pair("document_type__isnull",&if let Some(isnull) = self.document_type_isnull { isnull.to_string() } else { String::default() })
            .append_pair("document_type__id__in", &self.document_type_id_in.iter().map(|id| id.to_string()).collect::<Vec<String>>().join(","))
            .append_pair("document_type__id", &self.document_type_id.map(|id| id.to_string()).unwrap_or_default())
            .append_pair("document_type__name__istartswith", &self.document_type_name_starts_with.unwrap_or_default())
            .append_pair("document_type__name__iendswith", &self.document_type_name_ends_with.unwrap_or_default())
            .append_pair("document_type__name__icontains", &self.document_type_name_contains.unwrap_or_default())
            .append_pair("document_type__name__iexact", &self.document_type_name_is.unwrap_or_default())
            .append_pair("storage_path__isnull",&if let Some(isnull) = self.storage_path_isnull { isnull.to_string() } else { String::default() })
            .append_pair("storage_path__id__in", &self.storage_path_id_in.iter().map(|id| id.to_string()).collect::<Vec<String>>().join(","))
            .append_pair("storage_path__id", &self.storage_path_id.map(|id| id.to_string()).unwrap_or_default())
            .append_pair("storage_path__name__istartswith", &self.storage_path_name_starts_with.unwrap_or_default())
            .append_pair("storage_path__name__iendswith", &self.storage_path_name_ends_with.unwrap_or_default())
            .append_pair("storage_path__name__icontains", &self.storage_path_name_contains.unwrap_or_default())
            .append_pair("storage_path__name__iexact", &self.storage_path_name_is.unwrap_or_default());
    }

    /// Create a filter from view rules
    pub fn from_filter_rules(filter_rules: &[saved_view::FilterRule]) -> Self {
        let mut filter = Self::default();
        for rule in filter_rules {
            match rule {
                saved_view::FilterRule::TitleContains(v) => {
                    filter.title_contains = v.clone();
                }
                saved_view::FilterRule::ContentContains(v) => {
                    filter.content_contains = v.clone();
                }
                saved_view::FilterRule::ASNIs(Some(v)) => {
                    filter.archive_serial_number_is = Some(v.clone());
                }
                saved_view::FilterRule::ASNIs(None) => {
                    filter.archive_serial_number_isnull = Some(true);
                }
                saved_view::FilterRule::CorrespondentIs(Some(v)) => {
                    filter.correspondent_id = Some(v.clone());
                }
                saved_view::FilterRule::CorrespondentIs(None) => {
                    filter.correspondent_isnull = Some(true);
                }
                saved_view::FilterRule::DocumentTypeIs(Some(v)) => {
                    filter.document_type_id = Some(v.clone());
                }
                saved_view::FilterRule::DocumentTypeIs(None) => {
                    filter.document_type_isnull = Some(true);
                }
                saved_view::FilterRule::IsInInbox(v) => {
                    filter.is_in_inbox = v.clone();
                }
                saved_view::FilterRule::HasTag(Some(tag)) => {
                    filter.tag_id_all.push(tag.clone());
                }
                saved_view::FilterRule::HasAnyTag(v) => {
                    filter.is_tagged = v.clone();
                }
                saved_view::FilterRule::CreatedBefore(v) => {
                    filter.created_lt = v.clone();
                }
                saved_view::FilterRule::CreatedAfter(v) => {
                    filter.created_gt = v.clone();
                }
                saved_view::FilterRule::CreatedYearIs(v) => {
                    filter.created_year = v.clone();
                }
                saved_view::FilterRule::CreatedMountIs(v) => {
                    filter.created_month = v.clone();
                }
                saved_view::FilterRule::CreatedDayIs(v) => {
                    filter.created_day = v.clone();
                }
                saved_view::FilterRule::AddedBefore(v) => {
                    filter.added_lt = v.clone();
                }
                saved_view::FilterRule::AddedAfter(v) => {
                    filter.added_gt = v.clone();
                }
                saved_view::FilterRule::ModifiedBefore(v) => {
                    filter.modified_lt = v.clone();
                }
                saved_view::FilterRule::ModifiedAfter(v) => {
                    filter.modified_gt = v.clone();
                }
                saved_view::FilterRule::DontHaveTag(Some(tag)) => {
                    filter.tag_id_none.push(tag.clone());
                }
                saved_view::FilterRule::DontHaveASN(v) => {
                    filter.archive_serial_number_isnull = v.clone();
                }
                saved_view::FilterRule::TitleOrContentContains(v) => {
                    filter.title_content_contains = v.clone();
                }
                saved_view::FilterRule::FullTextQuery(v) => {
                    filter.query = v.clone();
                }
                saved_view::FilterRule::MoreLikeThis(v) => {
                    filter.more_like = v.clone();
                }
                saved_view::FilterRule::HasTagIn(Some(tag)) => {
                    filter.tag_id_in.push(tag.clone());
                }
                saved_view::FilterRule::ASNGreaterThan(v) => {
                    filter.archive_serial_numer_gt = v.clone();
                }
                saved_view::FilterRule::ASNLessThan(v) => {
                    filter.archive_serial_numer_lt = v.clone();
                }
                saved_view::FilterRule::StoragePathIs(Some(v)) => {
                    filter.storage_path_id = Some(v.clone());
                }
                saved_view::FilterRule::StoragePathIs(None) => {
                    filter.storage_path_isnull = Some(true);
                }
                r => {
                    println!("Ignore {:?}", r)
                }
            }
        }
        filter
    }
}

//https://paperless.joel.rs/api/documents/?more_like_id=&query=&title_content=&is_in_inbox=&title__istartswith=&title__iendswith=&title__icontains=&title__iexact=&content__istartswith=&content__iendswith=&content__icontains=&content__iexact=&archive_serial_number=&archive_serial_number__gt=&archive_serial_number__gte=&archive_serial_number__lt=&archive_serial_number__lte=&archive_serial_number__isnull=&correspondent__isnull=&correspondent__id__in=&correspondent__id=5&correspondent__name__istartswith=&correspondent__name__iendswith=&correspondent__name__icontains=&correspondent__name__iexact=&is_tagged=&tags__id__in=&tags__id__all=&tags__id__none=&tags__id=&tags__name__istartswith=&tags__name__iendswith=&tags__name__icontains=&tags__name__iexact=&document_type__isnull=&document_type__id__in=&document_type__id=&document_type__name__istartswith=&document_type__name__iendswith=&document_type__name__icontains=&document_type__name__iexact=&storage_path__isnull=&storage_path__id__in=&storage_path__id=&storage_path__name__istartswith=&storage_path__name__iendswith=&storage_path__name__icontains=&storage_path__name__iexact=
//https://paperless.joel.rs/api/documents/?title__istartswith=&title__iendswith=&title__icontains=&title__iexact=&content__istartswith=&content__iendswith=&content__icontains=&content__iexact=&archive_serial_number=&archive_serial_number__gt=&archive_serial_number__gte=&archive_serial_number__lt=&archive_serial_number__lte=&archive_serial_number__isnull=&created__year=&created__month=&created__day=&created__date__gt=&created__gt=&created__date__lt=&created__lt=&added__year=&added__month=&added__day=&added__date__gt=&added__gt=&added__date__lt=&added__lt=&modified__year=&modified__month=&modified__day=&modified__date__gt=&modified__gt=&modified__date__lt=&modified__lt=&correspondent__isnull=&correspondent__id__in=&correspondent__id=5&correspondent__name__istartswith=&correspondent__name__iendswith=&correspondent__name__icontains=&correspondent__name__iexact=&tags__id__in=&tags__id=&tags__name__istartswith=&tags__name__iendswith=&tags__name__icontains=&tags__name__iexact=&document_type__isnull=&document_type__id__in=&document_type__id=&document_type__name__istartswith=&document_type__name__iendswith=&document_type__name__icontains=&document_type__name__iexact=&storage_path__isnull=&storage_path__id__in=&storage_path__id=&storage_path__name__istartswith=&storage_path__name__iendswith=&storage_path__name__icontains=&storage_path__name__iexact=&is_tagged=&tags__id__all=&tags__id__none=&is_in_inbox=&title_content=
