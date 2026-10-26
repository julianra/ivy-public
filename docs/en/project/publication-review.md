<!-- doc-key: publication-review -->
<!-- parity-version: 3 -->
<!-- sections: legal,product,community,security,github,launch -->

# Publication review

[Nederlands](../../nl/project/publication-review.md) | **English**

## Why this checklist?

This list helps prevent **accidental publication of private code, personal data or insufficiently reviewed content**. A checked item records that specific step; it never authorizes making the repository public. **Publication always requires separate explicit owner approval.**

The repository must remain private until every blocker is resolved and the owner
gives separate explicit publication permission.

<!-- section: legal -->
## Legal decisions

- [x] Apply Apache-2.0 to the Ivy Developer Kit in this repository.
- [x] Use DCO 1.1 for contributions; no CLA.
- [x] Apply the repository trademark/badge policy and NOTICE boundary.
- [ ] Confirm any ownership-specific copyright notice.
- [ ] Approve System Module and Community Module licences.

<!-- section: product -->
## Product decisions

- [ ] Approve public organisation/repository name.
- [ ] Approve compatibility promise and manifest-subset stability.
- [ ] Select the first public reference module.
- [ ] Decide whether public components become canonical here.
- [ ] Approve `ANWOR Official` versus `ANWOR Verified` policy.

<!-- section: community -->
## Community decisions

- [ ] Select final standard Code of Conduct and private contacts.
- [ ] Name maintainers and CODEOWNERS.
- [ ] Approve Discussion categories, response expectations and reward process.

<!-- section: security -->
## Security and privacy

- [ ] Independent scan and human review of exact launch commit and history.
- [ ] Human review of every screenshot/media asset.
- [ ] `MANUAL GITHUB SETTING REQUIRED`: enable and verify GitHub Private Vulnerability Reporting.
- [ ] Re-run clean-clone builds without private repository access.

<!-- section: github -->
## GitHub controls

- [x] `VERIFIED ACTIVE`: CI, Gitleaks, repository-safety and Dependabot security controls.
- [x] `READY WHEN PUBLIC`: CodeQL and Dependency Review workflows.
- [ ] `MANUAL GITHUB SETTING REQUIRED`: branch rules with review, CODEOWNERS, CI and no force-push/delete.
- [ ] `MANUAL GITHUB SETTING REQUIRED`: native secret scanning and push protection.
- [ ] Confirm no open automated PR or extra branch.

<!-- section: launch -->
## Launch authority

- [ ] Repository owner gives a separate explicit instruction to change visibility
      from PRIVATE to PUBLIC.

No release, announcement, package publication or visibility change is authorised
before the final item.
