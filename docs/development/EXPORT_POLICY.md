# Private-source export policy

The public repository is not a mirror of the private monorepo. Files may enter
only through an explicit allowlist and review.

## Current allowlist

No private source path was copied into the initial candidate. The following
private materials were used only as read-only design evidence:

- canonical `ARCHITECTURE.md`;
- accepted architecture decisions;
- official module manifests;
- public-surface SDK and tool manifests;
- module and platform tests used to confirm implemented behavior.

The resulting files are newly authored public docs, schemas, SDK code, tools and
examples.

## Permanent denylist

- Core and Core UI source;
- official module implementation source pending approved licences;
- private runtime/mobile wire protocol;
- production signing, recovery and key-protection implementation;
- Juma, Attraxi and other business operations;
- internal roadmaps, work packets, reviews and worker state;
- local paths, logs, hostnames, credentials and release artefacts;
- Manager, Roadmap Runner and Orchestrator implementation/state.

## Future export gate

Every proposed export must record source revision, exact paths, rights holder,
licence decision, third-party origin, generated-file provenance, independent
build result, secret scan and human approval. Synchronisation is never automatic.
