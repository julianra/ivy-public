/// Public, standalone development types for Ivy module authors.
library;

enum CapabilityKind { query, command }

enum RetrySemantics {
  resultReplay,
  externalEffectMayRepeat,
  notRetryable,
}

final class CapabilityDeclaration {
  const CapabilityDeclaration({
    required this.id,
    required this.version,
    required this.kind,
    required this.schema,
    required this.externallyInvocable,
    this.retrySemantics,
  });

  final String id;
  final int version;
  final CapabilityKind kind;
  final String schema;
  final RetrySemantics? retrySemantics;
  final bool externallyInvocable;

  List<String> validate() {
    final problems = <String>[];
    final identifier = RegExp(r'^[a-z0-9]+(?:[._][a-z0-9]+){2,}$');
    if (!identifier.hasMatch(id)) problems.add('invalid capability id: $id');
    if (version < 1) problems.add('version must be positive');
    if (!schema.startsWith('schema:')) problems.add('invalid schema reference');
    if (kind == CapabilityKind.command && retrySemantics == null) {
      problems.add('commands must declare retry semantics');
    }
    return List.unmodifiable(problems);
  }
}

final class DevelopmentGrant {
  const DevelopmentGrant({required this.consumerId, required this.capabilityId});

  final String consumerId;
  final String capabilityId;
}
