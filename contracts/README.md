# Public developer contracts

This directory contains the developer-facing, independently usable subset of
Ivy's contract model.

- `schemas/module-manifest-v3.schema.json` describes the public manifest subset.
- `proto/ivy/developer/v1/developer.proto` documents capability declarations and
  development invocations.

These files intentionally exclude the proprietary Core administration wire
protocol, mobile authentication, secrets, internal lifecycle messages and
security-sensitive Bridge implementation details.

The Rust validator is authoritative for this preview and performs semantic
checks that JSON Schema cannot express, including capability-domain ownership
and command retry declarations.
