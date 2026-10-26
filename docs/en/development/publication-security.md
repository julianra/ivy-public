<!-- doc-key: publication-security -->
<!-- parity-version: 3 -->
<!-- sections: matrix,allowlist,layers,dependencies,generated,github,limits -->

# Publishing Ivy Public safely

[Nederlands](../../nl/development/publication-security.md) | **English**

## Why do we need this review?

Ivy Public is intended for external developers. Private Ivy Core source and internal business information must **never be accidentally published on GitHub**.

Before publication, we therefore check not only whether code works, but also **what is being released, who owns it, whether sensitive data is included and whether GitHub security is configured correctly**.

**Important:** green CI checks do not automatically mean a repository is safe to publish. The owner must review the exact revision and its Git history and explicitly authorize publication.

<!-- section: matrix -->
## What may be made public?

This table distinguishes public development code from components that must remain private.

| Component | Public scope | Technical status | License |
|---|---|---|---|
| Core / Core UI | Architecture only; no source code | Intentionally excluded | Proprietary |
| Bridge / Vault / Hooman / Flow | Concepts only, pending separate review | Not in repository | System licensing pending |
| Agenda / Pulse / BookMe | Use cases and explanations; possible later reference material | Not in repository | Classification/licensing pending |
| Rust/Dart Developer Kit | Independently authored public developer tools | Builds and tests independently | Apache-2.0 |
| Manifest and protobuf contracts | Developer-facing public subset | Schema and semantic tests | Apache-2.0 |
| CLI, widget preview and examples | Public development and testing tools | Build and test independently | Apache-2.0 |
| Marketplace | Policy and proposed verification record | Documentation only | Legal/product approval pending |
| Business and internal tools | Nothing | Not applicable | Private/internal |

<!-- section: allowlist -->
## How do we keep private files out?

We maintain two lists:

- An **allowlist** of content categories that may appear after review: documentation, contracts, SDK, tools, examples, conformance, media, scripts and GitHub configuration.
- A **denylist** of material that must not be included: Core, Core UI, official module implementations, private runtime/mobile communication contracts, production signing, business operations, internal roadmap and worker state, and release artifacts.

Export is **manually reviewed and pinned to a specific commit**. There is no automatic synchronization from the private repository into Ivy Public.

<!-- section: layers -->
## What security checks protect the repository?

Several checks work together:

- **Gitleaks** looks for possible keys and other secrets.
- **File and history scans** look for prohibited paths and patterns in both current files and Git history.
- **Privacy and infrastructure checks** look for personal or operational details.
- **Top-level allowlist and media checks** assess folders and images.
- **Dependency license inventory** records licenses of third-party software.
- **Rust, Dart and manifest tests** verify technical behavior.
- **Bilingual documentation and link checks** verify matching language files and working references.

Publication history requires the configured **GitHub noreply email** for every author and committer. Each proposed pull-request commit must also contain a **DCO 1.1 Signed-off-by**, certifying the contributor's right to submit the code.

<!-- section: dependencies -->
## How do we review third-party libraries?

Software often relies on other packages (*dependencies*), each with its own licensing terms.

Every registry package must provide a license expression. The inventory rejects unknown registry licenses and reports dependency changes for human review.

An **SPDX inventory** is a technical license record, **not legal approval** of all reused code.

<!-- section: generated -->
## How do we handle generated files and keys?

Only reviewed lockfiles may be committed. Before generated bindings are added, the generator and reproducible steps must be documented.

Development keys, signatures, binaries, packages and build outputs are ignored or rejected by safety checks. They must not simply enter public source history.

<!-- section: github -->
## What GitHub protection is prepared?

These are recorded project readiness statuses. **They do not prove every setting has been rechecked live today.** Verify each one before launch.

| Control | Recorded status | What it means |
|---|---|---|
| Rust, Dart, docs, Gitleaks and repository-safety CI | `VERIFIED ACTIVE` | Previously executed on specific private commits |
| Dependabot vulnerability alerts and automated security fixes | `VERIFIED ACTIVE` | Previously confirmed via GitHub API; routine version-update PRs remain disabled |
| CodeQL | `READY WHEN PUBLIC` | Workflow exists; configuration runs it only when public |
| Dependency Review | `READY WHEN PUBLIC` | PR workflow prepared but inactive for the private repository |
| Branch rules | `MANUAL GITHUB SETTING REQUIRED` | Verify `main` is truly protected; require appropriate reviews, CODEOWNERS and CI; block force pushes and deletion |
| GitHub secret scanning and push protection | `MANUAL GITHUB SETTING REQUIRED` | Check feature availability and activation in GitHub Settings |
| Private Vulnerability Reporting | `MANUAL GITHUB SETTING REQUIRED` | Verify that vulnerabilities can actually be submitted privately |

**Branch rules caveat:** a ruleset for `main` has been prepared, but while GitHub does not enforce it for this private repository due to plan restrictions, **branch protection is not proven active**. After any approved public launch, test the required checks and blocked merges with a real test PR.

This list **does not authorize changes to repository visibility**.

<!-- section: limits -->
## Why is a final human review still necessary?

Automated scans may miss a secret that does not resemble a typical key, misjudge a license conflict or overlook a sensitive screenshot.

Before publication, a person must therefore review **the exact launch commit, all relevant files and the complete Git history**. Also check that public descriptions do not claim features that are not yet available.

**The final rule:** publish only after explicit approval. Passing tests is necessary, but never sufficient on its own.
