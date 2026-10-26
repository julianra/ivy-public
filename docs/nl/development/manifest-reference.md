<!-- doc-key: manifest-reference -->
<!-- parity-version: 2 -->
<!-- sections: scope,syntax,semantics,fixtures,generator -->

# Modulemanifest: hoe beschrijf je je Ivy-module?

**Nederlands** | [English](../../en/development/manifest-reference.md)

Een **manifest** is de technische identiteitskaart van een module. Het beschrijft bijvoorbeeld haar naam, versie, functies (*capabilities*) en de domeinen waarvoor ze verantwoordelijk wil zijn. Ivy gebruikt afgesproken regels om die beschrijving te controleren.

Stel dat je een module voor slimme verlichting maakt. Je kunt verklaren dat ze `home.lighting` beheert en functies zoals `home.lighting.turn_on` aanbiedt. De validator controleert dan of die gegevens geldig zijn opgebouwd en of de aanvullende inhoudelijke regels worden gevolgd.

**Let op:** een geldig manifest is nog geen toestemming om een module te installeren of haar functies te gebruiken.

<!-- section: scope -->
## Wat beschrijft dit publieke schema?

Het [publieke JSON Schema](../../../contracts/schemas/module-manifest-v3.schema.json) is een **strikte ontwikkelaarssubset** van het private manifestformaat v3. Daarmee kun je publieke modulebeschrijvingen bouwen en controleren, zonder toegang tot de private Core.

Productievelden voor runtime, IPC (communicatie tussen processen), sandbox, platform, resources, distributie en permissions zijn bewust weggelaten. Deze Developer Kit is dus geen volledige productie-installer.

<!-- section: syntax -->
## Welke schrijfregels gelden?

De schema-validator controleert de **structuur**: bestaan de juiste velden en voldoen namen en waarden aan de verwachte vorm?

| Onderdeel | Regel | Voorbeeld |
|---|---|---|
| Module-ID | Kleine ASCII-letters/cijfers, eventueel gescheiden door `-` | `smart-lighting` |
| Namespace of domein | Minstens twee puntgescheiden segmenten; elk begint met een kleine letter en mag daarna kleine letters, cijfers en `_` bevatten | `home.lighting` |
| Capability-ID | Minstens drie puntgescheiden segmenten | `home.lighting.turn_on` |
| Moduleversie | Semantic Versioning (SemVer 2.0.0) | `1.2.0` |
| Schemareferentie | `schema:<naam-met-minstens-twee-segmenten>:v<positief-getal>` | `schema:home.lighting:v1` |
| Command | Moet `retry_semantics` opgeven (wat bij herhaling mag gebeuren) | `not_retryable` |
| Query | Mag geen `retry_semantics` bevatten | Een status opvragen |

Alle toegestane velden zijn expliciet vastgelegd. **Onbekende velden worden geweigerd**. Zo vallen typefouten sneller op.

Een command is bedoeld voor een handeling; een query voor het opvragen van informatie. De declaratie alleen bewijst niet dat de implementatie zich correct gedraagt.

<!-- section: semantics -->
## Waarom controleert Rust nog extra regels?

Een manifest kan netjes geschreven zijn en toch inhoudelijk tegenstrijdig zijn. Daarom voert de **Rust-validator** aanvullende (*semantische*) controles uit.

Hij weigert onder andere:
- meerdere declaraties met dezelfde capability-ID;
- overlappende domeinen binnen hetzelfde manifest;
- aangeboden capabilities die niet onder een eigen domein vallen.

De simulator controleert daarnaast conflicten met **al geregistreerde modules** en of voor iedere aangeboden functie de exacte handler aanwezig is. Pas daarna wordt de registratie **atomisch** uitgevoerd: bij een fout blijft er geen half geregistreerde module achter.

<!-- section: fixtures -->
## Hoe testen we deze regels?

Onder [conformance/manifests](../../../conformance/manifests) staan voorbeeldbestanden, ook *fixtures* genoemd.

| Testcategorie | Verwachte uitkomst |
|---|---|
| `valid` | Zowel JSON Schema als Rust accepteert het manifest |
| `schema-invalid` | Beide validators weigeren de ongeldige structuur |
| `semantic-invalid` | JSON Schema accepteert de structuur, maar Rust weigert de inhoud |

Beide validators krijgen dezelfde voorbeelden. Zo wordt sneller zichtbaar wanneer hun gedrag niet meer overeenkomt met de afgesproken regels.

<!-- section: generator -->
## Moet ik zo'n manifest helemaal zelf schrijven?

Nee. De **Ivy CLI** kan een startstructuur genereren en valideren:

```bash
cargo run -p ivy-cli -- new /tmp/status-module --id status-module --namespace community.example.status_module
cargo run -p ivy-cli -- validate /tmp/status-module/module.yaml
```

De generator maakt alleen manifests die de canonieke validator accepteert. De gegenereerde bestanden zijn **ontwikkelstartpunten**, geen installeerbare Ivy-productiepakketten.

Lees ook de [Developer Kit-handleiding](developer-kit.md) en de uitleg over [Capabilities](../concepts/capabilities.md).
