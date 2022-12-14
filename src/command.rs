// Copyright (c) nftables-json Developers
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Provides types related to inputting commands to `nft --json`

use crate::expression::*;
use crate::statement::*;

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AddChain {
    pub family: String,
    pub table: String,
    pub name: String,
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
pub struct AddCounter {
    pub family: String,
    pub table: String,
    pub name: String,
    pub comment: Option<String>,
    pub packets: Option<isize>,
    pub bytes: Option<isize>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AddCtExpectation {
    pub family: String,
    pub table: String,
    pub name: String,
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
pub struct AddCtHelper {
    pub family: String,
    pub table: String,
    pub name: String,
    pub comment: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    pub protocol: Option<String>,
    pub l3proto: Option<String>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AddCtTimeout {
    pub family: String,
    pub table: String,
    pub name: String,
    pub comment: Option<String>,
    pub protocol: Option<String>,
    pub l3proto: Option<String>,
    pub policy: Option<std::collections::HashMap<String, isize>>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AddElement {
    pub family: String,
    pub table: String,
    pub name: String,
    #[serde(with = "serde_with::As::<Option<serde_with::OneOrMany<serde_with::Same>>>")]
    pub elem: Option<Vec<Expression>>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AddFlowtable {
    pub family: String,
    pub table: String,
    pub name: String,
    pub comment: Option<String>,
    pub hook: Option<String>,
    pub prio: Option<isize>,
    #[serde(with = "serde_with::As::<Option<serde_with::OneOrMany<serde_with::Same>>>")]
    pub dev: Option<Vec<String>>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AddLimit {
    pub family: String,
    pub table: String,
    pub name: String,
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
pub struct AddMap {
    pub family: String,
    pub table: String,
    pub name: String,
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
pub struct AddQuota {
    pub family: String,
    pub table: String,
    pub name: String,
    pub comment: Option<String>,
    pub bytes: Option<isize>,
    pub used: Option<isize>,
    pub inv: Option<bool>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AddRule {
    pub family: String,
    pub table: String,
    pub chain: String,
    pub index: Option<isize>,
    pub comment: Option<String>,
    pub expr: Option<Vec<Statement>>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AddSecmark {
    pub family: String,
    pub table: String,
    pub name: String,
    pub comment: Option<String>,
    pub context: Option<String>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AddSet {
    pub family: String,
    pub table: String,
    pub name: String,
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
pub struct AddSynproxy {
    pub family: String,
    pub table: String,
    pub name: String,
    pub comment: Option<String>,
    pub mss: Option<isize>,
    pub wscale: Option<isize>,
    #[serde(with = "serde_with::As::<Option<serde_with::OneOrMany<serde_with::Same>>>")]
    pub flags: Option<Vec<String>>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AddTable {
    pub family: String,
    pub name: String,
    pub flags: Option<Vec<String>>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[non_exhaustive]
pub enum Add {
    Table(AddTable),
    Chain(AddChain),
    Rule(AddRule),
    Set(AddSet),
    Map(AddMap),
    Element(AddElement),
    Flowtable(AddFlowtable),
    Counter(AddCounter),
    Quota(AddQuota),
    #[serde(rename = "ct helper")]
    CtHelper(AddCtHelper),
    #[serde(rename = "ct timeout")]
    CtTimeout(AddCtTimeout),
    #[serde(rename = "ct expectation")]
    CtExpectation(AddCtExpectation),
    Limit(AddLimit),
    Secmark(AddSecmark),
    Synproxy(AddSynproxy),
}

pub type ReplaceRule = AddRule;

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[non_exhaustive]
pub enum Replace {
    Rule(ReplaceRule),
}

pub type InsertRule = AddRule;

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[non_exhaustive]
pub enum Insert {
    Rule(InsertRule),
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeleteTable {
    pub family: String,
    pub name: Option<String>,
    pub handle: Option<isize>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeleteChain {
    pub family: String,
    pub table: String,
    pub name: Option<String>,
    pub handle: Option<isize>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeleteRule {
    pub family: String,
    pub table: String,
    pub chain: String,
    pub handle: isize,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeleteSet {
    pub family: String,
    pub table: String,
    pub name: Option<String>,
    pub handle: Option<isize>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeleteMap {
    pub family: String,
    pub table: String,
    pub name: Option<String>,
    pub handle: Option<isize>,
}

pub type DeleteElement = AddElement;

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeleteFlowtable {
    pub family: String,
    pub table: String,
    pub name: Option<String>,
    pub handle: Option<isize>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeleteCounter {
    pub family: String,
    pub table: String,
    pub name: Option<String>,
    pub handle: Option<isize>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeleteQuota {
    pub family: String,
    pub table: String,
    pub name: Option<String>,
    pub handle: Option<isize>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeleteCtHelper {
    pub family: String,
    pub table: String,
    pub name: Option<String>,
    pub handle: Option<isize>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeleteCtTimeout {
    pub family: String,
    pub table: String,
    pub name: Option<String>,
    pub handle: Option<isize>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeleteCtExpectation {
    pub family: String,
    pub table: String,
    pub name: Option<String>,
    pub handle: Option<isize>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeleteLimit {
    pub family: String,
    pub table: String,
    pub name: Option<String>,
    pub handle: Option<isize>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeleteSecmark {
    pub family: String,
    pub table: String,
    pub name: Option<String>,
    pub handle: Option<isize>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[non_exhaustive]
pub enum Delete {
    Table(DeleteTable),
    Chain(DeleteChain),
    Rule(DeleteRule),
    Set(DeleteSet),
    Map(DeleteMap),
    Element(DeleteElement),
    Flowtable(DeleteFlowtable),
    Counter(DeleteCounter),
    Quota(DeleteQuota),
    #[serde(rename = "ct helper")]
    CtHelper(DeleteCtHelper),
    #[serde(rename = "ct timeout")]
    CtTimeout(DeleteCtTimeout),
    #[serde(rename = "ct expectation")]
    CtExpectation(DeleteCtExpectation),
    Limit(DeleteLimit),
    Secmark(DeleteSecmark),
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListTable {
    pub family: String,
    pub name: String,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListChain {
    pub family: String,
    pub table: String,
    pub name: String,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListSet {
    pub family: String,
    pub table: String,
    pub name: String,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListMap {
    pub family: String,
    pub table: String,
    pub name: String,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListCounter {
    pub family: String,
    pub table: String,
    pub name: String,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListQuota {
    pub family: String,
    pub table: String,
    pub name: String,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListCtHelper {
    pub family: String,
    pub table: String,
    pub name: String,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListCtTimeout {
    pub family: String,
    pub table: String,
    pub name: String,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListCtExpectation {
    pub family: String,
    pub table: String,
    pub name: String,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListLimit {
    pub family: String,
    pub table: String,
    pub name: String,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListMeter {
    pub family: String,
    pub table: String,
    pub name: String,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListFlowtable {
    pub family: String,
    pub table: String,
    pub name: String,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListSecmark {
    pub family: String,
    pub table: String,
    pub name: String,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListTables {
    pub family: String,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListChains {
    pub family: String,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListSets {
    pub family: Option<String>,
    pub table: String,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListMaps {
    pub family: String,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListCounters {
    pub family: Option<String>,
    pub table: String,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListQuotas {
    pub family: String,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListCtHelpers {
    pub family: Option<String>,
    pub table: String,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListLimits {
    pub family: String,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListRuleset {
    pub family: String,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListMeters {
    pub family: String,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListFlowtables {
    pub family: String,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListSecmarks {
    pub family: String,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[non_exhaustive]
pub enum List {
    Table(ListTable),
    Chain(ListChain),
    Set(ListSet),
    Map(ListMap),
    Counter(ListCounter),
    Quota(ListQuota),
    #[serde(rename = "ct helper")]
    CtHelper(ListCtHelper),
    #[serde(rename = "ct timeout")]
    CtTimeout(ListCtTimeout),
    #[serde(rename = "ct expectation")]
    CtExpectation(ListCtExpectation),
    Limit(ListLimit),
    Meter(ListMeter),
    Flowtable(ListFlowtable),
    Secmark(ListSecmark),
    Tables(Option<ListTables>),
    Chains(Option<ListChains>),
    Sets(Option<ListSets>),
    Maps(Option<ListMaps>),
    Counters(Option<ListCounters>),
    Quotas(Option<ListQuotas>),
    #[serde(rename = "ct helpers")]
    CtHelpers(Option<ListCtHelpers>),
    Limits(Option<ListLimits>),
    Ruleset(Option<ListRuleset>),
    Meters(Option<ListMeters>),
    Flowtables(Option<ListFlowtables>),
    Secmarks(Option<ListSecmarks>),
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResetCounter {
    pub family: String,
    pub table: String,
    pub name: String,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResetCounters {
    pub family: Option<String>,
    pub table: String,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResetQuota {
    pub family: String,
    pub table: String,
    pub name: String,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResetQuotas {
    pub family: String,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[non_exhaustive]
pub enum Reset {
    Counter(ResetCounter),
    Counters(Option<ResetCounters>),
    Quota(ResetQuota),
    Quotas(Option<ResetQuotas>),
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FlushTable {
    pub family: String,
    pub name: String,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FlushChain {
    pub family: String,
    pub table: String,
    pub name: String,
}

#[serde_with::apply(Option => # [serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FlushSet {
    pub family: String,
    pub table: String,
    pub name: String,
}

#[serde_with::apply(Option => # [serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FlushMap {
    pub family: String,
    pub table: String,
    pub name: String,
}

#[serde_with::apply(Option => # [serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FlushMeter {
    pub family: String,
    pub table: String,
    pub name: String,
}

#[serde_with::apply(Option => # [serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FlushRuleset {
    pub family: String,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[non_exhaustive]
pub enum Flush {
    Table(FlushTable),
    Chain(FlushChain),
    Set(FlushSet),
    Map(FlushMap),
    Meter(FlushMeter),
    Ruleset(Option<FlushRuleset>),
}

#[serde_with::apply(Option => # [serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RenameChain {
    pub family: String,
    pub table: String,
    pub name: String,
    pub newname: String,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[non_exhaustive]
pub enum Rename {
    Chain(RenameChain),
}

/// Represents a command to issue to the `nft` binary
#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[non_exhaustive]
pub enum Command {
    Metainfo { json_schema_version: isize },
    Add(Add),
    Replace(Replace),
    Create(Add),
    Insert(Insert),
    Delete(Delete),
    List(List),
    Reset(Reset),
    Flush(Flush),
    Rename(Rename),
}

/// Represents a list of commands to issue to the `nft` binary
#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Commands {
    #[serde(rename = "nftables")]
    pub commands: Vec<Command>,
}

impl Commands {
    pub fn new<I: IntoIterator<Item = Command>>(commands: I) -> Self {
        Self { commands: commands.into_iter().collect::<Vec<_>>() }
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

    pub fn from_reader<R, T>(reader: R) -> serde_json::Result<Self>
    where
        R: std::io::Read,
        T: serde::de::DeserializeOwned,
    {
        serde_json::from_reader(reader)
    }

    pub fn to_value(&self) -> serde_json::Result<serde_json::Value> {
        serde_json::to_value(self)
    }

    pub fn to_string(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }
}

impl std::fmt::Display for Commands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string().map_err(|_| std::fmt::Error::default())?)
    }
}
