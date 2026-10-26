use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;
use ivy_devkit::{DevelopmentSimulator, Handler, InvocationError, parse_manifest_yaml};

const CAPABILITY: &str = "example.clock.now";
const CONSUMER: &str = "example-dashboard";

fn main() -> Result<()> {
    let manifest = parse_manifest_yaml(include_str!("../module.yaml"))?;
    let consumer = parse_manifest_yaml(include_str!("../consumer.yaml"))?;
    let mut handlers = BTreeMap::new();
    handlers.insert(CAPABILITY.into(), clock_handler());

    let mut simulator = DevelopmentSimulator::default();
    simulator.register(&manifest, handlers)?;
    simulator.register(&consumer, BTreeMap::new())?;

    println!("provider: {}", simulator.provider_for(CAPABILITY).unwrap());
    show_denied(simulator.invoke(CONSUMER, CAPABILITY, "{}"));
    simulator.grant_offer("hello-clock", CAPABILITY)?;
    show_denied(simulator.invoke(CONSUMER, CAPABILITY, "{}"));
    simulator.grant_consumer(CONSUMER, CAPABILITY)?;
    println!(
        "after grant: {}",
        simulator.invoke(CONSUMER, CAPABILITY, "{}")?
    );
    simulator.revoke_consumer(CONSUMER, CAPABILITY);
    show_denied(simulator.invoke(CONSUMER, CAPABILITY, "{}"));
    Ok(())
}

fn clock_handler() -> Handler {
    Arc::new(|_| {
        let seconds = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| InvocationError::Provider(error.to_string()))?
            .as_secs();
        Ok(format!("{{\"unix_seconds\":{seconds}}}"))
    })
}

fn show_denied(result: Result<String, InvocationError>) {
    match result {
        Err(InvocationError::OfferDenied { .. } | InvocationError::ConsumerDenied { .. }) => {
            println!("default deny: access refused")
        }
        other => panic!("expected access denial, got {other:?}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grant_is_required_and_revocation_is_immediate() {
        let manifest = parse_manifest_yaml(include_str!("../module.yaml")).unwrap();
        let consumer = parse_manifest_yaml(include_str!("../consumer.yaml")).unwrap();
        let mut handlers = BTreeMap::new();
        handlers.insert(CAPABILITY.into(), clock_handler());
        let mut simulator = DevelopmentSimulator::default();
        simulator.register(&manifest, handlers).unwrap();
        simulator.register(&consumer, BTreeMap::new()).unwrap();

        assert!(matches!(
            simulator.invoke(CONSUMER, CAPABILITY, "{}"),
            Err(InvocationError::OfferDenied { .. })
        ));
        simulator.grant_offer("hello-clock", CAPABILITY).unwrap();
        assert!(matches!(
            simulator.invoke(CONSUMER, CAPABILITY, "{}"),
            Err(InvocationError::ConsumerDenied { .. })
        ));
        simulator.grant_consumer(CONSUMER, CAPABILITY).unwrap();
        assert!(simulator.invoke(CONSUMER, CAPABILITY, "{}").is_ok());
        simulator.revoke_consumer(CONSUMER, CAPABILITY);
        assert!(matches!(
            simulator.invoke(CONSUMER, CAPABILITY, "{}"),
            Err(InvocationError::ConsumerDenied { .. })
        ));
    }
}
