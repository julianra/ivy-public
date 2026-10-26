#!/usr/bin/env bash
set -euo pipefail

python3 scripts/repository_safety.py
python3 scripts/check_dependencies.py
