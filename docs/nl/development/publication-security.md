<!-- doc-key: publication-security -->
<!-- parity-version: 3 -->
<!-- sections: matrix,allowlist,layers,dependencies,generated,github,limits -->

# Ivy Public veilig publiceren

**Nederlands** | [English](../../en/development/publication-security.md)

## Waarom bestaat deze controle?

Ivy Public is bedoeld voor externe ontwikkelaars. De private Ivy Core en interne bedrijfsinformatie mogen daarbij **niet per ongeluk op GitHub verschijnen**.

Daarom controleren we vóór publicatie niet alleen of de code werkt, maar ook **wat er wordt vrijgegeven, van wie de code is, of er gevoelige gegevens in staan en of de GitHub-beveiliging goed is ingesteld**.

**Belangrijk:** een groene CI-check betekent niet automatisch dat de repository veilig gepubliceerd kan worden. De eigenaar moet de exacte versie én de Gitgeschiedenis eerst beoordelen en de publicatie uitdrukkelijk goedkeuren.

<!-- section: matrix -->
## Wat mag er publiek worden?

De volgende tabel maakt onderscheid tussen publieke ontwikkelcode en onderdelen die privé moeten blijven.

| Onderdeel | Wat mag publiek? | Technische status | Licentie |
|---|---|---|---|
| Core / Core UI | Alleen architectuuruitleg, geen broncode | Bewust niet meegeleverd | Proprietary |
| Bridge / Vault / Hooman / Flow | Alleen concepten, na afzonderlijke review | Niet in deze repository | Systeemlicentie nog te bepalen |
| Agenda / Pulse / BookMe | Uitleg en toepassingsvoorbeelden; eventueel later referentiemateriaal | Niet in deze repository | Classificatie/licentie nog te bepalen |
| Rust/Dart Developer Kit | Onafhankelijk geschreven publieke ontwikkeltools | Bouwt en test zelfstandig | Apache-2.0 |
| Manifest- en protobufcontracten | Developergerichte publieke subset | Schema- en semantische tests | Apache-2.0 |
| CLI, widgetpreview en voorbeelden | Publieke hulpmiddelen om te ontwikkelen en testen | Bouwen en testen zelfstandig | Apache-2.0 |
| Marketplace | Beleid en voorgesteld verificatierecord | Alleen documentatie | Juridische/productgoedkeuring ontbreekt nog |
| Zakelijke en interne tooling | Niets | Niet van toepassing | Privé/intern |

<!-- section: allowlist -->
## Hoe voorkomen we dat private bestanden meekomen?

We gebruiken twee lijsten:

- Een **allowlist** van soorten inhoud die na review in de publieke repository mogen staan: documentatie, contracten, SDK, tools, voorbeelden, conformance, media, scripts en GitHub-configuratie.
- Een **denylist** van onderdelen die daar niet thuishoren: Core, Core UI, officiële module-implementaties, private runtime- en mobiele communicatiecontracten, productieondertekening, bedrijfsoperaties, interne roadmap en workerstatus, en releaseartefacten.

De export wordt **handmatig gecontroleerd en aan een exacte commit gekoppeld**. Er bestaat geen automatische synchronisatie van de private repository naar Ivy Public.

<!-- section: layers -->
## Welke controles beschermen de repository?

Verschillende controles vullen elkaar aan:

- **Gitleaks** zoekt mogelijke sleutels en andere geheimen (*secrets*).
- **Bestands- en geschiedeniscontroles** zoeken verboden paden en patronen, niet alleen in de huidige bestanden maar ook in de Gitgeschiedenis.
- **Privacy- en infrastructuurcontroles** zoeken gevoelige persoonsgegevens of operationele details.
- **Top-level allowlist en mediacontrole** controleren welke mappen en afbeeldingen zijn opgenomen.
- **Dependencylicentie-inventory** registreert licenties van externe software.
- **Rust-, Dart- en manifesttests** controleren technische werking.
- **Tweetalige documentatie- en linkchecks** controleren of verwijzingen werken en beide talen overeenkomen.

Voor de publicatiegeschiedenis moeten zowel auteurs als committers het ingestelde **GitHub noreply-adres** gebruiken. Bij Pull Requests moet iedere voorgestelde commit ook een **DCO 1.1 Signed-off-by** hebben: een verklaring dat de bijdrager het recht heeft de code in te dienen.

<!-- section: dependencies -->
## Hoe controleren we gebruikte bibliotheken?

Veel software gebruikt externe pakketten (*dependencies*). Die hebben elk hun eigen licentievoorwaarden.

Ieder pakket uit een package registry moet een licentie-expressie melden. De inventory weigert onbekende registrylicenties en meldt wijzigingen voor menselijke controle.

Een **SPDX-inventory** is een technische lijst van licenties. Zo'n lijst is nuttig, maar vormt **geen juridische goedkeuring** van alle hergebruikte code.

<!-- section: generated -->
## Wat doen we met gegenereerde bestanden en sleutels?

Alleen beoordeelde lockfiles mogen worden gecommit. Voor automatisch gegenereerde bindings moet eerst duidelijk zijn **met welke generator en stappen je ze reproduceerbaar maakt**.

Developmentkeys, handtekeningen, binaries, packages en buildresultaten worden genegeerd of door de veiligheidscontroles geweigerd. Ze horen niet zomaar in de publieke broncodegeschiedenis terecht te komen.

<!-- section: github -->
## Welke GitHub-beveiliging is voorbereid?

De onderstaande statussen zijn de vastgelegde projectstatussen. **Ze bewijzen niet dat iedere instelling vandaag opnieuw live is gecontroleerd.** Controleer dit vóór de publieke lancering.

| Controle | Gedocumenteerde status | Wat betekent dat? |
|---|---|---|
| Rust-, Dart-, docs-, Gitleaks- en repository-safety-CI | `VERIFIED ACTIVE` | Eerder op exacte private commits uitgevoerd |
| Dependabot vulnerability alerts en automatische securityfixes | `VERIFIED ACTIVE` | Eerder via GitHub-API bevestigd; gewone versie-update-PR's staan uit |
| CodeQL | `READY WHEN PUBLIC` | Workflow aanwezig; wordt volgens configuratie pas op publieke repository uitgevoerd |
| Dependency Review | `READY WHEN PUBLIC` | PR-workflow voorbereid, maar nu niet actief op private repository |
| Branch rules | `MANUAL GITHUB SETTING REQUIRED` | Controleer na activering of `main` echt beschermd is; vereis passende reviews, CODEOWNERS en CI, verbied force pushes en verwijderen |
| GitHub secret scanning en push protection | `MANUAL GITHUB SETTING REQUIRED` | Controleer beschikbaarheid en actuele activering in GitHub Settings |
| Private Vulnerability Reporting | `MANUAL GITHUB SETTING REQUIRED` | Controleer of meldingen daadwerkelijk privé kunnen worden ingestuurd |

**Let op bij branch rules:** er is inmiddels een ruleset voor `main` voorbereid, maar zolang GitHub de regels op deze private repository wegens het abonnement niet afdwingt, is dat **nog geen bewezen branchbescherming**. Na een eventuele publieke lancering moeten de statuschecks en blokkering met een echte test-PR worden geverifieerd.

Deze lijst **geeft geen toestemming om de zichtbaarheid van de repository te veranderen**.

<!-- section: limits -->
## Waarom blijft een menselijke laatste controle nodig?

Automatische scans kunnen bijvoorbeeld een geheim missen dat geen herkenbare sleutelvorm heeft, een licentieconflict verkeerd inschatten of een gevoelige screenshot niet herkennen.

Daarom hoort vóór publicatie een menselijke review van **de exacte launchcommit, alle relevante bestanden en de volledige Gitgeschiedenis**. Controleer ook of de publieke beschrijvingen geen functionaliteit beloven die nog niet beschikbaar is.

**De eindregel:** pas publiceren na expliciete goedkeuring. Een geslaagde test is nodig, maar op zichzelf nooit voldoende.
