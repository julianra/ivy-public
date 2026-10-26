#!/usr/bin/env bash
set -euo pipefail

cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --locked
cargo run -p ivy-cli -- validate examples/hello-clock/module.yaml
ivy_generator_tmp="$(mktemp -d)"
cargo run -p ivy-cli -- new "$ivy_generator_tmp/generated" --id generated-clock --namespace community.generated.clock
cargo run -p ivy-cli -- validate "$ivy_generator_tmp/generated/module.yaml"
cargo run -p ivy-hello-clock
cargo run -p ivy-widget-preview -- examples/hello-clock/widget.json --output /tmp/ivy-widget-preview.html
python3 scripts/check_docs.py
./scripts/repository-safety.sh

if command -v dart >/dev/null 2>&1; then
  (
    cd sdk/dart/ivy_developer_contracts
    dart pub get
    dart format --output=none --set-exit-if-changed .
    dart analyze
    dart run test/contracts_test.dart
  )
else
  echo "dart not found; local Dart checks skipped (CI remains mandatory)" >&2
fi
