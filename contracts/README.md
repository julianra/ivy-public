# Public Developer Contracts / Publieke developercontracten

## English

### What is this for?

Ivy allows developers to create modules that work together within a modular software platform.

But how does an external developer know how to describe a module or exchange information using Ivy's public interfaces?

That's what **Developer Contracts** are for.

A contract defines a technical agreement between Ivy and software that integrates with it. These agreements help different developers build compatible software without needing access to Ivy's private internal code.

Think of it as a common language: when both sides follow the same definitions, they can understand the information being exchanged.

### What can you find here?

This directory contains two types of public contracts:

| Component | Purpose |
| --- | --- |
| **JSON Schema** | Defines the expected structure of a module manifest, including which fields and data formats are allowed. |
| **Protocol Buffers (protobuf)** | Defines public developer-facing message structures that software can use when implementing compatible communication interfaces. |

A module manifest is a structured description of a module, including its identity and declared capabilities.

### A practical example

Imagine developing an Ivy module that controls smart lighting.

First, your module needs to describe itself in a manifest.

The JSON Schema defines how that manifest must be structured.

If your integration needs to exchange messages through a supported public protobuf interface, the protobuf declarations define the format of those messages.

This allows you to develop against documented interfaces instead of depending on private Ivy implementation details.

### What is included and excluded?

The public contracts are an independently usable subset of Ivy's internal contracts.

They intentionally exclude private Core administration, mobile authentication, secrets, lifecycle messages, and Bridge internals.

These exclusions protect the separation between the public developer interface and Ivy's private implementation.

**Important:** A public contract defines an interface. It does not automatically grant access to Ivy functionality, bypass permission checks, or guarantee that a module will work with every Ivy version.

### How are contracts validated?

The JSON Schema checks the structure of a module manifest.

The Rust validator adds semantic checks, including ownership constraints that cannot be fully expressed through structural validation alone.

The Conformance Tests verify selected rules using valid and invalid examples.

### Directory structure

- `schemas/` — Public JSON schemas, including `module-manifest-v3.schema.json`.
- `proto/` — Public developer protobuf declarations.

### Further documentation

See the [English manifest reference](../docs/en/development/manifest-reference.md) for more information about manifest fields and their meaning.

See the [Conformance documentation](../conformance/README.md) to learn how Ivy tests compliance with its rules.

---

## Nederlands

### Waarvoor dient dit?

Ivy is een modulair softwareplatform waarop ontwikkelaars hun eigen modules kunnen bouwen en verschillende toepassingen met elkaar kunnen laten samenwerken.

Maar hoe weet een externe ontwikkelaar hoe een module beschreven moet worden of hoe gegevens via de publieke interfaces van Ivy uitgewisseld moeten worden?

Daarvoor bestaan de **Developer Contracts**, ofwel ontwikkelaarscontracten.

Een contract is een verzameling technische afspraken tussen Ivy en software die ermee samenwerkt.

Dankzij die afspraken kunnen verschillende ontwikkelaars compatibele software bouwen, zonder toegang te moeten hebben tot de private interne broncode van Ivy.

Je kunt het vergelijken met een gemeenschappelijke taal. Wanneer beide systemen dezelfde afspraken volgen, begrijpen ze hoe de uitgewisselde informatie opgebouwd is.

### Wat vind je in deze map?

Deze map bevat twee soorten publieke contracten:

| Onderdeel | Doel |
| --- | --- |
| **JSON Schema** | Beschrijft hoe het manifest van een module opgebouwd moet zijn, inclusief toegelaten velden en gegevensformaten. |
| **Protocol Buffers (protobuf)** | Beschrijven publieke berichtstructuren die ontwikkelaars kunnen gebruiken om compatibele communicatie-interfaces te implementeren. |

Een modulemanifest is een gestructureerde beschrijving van een module, waaronder haar identiteit en de functionaliteit die ze verklaart aan te bieden.

### Een praktisch voorbeeld

Stel dat je een Ivy-module ontwikkelt waarmee je slimme verlichting kunt bedienen.

Eerst moet je module zichzelf beschrijven in een manifest.

Het JSON Schema bepaalt hoe dat manifest opgebouwd moet zijn.

Wanneer je module berichten moet uitwisselen via een ondersteunde publieke protobufinterface, bepalen de protobufdeclaraties hoe die berichten gestructureerd zijn.

Zo kun je software ontwikkelen op basis van gedocumenteerde afspraken, zonder afhankelijk te zijn van de private implementatie van Ivy.

### Wat is wel en niet inbegrepen?

De publieke contracten vormen een zelfstandig bruikbaar gedeelte van de interne Ivy-contracten.

Bepaalde onderdelen zijn bewust niet openbaar opgenomen, waaronder:

- Private administratie van Ivy Core.
- Mobiele authenticatie.
- Secrets en geheime gegevens.
- Interne lifecycleberichten.
- Interne communicatie en implementatiedetails van Bridge.

Hiermee blijft de publieke ontwikkelaarsinterface gescheiden van de private interne werking van Ivy.

**Belangrijk:** Een publiek contract beschrijft een interface. Het geeft niet automatisch toegang tot Ivy-functionaliteit, omzeilt geen toegangsrechten en garandeert niet dat een module met iedere Ivy-versie compatibel is.

### Hoe worden de contracten gecontroleerd?

Het JSON Schema controleert of de structuur van een modulemanifest geldig is.

De Rust-validator voert aanvullende inhoudelijke controles uit, waaronder regels over welke module eigenaar mag zijn van bepaalde functionaliteit.

De Conformance Tests gebruiken geldige en ongeldige voorbeelden om specifieke afspraken automatisch te controleren.

### Mapstructuur

- `schemas/` — Publieke JSON-schema's, waaronder `module-manifest-v3.schema.json`.
- `proto/` — Publieke protobufdeclaraties voor ontwikkelaars.

### Verdere documentatie

Bekijk de [Nederlandse manifestreferentie](../docs/nl/development/manifest-reference.md) voor meer informatie over de velden en betekenis van een modulemanifest.

Bekijk de [Conformance-documentatie](../conformance/README.md) om te begrijpen hoe Ivy controleert of de geteste regels worden nageleefd.
