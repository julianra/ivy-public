# Conformance

The conformance checks currently live in the Rust workspace tests and cover:

- manifest schema version and semantic validation;
- capability identifiers and exclusive domain ownership;
- exact handlers for declared capabilities;
- default-deny invocation;
- exact grants and immediate revocation;
- development signature integrity;
- HTML escaping in the widget preview tool;
- the complete `hello-clock` flow.

Run all checks with `./scripts/check.sh`.

This is a Developer Kit conformance suite. Passing it does not certify a module
for the official Ivy Marketplace and does not replace ANWOR review.
