# Project status

Last audited against the private Ivy source snapshot on 2026-10-26.

Ivy is a substantial pre-1.0 platform under active development. The private
source contains Core, Core UI, Prism, system modules, SDKs, tooling and tests.
This repository intentionally exposes only an independently testable Developer
Kit candidate.

## Implemented in the private Ivy platform

- exclusive capability-domain ownership and explicit provider declarations;
- default-deny consumer grants and capability-based routing;
- generic lifecycle, health, audit and package verification foundations;
- local Linux host implementation with Unix IPC and sandbox boundaries;
- native Prism client flow and declarative widget rendering;
- Bridge external-boundary foundations;
- Vault storage, Hooman identity, Pulse time, Flow automation, Agenda and
  BookMe module implementations;
- signed local module packages and release manifests;
- restart, retry, idempotency, boundedness and negative-path tests.

These statements describe private development source, not a supported public
release.

## Experimental in this repository

- public manifest-v3 subset;
- standalone Rust manifest validator;
- development-only capability simulator;
- Rust module generator;
- development-only signing helper;
- Dart developer contract types;
- offline widget fixture preview;
- `hello-clock` example and conformance tests.

Compatibility may change before 1.0.

## Planned

- legally approved licences for public and official module categories;
- stable public SDK compatibility policy;
- reproducible contract generation for Rust and Dart;
- a richer mock broker and event test harness;
- public package layout and package conformance;
- a reviewed reference module;
- module catalogue and version-specific verification metadata;
- end-user installation, update, rollback, backup and restore flows.

## Exploring

- federation between independent Ivy installations;
- offline store-and-forward and mesh transport;
- navigation, mobility and offline maps;
- provider-neutral optimisation;
- local and remote AI providers;
- manufacturing and public-data modules.

## Explicitly not claimed

- production readiness;
- a stable 1.0 API;
- a complete open-source Ivy runtime;
- marketplace approval for code in this repository;
- end-user support or production security certification;
- compatibility between this simulator and every private Core behavior.
