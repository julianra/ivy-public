import 'package:ivy_developer_contracts/ivy_developer_contracts.dart';

void main() {
  const valid = CapabilityDeclaration(
    id: 'example.clock.now',
    version: 1,
    kind: CapabilityKind.query,
    schema: 'schema:example.clock.now:v1',
    externallyInvocable: false,
  );
  assert(valid.validate().isEmpty);

  const invalidCommand = CapabilityDeclaration(
    id: 'example.clock.set',
    version: 1,
    kind: CapabilityKind.command,
    schema: 'schema:example.clock.set:v1',
    externallyInvocable: false,
  );
  assert(invalidCommand.validate().isNotEmpty);
  print('Dart developer contracts: OK');
}
