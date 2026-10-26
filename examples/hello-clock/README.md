# Hello Clock

This standalone example demonstrates four Ivy invariants without using the
proprietary Core:

1. a module owns the `example.clock` capability domain;
2. it provides `example.clock.now`;
3. a consumer is denied until it receives an exact grant;
4. revoking that grant takes effect immediately.

Run it from the repository root:

```bash
cargo run -p ivy-hello-clock
```

The `DevelopmentSimulator` is a test harness, not Ivy Core and not a production
security boundary.
