<!-- doc-key: first-module -->
<!-- parity-version: 2 -->
<!-- sections: generate,inspect,validate,simulate,next -->

# Handleiding: eerste developmentmodule

**Nederlands** | [English](../../en/tutorials/first-module.md)

## Wat ga je maken?

In deze oefening maak je een **voorbeeldmodule**, bekijk je haar manifest (technische beschrijving) en test je de declaraties en rechten via de publieke tools. Je hebt Rust en Cargo nodig, maar geen private Core. Het is **geen installatie van een productiemodule**. Voer de commando's uit vanuit de repositoryhoofdmap; `/tmp` is een Unix-voorbeeldpad.

<!-- section: generate -->
## Genereer

```bash
cargo run -p ivy-cli -- new /tmp/status-module --id status-module --namespace community.example.status_module
```

<!-- section: inspect -->
## Inspecteer

Het gegenereerde manifest bezit `community.status_module` en biedt één query.
Ownership is exclusief; installatie zou op zichzelf geen consumertoegang geven.

<!-- section: validate -->
## Valideer

```bash
cargo run -p ivy-cli -- validate /tmp/status-module/module.yaml
```

<!-- section: simulate -->
## Simuleer

Bestudeer `examples/hello-clock`: het registreert provider- en
consumermanifests, vereist een provider offer grant en consumer invocation grant
en bewijst daarna onmiddellijke revocation. Draai het met:

```bash
cargo run -p ivy-hello-clock
```

<!-- section: next -->
## Volgende stappen

Voeg negatieve fixtures en tests toe vóór je een nieuw publiek contract
voorstelt. Een gegenereerde skeleton is geen installeerbare productiemodule;
runtimebindings, packaging, officiële signing en installatie blijven toekomstige
integratiewerkzaamheden.
