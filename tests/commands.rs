// Copyright (c) nftables-json Developers
// SPDX-License-Identifier: GPL-2.0-only

use nftables_json::*;

const TESTCASES: &[&'static str] = &[include_str!("commands/netdev.json")];

#[test]
fn test() {
    for (i, testcase) in TESTCASES.iter().enumerate() {
        let expected = serde_json::from_str::<serde_json::Value>(testcase).unwrap().to_string();
        eprintln!("[{}] expected: {}", i, expected);
        let deserialized = serde_json::from_str(testcase).unwrap();
        let reserialized = command::Commands::to_string(&deserialized).unwrap();
        let actual = serde_json::from_str::<serde_json::Value>(&reserialized).unwrap().to_string();
        eprintln!("[{}] actual:   {}", i, actual);
        eprintln!("[{}] repr:     {:?}", i, deserialized);
        assert_eq!(expected, actual);
        eprintln!("------------------")
    }
}
