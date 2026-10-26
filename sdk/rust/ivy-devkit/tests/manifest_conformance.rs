use std::fs;
use std::path::{Path, PathBuf};

use ivy_devkit::parse_manifest_yaml;
use serde_json::Value;

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

fn fixtures(category: &str) -> Vec<PathBuf> {
    let mut paths = fs::read_dir(root().join("conformance/manifests").join(category))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "yaml")
        })
        .collect::<Vec<_>>();
    paths.sort();
    paths
}

fn schema_validator() -> jsonschema::Validator {
    let schema: Value = serde_json::from_str(
        &fs::read_to_string(root().join("contracts/schemas/module-manifest-v3.schema.json"))
            .unwrap(),
    )
    .unwrap();
    jsonschema::validator_for(&schema).unwrap()
}

fn yaml_value(path: &Path) -> Value {
    serde_yaml::from_str(&fs::read_to_string(path).unwrap()).unwrap()
}

#[test]
fn valid_fixtures_pass_schema_and_semantic_validation() {
    let validator = schema_validator();
    for path in fixtures("valid") {
        let text = fs::read_to_string(&path).unwrap();
        assert!(validator.is_valid(&yaml_value(&path)), "schema: {path:?}");
        assert!(parse_manifest_yaml(&text).is_ok(), "semantic: {path:?}");
    }
}

#[test]
fn structural_fixtures_fail_both_validation_paths() {
    let validator = schema_validator();
    for path in fixtures("schema-invalid") {
        let text = fs::read_to_string(&path).unwrap();
        assert!(!validator.is_valid(&yaml_value(&path)), "schema: {path:?}");
        assert!(parse_manifest_yaml(&text).is_err(), "semantic: {path:?}");
    }
}

#[test]
fn semantic_fixtures_pass_schema_but_fail_rust_invariants() {
    let validator = schema_validator();
    for path in fixtures("semantic-invalid") {
        let text = fs::read_to_string(&path).unwrap();
        assert!(validator.is_valid(&yaml_value(&path)), "schema: {path:?}");
        assert!(parse_manifest_yaml(&text).is_err(), "semantic: {path:?}");
    }
}
