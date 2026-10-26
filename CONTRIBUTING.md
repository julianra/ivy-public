# Contributing

Thank you for helping make Ivy's public developer surface clearer and more
reliable.

## Before opening a pull request

1. Discuss authority, trust or compatibility changes in an RFC issue first.
2. Keep the change narrowly scoped.
3. Add tests for positive and negative behavior.
4. Run `./scripts/check.sh`.
5. Confirm that no secrets, private Ivy source, internal paths or company data
   are included.
6. State the origin and licence of any third-party code or asset.

## Licensing

The Developer Kit licence remains pending. Do not contribute code until you are
comfortable with [LICENSES.md](LICENSES.md) and the final contribution terms
shown by GitHub at launch. No contribution creates an automatic right to
payment, credits or marketplace approval.

External module authors keep ownership and choose the licence for their own
modules. A module should live in its author's repository; this repository is
for the shared Developer Kit and selected examples.

## Commit certificate

The project intends to use the Developer Certificate of Origin once the public
licence is approved. Contributors will then add `Signed-off-by` to commits. This
requirement is not active until the final licence decision is recorded.

## Quality expectations

- no production secret or personal data;
- no network dependency in basic tests;
- deterministic tests where practical;
- bounded inputs and clear error handling;
- documentation distinguishes implemented, experimental and planned behavior;
- the development simulator is never described as Core.
