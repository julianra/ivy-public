# Ivy public architecture

This document describes accepted, public architectural boundaries. It omits
private implementation details, operational configuration and security-sensitive
wire protocols.

## Platform responsibility

Ivy Core is the platform backbone, not a domain brain. It understands generic
concepts: installation, module identity, capability declarations, grants,
routing, lifecycle, health, audit and package trust. It must not encode agenda,
commerce, identity-provider or business-specific semantics.

Core and Core UI are proprietary and are not part of this repository.

## Canonical identifiers

- A **module ID** identifies an installed implementation.
- A **package namespace** identifies package provenance.
- A **capability domain** is an authority boundary, such as `example.clock`.
- A **capability ID** identifies an operation, such as `example.clock.now`.
- Event and widget identifiers are separate namespaces and do not grant
  capability access.

## Exclusive domain ownership

Only one installed owner may control a capability domain. Ancestor and
descendant domains may not overlap across owners. Replacement is explicit; a
second provider cannot silently take over a domain.

Ownership is not permission. A provider still needs an offer declaration, and a
consumer needs an exact grant.

## Default-deny invocation

A production invocation is accepted only when the platform can establish:

1. an authenticated caller;
2. a declared static need or approved dynamic request;
3. an exact consumer or external-principal grant;
4. the exclusive owner of the capability domain;
5. a compatible provider offer and schema;
6. a healthy provider and valid lifecycle state;
7. trusted context supplied by the platform rather than caller payload;
8. auditable outcome and bounded failure behavior.

The public simulator exercises only domain ownership, declaration, exact grant,
routing and revocation. It does not authenticate operating-system processes,
sandbox code, persist audit data or reproduce Core lifecycle guarantees.

## Events and widgets

Events describe facts and delivery semantics; they are not commands. Widgets
are declarative presentation data; they are not authorization. A widget action
ultimately maps to a capability invocation subject to normal policy.

## External boundary

Core has no direct external network role. Connectivity providers, currently
Bridge in the private implementation, terminate external transport and enforce
channel-specific security. Network access is not a generic right of every
module.

## Official system responsibilities

- **Prism** composes generic user experiences and renders module widgets.
- **Bridge** owns the current external connectivity boundary.
- **Vault** provides durable local storage capabilities.
- **Hooman** owns people and identity domains.
- **Pulse** owns time-management capabilities.
- **Flow** coordinates durable routines without becoming a second authority.
- **Agenda** owns calendar, availability and time-model semantics.
- **BookMe** composes booking behavior from other capabilities.

Official module source is not included pending separate licensing decisions.

## Package trust

Official Ivy packages use real content hashes, signatures and provenance.
Development signing in this repository is deliberately marked
`ivy-development-only`; it cannot create an official package or establish ANWOR
provenance.

## Host boundary

The existing Core host implementation targets Linux and uses host facilities
such as systemd, Unix IPC, process identity and cgroup/sandbox controls. Client
software is not inherently Linux-only. Public distribution targets are not yet
committed.
