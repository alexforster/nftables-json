// Copyright (c) nftables-json Developers
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod command;
pub mod expression;
pub mod object;
pub mod statement;

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub enum Command {
    Add(command::Add),
    Replace(command::Replace),
    Create(command::Add),
    Insert(command::Insert),
    Delete(command::Delete),
    List(command::List),
    Reset(command::Reset),
    Flush(command::Flush),
    Rename(command::Rename),
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, untagged)]
pub enum Expression {
    Immediate(expression::Immediate),
    List(Vec<Self>),
    Concat {
        concat: Vec<Self>,
    },
    Set {
        set: Vec<Self>,
    },
    Map {
        map: expression::Map,
    },
    Prefix {
        prefix: expression::Prefix,
    },
    Range {
        range: (expression::Immediate, expression::Immediate),
    },
    Payload {
        payload: expression::Payload,
    },
    Exthdr {
        exthdr: expression::Exthdr,
    },
    IpOption {
        #[serde(rename = "ip option")]
        ip_option: expression::IpOption,
    },
    TcpOption {
        #[serde(rename = "tcp option")]
        tcp_option: expression::TcpOption,
    },
    SctpChunk {
        #[serde(rename = "sctp chunk")]
        sctp_chunk: expression::SctpChunk,
    },
    Meta {
        meta: expression::Meta,
    },
    Rt {
        rt: expression::Rt,
    },
    Ct {
        ct: expression::Ct,
    },
    Numgen {
        numgen: expression::Numgen,
    },
    Hash(expression::Hash),
    Fib {
        fib: expression::Fib,
    },
    Xfrm {
        ipsec: expression::Xfrm,
    },
    Binary(expression::Binary),
    Verdict(expression::Verdict),
    Elem {
        elem: expression::Elem,
    },
    Socket {
        socket: expression::Socket,
    },
    Osf {
        osf: expression::Osf,
    },
    Synproxy {
        synproxy: Option<expression::Synproxy>,
    },
    Tproxy {
        tproxy: expression::Tproxy,
    },
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub enum Object {
    Metainfo(object::Metainfo),
    Table(object::Table),
    Chain(object::Chain),
    Rule(object::Rule),
    Set(object::Set),
    #[serde(alias = "meter")]
    Map(object::Map),
    Element(object::Element),
    Flowtable(object::Flowtable),
    Counter(object::Counter),
    Quota(object::Quota),
    #[serde(rename = "ct helper")]
    CtHelper(object::CtHelper),
    #[serde(rename = "ct timeout")]
    CtTimeout(object::CtTimeout),
    #[serde(rename = "ct expectation")]
    CtExpectation(object::CtExpectation),
    Limit(object::Limit),
    Secmark(object::Secmark),
    Synproxy(object::Synproxy),
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub enum Statement {
    Accept(()),
    Drop(()),
    Continue(()),
    Jump {
        target: String,
    },
    Goto {
        target: String,
    },
    Return(()),
    Match(statement::Match),
    Counter(Option<statement::Counter>),
    Mangle(statement::Mangle),
    Quota(statement::Quota),
    Limit(statement::Limit),
    Flow(statement::Flow),
    Fwd(statement::Fwd),
    Notrack(Option<()>),
    Dup(statement::Dup),
    Snat(statement::Nat),
    Dnat(statement::Nat),
    Masquerade(Option<statement::Nat>),
    Redirect(Option<statement::Nat>),
    Reject(Option<statement::Reject>),
    Set(statement::Set),
    Log(Option<statement::Log>),
    #[serde(rename = "ct helper")]
    CtHelper(Box<Expression>),
    #[serde(rename = "ct timeout")]
    CtTimeout(Box<Expression>),
    #[serde(rename = "ct expectation")]
    CtExpectation(Box<Expression>),
    Meter(statement::Meter),
    Queue(Option<statement::Queue>),
    #[serde(rename = "ct count")]
    CtCount(statement::CtCount),
    Tproxy(statement::Tproxy),
    Synproxy(Option<statement::Synproxy>),
    Reset {
        #[serde(rename = "tcp option")]
        tcp_option: Box<expression::TcpOption>,
    },
    Secmark(Box<Expression>),
    Vmap(statement::Vmap),
    Xt(Option<String>),
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Commands {
    nftables: Vec<Command>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Objects {
    nftables: Vec<Object>,
}
