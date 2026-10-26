<!-- doc-key: marketplace-verification -->
<!-- parity-version: 2 -->
<!-- sections: purpose,record,process,labels,limits -->

# Marketplace verification

[Nederlands](../../nl/marketplace/verification.md) | **English**

## What does Marketplace verification mean?

A label indicates that **a specific version of a module** was reviewed. It does not mean ANWOR owns the module or automatically approves future versions. This page describes **planned policy**, not an active public certification service.

<!-- section: purpose -->
## Purpose

Marketplace review communicates provenance and review status without taking
ownership away from an external developer.

<!-- section: record -->
## Version-specific record

Approval must bind module namespace and ID, exact version, package digest,
signer/provenance, review-policy version, review result, timestamp and supported
Ivy range.

<!-- section: process -->
## Proposed process

1. Author submits immutable package identity and rights information.
2. ANWOR reviews declared authority, dependencies, security and compatibility.
3. The exact result is recorded and, when approved, signed.
4. Any changed package or new version requires a new review.

<!-- section: labels -->
## Labels

`ANWOR Official` is reserved for controlled official releases. `ANWOR Verified`
identifies an exact reviewed third-party package. Neither label is inherited by
forks or future versions.

<!-- section: limits -->
## Current limits

The Marketplace implementation, review policy, legal terms and signed record
format are planned. This repository grants no certification.
