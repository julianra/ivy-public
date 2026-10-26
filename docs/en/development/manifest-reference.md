<!-- doc-key: manifest-reference -->
<!-- parity-version: 2 -->
<!-- sections: scope,syntax,semantics,fixtures,generator -->

# Module manifests: how to describe your Ivy module

[Nederlands](../../nl/development/manifest-reference.md) | **English**

A **manifest** is a module's technical identity card. It describes its name, version, functions (*capabilities*) and the domains it intends to own. Ivy uses agreed rules to validate that description.

Imagine building a smart-lighting module. You might declare ownership of `home.lighting` and provide a function such as `home.lighting.turn_on`. Validators check whether the description has the correct structure and follows additional semantic rules.

**Important:** a valid manifest does not itself authorize installation or access to capabilities.

<!-- section: scope -->
## What does this public schema describe?

The [public JSON Schema](../../../contracts/schemas/module-manifest-v3.schema.json) is a **strict developer-facing subset** of the private manifest format v3. It lets you build and validate public module descriptions without access to private Core.

Production fields for runtime, IPC (inter-process communication), sandbox, platform, resources, distribution and permissions are intentionally excluded. The Developer Kit is not a complete production installer.

<!-- section: syntax -->
## What formatting rules apply?

The schema validator checks **structure**: are the expected fields present, and do names and values use the required form?

| Element | Rule | Example |
|---|---|---|
| Module ID | Lowercase ASCII letters/digits, optionally separated with `-` | `smart-lighting` |
| Namespace or domain | At least two dot-separated segments; each starts with a lowercase letter and may continue with letters, digits or `_` | `home.lighting` |
| Capability ID | At least three dot-separated segments | `home.lighting.turn_on` |
| Module version | Semantic Versioning (SemVer 2.0.0) | `1.2.0` |
| Schema reference | `schema:<two-or-more-segment-name>:v<positive-integer>` | `schema:home.lighting:v1` |
| Command | Must declare `retry_semantics` (what may happen when retried) | `not_retryable` |
| Query | Must not contain `retry_semantics` | Reading a status |

All permitted fields are explicitly defined. **Unknown fields are rejected**, making mistakes easier to catch.

A command requests an action; a query requests information. The declaration alone does not prove how the implementation behaves.

<!-- section: semantics -->
## Why does Rust check additional rules?

A manifest may be correctly formatted yet contain contradictory declarations. The **Rust validator** adds *semantic* checks.

It rejects, among other things:
- multiple declarations sharing the same capability ID;
- overlapping owned domains within one manifest;
- provided capabilities outside an owned domain.

The simulator also checks for conflicts with **already registered modules** and ensures that every offered function has its exact handler. Registration is then **atomic**: failure cannot leave a partially registered module.

<!-- section: fixtures -->
## How do we test the rules?

The [conformance/manifests](../../../conformance/manifests) directory contains example files known as *fixtures*.

| Test category | Expected result |
|---|---|
| `valid` | Both JSON Schema and Rust accept the manifest |
| `schema-invalid` | Both validators reject the invalid structure |
| `semantic-invalid` | JSON Schema accepts the structure, but Rust rejects its meaning |

Both validators run against the same examples. This makes it easier to detect differences from the intended contract.

<!-- section: generator -->
## Do I have to write a manifest from scratch?

No. The **Ivy CLI** can generate a starting structure and validate it:

```bash
cargo run -p ivy-cli -- new /tmp/status-module --id status-module --namespace community.example.status_module
cargo run -p ivy-cli -- validate /tmp/status-module/module.yaml
```

The generator creates only manifests accepted by the canonical validator. Generated files are **development starting points**, not installable Ivy production packages.

See also the [Developer Kit guide](developer-kit.md) and [Capabilities](../concepts/capabilities.md).
