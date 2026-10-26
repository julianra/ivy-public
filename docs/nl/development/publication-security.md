<!-- doc-key: publication-security -->
<!-- parity-version: 3 -->
<!-- sections: matrix,allowlist,layers,dependencies,generated,github,limits -->

# Publicatie-, security- en dependencyreview

**Nederlands** | [English](../../en/development/publication-security.md)

<!-- section: matrix -->
## Publicatiematrix

| Onderdeel | Publieke beslissing | Buildstatus | Licentiestatus |
|---|---|---|---|
| Core/Core UI | Alleen architectuur; geen broncode | Bewust niet beschikbaar | Proprietary |
| Bridge/Vault/Hooman/Flow | Alleen concepten in afwachting van afzonderlijke review | Niet in repository | Systeemlicentie wacht |
| Agenda/Pulse/BookMe | Alleen toepassingen; mogelijk later referentie | Niet in repository | Classificatie/licentie wacht |
| Rust/Dart Developer Kit | Nieuw geschreven onafhankelijke subset | Bouwt en test onafhankelijk | Apache-2.0 |
| Manifest-/protocontracten | Developergerichte subset | Schema- en semantische tests | Apache-2.0 |
| CLI/widgetpreview/voorbeelden | Publieke developmenttools | Bouwen en testen onafhankelijk | Apache-2.0 |
| Marketplace | Beleid en voorgesteld verificatierecord | Alleen documentatie | Juridische/productgoedkeuring wacht |
| Zakelijke/interne tooling | Wordt hier nooit geëxporteerd | Niet van toepassing | Private/intern |

<!-- section: allowlist -->
## Allowlist

Toegelaten top-levelinhoud is beperkt tot gereviewde docs, contracten, SDK,
tools, voorbeelden, conformance, media, scripts en GitHub-configuratie. Core,
Core UI, officiële module-implementaties, private runtime/mobile wire,
productiesigning, bedrijfsoperaties, interne roadmap/workerstate en
releaseartefacten staan op de denylist. Export is handmatig en commitgebonden; er
bestaat geen automatische private sync.

<!-- section: layers -->
## Securitylagen

CI combineert Gitleaks, patroonchecks van huidige tree en Git-historie, privacy-
en infrastructuurchecks, proprietary-pathchecks, expliciete top-levelallowlist,
media-inspectie, dependencylicentie-inventory, Rust/Dart-tests, manifestfixtures
en tweetalige documentatie-/linkvalidatie. Publicatiehistorie vereist bovendien
het ingestelde GitHub-noreplyadres voor elke author en committer; pull requests
controleren DCO 1.1-sign-off op elke voorgestelde commit.

<!-- section: dependencies -->
## Dependencies

Elke registrypackage moet een licentie-expressie melden. De inventory weigert
onbekende registrylicenties en rapporteert dependencywijzigingen voor menselijke
review. Een technische SPDX-inventory is geen juridische goedkeuring.

<!-- section: generated -->
## Gegenereerde bestanden

Alleen gereviewde lockfiles worden gecommit. Gegenereerde bindings vereisen vóór
toevoeging een gedocumenteerde, reproduceerbare generator. Developmentkeys,
signatures, binaries, packages en buildoutputs worden genegeerd of geweigerd.

<!-- section: github -->
## GitHub-controls

| Control | Status | Bewijs/actie |
|---|---|---|
| Rust-, Dart-, docs-, Gitleaks- en repository-safety-CI | `VERIFIED ACTIVE` | Exacte private commits voeren deze checks uit |
| Dependabot vulnerability alerts en automatische securityfixes | `VERIFIED ACTIVE` | GitHub-API geverifieerd; gewone versie-PR's blijven uit |
| CodeQL-workflow | `READY WHEN PUBLIC` | Gecommitte workflow activeert automatisch bij publieke visibility |
| Dependency Review-workflow | `READY WHEN PUBLIC` | Gecommitte pull-requestworkflow activeert automatisch wanneer publiek |
| Branch rules | `MANUAL GITHUB SETTING REQUIRED` | Private plan-API meldt onbeschikbaar; vereis reviews, CODEOWNERS, CI en blokkeer force-push/delete |
| Native secret scanning en push protection | `MANUAL GITHUB SETTING REQUIRED` | Geen actieve setting verifieerbaar zolang private |
| Private Vulnerability Reporting | `MANUAL GITHUB SETTING REQUIRED` | Gewenste route is gedocumenteerd, maar actieve setting kon niet worden bevestigd |

Deze readinessstatus geeft geen toestemming om visibility te wijzigen.

<!-- section: limits -->
## Grenzen

Automatische scans kunnen contextuele secrets, rechtenconflicten, misleidende
claims, gevoelige screenshots of nieuwe dataformaten missen. Groene automatisering
is noodzakelijk maar nooit voldoende: menselijke review van exacte launchcommit
en historie blijft verplicht.
