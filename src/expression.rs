// Copyright (c) nftables-json Developers
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Provides types related to specifying a rule's match criteria

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub enum Binary {
    #[serde(rename = "|")]
    Or((Box<Expression>, Box<Expression>)),
    #[serde(rename = "^")]
    Not((Box<Expression>, Box<Expression>)),
    #[serde(rename = "&")]
    And((Box<Expression>, Box<Expression>)),
    #[serde(rename = "<<")]
    LShift((Box<Expression>, Box<Expression>)),
    #[serde(rename = ">>")]
    RShift((Box<Expression>, Box<Expression>)),
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ct {
    pub key: String,
    pub dir: Option<String>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Elem {
    pub val: Box<Expression>,
    pub timeout: Option<isize>,
    pub expires: Option<isize>,
    pub comment: Option<String>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Exthdr {
    pub name: String,
    pub field: Option<String>,
    pub offset: Option<isize>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Fib {
    pub result: String,
    #[serde(with = "serde_with::As::<Option<serde_with::OneOrMany<serde_with::Same>>>")]
    pub flags: Option<Vec<String>>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[non_exhaustive]
pub enum Hash {
    Jhash {
        #[serde(rename = "mod")]
        r#mod: isize,
        offset: Option<isize>,
        expr: Box<Expression>,
        seed: Option<isize>,
    },
    Symhash {
        #[serde(rename = "mod")]
        r#mod: isize,
        offset: Option<isize>,
    },
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, untagged)]
pub enum Immediate {
    String(String),
    Number(isize),
    Boolean(bool),
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IpOption {
    pub name: String,
    pub field: Option<String>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Xfrm {
    pub key: String,
    pub family: Option<String>,
    pub dir: Option<String>,
    pub spnum: Option<isize>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Map {
    pub key: Box<Expression>,
    pub data: Box<Expression>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Meta {
    pub key: String,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Numgen {
    pub mode: String,
    #[serde(rename = "mod")]
    pub r#mod: isize,
    pub offset: Option<isize>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Osf {
    pub key: String,
    pub ttl: Option<String>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, untagged)]
pub enum Payload {
    Raw { base: String, offset: isize, len: isize },
    Named { protocol: String, field: String },
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Prefix {
    pub addr: Box<Expression>,
    pub len: isize,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Rt {
    pub key: String,
    pub family: Option<String>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SctpChunk {
    pub name: String,
    pub field: Option<String>,
}

#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Socket {
    pub key: Option<String>,
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
#[serde(deny_unknown_fields, untagged)]
pub enum TcpOption {
    Raw { base: isize, offset: isize, len: isize },
    Named { name: String, field: Option<String> },
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
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[non_exhaustive]
pub enum Verdict {
    Accept(()),
    Drop(()),
    Continue(()),
    Return(()),
    Jump { target: String },
    Goto { target: String },
}

/// Represents a rule's match criteria
#[serde_with::apply(Option => #[serde(default, skip_serializing_if = "Option::is_none")])]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, untagged)]
#[non_exhaustive]
pub enum Expression {
    Immediate(Immediate),
    List(Vec<Self>),
    Concat {
        concat: Vec<Self>,
    },
    Set {
        set: Vec<Self>,
    },
    Map {
        map: Map,
    },
    Prefix {
        prefix: Prefix,
    },
    Range {
        range: (Immediate, Immediate),
    },
    Payload {
        payload: Payload,
    },
    Exthdr {
        exthdr: Exthdr,
    },
    IpOption {
        #[serde(rename = "ip option")]
        ip_option: IpOption,
    },
    TcpOption {
        #[serde(rename = "tcp option")]
        tcp_option: TcpOption,
    },
    SctpChunk {
        #[serde(rename = "sctp chunk")]
        sctp_chunk: SctpChunk,
    },
    Meta {
        meta: Meta,
    },
    Rt {
        rt: Rt,
    },
    Ct {
        ct: Ct,
    },
    Numgen {
        numgen: Numgen,
    },
    Hash(Hash),
    Fib {
        fib: Fib,
    },
    Xfrm {
        ipsec: Xfrm,
    },
    Binary(Binary),
    Verdict(Verdict),
    Elem {
        elem: Elem,
    },
    Socket {
        socket: Socket,
    },
    Osf {
        osf: Osf,
    },
    Synproxy {
        synproxy: Option<Synproxy>,
    },
    Tproxy {
        tproxy: Tproxy,
    },
}
