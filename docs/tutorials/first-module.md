# Tutorial: your first module

Generate a skeleton:

```bash
cargo run -p ivy-cli -- new /tmp/status-module \
  --id status-module \
  --namespace community.example.status_module
```

Validate it:

```bash
cargo run -p ivy-cli -- validate /tmp/status-module/module.yaml
```

The generated provider owns `community.status_module` and declares one query.
Implementing a production provider also requires the official runtime protocol,
packaging and host integration, which are not public yet. Use `hello-clock` and
the development simulator to test declarations and grants in the meantime.
