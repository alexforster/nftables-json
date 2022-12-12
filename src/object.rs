// Copyright (c) nftables-json Developers
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::*;

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Metainfo {
    pub version: Option<String>,
    pub release_name: Option<String>,
    pub json_schema_version: isize,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Table {
    pub family: String,
    pub name: String,
    pub handle: Option<isize>,
    pub flags: Option<Vec<String>>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Rule {
    pub family: String,
    pub table: String,
    pub chain: String,
    pub handle: Option<isize>,
    pub index: Option<isize>,
    pub comment: Option<String>,
    pub expr: Option<Vec<Box<Statement>>>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
    pub elem: Option<Vec<Box<Expression>>>,
    pub stmt: Option<Vec<Box<Statement>>>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
    pub elem: Option<Vec<Box<Expression>>>,
    pub stmt: Option<Vec<Box<Statement>>>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Element {
    pub family: String,
    pub table: String,
    pub name: String,
    #[serde(with = "serde_with::As::<Option<serde_with::OneOrMany<serde_with::Same>>>")]
    pub elem: Option<Vec<Box<Expression>>>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
