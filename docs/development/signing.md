# Development signing

`ivy dev-keygen` and `ivy dev-sign` create Ed25519 signatures with the explicit
purpose `ivy-development-only`. They help test deterministic file integrity and
tooling.

They do not:

- create an official Ivy module package;
- establish ANWOR provenance;
- interact with production key protection;
- support recovery, rotation or offline official signing;
- grant marketplace status.

Generated `*.devkey` files are ignored by Git. Never commit them.
