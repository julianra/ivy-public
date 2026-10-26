#!/usr/bin/env bash
set -euo pipefail

fail=0

check() {
  local description="$1"
  local pattern="$2"
  if git grep -I -n -E "$pattern" -- . \
    ':(exclude)scripts/repository-safety.sh' \
    ':(exclude)docs/development/AUDIT_REPORT.md'; then
    echo "repository-safety: found $description" >&2
    fail=1
  fi
}

check "private home paths" '/home/[A-Za-z0-9._-]+'
check "private key blocks" 'BEGIN (RSA |EC |OPENSSH )?PRIVATE KEY'
check "likely GitHub tokens" 'gh[pousr]_[A-Za-z0-9_]{20,}'
check "private source path references" '(^|/)core/src/|(^|/)core_ui/'
check "known internal developer identity" 'ivy-dev-julian'

if [[ "$fail" -ne 0 ]]; then
  exit 1
fi

echo "repository-safety: OK"
