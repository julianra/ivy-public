# Trust model

Ivy distinguishes identity, declarations, ownership and permission.

- Package identity says who produced an artefact; it does not grant runtime
  authority.
- Installation makes code available; it does not grant capability access.
- Domain ownership chooses the one installed provider for a namespace; it does
  not authorise consumers.
- An exact grant permits a caller to invoke one declared capability.
- Trusted invocation context comes from the platform, not from untrusted payload
  fields.
- External transport terminates at a connectivity boundary, not in Core.

The public simulator models only a small subset and offers no operating-system
isolation, authenticated process identity, persistence or audit guarantee.
