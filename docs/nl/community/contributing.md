<!-- doc-key: contributing -->
<!-- parity-version: 2 -->
<!-- sections: first,issues,prs,rfc,modules,marketplace,licensing,rewards,bilingual -->

# Bijdragen

**Nederlands** | [English](../../en/community/contributing.md)

<!-- section: first -->
## Eerste bijdrage

Kies een begrensd issue, plaats vóór substantieel werk een reactie, maak een
gerichte branch, draai `./scripts/check.sh` en leg positief en negatief gedrag uit
in de pull request. Voeg nooit private Ivy-broncode, credentials, persoonsgegevens
of operationele informatie toe.

<!-- section: issues -->
## Issues melden

Gebruik een reproduceerbaar Developer Kit-bugrapport voor codefouten en Q&A
Discussions voor vragen. Vermeld revision, minimale commando's, verwacht/werkelijk
gedrag en omgeving. Meld kwetsbaarheden privé zodra het securitycontact is
ingesteld; publiceer ze nooit in een openbaar issue.

<!-- section: prs -->
## Pull requests

Behandel één onderwerp per PR. Voeg tests toe, documenteer contractimpact, meld
bronnen en licenties van derden en werk zowel de Engelse als Nederlandse publieke
documentatie bij. Dezelfde auteur keurt een architectuur- of securitygevoelige
wijziging niet zelf goed.

<!-- section: rfc -->
## RFC's

Wijzigingen aan capabilities, authority, trust, compatibiliteit, packages of
Marketplace vereisen een RFC. Beschrijf probleem, ownershipgrens, voorgesteld
contract, failure- en retrygedrag, migratie, rollback en verworpen alternatieven.

<!-- section: modules -->
## Modules ontwikkelen en voorstellen

Definieer één begrensd domein, vermeld provided en required capabilities,
beschrijf wat de module niet bezit, voeg conformancefixtures toe en gebruik het
moduleproposal-template. Bewaar een onafhankelijke module in de repository van de
auteur tenzij ANWOR ze expliciet als gedeelde referentiecode aanvaardt.

<!-- section: marketplace -->
## Marketplaceverificatie

Marketplaceopname is een afzonderlijke review van exacte module-identiteit,
versie en digest. `ANWOR Official` en `ANWOR Verified` dragen geen eigendom over
en gelden niet automatisch voor wijzigingen of latere releases.

<!-- section: licensing -->
## Licenties en eigendom

Externe auteurs behouden rechten op originele onafhankelijke modules en kiezen
gratis, commerciële, proprietary of open-sourcedistributie binnen hun rechten.
De licentie van de gedeelde Developer Kit is nog niet goedgekeurd; neem niet aan
dat Apache-2.0 al verleend is.

<!-- section: rewards -->
## Beloningen

ANWOR kan afzonderlijk erkenning, credits, betaling of een andere beloning
afspreken. Er is geen automatisch recht en geen bijdrage belooft vergoeding.

<!-- section: bilingual -->
## Regel voor tweetalige documentatie

Elke wijziging aan mensgerichte publieke documentatie moet in dezelfde PR het
overeenkomstige `en`- en `nl`-bestand bijwerken, paritymetadata en sectie-ID's
gelijk houden en uitvoerbare codeblokken behouden. CI weigert ontbrekende
tegenhangers, verschillende secties, commandodrift en kapotte interne links.
