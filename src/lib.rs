// Copyright (c) nftables-json Developers
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Serde JSON model for interacting with the nftables `nft` executable
//!
//! Provides Rust types that map directly to the nftables JSON object model,
//! allowing serialization and deserialization of input and output from the
//! `nft --json` command using [Serde](https://crates.io/crates/serde) and
//! [`serde_json`](https://crates.io/crates/serde_json).
//!
//! # Example
//!
//! Creating commands that can be piped into `nft --json --file -`
//!
//! ```rust
//! use nftables_json::{command::*, expression::*, statement::*};
//!
//! let mut commands = Commands::default();
//! commands.extend([
//!     // flush rulesets for all families
//!     Command::Flush(Flush::Ruleset(None)),
//!     // create a new table called "default"
//!     Command::Add(Add::Table(AddTable {
//!         family: "inet".into(),
//!         name: "default".into(),
//!         ..AddTable::default()
//!     })),
//!     // attach a chain to the "default" table called "input"
//!     Command::Add(Add::Chain(AddChain {
//!         family: "inet".into(),
//!         table: "default".into(),
//!         name: "input".into(),
//!         r#type: Some("filter".into()),
//!         hook: Some("input".into()),
//!         prio: Some(0),
//!         policy: Some("accept".into()),
//!         ..AddChain::default()
//!     })),
//!     Command::Add(Add::Rule(AddRule {
//!         // attach a rule to the "input" chain in the "default" table that drops udp sport 53
//!         family: "inet".into(),
//!         table: "default".into(),
//!         chain: "input".into(),
//!         expr: Some(vec![
//!             Statement::Match(Match {
//!                 left: Expression::Payload {
//!                     payload: Payload::Named { protocol: "udp".into(), field: "sport".into() },
//!                 }
//!                 .into(),
//!                 right: Expression::Immediate(Immediate::Number(53)).into(),
//!                 op: "==".into(),
//!             }),
//!             Statement::Drop(()),
//!         ]),
//!         ..AddRule::default()
//!     })),
//! ]);
//!
//! // not shown: how to invoke `nft` from Rust and pipe json to stdin
//! println!("{}", commands.to_string().unwrap());
//!
//! /*
//! {"nftables":[
//!   {"flush":{"ruleset":null}},
//!   {"add":{"table":{"family":"inet","name":"default"}}},
//!   {"add":{"chain":{"family":"inet","table":"default","name":"input","policy":"accept","type":"filter","hook":"input","prio":0}}},
//!   {"add":{"rule":{"family":"inet","table":"default","chain":"input","expr":[
//!     {"match":{"left":{"payload":{"protocol":"udp","field":"sport"}},"right":53,"op":"=="}},
//!     {"drop":null}
//!   ]}}}
//! ]}
//! */
//! ```
//!
//! # Example
//!
//! Parsing a ruleset produced by
//! `printf '{"nftables":[{"list":{"ruleset":null}}]}' | nft --json --file -`
//!
//! ```rust
//! use nftables_json::{object::*, expression::*, statement::*};
//!
//! // not shown: how to invoke `nft` from Rust and collect stdout
//! let output_str = r#"
//! {"nftables":[
//!   {"metainfo":{"version":"0.9.8","release_name":"E.D.S.","json_schema_version":1}},
//!   {"table":{"family":"inet","name":"default","handle":3}},
//!   {"chain":{"family":"inet","table":"default","name":"input","handle":1,"type":"filter","hook":"input","prio":0,"policy":"accept"}},
//!   {"rule":{"family":"inet","table":"default","chain":"input","handle":3,"expr":[
//!     {"match":{"op":"==","left":{"payload":{"protocol":"udp","field":"sport"}},"right":53}},
//!     {"drop":null}
//!   ]}},
//!   {"chain":{"family":"inet","table":"default","name":"output","handle":2,"type":"filter","hook":"output","prio":0,"policy":"accept"}}
//! ]}
//! "#;
//!
//! let objects = Objects::from_str(output_str).unwrap();
//!
//! for object in objects.iter() {
//!     match object {
//!         Object::Metainfo(Metainfo { json_schema_version, .. }) => {
//!             eprintln!("[metainfo] schema version {json_schema_version}");
//!         }
//!         Object::Table(Table { family, name, .. }) => {
//!             eprintln!("[table] {family} {name}");
//!         }
//!         Object::Chain(Chain { family, table, name, hook: Some(hook), .. }) => {
//!             eprintln!("[chain] {family} {table} {name} hook {hook}");
//!         }
//!         Object::Rule(Rule { family, table, chain, .. }) => {
//!             eprintln!("[rule] {family} {table} {chain}");
//!         }
//!         _ => {}
//!     }
//! }
//!
//! /*
//! schema version 1
//! table family inet name default
//! chain family inet table default name input hook input
//! rule family inet table default chain input
//! chain family inet table default name output hook output
//! */
//! ```

pub mod command;
pub mod expression;
pub mod object;
pub mod statement;
