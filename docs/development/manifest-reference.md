# Public manifest reference

The public schema is a strict subset of private Ivy manifest schema version 3.
It contains only the fields needed for standalone module development:

- `schema_version`: currently `3`;
- `id`: lowercase module identifier;
- `namespace`: package provenance namespace;
- `version`: semantic version;
- `language`: implementation language;
- `capability_domains.owns`: exclusive domains;
- `capabilities.provides`: offered capability contracts;
- `capabilities.requires`: static dependencies;
- `capabilities.dynamic_requires`: whether runtime requests may be needed.

Commands require explicit `retry_semantics`. Provided capabilities must live
below one of the module's owned domains.

The private installation manifest also includes runtime, IPC, sandbox, platform,
distribution, resource and permission fields. Those are intentionally excluded
until their public stability and licensing are approved.
