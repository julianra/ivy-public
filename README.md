# Ivy Developer Kit

> **Private launch candidate — not published.** This repository is being
> prepared for a later public launch. It is a developer preview, not a
> production release and not the complete Ivy runtime.

Ivy is a local-first capability platform for building private, modular software
systems that remain under the user's control. Instead of allowing one large
application to own every domain, Ivy lets independent modules provide explicit,
versioned capabilities under a default-deny permission model.

## Why Ivy

Useful digital systems often become difficult to replace: identity, storage,
automation, user interfaces and domain logic are tightly coupled. Ivy separates
those responsibilities. A consumer invokes a capability such as
`example.clock.now`; it does not call a hard-coded module. The platform resolves
the installed provider, checks declarations and grants, passes trusted context
separately from the payload, and records the decision.

This creates room for replaceable providers, local operation, bounded authority
and explicit failure behavior.

## Architecture in one minute

```text
users and external systems
       |             |
     Prism         Bridge
       |             |
       +--- capabilities ---+
                             |
                      proprietary Core
                  registry / grants / audit
                             |
          +------------------+------------------+
          |                  |                  |
       Agenda              Flow               other
          |                  |                modules
          +-------------- Vault ----------------+
```

- **Capabilities** are versioned operations identified independently from a
  module.
- **Modules** declare the capability domains they own, what they provide and
  what they require.
- **Grants** are exact permissions. Installed ownership never implies access.
- **Core** owns registry, grants, routing, lifecycle and audit. Its source is
  proprietary and is not included here.
- **Bridge** is the external network boundary; **Prism** composes user
  interfaces; **Vault** provides durable storage; **Flow** coordinates durable
  routines. Official system-module source is not included in this preview.

Read [ARCHITECTURE.md](ARCHITECTURE.md) and the
[concept guides](docs/concepts/capabilities.md) for the boundaries.

## What is in this repository

This repository contains an independently buildable Developer Kit:

- a public subset of module and capability contracts;
- a Rust SDK with manifest validation and a development simulator;
- dependency-free Dart development types;
- a module generator and manifest validator;
- development-only Ed25519 signing helpers;
- an offline widget-preview generator;
- a complete `hello-clock` provider example;
- conformance tests for ownership, grants, revocation and signing.

The simulator is **not Ivy Core**, is not a production security boundary and
must never be used to claim production compatibility.

## Status

| Area | Status | Included here |
|---|---|---|
| Capability/domain model | Implemented in Ivy; preview contract here | Yes |
| Manifest schema v3 | Implemented; public subset may evolve | Yes |
| Default-deny grants | Implemented in Ivy; simulated here | Yes |
| Rust developer kit | Experimental | Yes |
| Dart developer types | Experimental | Yes |
| Core and Core UI | Implemented private development | No, proprietary |
| Official system modules | Implemented private development | Docs only |
| Simple end-user installer | Planned | No |
| Federation and offline mesh | Exploring | No |

See [STATUS.md](STATUS.md) for precise claims.

## Quick start

Requirements: a current stable Rust toolchain. Dart is optional.

```bash
cargo test --workspace
cargo run -p ivy-cli -- validate examples/hello-clock/module.yaml
cargo run -p ivy-hello-clock
cargo run -p ivy-widget-preview -- examples/hello-clock/widget.json
```

The example first receives an access denial, then an exact grant, successfully
invokes the provider, revokes the grant and receives an immediate denial.

Generate a module skeleton:

```bash
cargo run -p ivy-cli -- new /tmp/my-ivy-module \
  --id my-module \
  --namespace community.example.my_module
```

## Who this is for

- systems and security engineers interested in local-first software;
- Rust and Dart developers building Ivy tools or modules;
- makers exploring capability-based integrations;
- self-hosters who want to follow Ivy's path toward installable releases;
- researchers working on durable, replaceable or offline systems.

Ivy is not yet intended for non-technical production users.

## Building modules

Start with the [module tutorial](docs/tutorials/first-module.md), the
[`hello-clock`](examples/hello-clock/) example and the
[manifest reference](docs/development/manifest-reference.md). Public tooling can
validate development manifests, but official marketplace admission is a
separate version-specific ANWOR review.

## Contributing

External developers retain ownership of their original modules and choose their
own module licences, subject to applicable law and third-party rights. Changes
to this Developer Kit remain subject to the repository's pending licensing
decision. Read [CONTRIBUTING.md](CONTRIBUTING.md), [GOVERNANCE.md](GOVERNANCE.md)
and [LICENSES.md](LICENSES.md) before submitting code.

## ANWOR and proprietary components

Ivy is developed under ANWOR stewardship. Ivy Core and Core UI remain
proprietary. Official system modules and ANWOR community modules have separate
licensing policies that are still under legal review. No source licence for
those components is granted by this repository.

The proposed licence for independently usable Developer Kit components is
Apache-2.0, but it has **not been finally approved or applied**. See
[PUBLICATION_REVIEW.md](PUBLICATION_REVIEW.md).

## Security

Do not report vulnerabilities in public issues. Follow [SECURITY.md](SECURITY.md).
This preview contains no production credentials, official signing keys or
private Core implementation.
