# Conformance / Conformiteit

## English

The shared manifest fixtures are split into `valid`, `schema-invalid` and
`semantic-invalid`. Rust tests run the JSON Schema and Rust validator over the
same files. Semantic-only cases intentionally pass structural schema validation
and fail Rust validation. The suite also covers segment-based domain ownership,
atomic registration, exact handlers, separate grants, declaration enforcement,
revocation, availability, signing and widget escaping.

Passing is not Marketplace certification. Run `./scripts/check.sh`.

## Nederlands

De gedeelde manifestfixtures zijn verdeeld in `valid`, `schema-invalid` en
`semantic-invalid`. Rust-tests voeren JSON Schema en de Rust-validator over
dezelfde bestanden uit. Semantische gevallen slagen bewust voor structurele
schemavalidatie en falen in Rust. De suite controleert ook segmentgebaseerd
domeineigendom, atomische registratie, exacte handlers, afzonderlijke grants,
declaratiehandhaving, intrekking, beschikbaarheid, signing en widgetescaping.

Slagen is geen Marketplace-certificering. Voer `./scripts/check.sh` uit.
