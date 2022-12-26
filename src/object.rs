// Copyright (c) nftables-json Developers
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Provides types related to parsing output from `nft --json`

use crate::expression::*;
use crate::statement::*;

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Metainfo {
    pub version: Option<String>,
    pub release_name: Option<String>,
    pub json_schema_version: isize,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Table {
    pub family: String,
    pub name: String,
    pub handle: Option<isize>,
    pub flags: Option<Vec<String>>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Chain {
    pub family: String,
    pub table: String,
    pub name: String,
    pub handle: Option<isize>,
    pub dev: Option<String>,
    pub policy: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    pub hook: Option<String>,
    pub prio: Option<isize>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Rule {
    pub family: String,
    pub table: String,
    pub chain: String,
    pub handle: Option<isize>,
    pub index: Option<isize>,
    pub comment: Option<String>,
    pub expr: Option<Vec<Statement>>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Set {
    pub family: String,
    pub table: String,
    pub name: String,
    pub handle: Option<isize>,
    pub comment: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    pub policy: Option<String>,
    pub flags: Option<Vec<String>>,
    pub size: Option<isize>,
    pub timeout: Option<isize>,
    #[serde(rename = "gc-interval")]
    pub gc_interval: Option<isize>,
    #[serde(with = "serde_with::As::<Option<serde_with::OneOrMany<serde_with::Same>>>")]
    pub elem: Option<Vec<Expression>>,
    pub stmt: Option<Vec<Statement>>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Map {
    pub family: String,
    pub table: String,
    pub name: String,
    pub handle: Option<isize>,
    pub comment: Option<String>,
    pub map: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    pub policy: Option<String>,
    pub flags: Option<Vec<String>>,
    pub size: Option<isize>,
    pub timeout: Option<isize>,
    #[serde(rename = "gc-interval")]
    pub gc_interval: Option<isize>,
    #[serde(with = "serde_with::As::<Option<serde_with::OneOrMany<serde_with::Same>>>")]
    pub elem: Option<Vec<Expression>>,
    pub stmt: Option<Vec<Statement>>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Element {
    pub family: String,
    pub table: String,
    pub name: String,
    #[serde(with = "serde_with::As::<Option<serde_with::OneOrMany<serde_with::Same>>>")]
    pub elem: Option<Vec<Expression>>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Flowtable {
    pub family: String,
    pub table: String,
    pub name: String,
    pub handle: Option<isize>,
    pub comment: Option<String>,
    pub hook: Option<String>,
    pub prio: Option<isize>,
    #[serde(with = "serde_with::As::<Option<serde_with::OneOrMany<serde_with::Same>>>")]
    pub dev: Option<Vec<String>>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Counter {
    pub family: String,
    pub table: String,
    pub name: String,
    pub handle: Option<isize>,
    pub comment: Option<String>,
    pub packets: Option<isize>,
    pub bytes: Option<isize>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Quota {
    pub family: String,
    pub table: String,
    pub name: String,
    pub handle: Option<isize>,
    pub comment: Option<String>,
    pub bytes: Option<isize>,
    pub used: Option<isize>,
    pub inv: Option<bool>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CtHelper {
    pub family: String,
    pub table: String,
    pub name: String,
    pub handle: Option<isize>,
    pub comment: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    pub protocol: Option<String>,
    pub l3proto: Option<String>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CtTimeout {
    pub family: String,
    pub table: String,
    pub name: String,
    pub handle: Option<isize>,
    pub comment: Option<String>,
    pub protocol: Option<String>,
    pub l3proto: Option<String>,
    pub policy: Option<std::collections::HashMap<String, isize>>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CtExpectation {
    pub family: String,
    pub table: String,
    pub name: String,
    pub handle: Option<isize>,
    pub comment: Option<String>,
    pub protocol: Option<String>,
    pub l3proto: Option<String>,
    pub dport: Option<String>,
    pub timeout: Option<isize>,
    pub size: Option<String>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Limit {
    pub family: String,
    pub table: String,
    pub name: String,
    pub handle: Option<isize>,
    pub comment: Option<String>,
    pub rate: Option<isize>,
    pub rate_unit: Option<String>,
    pub per: Option<String>,
    pub burst: Option<isize>,
    pub burst_unit: Option<String>,
    pub inv: Option<bool>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Secmark {
    pub family: String,
    pub table: String,
    pub name: String,
    pub handle: Option<isize>,
    pub comment: Option<String>,
    pub context: Option<String>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Synproxy {
    pub family: String,
    pub table: String,
    pub name: String,
    pub handle: Option<isize>,
    pub comment: Option<String>,
    pub mss: Option<isize>,
    pub wscale: Option<isize>,
    #[serde(with = "serde_with::As::<Option<serde_with::OneOrMany<serde_with::Same>>>")]
    pub flags: Option<Vec<String>>,
}

/// Represents a component of an nftables ruleset
#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[non_exhaustive]
pub enum Object {
    Metainfo(Metainfo),
    Table(Table),
    Chain(Chain),
    Rule(Rule),
    Set(Set),
    #[serde(alias = "meter")]
    Map(Map),
    Element(Element),
    Flowtable(Flowtable),
    Counter(Counter),
    Quota(Quota),
    #[serde(rename = "ct helper")]
    CtHelper(CtHelper),
    #[serde(rename = "ct timeout")]
    CtTimeout(CtTimeout),
    #[serde(rename = "ct expectation")]
    CtExpectation(CtExpectation),
    Limit(Limit),
    Secmark(Secmark),
    Synproxy(Synproxy),
}

/// Represents the components of an nftables ruleset
#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Objects {
    #[serde(rename = "nftables")]
    objects: Vec<Object>,
}

impl Objects {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_value(value: serde_json::Value) -> serde_json::Result<Self> {
        serde_json::from_value(value)
    }

    pub fn from_str(string: &str) -> serde_json::Result<Self> {
        serde_json::from_str(string)
    }

    pub fn from_slice(slice: &[u8]) -> serde_json::Result<Self> {
        serde_json::from_slice(slice)
    }

    pub fn from_reader<R: std::io::Read>(reader: R) -> serde_json::Result<Self> {
        serde_json::from_reader(reader)
    }

    pub fn to_value(&self) -> serde_json::Result<serde_json::Value> {
        serde_json::to_value(self)
    }

    pub fn to_string(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }
}

impl From<&[Object]> for Objects {
    fn from(value: &[Object]) -> Self {
        Self { objects: Vec::from(value) }
    }
}

impl From<&mut [Object]> for Objects {
    fn from(value: &mut [Object]) -> Self {
        Self { objects: Vec::from(value) }
    }
}

impl From<Box<[Object]>> for Objects {
    fn from(value: Box<[Object]>) -> Self {
        Self { objects: Vec::from(value) }
    }
}

impl From<Vec<Object>> for Objects {
    fn from(value: Vec<Object>) -> Self {
        Self { objects: Vec::from(value) }
    }
}

impl FromIterator<Object> for Objects {
    fn from_iter<T: IntoIterator<Item = Object>>(iter: T) -> Self {
        Self { objects: Vec::from_iter(iter) }
    }
}

impl Extend<Object> for Objects {
    fn extend<T: IntoIterator<Item = Object>>(&mut self, iter: T) {
        self.objects.extend(iter.into_iter());
    }
}

impl<'a> Extend<&'a Object> for Objects {
    fn extend<T: IntoIterator<Item = &'a Object>>(&mut self, iter: T) {
        self.objects.extend(iter.into_iter().map(|item| item.clone()));
    }
}

impl std::ops::Deref for Objects {
    type Target = [Object];

    fn deref(&self) -> &Self::Target {
        &self.objects
    }
}

impl std::ops::DerefMut for Objects {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.objects
    }
}

impl std::fmt::Display for Objects {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string().map_err(|_| std::fmt::Error::default())?)
    }
}
