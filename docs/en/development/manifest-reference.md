<!-- doc-key: manifest-reference -->
<!-- parity-version: 2 -->
<!-- sections: scope,syntax,semantics,fixtures,generator -->

# Canonical public manifest subset

[Nederlands](../../nl/development/manifest-reference.md) | **English**

<!-- section: scope -->
## Scope

The public schema is a strict developer-facing subset of private manifest schema
version 3. It deliberately omits production runtime, IPC, sandbox, platform,
resource, distribution and permission fields.

<!-- section: syntax -->
## Structural rules

- module ID: lowercase ASCII words separated by `-`;
- namespace/domain segments: start with a lowercase letter and contain lowercase
  letters, digits or `_`, separated only by `.`;
- namespace/domain: at least two segments;
- capability ID: at least three segments;
- version: SemVer 2.0.0;
- schema reference: `schema:<two-or-more-segment-name>:v<positive-integer>`;
- commands require `retry_semantics`; queries forbid it;
- all fields are explicit and unknown fields fail validation.

<!-- section: semantics -->
## Rust-only semantic rules

JSON Schema cannot express every invariant. Rust additionally rejects duplicate
capability IDs, overlapping domains within one manifest and provided
capabilities outside an owned domain. The simulator checks conflicts with
already registered modules and exact handler coverage before atomically applying
registration.

<!-- section: fixtures -->
## Shared fixtures

Every fixture is evaluated through both paths. `schema-invalid` must fail both;
`semantic-invalid` must pass structure but fail Rust semantics; `valid` must pass
both. This prevents the schema, parser, examples and documentation from silently
drifting apart.

<!-- section: generator -->
## Generator

The CLI creates only manifests accepted by the canonical validator. Generated
skeletons are development starting points, not installable Ivy packages.
