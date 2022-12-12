// Copyright (c) nftables-json Developers
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::*;

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, untagged)]
pub enum Counter {
    Named(Box<Expression>),
    Anonymous {
        #[serde(default)]
        packets: isize,
        #[serde(default)]
        bytes: isize,
    },
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CtCount {
    pub val: isize,
    pub inv: Option<bool>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Dup {
    pub addr: Box<Expression>,
    pub dev: Option<Box<Expression>>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Flow {
    pub op: String,
    pub flowtable: String,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Fwd {
    pub dev: Box<Expression>,
    pub family: Option<String>,
    pub addr: Option<Box<Expression>>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, untagged)]
pub enum Limit {
    Named(Box<Expression>),
    Anonymous {
        rate: isize,
        rate_unit: Option<String>,
        per: String,
        burst: Option<isize>,
        burst_unit: Option<String>,
        inv: Option<bool>,
    },
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Log {
    pub prefix: Option<String>,
    pub group: Option<isize>,
    pub snaplen: Option<isize>,
    #[serde(rename = "queue-threshold")]
    pub queue_threshold: Option<isize>,
    pub level: Option<String>,
    #[serde(with = "serde_with::As::<Option<serde_with::OneOrMany<serde_with::Same>>>")]
    pub flags: Option<Vec<String>>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Mangle {
    pub key: Box<Expression>,
    pub value: Box<Expression>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Match {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub op: String,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Meter {
    pub name: String,
    pub size: Option<isize>,
    pub key: Box<Expression>,
    pub stmt: Box<Statement>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Nat {
    pub addr: Option<Box<Expression>>,
    pub family: Option<String>,
    pub port: Option<Box<Expression>>,
    #[serde(with = "serde_with::As::<Option<serde_with::OneOrMany<serde_with::Same>>>")]
    pub flags: Option<Vec<String>>,
    #[serde(with = "serde_with::As::<Option<serde_with::OneOrMany<serde_with::Same>>>")]
    pub type_flags: Option<Vec<String>>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Queue {
    pub num: Option<Box<Expression>>,
    #[serde(with = "serde_with::As::<Option<serde_with::OneOrMany<serde_with::Same>>>")]
    pub flags: Option<Vec<String>>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, untagged)]
pub enum Quota {
    Named(Box<Expression>),
    Anonymous { val: isize, val_unit: Option<String>, used: Option<isize>, used_unit: Option<isize>, inv: Option<bool> },
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Reject {
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    pub expr: Option<String>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Set {
    pub op: String,
    pub set: String,
    pub elem: Box<Expression>,
    pub stmt: Option<Vec<Box<Statement>>>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Synproxy {
    pub mss: Option<isize>,
    pub wscale: Option<isize>,
    #[serde(with = "serde_with::As::<Option<serde_with::OneOrMany<serde_with::Same>>>")]
    pub flags: Option<Vec<String>>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tproxy {
    pub addr: Option<String>,
    pub family: Option<String>,
    pub port: Option<isize>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Vmap {
    pub key: Box<Expression>,
    pub data: Box<Expression>,
}
