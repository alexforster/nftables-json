// Copyright (c) nftables-json Developers
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::*;

pub mod add {
    pub use super::object::Chain;
    pub use super::object::Counter;
    pub use super::object::CtExpectation;
    pub use super::object::CtHelper;
    pub use super::object::CtTimeout;
    pub use super::object::Element;
    pub use super::object::Flowtable;
    pub use super::object::Limit;
    pub use super::object::Map;
    pub use super::object::Quota;
    pub use super::object::Rule;
    pub use super::object::Secmark;
    pub use super::object::Set;
    pub use super::object::Synproxy;
    pub use super::object::Table;
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub enum Add {
    Table(add::Table),
    Chain(add::Chain),
    Rule(add::Rule),
    Set(add::Set),
    Map(add::Map),
    Element(add::Element),
    Flowtable(add::Flowtable),
    Counter(add::Counter),
    Quota(add::Quota),
    #[serde(rename = "ct helper")]
    CtHelper(add::CtHelper),
    #[serde(rename = "ct timeout")]
    CtTimeout(add::CtTimeout),
    #[serde(rename = "ct expectation")]
    CtExpectation(add::CtExpectation),
    Limit(add::Limit),
    Secmark(add::Secmark),
    Synproxy(add::Synproxy),
}

pub mod replace {
    pub use super::object::Rule;
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub enum Replace {
    Rule(replace::Rule),
}

pub mod insert {
    pub use super::object::Rule;
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub enum Insert {
    Rule(insert::Rule),
}

pub mod delete {
    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Table {
        pub family: String,
        pub name: Option<String>,
        pub handle: Option<isize>,
    }

    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Chain {
        pub family: String,
        pub table: String,
        pub name: Option<String>,
        pub handle: Option<isize>,
    }

    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Rule {
        pub family: String,
        pub table: String,
        pub chain: String,
        pub handle: isize,
    }

    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Set {
        pub family: String,
        pub table: String,
        pub name: Option<String>,
        pub handle: Option<isize>,
    }

    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Map {
        pub family: String,
        pub table: String,
        pub name: Option<String>,
        pub handle: Option<isize>,
    }

    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Flowtable {
        pub family: String,
        pub table: String,
        pub name: Option<String>,
        pub handle: Option<isize>,
    }

    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Counter {
        pub family: String,
        pub table: String,
        pub name: Option<String>,
        pub handle: Option<isize>,
    }

    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Quota {
        pub family: String,
        pub table: String,
        pub name: Option<String>,
        pub handle: Option<isize>,
    }

    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct CtHelper {
        pub family: String,
        pub table: String,
        pub name: Option<String>,
        pub handle: Option<isize>,
    }

    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct CtTimeout {
        pub family: String,
        pub table: String,
        pub name: Option<String>,
        pub handle: Option<isize>,
    }

    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct CtExpectation {
        pub family: String,
        pub table: String,
        pub name: Option<String>,
        pub handle: Option<isize>,
    }

    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Limit {
        pub family: String,
        pub table: String,
        pub name: Option<String>,
        pub handle: Option<isize>,
    }

    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Secmark {
        pub family: String,
        pub table: String,
        pub name: Option<String>,
        pub handle: Option<isize>,
    }
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub enum Delete {
    Table(delete::Table),
    Chain(delete::Chain),
    Rule(delete::Rule),
    Set(delete::Set),
    Map(delete::Map),
    Element(object::Element),
    Flowtable(delete::Flowtable),
    Counter(delete::Counter),
    Quota(delete::Quota),
    #[serde(rename = "ct helper")]
    CtHelper(delete::CtHelper),
    #[serde(rename = "ct timeout")]
    CtTimeout(delete::CtTimeout),
    #[serde(rename = "ct expectation")]
    CtExpectation(delete::CtExpectation),
    Limit(delete::Limit),
    Secmark(delete::Secmark),
}

pub mod list {
    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Table {
        pub family: String,
        pub name: String,
    }

    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Chain {
        pub family: String,
        pub table: String,
        pub name: String,
    }

    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Set {
        pub family: String,
        pub table: String,
        pub name: String,
    }

    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Map {
        pub family: String,
        pub table: String,
        pub name: String,
    }

    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Counter {
        pub family: String,
        pub table: String,
        pub name: String,
    }

    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Quota {
        pub family: String,
        pub table: String,
        pub name: String,
    }

    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct CtHelper {
        pub family: String,
        pub table: String,
        pub name: String,
    }

    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct CtTimeout {
        pub family: String,
        pub table: String,
        pub name: String,
    }

    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct CtExpectation {
        pub family: String,
        pub table: String,
        pub name: String,
    }

    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Limit {
        pub family: String,
        pub table: String,
        pub name: String,
    }

    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Meter {
        pub family: String,
        pub table: String,
        pub name: String,
    }

    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Flowtable {
        pub family: String,
        pub table: String,
        pub name: String,
    }

    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Secmark {
        pub family: String,
        pub table: String,
        pub name: String,
    }

    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Tables {
        pub family: String,
    }

    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Chains {
        pub family: String,
    }

    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Sets {
        pub family: Option<String>,
        pub table: String,
    }

    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Maps {
        pub family: String,
    }

    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Counters {
        pub family: Option<String>,
        pub table: String,
    }

    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Quotas {
        pub family: String,
    }

    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct CtHelpers {
        pub family: Option<String>,
        pub table: String,
    }

    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Limits {
        pub family: String,
    }

    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Ruleset {
        pub family: String,
    }

    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Meters {
        pub family: String,
    }

    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Flowtables {
        pub family: String,
    }

    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Secmarks {
        pub family: String,
    }
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub enum List {
    Table(list::Table),
    Chain(list::Chain),
    Set(list::Set),
    Map(list::Map),
    Counter(list::Counter),
    Quota(list::Quota),
    #[serde(rename = "ct helper")]
    CtHelper(list::CtHelper),
    #[serde(rename = "ct timeout")]
    CtTimeout(list::CtTimeout),
    #[serde(rename = "ct expectation")]
    CtExpectation(list::CtExpectation),
    Limit(list::Limit),
    Meter(list::Meter),
    Flowtable(list::Flowtable),
    Secmark(list::Secmark),
    Tables(Option<list::Tables>),
    Chains(Option<list::Chains>),
    Sets(Option<list::Sets>),
    Maps(Option<list::Maps>),
    Counters(Option<list::Counters>),
    Quotas(Option<list::Quotas>),
    #[serde(rename = "ct helpers")]
    CtHelpers(Option<list::CtHelpers>),
    Limits(Option<list::Limits>),
    Ruleset(Option<list::Ruleset>),
    Meters(Option<list::Meters>),
    Flowtables(Option<list::Flowtables>),
    Secmarks(Option<list::Secmarks>),
}

pub mod reset {
    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Counter {
        pub family: String,
        pub table: String,
        pub name: String,
    }

    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Counters {
        pub family: Option<String>,
        pub table: String,
    }

    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Quota {
        pub family: String,
        pub table: String,
        pub name: String,
    }

    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Quotas {
        pub family: String,
    }
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub enum Reset {
    Counter(reset::Counter),
    Counters(Option<reset::Counters>),
    Quota(reset::Quota),
    Quotas(Option<reset::Quotas>),
}

pub mod flush {
    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Table {
        pub family: String,
        pub name: String,
    }

    #[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Chain {
        pub family: String,
        pub table: String,
        pub name: String,
    }

    #[serde_with::apply(Option => # [serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Set {
        pub family: String,
        pub table: String,
        pub name: String,
    }

    #[serde_with::apply(Option => # [serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Map {
        pub family: String,
        pub table: String,
        pub name: String,
    }

    #[serde_with::apply(Option => # [serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Meter {
        pub family: String,
        pub table: String,
        pub name: String,
    }

    #[serde_with::apply(Option => # [serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Ruleset {
        pub family: String,
    }
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub enum Flush {
    Table(flush::Table),
    Chain(flush::Chain),
    Set(flush::Set),
    Map(flush::Map),
    Meter(flush::Meter),
    Ruleset(Option<flush::Ruleset>),
}

pub mod rename {
    #[serde_with::apply(Option => # [serde(default, skip_serializing_if = "Option::is_none")])]
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Chain {
        pub family: String,
        pub table: String,
        pub name: String,
        pub newname: String,
    }
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub enum Rename {
    Chain(rename::Chain),
}
