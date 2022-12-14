// Copyright (c) nftables-json Developers
// SPDX-License-Identifier: GPL-2.0-only

use nftables_json::*;

const TESTCASES: &[&'static str] = &[
    include_str!("rulesets/docker.json"),
    include_str!("rulesets/obj_comment.json"),
    include_str!("rulesets/secmark_objref.json"),
    include_str!("rulesets/set_statements.json"),
    include_str!("rulesets/table_map.json"),
];

#[test]
fn test() {
    for (i, testcase) in TESTCASES.iter().enumerate() {
        let expected = serde_json::from_str::<serde_json::Value>(testcase).unwrap().to_string();
        eprintln!("[{}] expected: {}", i, expected);
        let deserialized = object::Objects::from_str(testcase).unwrap();
        let reserialized = serde_json::to_string(&deserialized).unwrap();
        let actual = serde_json::from_str::<serde_json::Value>(&reserialized).unwrap().to_string();
        eprintln!("[{}] actual:   {}", i, actual);
        eprintln!("[{}] repr:     {:?}", i, deserialized);
        assert_eq!(expected, actual);
        eprintln!("------------------")
    }
}
