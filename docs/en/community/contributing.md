<!-- doc-key: contributing -->
<!-- parity-version: 3 -->
<!-- sections: first,issues,prs,rfc,modules,marketplace,licensing,rewards,bilingual -->

# Contributing to Ivy

[Nederlands](../../nl/community/contributing.md) | **English**

## Welcome!

Thanks for helping improve Ivy. You do not need to know the entire project: a clear bug report, a better explanation or a small test can already make a difference.

This repository contains the **public, experimental Ivy Developer Kit**: tools, contracts and examples for module developers. It does not contain the full private Ivy Core.

You can contribute by reporting problems, asking questions, improving documentation, adding tests or proposing code. This guide explains how to get started and the rules that protect everyone involved.

**Three GitHub terms worth knowing:**
- **Issue:** a bug report or a clearly defined task.
- **Branch:** a separate line of development that leaves existing code unchanged while you work.
- **Pull request (PR):** a proposal to merge your changes after review.

<!-- section: first -->
## Your first contribution

Start small whenever possible. That makes it easier to discuss the approach and impact before investing significant effort.

1. Find a well-scoped issue and leave a comment saying you would like to work on it **before starting substantial implementation work**.
2. Create a separate branch for that change. You can work from a fork of your own.
3. Run the checks from the repository root:

   ```bash
   ./scripts/check.sh
   ```

4. Open a pull request and describe both what should work (**positive tests**) and what should be rejected or handled safely (**negative tests**).

**Never include private Ivy source code, credentials such as passwords or tokens, personal data, or internal operational information.** This also applies to examples, screenshots, logs and PR attachments.

<!-- section: issues -->
## Reporting a bug or asking a question

If you find a Developer Kit bug, open an issue with enough information for someone else to reproduce it:
- the version or commit used (*revision*);
- the smallest steps or commands that reproduce the bug;
- what you expected and what actually happened;
- the relevant environment, such as operating system and tools.

For ordinary developer questions, use **GitHub Discussions → Q&A** rather than a bug issue.

**Found a security vulnerability?** Do not publish details in a public issue or Discussion. Use **GitHub Private Vulnerability Reporting once the launch checklist has confirmed that it is active**. Until then, this guide does not identify a verified private reporting channel.

<!-- section: prs -->
## Opening a pull request

Keep each PR focused on **one concern**. This helps reviewers understand and check the change.

- Add appropriate tests and explain any impact on public technical agreements (*contracts*).
- Credit third-party sources and identify their licenses when reusing someone else's work.
- Update both English and Dutch versions when changing public documentation.
- Arrange independent review for architecture- or security-sensitive changes: authors do not approve their own sensitive changes.

Every commit must include a **Developer Certificate of Origin (DCO) 1.1** sign-off:

```bash
git commit -s -m "Describe the change"
```

This adds a `Signed-off-by` line confirming that you have the right to submit the contribution under the project's terms. It is **not a transfer of copyright**. See [DCO](../../../DCO).

<!-- section: rfc -->
## Discussing larger ideas: RFCs

An **RFC (Request for Comments)** is a design proposal others can discuss before major changes are implemented.

An RFC is required for changes to capabilities (functions), authority, trust, compatibility, packages or the Marketplace.

Explain:
- the problem you want to solve;
- which module or domain is responsible (*ownership*);
- the proposed new or changed technical contract;
- failure and retry behavior;
- how existing users migrate and how the change can be rolled back;
- alternatives you considered and rejected.

A small wording correction does not normally need an RFC.

<!-- section: modules -->
## Building your own Ivy module

A **module** adds a focused feature to Ivy. You can develop an independent module without transferring its source code to ANWOR.

Your proposal should describe:
- the domain it is responsible for, and explicitly what it **does not** own;
- the functions it offers (*provided capabilities*);
- the functions it needs from others (*required capabilities*);
- examples testing valid and invalid behavior (*conformance fixtures*).

Use the **module proposal template**. Keep independent module code in **your own repository**, unless ANWOR expressly accepts it as shared reference code in the Developer Kit.

<!-- section: marketplace -->
## Offering a module through the Ivy Marketplace

A working module is **not automatically listed** in the official Marketplace. Admission requires a separate review of its exact identity, version and content fingerprint (*digest*).

The `ANWOR Official` and `ANWOR Verified` labels are review distinctions, **not transfers of ownership**. They do not automatically extend to modified modules or later releases.

<!-- section: licensing -->
## Who owns the code?

**Your independent module remains yours.** As an external author, you may choose free, commercial, proprietary or open-source distribution, subject to your rights and applicable third-party licenses.

**Contributions accepted into this shared Developer Kit are licensed under Apache-2.0.** We use DCO 1.1; there is no separate Contributor License Agreement (**CLA**).

The code license grants **no rights** to Ivy or ANWOR names, logos, trademarks or official badges. See [LICENSE](../../../LICENSE) and [DCO](../../../DCO).

<!-- section: rewards -->
## Recognition and rewards

ANWOR may separately agree on recognition, credits, payment or another reward for a contribution. Compensation is not automatic: submitting or accepting a contribution does not itself create a right to payment.

<!-- section: bilingual -->
## Documentation in two languages

We want English-speaking and Dutch-speaking developers to receive the same information.

When editing human-facing public documentation, update **both** the `en` and matching `nl` file **in the same PR**. Keep internal parity metadata and section IDs aligned, and preserve working code examples.

Automated CI checks reject missing counterparts, mismatched sections, differences in commands (*command drift*) and broken internal links.
