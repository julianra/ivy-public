# GitHub configuration

Applied to the private launch candidate on 2026-10-26:

- repository visibility: `PRIVATE`;
- default branch: `main`;
- Issues, Projects and Discussions enabled;
- Wiki disabled;
- squash merge enabled; merge commits and rebase merge disabled;
- head-branch updates and automatic branch deletion after merge enabled;
- repository description and six technical topics configured;
- proposed community labels created;
- Dependabot vulnerability alerts and automated security fixes enabled;
- CODEOWNERS, CI, CodeQL, dependency review and secret-scan workflows committed.

## Account-plan limitations while private

GitHub rejected branch protection for this private personal repository with:
`Upgrade to GitHub Pro or make this repository public to enable this feature.`
The repository was kept private; no visibility workaround was attempted.

GitHub also reported that native secret scanning is not available for this
repository. CI therefore includes a repository safety gate and Gitleaks action,
but native push protection remains a launch/account-plan task.

Private vulnerability reporting returned `404` while the repository is private.
It must be enabled and verified when GitHub makes it available for the selected
visibility/plan. Security reports must not use public issues.

## Required later action

Before public launch, configure a ruleset or branch protection requiring pull
requests, one approving review, CODEOWNERS review, fresh review after the last
push, successful CI, linear history, resolved conversations, and no force-push
or deletion.
