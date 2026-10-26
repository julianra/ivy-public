# Marketplace verification model

Marketplace trust must bind approval to immutable identity:

- module namespace and ID;
- exact version;
- package digest;
- signer or provenance record;
- review policy version;
- review result and timestamp;
- supported Ivy compatibility range.

`ANWOR Official` identifies controlled official releases. `ANWOR Verified`
identifies a reviewed third-party package. Neither label may be inherited by a
modified build or unreviewed update.

The Marketplace implementation and commercial terms are not part of this
Developer Kit preview.
