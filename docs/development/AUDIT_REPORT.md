# Source audit report

Audit date: 2026-10-26
Private source revision: `453b2bea65c7765419ac1a11e4185479b2234c1b`

## Scope and evidence

The audit inspected the canonical architecture, accepted/proposed ADR status,
workspace manifests, Core/Core UI boundaries, official module manifests, Rust
and Dart SDKs, CLI and widget tooling, tests, signing, package layout and current
roadmap direction. Historical work packets and worker state were not used as
publication sources.

The private snapshot contains 1,591 tracked files, including substantial Rust
and Dart source and hundreds of Rust test annotations. The repository itself was
clean at the start of this work.

## Findings

1. The private CLI imports the proprietary Core crate directly and cannot be
   published or independently built as-is.
2. The private Rust runtime SDK is technically separated as a crate but exposes
   broad runtime, authorization and mobile-wire contracts. Publishing it whole
   would exceed the approved public boundary.
3. The private Dart package is generated from the same broad runtime protocol
   and has no independently documented public generation boundary.
4. The platform-contract crate has useful provider test helpers, but currently
   includes storage and generated runtime types whose licence and public API are
   not approved.
5. Existing module manifests consistently use schema version 3, explicit
   capability domains, provided/required capabilities and retry semantics.
6. Existing widget preview code assumes the private monorepo layout. A new
   offline fixture preview is safer and independently usable.
7. Official package signing has protected-key and operational behavior that
   must remain private. The public kit therefore uses purpose-bound development
   signatures only.
8. Accepted architecture and implementation agree on exclusive ownership,
   default-deny grants, capability routing, the external Bridge boundary and
   declarative widgets.
9. Several ADRs are still Proposed, including distribution/federation areas.
   Public status documents therefore classify those items as Planned or
   Exploring rather than implemented.
10. Some internal module documentation is historical or status-sensitive. It
    was summarised manually instead of copied.

## Publication decision

No private source file was bulk-exported. The repository contains newly authored
public contracts, tools, examples and documentation derived from accepted
concepts. This avoids Core leakage and produces an independently testable kit.

The complete component-by-component decision is in
[`PUBLICATION_MATRIX.md`](PUBLICATION_MATRIX.md).
