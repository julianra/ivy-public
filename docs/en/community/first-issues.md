<!-- doc-key: first-issues -->
<!-- parity-version: 2 -->
<!-- sections: rules,good-first,help-wanted,done -->

# Prepared contribution tasks

[Nederlands](../../nl/community/first-issues.md) | **English**

<!-- section: rules -->
## Scope rules

Each task must remain independent of proprietary Core, update bilingual docs when
needed and pass the full repository check. The GitHub issues link back here.

<!-- section: good-first -->
## Good first issues

1. Add boundary fixtures for digits and underscores in namespace segments.
2. Add an accessible second widget fixture and test its escaped HTML output.
3. Improve error messages for two invalid manifest fixtures without changing
   accepted input.
4. Add alternative-text validation for future SVG diagrams.

<!-- section: help-wanted -->
## Help wanted

1. Design a Go or TypeScript contract-types prototype generated from the public
   developer proto, without runtime claims.
2. Add deterministic failure injection to the simulator for provider errors.
3. Propose a public event-test harness through an RFC.
4. Audit keyboard and screen-reader requirements for the future widget preview.

<!-- section: done -->
## Definition of done

A task needs tests, matching English/Dutch documentation, no new private
dependency, documented licence origin and a clean `./scripts/check.sh` result.
