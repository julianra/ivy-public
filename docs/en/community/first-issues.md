<!-- doc-key: first-issues -->
<!-- parity-version: 2 -->
<!-- sections: rules,good-first,help-wanted,done -->

# Help improve Ivy Public

[Nederlands](../../nl/community/first-issues.md) | **English**

Want to contribute but not sure where to start? This page lists practical tasks for the **public Ivy Developer Kit**. You do not need to know the private Ivy Core. Starting small is welcome: even a clearer error message helps.

For each task, we explain **why it matters**, **what to build** and **what done looks like**. Want to discuss an idea first? Leave a comment on the related GitHub issue.

<!-- section: rules -->
## A few things to know first

- Work only with public Developer Kit code, example data and public contracts. **Never** add private Core source, secrets, personal data or internal operational information.
- Keep changes focused. Discuss your approach in the issue before investing substantial effort.
- Update Dutch and English documentation together whenever you change human-facing docs.
- GitHub issues describe and track individual tasks; this page is the beginner-friendly overview.

<!-- section: good-first -->
## 🌱 Good first issue

### Make manifest errors easier to understand

**Why?** A developer who makes a mistake in a module description should quickly understand *what* went wrong and *how* to fix it.

**What we're asking for:** Improve the error messages for **two existing invalid manifest examples**. Clearly explain which rule is violated, without changing which manifests Ivy accepts or rejects.

**Done when:** The two messages are clearer, relevant tests demonstrate their behavior, and previously valid manifests remain valid.

<!-- section: help-wanted -->
## 🛠️ Help wanted — for a deeper technical challenge

### Prototype contract types for Go or TypeScript

**Why?** More developers should be able to work with Ivy's public contracts in their preferred programming language.

**What we're asking for:** Build a limited prototype that derives types from the public developer Protocol Buffer declarations. **Do not build a full Ivy runtime** or claim the language has production-ready support.

**Done when:** The prototype is reproducible, has a usage example and clearly explains its limits.

### Make the simulator fail on purpose, predictably

**Why?** Robust software must handle a provider returning an error.

**What we're asking for:** Add a way to simulate selected provider errors consistently, so developers can reproduce and test failure handling.

**Done when:** Tests reliably repeat the same failure scenarios without changing normal simulator behavior.

### Help design an event-testing tool

**Why?** Modules exchange events, such as 'a device changed state'. Developers need a way to check whether those events are handled correctly.

**What we're asking for:** Write an **RFC (design proposal)** for a public test harness that feeds events into a test and checks expected results. This is **research and design**, not a request to build the entire tool now.

**Done when:** The proposal includes examples, boundaries, failure scenarios, alternatives and open questions for review.

### Review widget accessibility

**Why?** A future widget preview should also be usable with a keyboard or screen reader.

**What we're asking for:** Review accessibility requirements and the current preview, identify concrete problems and propose practical improvements.

**Done when:** There is a verifiable report with findings, examples and recommended next steps. Implementing every improvement is not required in this task.

<!-- section: done -->
## When is a contribution done?

- The agreed result is delivered and explained clearly.
- **For code changes:** relevant positive and negative tests are added or updated, and `./scripts/check.sh` passes.
- **For research or RFC tasks:** there is a useful, reviewable report or proposal; new code or tests are required only when the task calls for them.
- Changed human-facing documentation is updated in both English and Dutch.
- Third-party sources and licenses are documented.
- No new dependencies on the private Ivy Core are introduced.

See the [contribution guide](contributing.md) for opening an issue or pull request.
