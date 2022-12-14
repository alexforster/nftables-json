# nftables-json

**Serde JSON model for interacting with the nftables `nft` executable**

Provides Rust types that map directly to the nftables JSON object model,
allowing serialization and deserialization of input and output from the
`nft --json` command using [Serde](https://crates.io/crates/serde) and
[`serde_json`](https://crates.io/crates/serde_json).

### Contributing

This library is made available under the terms of either the [Apache License,
Version 2.0](LICENSE.Apache-2.0) or the [MIT License](LICENSE.MIT), at your
option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this library by you shall be dual licensed as above, without
any additional terms or conditions.

```
Copyright (c) nftables-json Developers

SPDX-License-Identifier: MIT OR Apache-2.0
```

Note that the tests for this library are made available under a different
license. See the [README in the `tests/` directory](tests/README.md) for more
information.
