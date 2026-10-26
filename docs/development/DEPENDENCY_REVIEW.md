# Dependency review

Review date: 2026-10-26. The Rust lockfile resolved 63 third-party packages.
`cargo metadata` reported an SPDX-style licence expression for every third-party
package; only the four local Ivy packages correctly report no licence while the
project decision is pending.

Observed third-party licence families are:

- MIT and/or Apache-2.0;
- BSD-3-Clause and BSD-1-Clause;
- Unlicense or MIT;
- Unicode-3.0 combined with MIT/Apache-2.0;
- Apache-2.0 with LLVM exception;
- BSL-1.0 as an alternative expression for one dependency that also offers
  Apache-2.0.

No copyleft dependency was observed in the current Rust resolution. The Dart
package has no third-party runtime dependencies.

This is a technical inventory, not legal approval. Before public launch:

1. preserve the exact lockfile;
2. generate a full notice/inventory from the launch commit;
3. verify selected alternatives in compound licence expressions;
4. review bundled assets and GitHub Actions separately;
5. obtain authorised approval for the Developer Kit licence.
