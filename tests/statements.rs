// Copyright (c) nftables-json Developers
// SPDX-License-Identifier: GPL-2.0-only

use nftables_json::*;

const TESTCASES: &[&'static str] = &[
    include_str!("statements/any/counter.t.json"),
    include_str!("statements/any/ct.t.json"),
    include_str!("statements/any/limit.t.json"),
    include_str!("statements/any/log.t.json"),
    include_str!("statements/any/meta.t.json"),
    include_str!("statements/any/queue.t.json"),
    include_str!("statements/any/quota.t.json"),
    include_str!("statements/any/rawpayload.t.json"),
    include_str!("statements/any/rt.t.json"),
    include_str!("statements/any/tcpopt.t.json"),
    include_str!("statements/arp/arp.t.json"),
    include_str!("statements/bridge/ether.t.json"),
    include_str!("statements/bridge/icmpX.t.json"),
    include_str!("statements/bridge/meta.t.json"),
    include_str!("statements/bridge/reject.t.json"),
    include_str!("statements/bridge/vlan.t.json"),
    include_str!("statements/inet/ah.t.json"),
    include_str!("statements/inet/comp.t.json"),
    include_str!("statements/inet/ct.t.json"),
    include_str!("statements/inet/dccp.t.json"),
    include_str!("statements/inet/dnat.t.json"),
    include_str!("statements/inet/esp.t.json"),
    include_str!("statements/inet/ether-ip.t.json"),
    include_str!("statements/inet/ether.t.json"),
    include_str!("statements/inet/fib.t.json"),
    include_str!("statements/inet/icmp.t.json"),
    include_str!("statements/inet/icmpX.t.json"),
    include_str!("statements/inet/ip.t.json"),
    include_str!("statements/inet/ip_tcp.t.json"),
    include_str!("statements/inet/ipsec.t.json"),
    include_str!("statements/inet/map.t.json"),
    include_str!("statements/inet/meta.t.json"),
    include_str!("statements/inet/osf.t.json"),
    include_str!("statements/inet/reject.t.json"),
    include_str!("statements/inet/rt.t.json"),
    include_str!("statements/inet/sctp.t.json"),
    include_str!("statements/inet/sets.t.json"),
    include_str!("statements/inet/snat.t.json"),
    include_str!("statements/inet/socket.t.json"),
    include_str!("statements/inet/synproxy.t.json"),
    include_str!("statements/inet/tcp.t.json"),
    include_str!("statements/inet/tproxy.t.json"),
    include_str!("statements/inet/udp.t.json"),
    include_str!("statements/inet/udplite.t.json"),
    include_str!("statements/inet/vmap.t.json"),
    include_str!("statements/ip/ct.t.json"),
    include_str!("statements/ip/dnat.t.json"),
    include_str!("statements/ip/dup.t.json"),
    include_str!("statements/ip/ether.t.json"),
    include_str!("statements/ip/flowtable.t.json"),
    include_str!("statements/ip/hash.t.json"),
    include_str!("statements/ip/icmp.t.json"),
    include_str!("statements/ip/igmp.t.json"),
    include_str!("statements/ip/ip.t.json"),
    include_str!("statements/ip/ip_tcp.t.json"),
    include_str!("statements/ip/masquerade.t.json"),
    include_str!("statements/ip/meta.t.json"),
    include_str!("statements/ip/numgen.t.json"),
    include_str!("statements/ip/objects.t.json"),
    include_str!("statements/ip/redirect.t.json"),
    include_str!("statements/ip/reject.t.json"),
    include_str!("statements/ip/rt.t.json"),
    include_str!("statements/ip/sets.t.json"),
    include_str!("statements/ip/snat.t.json"),
    include_str!("statements/ip/tcp.t.json"),
    include_str!("statements/ip/tproxy.t.json"),
    include_str!("statements/ip6/dnat.t.json"),
    include_str!("statements/ip6/dst.t.json"),
    include_str!("statements/ip6/dup.t.json"),
    include_str!("statements/ip6/ether.t.json"),
    include_str!("statements/ip6/exthdr.t.json"),
    include_str!("statements/ip6/flowtable.t.json"),
    include_str!("statements/ip6/frag.t.json"),
    include_str!("statements/ip6/hbh.t.json"),
    include_str!("statements/ip6/icmpv6.t.json"),
    include_str!("statements/ip6/ip6.t.json"),
    include_str!("statements/ip6/map.t.json"),
    include_str!("statements/ip6/masquerade.t.json"),
    include_str!("statements/ip6/meta.t.json"),
    include_str!("statements/ip6/mh.t.json"),
    include_str!("statements/ip6/redirect.t.json"),
    include_str!("statements/ip6/reject.t.json"),
    include_str!("statements/ip6/rt.t.json"),
    include_str!("statements/ip6/rt0.t.json"),
    include_str!("statements/ip6/sets.t.json"),
    include_str!("statements/ip6/snat.t.json"),
    include_str!("statements/ip6/srh.t.json"),
    include_str!("statements/ip6/tproxy.t.json"),
    include_str!("statements/ip6/vmap.t.json"),
    include_str!("statements/netdev/dup.t.json"),
    include_str!("statements/netdev/fwd.t.json"),
    include_str!("statements/netdev/reject.t.json"),
];

#[test]
fn test() {
    let mut i = 0;
    for testcases in TESTCASES {
        let mut testcase_statement = String::default();
        let mut testcase_json = String::default();
        for line in testcases.lines() {
            if line.starts_with("#") {
                if !testcase_json.is_empty() {
                    test_case(i, &testcase_statement, &testcase_json);
                    i += 1;
                }
                testcase_statement = line.trim_start_matches("#").trim().to_string();
                testcase_json = String::default();
            } else {
                testcase_json += line;
                testcase_json += "\n";
            }
        }
    }
}

fn test_case(i: usize, statement: &str, json: &str) {
    eprintln!("[{}] stmt:     {:?}", i, statement);
    let expected = serde_json::from_str::<serde_json::Value>(json).unwrap().to_string();
    eprintln!("[{}] expected: {}", i, expected);
    let deserialized: Vec<Statement> = serde_json::from_str(json).unwrap();
    let reserialized = serde_json::to_string(&deserialized).unwrap();
    let actual = serde_json::from_str::<serde_json::Value>(&reserialized).unwrap().to_string();
    eprintln!("[{}] actual:   {}", i, actual);
    eprintln!("[{}] repr:     {:?}", i, deserialized);
    assert_eq!(expected, actual);
    eprintln!("------------------")
}
