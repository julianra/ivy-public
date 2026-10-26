<!-- doc-key: contributing -->
<!-- parity-version: 2 -->
<!-- sections: first,issues,prs,rfc,modules,marketplace,licensing,rewards,bilingual -->

# Contributing

[Nederlands](../../nl/community/contributing.md) | **English**

<!-- section: first -->
## First contribution

Choose a bounded issue, comment before substantial work, create a focused branch,
run `./scripts/check.sh`, and explain positive and negative behavior in the pull
request. Never include private Ivy source, credentials, personal data or
operational information.

<!-- section: issues -->
## Reporting issues

Use a reproducible Developer Kit bug report for code defects and Q&A Discussions
for questions. Include revision, minimal commands, expected/actual behavior and
environment. Report vulnerabilities privately once the security contact is
configured; never disclose them in a public issue.

<!-- section: prs -->
## Pull requests

Keep one concern per PR. Add tests, document contract effects, identify
third-party origins and licences, and update both English and Dutch public docs.
The same author does not self-approve an architectural or security-sensitive
change.

<!-- section: rfc -->
## RFCs

Capability, authority, trust, compatibility, package or Marketplace changes need
an RFC. Describe the problem, ownership boundary, proposed contract, failure and
retry behavior, migration, rollback and rejected alternatives.

<!-- section: modules -->
## Developing and proposing modules

Define one bounded domain, list provided and required capabilities, state what
the module does not own, add conformance fixtures and use the module-proposal
template. Keep an independent module in the author's repository unless ANWOR
explicitly accepts it as shared reference code.

<!-- section: marketplace -->
## Marketplace verification

Marketplace admission is a separate review of the exact module identity, version
and digest. `ANWOR Official` and `ANWOR Verified` do not transfer ownership and
do not apply automatically to modifications or later releases.

<!-- section: licensing -->
## Licensing and ownership

External authors retain rights over original independent modules and choose free,
commercial, proprietary or open-source distribution within their rights. The
shared Developer Kit licence is still pending; do not assume Apache-2.0 has
already been granted.

<!-- section: rewards -->
## Rewards

ANWOR may separately agree recognition, credits, payment or another reward.
There is no automatic entitlement and no contribution promises compensation.

<!-- section: bilingual -->
## Bilingual documentation rule

Every human-facing public documentation change must update the matching `en` and
`nl` file in the same PR, keep parity metadata and section identifiers aligned,
and preserve executable code blocks. CI rejects missing counterparts, mismatched
sections, command drift and broken internal links.
