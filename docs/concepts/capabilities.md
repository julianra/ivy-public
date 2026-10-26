# Capabilities, domains and grants

A capability is a versioned operation such as `example.clock.now`. Its ID is
independent from the module that currently provides it.

A capability domain, such as `example.clock`, is an exclusive authority boundary.
One installed owner may provide capabilities below that domain. Overlapping
owners are rejected rather than resolved by priority.

Providers declare what they offer. Consumers declare what they need. Core grants
exact access; there is no implicit permission merely because two modules are
installed.

Queries should be side-effect free. Commands must state retry behavior, because a
caller needs to know whether replay returns the prior result, may repeat an
external effect or is not safe to retry.
