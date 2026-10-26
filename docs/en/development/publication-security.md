<!-- doc-key: publication-security -->
<!-- parity-version: 2 -->
<!-- sections: matrix,allowlist,layers,dependencies,generated,github,limits -->

# Publication, security and dependency review

[Nederlands](../../nl/development/publication-security.md) | **English**

<!-- section: matrix -->
## Publication matrix

| Area | Public decision | Build status | Licence status |
|---|---|---|---|
| Core/Core UI | Architecture only; no source | Intentionally unavailable | Proprietary |
| Bridge/Vault/Hooman/Flow | Concepts only pending separate review | Not in repository | System licence pending |
| Agenda/Pulse/BookMe | Use cases only; possible later references | Not in repository | Classification/licence pending |
| Rust/Dart Developer Kit | Newly authored independent subset | Builds and tests independently | Apache-2.0 proposed only |
| Manifest/proto contracts | Developer-facing subset | Schema and semantic tests | Apache-2.0 proposed only |
| CLI/widget preview/examples | Public development tools | Builds and tests independently | Apache-2.0 proposed only |
| Marketplace | Policy and proposed verification record | Documentation only | Legal/product approval pending |
| Business/internal tooling | Never exported here | Not applicable | Private/internal |

<!-- section: allowlist -->
## Allowlist

Allowed top-level content is restricted to reviewed docs, contracts, SDK, tools,
examples, conformance, media, scripts and GitHub configuration. Core, Core UI,
official module implementations, private runtime/mobile wire, production signing,
business operations, internal roadmap/worker state and release artefacts are
denylisted. Export is manual and commit-pinned; no automatic private sync exists.

<!-- section: layers -->
## Security layers

CI combines Gitleaks, current-tree and Git-history pattern scans, privacy and
infrastructure checks, proprietary-path checks, explicit top-level allowlist,
media inspection, dependency licence inventory, Rust/Dart tests, manifest
fixtures and bilingual documentation/link validation.

<!-- section: dependencies -->
## Dependencies

Every registry package must expose a licence expression. The inventory rejects
unknown registry licences and reports dependency changes for human review. A
technical SPDX inventory is not legal approval.

<!-- section: generated -->
## Generated files

Only reviewed lockfiles are committed. Generated bindings require a documented,
reproducible generator before addition. Development keys, signatures, binaries,
packages and build outputs are ignored or rejected.

<!-- section: github -->
## GitHub controls

The private personal-account plan currently blocks native branch protection,
secret push protection, Dependency Review and CodeQL. CI alternatives are active;
the native controls remain publication blockers. Dependabot vulnerability alerts
and automated security fixes are enabled; routine update PRs are disabled during
private preparation.

<!-- section: limits -->
## Limits

Automated scans can miss contextual secrets, rights conflicts, misleading
claims, sensitive screenshots or novel data formats. Green automation is
necessary but never sufficient: a human review of the exact launch commit and
history remains mandatory.
