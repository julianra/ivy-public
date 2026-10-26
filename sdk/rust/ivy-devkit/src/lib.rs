//! Standalone, developer-facing Ivy contracts and a development-only simulator.
//!
//! This crate is deliberately independent from the proprietary Ivy Core. The
//! simulator proves module declarations and grant behavior, but it is not a
//! security boundary and must not be represented as Core.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use base64::Engine as _;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand_core::OsRng;
use semver::Version;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

pub const MANIFEST_SCHEMA_VERSION: u32 = 3;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ModuleManifest {
    pub schema_version: u32,
    pub id: String,
    pub namespace: String,
    pub version: String,
    pub language: String,
    pub capability_domains: CapabilityDomains,
    pub capabilities: Capabilities,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CapabilityDomains {
    pub owns: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Capabilities {
    pub provides: Vec<ProvidedCapability>,
    pub requires: Vec<RequiredCapability>,
    pub dynamic_requires: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProvidedCapability {
    pub id: String,
    pub version: u32,
    pub kind: CapabilityKind,
    pub schema: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retry_semantics: Option<RetrySemantics>,
    pub externally_invocable: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RequiredCapability {
    pub id: String,
    pub version: u32,
    pub schema: String,
    pub required: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityKind {
    Query,
    Command,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RetrySemantics {
    ResultReplay,
    ExternalEffectMayRepeat,
    NotRetryable,
}

pub fn parse_manifest_yaml(input: &str) -> Result<ModuleManifest, ManifestError> {
    let manifest: ModuleManifest =
        serde_yaml::from_str(input).map_err(|error| ManifestError::Parse(error.to_string()))?;
    validate_manifest(&manifest)?;
    Ok(manifest)
}

pub fn validate_manifest(manifest: &ModuleManifest) -> Result<(), ManifestError> {
    let mut problems = Vec::new();

    if manifest.schema_version != MANIFEST_SCHEMA_VERSION {
        problems.push(format!(
            "schema_version must be {MANIFEST_SCHEMA_VERSION}, got {}",
            manifest.schema_version
        ));
    }
    if !valid_module_id(&manifest.id) {
        problems.push("id must use lowercase ASCII letters, digits and hyphens".into());
    }
    if !valid_namespace(&manifest.namespace) {
        problems
            .push("namespace must contain at least two dot-separated lowercase segments".into());
    }
    if Version::parse(&manifest.version).is_err() {
        problems.push("version must be valid semantic versioning".into());
    }
    if !valid_language_id(&manifest.language) {
        problems.push(
            "language must use lowercase ASCII letters, digits, plus, hyphen or underscore".into(),
        );
    }

    let mut domains = BTreeSet::new();
    for domain in &manifest.capability_domains.owns {
        if !valid_domain_id(domain) {
            problems.push(format!("invalid capability domain: {domain}"));
        }
        if !domains.insert(domain) {
            problems.push(format!("duplicate capability domain: {domain}"));
        }
    }
    for (index, domain) in manifest.capability_domains.owns.iter().enumerate() {
        for other in manifest.capability_domains.owns.iter().skip(index + 1) {
            if domains_overlap(domain, other) {
                problems.push(format!(
                    "overlapping capability domains in one manifest are forbidden: {domain} and {other}"
                ));
            }
        }
    }

    let mut provided = BTreeSet::new();
    for capability in &manifest.capabilities.provides {
        validate_capability_id(&capability.id, &mut problems);
        if capability.version == 0 {
            problems.push(format!("{} must have a non-zero version", capability.id));
        }
        if !provided.insert(&capability.id) {
            problems.push(format!("duplicate provided capability: {}", capability.id));
        }
        if !domains
            .iter()
            .any(|domain| is_descendant_of(&capability.id, domain))
        {
            problems.push(format!(
                "provided capability {} is outside the module's owned domains",
                capability.id
            ));
        }
        if capability.kind == CapabilityKind::Command && capability.retry_semantics.is_none() {
            problems.push(format!(
                "command {} must declare retry_semantics",
                capability.id
            ));
        }
        if capability.kind == CapabilityKind::Query && capability.retry_semantics.is_some() {
            problems.push(format!(
                "query {} must not declare retry_semantics",
                capability.id
            ));
        }
        if !valid_schema_reference(&capability.schema) {
            problems.push(format!("{} has an invalid schema reference", capability.id));
        }
    }

    let mut required = BTreeSet::new();
    for capability in &manifest.capabilities.requires {
        validate_capability_id(&capability.id, &mut problems);
        if capability.version == 0 {
            problems.push(format!("{} must have a non-zero version", capability.id));
        }
        if !required.insert(&capability.id) {
            problems.push(format!("duplicate required capability: {}", capability.id));
        }
        if !valid_schema_reference(&capability.schema) {
            problems.push(format!("{} has an invalid schema reference", capability.id));
        }
    }

    if problems.is_empty() {
        Ok(())
    } else {
        Err(ManifestError::Invalid(problems))
    }
}

fn validate_capability_id(id: &str, problems: &mut Vec<String>) {
    if !valid_segmented_identifier(id, 3) {
        problems.push(format!("invalid capability id: {id}"));
    }
}

fn valid_module_id(value: &str) -> bool {
    !value.is_empty()
        && !value.starts_with('-')
        && !value.ends_with('-')
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

fn valid_language_id(value: &str) -> bool {
    value.as_bytes().first().is_some_and(u8::is_ascii_lowercase)
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'+' | b'-' | b'_')
        })
}

fn valid_namespace(value: &str) -> bool {
    valid_segmented_identifier(value, 2)
}

fn valid_domain_id(value: &str) -> bool {
    valid_segmented_identifier(value, 2)
}

fn valid_segmented_identifier(value: &str, minimum_segments: usize) -> bool {
    let segments = value.split('.').collect::<Vec<_>>();
    segments.len() >= minimum_segments
        && segments.iter().all(|segment| {
            segment
                .as_bytes()
                .first()
                .is_some_and(u8::is_ascii_lowercase)
                && segment
                    .bytes()
                    .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
        })
}

fn valid_schema_reference(value: &str) -> bool {
    let Some(rest) = value.strip_prefix("schema:") else {
        return false;
    };
    let Some((name, version)) = rest.rsplit_once(":v") else {
        return false;
    };
    valid_segmented_identifier(name, 2)
        && !version.is_empty()
        && !version.starts_with('0')
        && version.bytes().all(|byte| byte.is_ascii_digit())
}

fn is_descendant_of(candidate: &str, domain: &str) -> bool {
    candidate
        .strip_prefix(domain)
        .is_some_and(|suffix| suffix.starts_with('.'))
}

fn domains_overlap(left: &str, right: &str) -> bool {
    left == right || is_descendant_of(left, right) || is_descendant_of(right, left)
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ManifestError {
    #[error("could not parse manifest: {0}")]
    Parse(String),
    #[error("manifest validation failed: {0:?}")]
    Invalid(Vec<String>),
}

pub type Handler = Arc<dyn Fn(&str) -> Result<String, InvocationError> + Send + Sync>;

#[derive(Default)]
pub struct DevelopmentSimulator {
    domains: BTreeMap<String, String>,
    consumers: BTreeMap<String, BTreeSet<String>>,
    providers: BTreeMap<String, Provider>,
    offer_grants: BTreeSet<(String, String)>,
    consumer_grants: BTreeSet<(String, String)>,
}

struct Provider {
    module_id: String,
    available: bool,
    handler: Handler,
}

impl DevelopmentSimulator {
    pub fn register(
        &mut self,
        manifest: &ModuleManifest,
        mut handlers: BTreeMap<String, Handler>,
    ) -> Result<(), SimulatorError> {
        validate_manifest(manifest)?;

        if self.consumers.contains_key(&manifest.id)
            || self.domains.values().any(|owner| owner == &manifest.id)
            || self
                .providers
                .values()
                .any(|provider| provider.module_id == manifest.id)
        {
            return Err(SimulatorError::ModuleAlreadyRegistered(manifest.id.clone()));
        }

        for claimed in &manifest.capability_domains.owns {
            if let Some((existing, owner)) = self
                .domains
                .iter()
                .find(|(existing, _)| domains_overlap(claimed, existing))
            {
                return Err(SimulatorError::DomainConflict {
                    claimed: claimed.clone(),
                    existing: existing.clone(),
                    owner: owner.clone(),
                });
            }
        }
        for capability in &manifest.capabilities.provides {
            if self.providers.contains_key(&capability.id) {
                return Err(SimulatorError::CapabilityConflict(capability.id.clone()));
            }
            if !handlers.contains_key(&capability.id) {
                return Err(SimulatorError::MissingHandler(capability.id.clone()));
            }
        }
        if let Some(extra) = handlers.keys().find(|id| {
            !manifest
                .capabilities
                .provides
                .iter()
                .any(|cap| &cap.id == *id)
        }) {
            return Err(SimulatorError::UndeclaredHandler(extra.clone()));
        }

        for domain in &manifest.capability_domains.owns {
            self.domains.insert(domain.clone(), manifest.id.clone());
        }
        self.consumers.insert(
            manifest.id.clone(),
            manifest
                .capabilities
                .requires
                .iter()
                .map(|capability| capability.id.clone())
                .collect(),
        );
        for capability in &manifest.capabilities.provides {
            let handler = handlers
                .remove(&capability.id)
                .expect("handler presence was checked");
            self.providers.insert(
                capability.id.clone(),
                Provider {
                    module_id: manifest.id.clone(),
                    available: true,
                    handler,
                },
            );
        }
        Ok(())
    }

    pub fn grant_offer(&mut self, provider: &str, capability: &str) -> Result<(), SimulatorError> {
        let installed = self
            .providers
            .get(capability)
            .ok_or_else(|| SimulatorError::UnknownCapability(capability.into()))?;
        if installed.module_id != provider {
            return Err(SimulatorError::WrongProvider {
                capability: capability.into(),
                expected: installed.module_id.clone(),
                actual: provider.into(),
            });
        }
        self.offer_grants
            .insert((provider.into(), capability.into()));
        Ok(())
    }

    pub fn revoke_offer(&mut self, provider: &str, capability: &str) {
        self.offer_grants
            .remove(&(provider.into(), capability.into()));
    }

    pub fn grant_consumer(
        &mut self,
        consumer: &str,
        capability: &str,
    ) -> Result<(), SimulatorError> {
        if !self.consumers.contains_key(consumer) {
            return Err(SimulatorError::UnknownConsumer(consumer.into()));
        }
        if !self.providers.contains_key(capability) {
            return Err(SimulatorError::UnknownCapability(capability.into()));
        }
        self.consumer_grants
            .insert((consumer.into(), capability.into()));
        Ok(())
    }

    pub fn revoke_consumer(&mut self, consumer: &str, capability: &str) {
        self.consumer_grants
            .remove(&(consumer.into(), capability.into()));
    }

    pub fn set_provider_available(
        &mut self,
        provider: &str,
        available: bool,
    ) -> Result<(), SimulatorError> {
        let mut found = false;
        for installed in self.providers.values_mut() {
            if installed.module_id == provider {
                installed.available = available;
                found = true;
            }
        }
        if found {
            Ok(())
        } else {
            Err(SimulatorError::UnknownModule(provider.into()))
        }
    }

    pub fn unregister(&mut self, module_id: &str) -> Result<(), SimulatorError> {
        let is_consumer = self.consumers.remove(module_id).is_some();
        let provided = self
            .providers
            .iter()
            .filter(|(_, provider)| provider.module_id == module_id)
            .map(|(capability, _)| capability.clone())
            .collect::<Vec<_>>();
        let owned = self
            .domains
            .iter()
            .filter(|(_, owner)| *owner == module_id)
            .map(|(domain, _)| domain.clone())
            .collect::<Vec<_>>();
        if !is_consumer && provided.is_empty() && owned.is_empty() {
            return Err(SimulatorError::UnknownModule(module_id.into()));
        }
        for capability in &provided {
            self.providers.remove(capability);
        }
        for domain in owned {
            self.domains.remove(&domain);
        }
        self.offer_grants.retain(|(provider, capability)| {
            provider != module_id && !provided.contains(capability)
        });
        self.consumer_grants.retain(|(consumer, capability)| {
            consumer != module_id && !provided.contains(capability)
        });
        Ok(())
    }

    pub fn invoke(
        &self,
        consumer: &str,
        capability: &str,
        payload: &str,
    ) -> Result<String, InvocationError> {
        let provider = self
            .providers
            .get(capability)
            .ok_or_else(|| InvocationError::Unavailable(capability.into()))?;
        if !provider.available {
            return Err(InvocationError::Unavailable(capability.into()));
        }
        let declared = self
            .consumers
            .get(consumer)
            .ok_or_else(|| InvocationError::UnknownConsumer(consumer.into()))?;
        if !declared.contains(capability) {
            return Err(InvocationError::UndeclaredCapability {
                consumer: consumer.into(),
                capability: capability.into(),
            });
        }
        if !self
            .offer_grants
            .contains(&(provider.module_id.clone(), capability.into()))
        {
            return Err(InvocationError::OfferDenied {
                provider: provider.module_id.clone(),
                capability: capability.into(),
            });
        }
        if !self
            .consumer_grants
            .contains(&(consumer.into(), capability.into()))
        {
            return Err(InvocationError::ConsumerDenied {
                consumer: consumer.into(),
                capability: capability.into(),
            });
        }
        (provider.handler)(payload)
    }

    pub fn provider_for(&self, capability: &str) -> Option<&str> {
        self.providers
            .get(capability)
            .map(|provider| provider.module_id.as_str())
    }
}

#[derive(Debug, Error)]
pub enum SimulatorError {
    #[error(transparent)]
    Manifest(#[from] ManifestError),
    #[error("capability domain {claimed} overlaps {existing}, owned by {owner}")]
    DomainConflict {
        claimed: String,
        existing: String,
        owner: String,
    },
    #[error("module is already registered: {0}")]
    ModuleAlreadyRegistered(String),
    #[error("capability is already provided: {0}")]
    CapabilityConflict(String),
    #[error("no handler supplied for declared capability: {0}")]
    MissingHandler(String),
    #[error("handler supplied for undeclared capability: {0}")]
    UndeclaredHandler(String),
    #[error("unknown capability: {0}")]
    UnknownCapability(String),
    #[error("unknown consumer: {0}")]
    UnknownConsumer(String),
    #[error("unknown module: {0}")]
    UnknownModule(String),
    #[error("{actual} does not provide {capability}; installed provider is {expected}")]
    WrongProvider {
        capability: String,
        expected: String,
        actual: String,
    },
}

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum InvocationError {
    #[error("unknown development consumer identity: {0}")]
    UnknownConsumer(String),
    #[error("access denied: {consumer} did not declare {capability}")]
    UndeclaredCapability {
        consumer: String,
        capability: String,
    },
    #[error("access denied: provider {provider} has no offer grant for {capability}")]
    OfferDenied {
        provider: String,
        capability: String,
    },
    #[error("access denied: consumer {consumer} has no invocation grant for {capability}")]
    ConsumerDenied {
        consumer: String,
        capability: String,
    },
    #[error("capability unavailable: {0}")]
    Unavailable(String),
    #[error("provider error: {0}")]
    Provider(String),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct DevelopmentSignature {
    pub algorithm: String,
    pub purpose: String,
    pub sha256: String,
    pub public_key: String,
    pub signature: String,
}

pub struct DevelopmentSigningKey(SigningKey);

impl DevelopmentSigningKey {
    pub fn generate() -> Self {
        Self(SigningKey::generate(&mut OsRng))
    }

    pub fn from_base64(encoded: &str) -> Result<Self, SigningError> {
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(encoded.trim())
            .map_err(|error| SigningError::Encoding(error.to_string()))?;
        let bytes: [u8; 32] = decoded
            .try_into()
            .map_err(|_| SigningError::Encoding("development key must be 32 bytes".into()))?;
        Ok(Self(SigningKey::from_bytes(&bytes)))
    }

    pub fn secret_base64(&self) -> String {
        base64::engine::general_purpose::STANDARD.encode(self.0.to_bytes())
    }

    pub fn sign(&self, bytes: &[u8]) -> DevelopmentSignature {
        let digest = Sha256::digest(bytes);
        let signature = self.0.sign(bytes);
        DevelopmentSignature {
            algorithm: "Ed25519".into(),
            purpose: "ivy-development-only".into(),
            sha256: hex(&digest),
            public_key: base64::engine::general_purpose::STANDARD
                .encode(self.0.verifying_key().to_bytes()),
            signature: base64::engine::general_purpose::STANDARD.encode(signature.to_bytes()),
        }
    }
}

pub fn verify_development_signature(
    bytes: &[u8],
    envelope: &DevelopmentSignature,
) -> Result<(), SigningError> {
    if envelope.algorithm != "Ed25519" || envelope.purpose != "ivy-development-only" {
        return Err(SigningError::WrongPurpose);
    }
    let digest = Sha256::digest(bytes);
    if envelope.sha256 != hex(&digest) {
        return Err(SigningError::DigestMismatch);
    }
    let public_key = decode_array::<32>(&envelope.public_key)?;
    let signature = decode_array::<64>(&envelope.signature)?;
    let verifying_key = VerifyingKey::from_bytes(&public_key)
        .map_err(|error| SigningError::Encoding(error.to_string()))?;
    verifying_key
        .verify(bytes, &Signature::from_bytes(&signature))
        .map_err(|_| SigningError::InvalidSignature)
}

fn decode_array<const N: usize>(encoded: &str) -> Result<[u8; N], SigningError> {
    base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .map_err(|error| SigningError::Encoding(error.to_string()))?
        .try_into()
        .map_err(|_| SigningError::Encoding(format!("expected {N} bytes")))
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SigningError {
    #[error("invalid key or signature encoding: {0}")]
    Encoding(String),
    #[error("signature has the wrong development purpose")]
    WrongPurpose,
    #[error("content digest does not match")]
    DigestMismatch,
    #[error("signature verification failed")]
    InvalidSignature,
}

#[cfg(test)]
mod tests {
    use super::*;

    const CAPABILITY: &str = "example.clock.now";

    fn provider_manifest() -> ModuleManifest {
        parse_manifest_yaml(
            r#"
schema_version: 3
id: hello-clock
namespace: community.example.hello_clock
version: 0.1.0
language: rust
capability_domains:
  owns: [example.clock]
capabilities:
  provides:
    - id: example.clock.now
      version: 1
      kind: query
      schema: schema:example.clock.now:v1
      externally_invocable: false
  requires: []
  dynamic_requires: false
"#,
        )
        .unwrap()
    }

    fn consumer_manifest(id: &str, declares_clock: bool) -> ModuleManifest {
        let requirement = if declares_clock {
            r#"
    - id: example.clock.now
      version: 1
      schema: schema:example.clock.now:v1
      required: true"#
        } else {
            " []"
        };
        parse_manifest_yaml(&format!(
            r#"
schema_version: 3
id: {id}
namespace: community.example.{id}
version: 0.1.0
language: rust
capability_domains:
  owns: []
capabilities:
  provides: []
  requires:{requirement}
  dynamic_requires: false
"#
        ))
        .unwrap()
    }

    fn domain_manifest(id: &str, domain: &str, capability: &str) -> ModuleManifest {
        parse_manifest_yaml(&format!(
            r#"
schema_version: 3
id: {id}
namespace: community.example.{id}
version: 0.1.0
language: rust
capability_domains:
  owns: [{domain}]
capabilities:
  provides:
    - id: {capability}
      version: 1
      kind: query
      schema: schema:{capability}:v1
      externally_invocable: false
  requires: []
  dynamic_requires: false
"#
        ))
        .unwrap()
    }

    fn clock_handler() -> Handler {
        Arc::new(|_: &str| Ok("12:00:00Z".into()))
    }

    fn registered_pair(consumer: ModuleManifest) -> DevelopmentSimulator {
        let provider = provider_manifest();
        let mut handlers = BTreeMap::new();
        handlers.insert(CAPABILITY.into(), clock_handler());
        let mut simulator = DevelopmentSimulator::default();
        simulator.register(&provider, handlers).unwrap();
        simulator.register(&consumer, BTreeMap::new()).unwrap();
        simulator
    }

    #[test]
    fn provider_offer_and_consumer_grants_are_independent_and_revocable() {
        let consumer = consumer_manifest("dashboard", true);

        let mut neither = registered_pair(consumer.clone());
        assert!(matches!(
            neither.invoke("dashboard", CAPABILITY, "{}"),
            Err(InvocationError::OfferDenied { .. })
        ));

        neither.grant_offer("hello-clock", CAPABILITY).unwrap();
        assert!(matches!(
            neither.invoke("dashboard", CAPABILITY, "{}"),
            Err(InvocationError::ConsumerDenied { .. })
        ));

        let mut consumer_only = registered_pair(consumer);
        consumer_only
            .grant_consumer("dashboard", CAPABILITY)
            .unwrap();
        assert!(matches!(
            consumer_only.invoke("dashboard", CAPABILITY, "{}"),
            Err(InvocationError::OfferDenied { .. })
        ));

        consumer_only
            .grant_offer("hello-clock", CAPABILITY)
            .unwrap();
        assert_eq!(
            consumer_only.invoke("dashboard", CAPABILITY, "{}").unwrap(),
            "12:00:00Z"
        );

        consumer_only.revoke_offer("hello-clock", CAPABILITY);
        assert!(matches!(
            consumer_only.invoke("dashboard", CAPABILITY, "{}"),
            Err(InvocationError::OfferDenied { .. })
        ));

        consumer_only
            .grant_offer("hello-clock", CAPABILITY)
            .unwrap();
        consumer_only.revoke_consumer("dashboard", CAPABILITY);
        assert!(matches!(
            consumer_only.invoke("dashboard", CAPABILITY, "{}"),
            Err(InvocationError::ConsumerDenied { .. })
        ));
    }

    #[test]
    fn consumer_grant_does_not_replace_static_declaration() {
        let mut simulator = registered_pair(consumer_manifest("undeclared", false));
        simulator.grant_offer("hello-clock", CAPABILITY).unwrap();
        simulator.grant_consumer("undeclared", CAPABILITY).unwrap();

        assert!(matches!(
            simulator.invoke("undeclared", CAPABILITY, "{}"),
            Err(InvocationError::UndeclaredCapability { .. })
        ));
    }

    #[test]
    fn unavailable_removed_and_unknown_providers_fail_closed() {
        let mut simulator = registered_pair(consumer_manifest("dashboard", true));
        simulator.grant_offer("hello-clock", CAPABILITY).unwrap();
        simulator.grant_consumer("dashboard", CAPABILITY).unwrap();

        assert!(matches!(
            simulator.invoke("dashboard", "example.unknown.read", "{}"),
            Err(InvocationError::Unavailable(_))
        ));

        simulator
            .set_provider_available("hello-clock", false)
            .unwrap();
        assert!(matches!(
            simulator.invoke("dashboard", CAPABILITY, "{}"),
            Err(InvocationError::Unavailable(_))
        ));

        simulator.unregister("hello-clock").unwrap();
        assert!(matches!(
            simulator.invoke("dashboard", CAPABILITY, "{}"),
            Err(InvocationError::Unavailable(_))
        ));
    }

    #[test]
    fn ancestor_and_descendant_domain_claims_conflict() {
        let mut simulator = DevelopmentSimulator::default();
        let parent = domain_manifest("parent", "example.clock", "example.clock.now");
        let child = domain_manifest("child", "example.clock.backup", "example.clock.backup.read");
        let mut handlers = BTreeMap::new();
        handlers.insert("example.clock.now".into(), clock_handler());
        simulator.register(&parent, handlers).unwrap();
        let mut child_handlers = BTreeMap::new();
        child_handlers.insert("example.clock.backup.read".into(), clock_handler());
        let error = simulator.register(&child, child_handlers).unwrap_err();
        assert!(matches!(error, SimulatorError::DomainConflict { .. }));
        assert_eq!(simulator.provider_for("example.clock.backup.read"), None);

        let mut reverse = DevelopmentSimulator::default();
        let mut child_handlers = BTreeMap::new();
        child_handlers.insert("example.clock.backup.read".into(), clock_handler());
        reverse.register(&child, child_handlers).unwrap();
        let mut parent_handlers = BTreeMap::new();
        parent_handlers.insert("example.clock.now".into(), clock_handler());
        assert!(matches!(
            reverse.register(&parent, parent_handlers),
            Err(SimulatorError::DomainConflict { .. })
        ));
        assert_eq!(reverse.provider_for("example.clock.now"), None);
    }

    #[test]
    fn segment_boundary_prevents_false_prefix_conflicts() {
        let clock = domain_manifest("clock", "example.clock", "example.clock.now");
        let clockwork =
            domain_manifest("clockwork", "example.clockwork", "example.clockwork.status");
        let mut simulator = DevelopmentSimulator::default();
        let mut clock_handlers = BTreeMap::new();
        clock_handlers.insert("example.clock.now".into(), clock_handler());
        simulator.register(&clock, clock_handlers).unwrap();
        let mut clockwork_handlers = BTreeMap::new();
        clockwork_handlers.insert("example.clockwork.status".into(), clock_handler());
        simulator.register(&clockwork, clockwork_handlers).unwrap();
        assert_eq!(
            simulator.provider_for("example.clockwork.status"),
            Some("clockwork")
        );
    }

    #[test]
    fn registration_is_atomic_after_late_validation_failure() {
        let manifest = domain_manifest("backup", "example.backup", "example.backup.read");
        let mut handlers = BTreeMap::new();
        handlers.insert("example.backup.read".into(), clock_handler());
        handlers.insert("example.backup.hidden".into(), clock_handler());
        let mut simulator = DevelopmentSimulator::default();
        assert!(matches!(
            simulator.register(&manifest, handlers),
            Err(SimulatorError::UndeclaredHandler(_))
        ));
        assert_eq!(simulator.provider_for("example.backup.read"), None);

        let mut valid_handlers = BTreeMap::new();
        valid_handlers.insert("example.backup.read".into(), clock_handler());
        simulator.register(&manifest, valid_handlers).unwrap();
    }

    #[test]
    fn overlapping_domains_inside_one_manifest_are_invalid() {
        let error = parse_manifest_yaml(
            r#"
schema_version: 3
id: overlap
namespace: community.example.overlap
version: 0.1.0
language: rust
capability_domains:
  owns: [example.clock, example.clock.backup]
capabilities:
  provides: []
  requires: []
  dynamic_requires: false
"#,
        )
        .unwrap_err();
        assert!(matches!(error, ManifestError::Invalid(_)));
    }

    #[test]
    fn development_signatures_are_bound_to_content() {
        let key = DevelopmentSigningKey::generate();
        let signature = key.sign(b"manifest");
        verify_development_signature(b"manifest", &signature).unwrap();
        assert_eq!(
            verify_development_signature(b"changed", &signature),
            Err(SigningError::DigestMismatch)
        );
    }
}
