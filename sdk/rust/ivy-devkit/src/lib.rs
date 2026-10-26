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
    #[serde(default)]
    pub capability_domains: CapabilityDomains,
    #[serde(default)]
    pub capabilities: Capabilities,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CapabilityDomains {
    #[serde(default)]
    pub owns: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Capabilities {
    #[serde(default)]
    pub provides: Vec<ProvidedCapability>,
    #[serde(default)]
    pub requires: Vec<RequiredCapability>,
    #[serde(default)]
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
    #[serde(default)]
    pub externally_invocable: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RequiredCapability {
    pub id: String,
    pub version: u32,
    pub schema: String,
    #[serde(default)]
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
    if !valid_dotted_identifier(&manifest.namespace) {
        problems.push("namespace must be a dotted lowercase identifier".into());
    }
    if Version::parse(&manifest.version).is_err() {
        problems.push("version must be valid semantic versioning".into());
    }
    if manifest.language.trim().is_empty() {
        problems.push("language must not be empty".into());
    }

    let mut domains = BTreeSet::new();
    for domain in &manifest.capability_domains.owns {
        if !valid_dotted_identifier(domain) {
            problems.push(format!("invalid capability domain: {domain}"));
        }
        if !domains.insert(domain) {
            problems.push(format!("duplicate capability domain: {domain}"));
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
            .any(|domain| capability.id.starts_with(&format!("{domain}.")))
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
        if !capability.schema.starts_with("schema:") {
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
        if !capability.schema.starts_with("schema:") {
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
    if !valid_dotted_identifier(id) || id.split('.').count() < 3 {
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

fn valid_dotted_identifier(value: &str) -> bool {
    value.split('.').all(|segment| {
        !segment.is_empty()
            && !segment.starts_with('_')
            && segment
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
    })
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
    providers: BTreeMap<String, Provider>,
    grants: BTreeSet<(String, String)>,
}

struct Provider {
    module_id: String,
    handler: Handler,
}

impl DevelopmentSimulator {
    pub fn register(
        &mut self,
        manifest: &ModuleManifest,
        mut handlers: BTreeMap<String, Handler>,
    ) -> Result<(), SimulatorError> {
        validate_manifest(manifest)?;

        for domain in &manifest.capability_domains.owns {
            if let Some(owner) = self.domains.get(domain) {
                return Err(SimulatorError::DomainConflict {
                    domain: domain.clone(),
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
        for capability in &manifest.capabilities.provides {
            let handler = handlers
                .remove(&capability.id)
                .expect("handler presence was checked");
            self.providers.insert(
                capability.id.clone(),
                Provider {
                    module_id: manifest.id.clone(),
                    handler,
                },
            );
        }
        Ok(())
    }

    pub fn grant(&mut self, consumer: &str, capability: &str) -> Result<(), SimulatorError> {
        if !self.providers.contains_key(capability) {
            return Err(SimulatorError::UnknownCapability(capability.into()));
        }
        self.grants.insert((consumer.into(), capability.into()));
        Ok(())
    }

    pub fn revoke(&mut self, consumer: &str, capability: &str) {
        self.grants.remove(&(consumer.into(), capability.into()));
    }

    pub fn invoke(
        &self,
        consumer: &str,
        capability: &str,
        payload: &str,
    ) -> Result<String, InvocationError> {
        if !self.grants.contains(&(consumer.into(), capability.into())) {
            return Err(InvocationError::AccessDenied {
                consumer: consumer.into(),
                capability: capability.into(),
            });
        }
        let provider = self
            .providers
            .get(capability)
            .ok_or_else(|| InvocationError::Unavailable(capability.into()))?;
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
    #[error("capability domain {domain} is already owned by {owner}")]
    DomainConflict { domain: String, owner: String },
    #[error("capability is already provided: {0}")]
    CapabilityConflict(String),
    #[error("no handler supplied for declared capability: {0}")]
    MissingHandler(String),
    #[error("handler supplied for undeclared capability: {0}")]
    UndeclaredHandler(String),
    #[error("unknown capability: {0}")]
    UnknownCapability(String),
}

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum InvocationError {
    #[error("access denied: {consumer} has no grant for {capability}")]
    AccessDenied {
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

    fn clock_manifest() -> ModuleManifest {
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

    #[test]
    fn default_deny_grant_then_revoke() {
        let manifest = clock_manifest();
        let mut handlers = BTreeMap::new();
        handlers.insert(
            "example.clock.now".into(),
            Arc::new(|_: &str| Ok("12:00:00Z".into())) as Handler,
        );
        let mut simulator = DevelopmentSimulator::default();
        simulator.register(&manifest, handlers).unwrap();

        assert!(matches!(
            simulator.invoke("consumer", "example.clock.now", "{}"),
            Err(InvocationError::AccessDenied { .. })
        ));
        simulator.grant("consumer", "example.clock.now").unwrap();
        assert_eq!(
            simulator
                .invoke("consumer", "example.clock.now", "{}")
                .unwrap(),
            "12:00:00Z"
        );
        simulator.revoke("consumer", "example.clock.now");
        assert!(matches!(
            simulator.invoke("consumer", "example.clock.now", "{}"),
            Err(InvocationError::AccessDenied { .. })
        ));
    }

    #[test]
    fn exclusive_domain_ownership_is_enforced() {
        let manifest = clock_manifest();
        let mut handlers = BTreeMap::new();
        handlers.insert(
            "example.clock.now".into(),
            Arc::new(|_: &str| Ok("time".into())) as Handler,
        );
        let mut simulator = DevelopmentSimulator::default();
        simulator.register(&manifest, handlers).unwrap();
        let error = simulator.register(&manifest, BTreeMap::new()).unwrap_err();
        assert!(matches!(error, SimulatorError::DomainConflict { .. }));
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
