<!-- doc-key: architecture-overview -->
<!-- parity-version: 2 -->
<!-- sections: rules,components,trust,compatibility,gaps,visuals -->

# Architecture and compatibility

[Nederlands](../../nl/architecture/overview.md) | **English**

<!-- section: rules -->
## Stable public rules

Core is a platform authority, not a domain brain. Capability domains have one
exclusive installed owner; overlapping ancestor and descendant domains are
forbidden. Installation, ownership, provider offer and consumer permission are
separate decisions. Events are facts, widgets are presentation, and neither is
a hidden capability grant. External traffic terminates at Bridge rather than in
Core.

<!-- section: components -->
## Components

| Component | Responsibility | Public source now |
|---|---|---|
| Core | Registry, grants, routing, lifecycle, audit, package trust | No; proprietary |
| Core UI | Local platform administration | No; proprietary |
| Prism | Generic UI and declarative widgets | No; documented only |
| Bridge | External network and channel boundary | No; documented only |
| Vault | Durable local storage capability family | No; contracts described |
| Flow | Durable routines, retries and approvals | No; private implementation |
| Agenda/Pulse/BookMe | Domain capabilities | No; status and use cases only |
| Developer Kit | Public contracts, simulator, tools and examples | Yes; experimental |

<!-- section: trust -->
## Trust and invocation

Production Core checks authenticated caller identity, declared need, consumer
grant, exclusive provider ownership, provider offer grant, schema compatibility,
health, trusted context and audit. The public simulator intentionally covers
only manifest validation, static declarations, ownership, availability, exact
offer grants, exact consumer grants, routing and revocation.

Simulator module IDs are test identities. They are not a substitute for
authenticated production principals. Dynamic capability requests are declared
in the schema but are not approved or simulated in this preview.

<!-- section: compatibility -->
## Compatibility matrix

| Concern | Public documentation | Simulator | Public contract subset | Private Core | Missing integration |
|---|---|---|---|---|---|
| Domain ownership | Normative summary | Segment-overlap enforced | `capability_domains.owns` | Full install authority | Package-to-install bridge |
| Provider offers | Explained | Exact grant/revoke | Provided capability declaration | Authenticated offer policy | Public admin/runtime API |
| Consumer access | Explained | Static declaration plus exact grant | Required capability declaration | Authenticated caller and grants | Production identity binding |
| Dynamic requirements | Architecture only | Not supported | Flag retained | Approved runtime requests | Public approval protocol |
| Lifecycle/health | Explained | Available/unavailable/removal only | Not encoded | Durable lifecycle and health | Public runtime adapter |
| Audit/persistence | Explained | Not supported | Not encoded | Durable and bounded | Public test protocol |
| Signing | Trust model | Development-purpose signatures | No official package claim | Protected official signing | Public package specification |
| Widgets/events | Concepts | Fixture preview only | Developer proto only | Registration/routing/delivery | Stable public wire generation |

<!-- section: gaps -->
## What an external production module still needs

Before a third party can build, package, sign and install a real production Ivy
module, Ivy still needs a stable public runtime protocol, generated bindings,
package-layout specification, official signing/provenance route, installation
API, sandbox/resource contract, compatibility policy and Marketplace review
record. This repository does not fake those missing steps.

<!-- section: visuals -->
## Visual status

The diagrams in this documentation describe reviewed architecture. No product
screenshots are included: existing Prism and Core UI screens still require a
privacy review. No mock interface is presented as implemented functionality.
