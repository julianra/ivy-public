# Conformance / Conformiteit

## English

### What is this for?

Ivy is a modular software platform where developers can build modules that describe what they offer and what they need.

But how can we verify that these modules follow Ivy's rules?

**Conformance tests** are automated checks that verify whether specific module descriptions and behaviors comply with Ivy's technical agreements.

For example, imagine developing a module that controls smart lights. The tests help verify whether the module correctly describes itself, respects domain ownership, and follows the tested rules for permissions and registration.

These tests cover:

- **Identity:** Does the module have a valid identity and manifest?
- **Functionality:** Does the module correctly declare its capabilities?
- **Ownership:** Does the module respect the boundaries of other modules?
- **Permissions:** Are access rights properly enforced and revoked?
- **Reliability:** What happens when registration fails or a module becomes unavailable?
- **Security-related rules:** Are signing requirements and widget escaping handled correctly?

### What do these tests prove?

Passing the tests means that the tested inputs and behaviors satisfy the specific rules covered by the test suite.

It does **not** guarantee that an entire module is bug-free, completely secure, or production-ready.

Passing conformance tests is also **not equivalent to Ivy Marketplace certification**.

### Technical details

The shared manifest test examples, called *fixtures*, are divided into three categories:

| Category | Expected result |
| --- | --- |
| `valid` | Both JSON Schema and the Rust validator accept the manifest. |
| `schema-invalid` | Both validators reject the manifest because of structural errors. |
| `semantic-invalid` | JSON Schema accepts the structure, but the Rust validator rejects its meaning. |

Rust tests apply both validators to the same fixture files.

Additional tests cover segment-based domain ownership, atomic registration, exact handlers, separate grants, declaration enforcement, revocation, availability, signing, and widget escaping.

### Running the tests

From the repository root:

```bash
./scripts/check.sh
```

---

## Nederlands

### Waarvoor dient dit?

Ivy is een modulair softwareplatform waarop ontwikkelaars eigen modules kunnen bouwen en bestaande functionaliteit met elkaar kunnen verbinden.

Maar hoe weten we of die modules zich aan de afspraken van Ivy houden?

**Conformance-tests** zijn automatische controles die nagaan of bepaalde beschrijvingen en gedragingen van modules voldoen aan de technische regels van Ivy.

Stel bijvoorbeeld dat je een module ontwikkelt waarmee je slimme lampen kunt bedienen.

De tests helpen controleren of de module zichzelf correct beschrijft, geen functionaliteit van andere modules onterecht opeist en de geteste regels voor rechten en registratie naleeft.

Deze tests controleren onder andere:

- **Identiteit:** Heeft de module een geldige identificatie en een correct manifest?
- **Functionaliteit:** Beschrijft de module correct welke mogelijkheden ze aanbiedt?
- **Eigendom:** Respecteert de module de grenzen van andere modules?
- **Toestemmingen:** Worden toegangsrechten correct gecontroleerd en ingetrokken?
- **Betrouwbaarheid:** Wat gebeurt er wanneer een registratie mislukt of een module niet beschikbaar is?
- **Beveiligingsregels:** Worden regels rond ondertekening en het veilig verwerken van widgetinhoud nageleefd?

### Wat bewijzen deze tests?

Wanneer de tests slagen, betekent dit dat de geteste invoer en gedragingen voldoen aan de specifieke regels die de tests controleren.

Het betekent **niet** dat een volledige module gegarandeerd foutloos, volledig veilig of geschikt voor productie is.

Een geslaagde Conformance Test betekent ook **niet** dat een module automatisch wordt goedgekeurd voor de officiële Ivy Marketplace.

### Technische uitleg

De gedeelde voorbeelden van manifesten, ook *fixtures* genoemd, zijn verdeeld in drie categorieën:

| Categorie | Verwacht resultaat |
| --- | --- |
| `valid` | Zowel JSON Schema als de Rust-validator aanvaarden het manifest. |
| `schema-invalid` | Beide validators weigeren het manifest omdat de structuur ongeldig is. |
| `semantic-invalid` | JSON Schema aanvaardt de structuur, maar de Rust-validator weigert de inhoudelijke betekenis. |

De Rust-tests voeren beide validators uit op dezelfde voorbeeldbestanden.

Daarnaast controleert de testsuite domeineigendom op basis van segmenten, atomische registratie, exacte handlers, afzonderlijke toegangsrechten, declaratiehandhaving, intrekking, beschikbaarheid, ondertekening en widgetescaping.

### Tests uitvoeren

Voer vanuit de hoofdmap van de repository het volgende commando uit:

```bash
./scripts/check.sh
```
