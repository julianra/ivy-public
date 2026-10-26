<!-- doc-key: architecture-overview -->
<!-- parity-version: 2 -->
<!-- sections: rules,components,trust,compatibility,gaps,visuals -->

# Architecture and compatibility

[Nederlands](../../nl/architecture/overview.md) | **English**

## What is Ivy, and what will you learn here?

Ivy is a modular software platform: different modules provide their own features while working together through shared agreements. One module might manage appointments, another store data, and another run automations. You do not need one large program that does everything.

**Ivy Core is the central platform authority, not the component that performs every task.** Core tracks installed modules, the functions they offer, and which other modules have permission to use them. Domain-specific logic belongs in modules.

This page explains how the parts fit together, why Ivy keeps permissions separate, and what external developers **can test today** versus what is not yet available as a public production interface.

> **Status:** this repository contains an experimental Developer Kit, not the full public Ivy runtime. Documented architecture does not automatically imply an available public implementation.

### An example: smart lighting

Imagine a developer building a module that controls lights:

1. The module **describes** itself and its functions in a manifest.
2. During installation and registration, Ivy checks which module may own the lighting domain.
3. An automation module can declare that it needs the 'turn on light' function.
4. The call is allowed only after the separate requirements and permissions are satisfied.
5. Prism may display a button, and an event may report that the light is on, but a button or event **does not grant permission**.

This illustrates the architecture; it does not claim that this exact example is already available as a complete public production workflow.

<!-- section: rules -->
## The main public rules

- **Core governs the platform; modules own their domains.** Core handles registration, access and routing, not the domain logic of lights, calendars or bookings.
- **One installed owner per capability domain.** A capability is a named function offered by a module. Domains group those functions. If one module owns `home.lighting`, another cannot also own `home.lighting.kitchen`: ancestor and descendant domains cannot overlap.
- **Installation is not permission.** Installing a module, assigning domain ownership, allowing a function to be offered (*provider offer*), and allowing another module to use it (*consumer grant*) are four separate decisions.
- **An event is information, not an instruction or permission.** For example: 'the light was turned on'.
- **A widget is a user interface, not an authorization.** A Prism button cannot bypass access checks.
- **Bridge is the boundary with the outside world.** External network traffic terminates at Bridge instead of entering Core directly.

<!-- section: components -->
## What are Ivy's components?

| Component | In plain language | Public source code in this repository? |
|---|---|---|
| Core | Central registry, permissions, routing, lifecycle, audit and package trust | No; proprietary |
| Core UI | Local screens for administering the platform | No; proprietary |
| Prism | Generic user interface with declarative widgets | No; documented only |
| Bridge | Controlled boundary for external networks and communication channels | No; documented only |
| Vault | Durable local storage accessible through capabilities | No; contracts described |
| Flow | Routines and automations, including retries and approvals | No; private implementation |
| Agenda / Pulse / BookMe | Domain-specific functionality, such as appointments and bookings | No; status and use cases only |
| Developer Kit | Public contracts, simulator, tools and developer examples | Yes; experimental |

The Developer Kit is therefore a **development and verification environment**, not a public copy of Core.

<!-- section: trust -->
## How does Ivy decide whether a module may invoke a function?

Suppose module A wants to call a function provided by module B. In production, Core checks, among other things:

1. **Who is calling?** The caller must have an authenticated identity.
2. **Was the function declared as a dependency?** The consumer must have declared its need.
3. **Is the caller allowed to use this function?** A consumer grant is required.
4. **Does the provider exclusively own the relevant domain?**
5. **Is the provider permitted to offer this exact function?** A separate provider-offer grant applies.
6. **Are the schemas compatible, is the provider healthy, and does the trusted context satisfy the rules?**
7. **Is the relevant activity audited?**

A *grant* is an explicit permission. A *provider* offers a function; a *consumer* uses it. An *invocation* is a specific function call.

### What does the public simulator actually test?

The simulator intentionally covers only manifest validation, static declarations, domain ownership, availability, exact provider-offer grants, exact consumer grants, routing and revocation.

It uses module IDs as **test identities**, not authenticated production principals. Dynamic capability requests are represented in the schema but are not approved or simulated in this preview. Passing simulator tests is therefore not proof that the entire production security model has been tested.

<!-- section: compatibility -->
## What is public today, and what is not?

The table distinguishes written documentation, a limited testing environment, public contracts, and the private production implementation.

| Concern | Public documentation | Simulator | Public contract subset | Private Core | What is still missing publicly? |
|---|---|---|---|---|---|
| Domain ownership | Rules explained | Segment-overlap checks | `capability_domains.owns` | Full installation authority | Package-to-installation bridge |
| Provider offers | Rules explained | Exact grant/revoke | Provided capability declaration | Authenticated offer policy | Public admin/runtime API |
| Consumer access | Rules explained | Static declaration plus exact grant | Required capability declaration | Authenticated caller and grants | Production identity binding |
| Dynamic requirements | Architecture described | Not supported | Flag present | Approved runtime requests | Public approval protocol |
| Lifecycle/health | Behavior described | Available/unavailable/removal only | Not encoded | Durable lifecycle and health | Public runtime adapter |
| Audit/persistence | Behavior described | Not supported | Not encoded | Durable and bounded | Public test protocol |
| Signing | Trust model described | Development-purpose signatures | No official package claim | Protected official signing | Public package specification |
| Widgets/events | Concepts described | Test-fixture preview only | Developer protobuf only | Registration, routing and delivery | Stable public wire generation |

**How to read this:** 'documented' does not mean 'publicly executable'. 'Works in the simulator' does not mean 'available through a public production API'. And 'not public' does not necessarily mean 'not implemented internally'.

<!-- section: gaps -->
## What is still needed for real third-party production modules?

An external developer ultimately needs more than a valid manifest and passing simulator tests:

- A **stable runtime protocol** and generated **language bindings** to communicate with Ivy.
- A **package-layout specification** explaining how to package files.
- An official **signing and provenance** route for packages.
- A public **installation API**.
- A **sandbox and resource contract** specifying which system resources a module may access.
- A **compatibility policy** explaining support across updates.
- A **Marketplace review record** for the official review process.

According to the current documentation, these public pieces are not yet complete. This repository therefore does not pretend that a complete external production runtime is available.

### Where should a developer start?

Read the [manifest reference](../development/manifest-reference.md) for module descriptions, the [public contracts](../../../contracts/README.md) for technical formats, and the [Conformance tests](../../../conformance/README.md) for the rules checked automatically.

<!-- section: visuals -->
## Diagrams and screenshots

The diagrams in this documentation show reviewed architecture, not necessarily publicly available software. There are no real product screenshots: existing Prism and Core UI screens still need privacy review. Mock interfaces are not presented as working features.
