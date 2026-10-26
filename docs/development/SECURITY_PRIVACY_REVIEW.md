# Security and privacy review

Review scope: complete private launch candidate before initial Git commit.

## Controls

- no source was bulk-copied from the private repository;
- a deny-pattern script rejects private home paths, private-key blocks, likely
  GitHub tokens, private Core source paths and the known internal developer ID;
- no production keys, certificates, package signatures or credentials exist;
- development keys are generated locally, ignored by Git and purpose-bound to
  `ivy-development-only`;
- the widget preview escapes untrusted fixture text and writes offline HTML;
- CI needs no private repository access or production secret;
- screenshots are withheld pending a human privacy review;
- public documentation distinguishes private implementation from included code.

## Residual risks

- regex secret checks cannot prove the absence of every secret;
- architectural descriptions still require owner review before publication;
- GitHub Actions are third-party dependencies and need pinning/risk review;
- development signing is educational tooling, not a hardened key-management
  product;
- the simulator is intentionally incomplete and must not be deployed as Core;
- final contact details for private vulnerability reporting are not configured.

## Result

No known secret, personal home path, internal hostname, private key or private
Core/module source is intentionally present. Public launch remains blocked on
the independent checks in `PUBLICATION_REVIEW.md`.
