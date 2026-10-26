<!-- doc-key: first-module -->
<!-- parity-version: 2 -->
<!-- sections: generate,inspect,validate,simulate,next -->

# Tutorial: first development module

[Nederlands](../../nl/tutorials/first-module.md) | **English**

## What will you build?

In this exercise you create an **example module**, inspect its manifest (technical description) and test declarations and permissions with public tools. You need Rust and Cargo, but not private Core. This is **not a production module installation**. Run commands from the repository root; `/tmp` is a Unix example path.

<!-- section: generate -->
## Generate

```bash
cargo run -p ivy-cli -- new /tmp/status-module --id status-module --namespace community.example.status_module
```

<!-- section: inspect -->
## Inspect

The generated manifest owns `community.status_module` and provides one query.
Ownership is exclusive; installation would not itself grant any consumer access.

<!-- section: validate -->
## Validate

```bash
cargo run -p ivy-cli -- validate /tmp/status-module/module.yaml
```

<!-- section: simulate -->
## Simulate

Study `examples/hello-clock`: it registers provider and consumer manifests,
requires a provider offer grant and a consumer invocation grant, then proves
immediate revocation. Run it with:

```bash
cargo run -p ivy-hello-clock
```

<!-- section: next -->
## Next steps

Add negative fixtures and tests before proposing a new public contract. A
generated skeleton is not an installable production module; runtime bindings,
packaging, official signing and installation remain future integration work.
