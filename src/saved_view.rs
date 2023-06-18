//! # Saved view
//!
//! A saved view is a saved filtered view, which can be created from the interface. It allows to
//! keep certain set of filters and have a fast access to a selection of documents

use crate::{asn, correspondent, document, document_type, storage_path, tag};
use chrono::{DateTime, Utc};
use serde::de::{MapAccess, Visitor};
use serde::{de, Deserialize, Deserializer};
use std::fmt;

#[derive(Debug, Copy, Clone, Deserialize)]
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

#[derive(Debug)]
pub enum FilterRule {
    TitleContains(Option<String>),
    ContentContains(Option<String>),
    ASNIs(Option<asn::ASN>),
    CorrespondentIs(Option<correspondent::Id>),
    DocumentTypeIs(Option<document_type::Id>),
    IsInInbox(Option<bool>),
    HasTag(Option<tag::Id>),
    HasAnyTag(Option<bool>),
    CreatedBefore(Option<DateTime<Utc>>),
    CreatedAfter(Option<DateTime<Utc>>),
    CreatedYearIs(Option<usize>),
    CreatedMountIs(Option<usize>),
    CreatedDayIs(Option<usize>),
    AddedBefore(Option<DateTime<Utc>>),
    AddedAfter(Option<DateTime<Utc>>),
    ModifiedBefore(Option<DateTime<Utc>>),
    ModifiedAfter(Option<DateTime<Utc>>),
    DontHaveTag(Option<tag::Id>),
    DontHaveASN(Option<bool>),
    TitleOrContentContains(Option<String>),
    FullTextQuery(Option<String>),
    MoreLikeThis(Option<document::Id>),
    HasTagIn(Option<tag::Id>),
    ASNGreaterThan(Option<asn::ASN>),
    ASNLessThan(Option<asn::ASN>),
    StoragePathIs(Option<storage_path::Id>),
}
impl<'de> Deserialize<'de> for FilterRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "rule_type")]
            RuleType,
            #[serde(rename = "value")]
            Value,
        }

        struct FilterRuleVisitor;

        impl<'de> Visitor<'de> for FilterRuleVisitor {
            type Value = FilterRule;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct FilterRule")
            }

            fn visit_map<V>(self, mut map: V) -> Result<FilterRule, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut rule_type: Option<u64> = None;
                let mut value: Option<Option<String>> = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::RuleType => {
                            if rule_type.is_some() {
                                return Err(de::Error::duplicate_field("rule_type"));
                            }
                            rule_type = Some(map.next_value()?);
                        }
                        Field::Value => {
                            if value.is_some() {
                                return Err(de::Error::duplicate_field("value"));
                            }
                            value = Some(map.next_value()?);
                        }
                    }
                }
                let rule_type = rule_type.ok_or_else(|| de::Error::missing_field("rule_type"))?;
                let value = value.unwrap_or(None);
                let u64_value: Option<u64> = match &value {
                    None => None,
                    Some(v) => match v.parse() {
                        Ok(v) => Some(v),
                        Err(_) => None,
                    },
                };
                let usize_value: Option<usize> = match &value {
                    None => None,
                    Some(v) => match v.parse() {
                        Ok(v) => Some(v),
                        Err(_) => None,
                    },
                };
                let bool_value: Option<bool> = match &value {
                    None => None,
                    Some(v) => match v.parse() {
                        Ok(v) => Some(v),
                        Err(_) => None,
                    },
                };
                let date_value = match &value {
                    None => None,
                    Some(v) => match DateTime::parse_from_rfc3339(v) {
                        Ok(v) => Some(v.naive_utc().and_utc()),
                        Err(_) => None,
                    },
                };
                match rule_type {
                    0 => Ok(FilterRule::TitleContains(value)),
                    1 => Ok(FilterRule::ContentContains(value)),
                    2 => Ok(FilterRule::ASNIs(u64_value.map(|v| v.into()))),
                    3 => Ok(FilterRule::CorrespondentIs(u64_value.map(|v| v.into()))),
                    4 => Ok(FilterRule::DocumentTypeIs(u64_value.map(|v| v.into()))),
                    5 => Ok(FilterRule::IsInInbox(bool_value)),
                    6 => Ok(FilterRule::HasTag(u64_value.map(|v| v.into()))),
                    7 => Ok(FilterRule::HasAnyTag(bool_value)),
                    8 => Ok(FilterRule::CreatedBefore(date_value)),
                    9 => Ok(FilterRule::CreatedAfter(date_value)),
                    10 => Ok(FilterRule::CreatedYearIs(usize_value)),
                    11 => Ok(FilterRule::CreatedMountIs(usize_value)),
                    12 => Ok(FilterRule::CreatedDayIs(usize_value)),
                    13 => Ok(FilterRule::AddedBefore(date_value)),
                    14 => Ok(FilterRule::AddedAfter(date_value)),
                    15 => Ok(FilterRule::ModifiedBefore(date_value)),
                    16 => Ok(FilterRule::ModifiedAfter(date_value)),
                    17 => Ok(FilterRule::DontHaveTag(u64_value.map(|v| v.into()))),
                    18 => Ok(FilterRule::DontHaveASN(bool_value)),
                    19 => Ok(FilterRule::TitleOrContentContains(value)),
                    20 => Ok(FilterRule::FullTextQuery(value)),
                    21 => Ok(FilterRule::MoreLikeThis(u64_value.map(|v| v.into()))),
                    22 => Ok(FilterRule::HasTagIn(u64_value.map(|v| v.into()))),
                    23 => Ok(FilterRule::ASNGreaterThan(u64_value.map(|v| v.into()))),
                    24 => Ok(FilterRule::ASNLessThan(u64_value.map(|v| v.into()))),
                    25 => Ok(FilterRule::StoragePathIs(u64_value.map(|v| v.into()))),
                    r => Err(de::Error::custom(format!("Invalid rule_type {}", r))),
                }
            }
        }

        const FIELDS: &'static [&'static str] = &["rule_type", "value"];
        deserializer.deserialize_struct("FilterRule", FIELDS, FilterRuleVisitor)
    }
}

#[derive(Debug, Deserialize)]
pub struct SaveView {
    pub id: Id,
    pub name: String,
    pub show_on_dashboard: bool,
    pub show_in_sidebar: bool,
    pub sort_field: String,
    pub sort_reverse: bool,
    pub filter_rules: Vec<FilterRule>,
}
