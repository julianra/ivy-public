#!/usr/bin/env bash
set -euo pipefail

cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p ivy-cli -- validate examples/hello-clock/module.yaml
cargo run -p ivy-hello-clock
cargo run -p ivy-widget-preview -- examples/hello-clock/widget.json --output /tmp/ivy-widget-preview.html

if command -v dart >/dev/null 2>&1; then
  (
    cd sdk/dart/ivy_developer_contracts
    dart pub get
    dart analyze
    dart run test/contracts_test.dart
  )
else
  echo "dart not found; skipping Dart checks" >&2
fi
