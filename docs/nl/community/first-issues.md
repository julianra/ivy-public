<!-- doc-key: first-issues -->
<!-- parity-version: 2 -->
<!-- sections: rules,good-first,help-wanted,done -->

# Help Ivy Public verbeteren

**Nederlands** | [English](../../en/community/first-issues.md)

Wil je bijdragen, maar weet je niet waar te beginnen? Op deze pagina staan concrete taken voor de **publieke Ivy Developer Kit**. Je hoeft de private Ivy Core niet te kennen. Je mag klein beginnen: een duidelijke foutmelding is ook waardevol.

Bij elke taak lees je **waarom het nuttig is**, **wat je kunt maken** en **wanneer het af is**. Liever eerst overleggen? Laat een reactie achter bij het bijbehorende GitHub-issue.

<!-- section: rules -->
## Eerst even dit

- Werk alleen met openbare Developer Kit-code, voorbeeldgegevens en publieke contracten. Voeg **nooit** private Core-broncode, secrets, persoonsgegevens of interne operationele gegevens toe.
- Houd je wijziging klein en gericht. Bespreek de aanpak in het issue voordat je veel code schrijft.
- Werk Nederlandstalige én Engelstalige documentatie samen bij wanneer je die wijzigt.
- De GitHub-issues beschrijven en volgen de afzonderlijke taken; deze pagina is het begrijpelijke overzicht.

<!-- section: good-first -->
## 🌱 Goed om mee te beginnen

### Maak manifestfouten begrijpelijker

**Waarom?** Een ontwikkelaar die een fout maakt in zijn modulebeschrijving moet snel begrijpen *wat* verkeerd is en *hoe* hij het oplost.

**Wat vragen we?** Verbeter de foutmeldingen bij **twee bestaande ongeldige manifestvoorbeelden**. Leg duidelijk uit welke regel wordt overtreden, zonder te veranderen welke manifesten Ivy wel of niet accepteert.

**Klaar wanneer:** de twee foutmeldingen duidelijker zijn, relevante tests het gedrag aantonen en voorheen geldige manifesten geldig blijven.

<!-- section: help-wanted -->
## 🛠️ Help wanted — voor wie graag wat dieper duikt

### Maak een eerste prototype voor Go of TypeScript

**Waarom?** Meer ontwikkelaars moeten Ivy's publieke contracten kunnen gebruiken in hun favoriete programmeertaal.

**Wat vragen we?** Maak een beperkt prototype dat types afleidt uit de publieke developer-protobufdeclaraties. **Geen volledige Ivy-runtime** en geen belofte dat deze taal al productieklaar ondersteund wordt.

**Klaar wanneer:** het prototype reproduceerbaar is, voorbeeldgebruik bevat en zijn beperkingen helder uitlegt.

### Laat de simulator expres en voorspelbaar fouten geven

**Waarom?** Goede software moet ook correct reageren als een provider een fout teruggeeft.

**Wat vragen we?** Voeg een manier toe om vooraf gekozen providerfouten steeds op dezelfde manier te simuleren. Zo kunnen ontwikkelaars de foutafhandeling herhaalbaar testen.

**Klaar wanneer:** tests dezelfde foutscenario's betrouwbaar kunnen herhalen en normaal simulatorgedrag behouden blijft.

### Denk mee over het testen van events

**Waarom?** Modules wisselen gebeurtenissen uit, zoals ‘een apparaat veranderde van toestand’. Ontwikkelaars moeten kunnen nagaan of die gebeurtenissen juist worden verwerkt.

**Wat vragen we?** Schrijf een **RFC (ontwerpvoorstel)** voor een publieke testomgeving waarin ontwikkelaars events kunnen aanbieden en de verwachte resultaten controleren. Dit is **onderzoek en ontwerp**, nog geen opdracht om de hele omgeving te bouwen.

**Klaar wanneer:** het voorstel voorbeelden, grenzen, foutgevallen, alternatieven en open vragen bevat, zodat we het samen kunnen beoordelen.

### Onderzoek of widgets voor iedereen bruikbaar zijn

**Waarom?** Een toekomstige widgetpreview moet ook te gebruiken zijn met een toetsenbord of schermlezer.

**Wat vragen we?** Onderzoek de toegankelijkheidseisen en de huidige preview, noteer concrete problemen en stel haalbare verbeteringen voor.

**Klaar wanneer:** er een controleerbaar verslag is met bevindingen, voorbeelden en aanbevolen vervolgstappen. Niet alle verbeteringen hoeven in deze taak al gebouwd te worden.

<!-- section: done -->
## Wanneer is een bijdrage klaar?

- Het afgesproken resultaat is opgeleverd en begrijpelijk beschreven.
- **Bij codewijzigingen:** relevante positieve en negatieve tests zijn toegevoegd of bijgewerkt en `./scripts/check.sh` slaagt.
- **Bij onderzoeks- of RFC-taken:** er is een bruikbaar, controleerbaar verslag of voorstel; nieuwe code of tests zijn alleen nodig als de taak dat vraagt.
- Gewijzigde mensgerichte documentatie is in het Nederlands en Engels bijgewerkt.
- Bronnen en licenties van hergebruikt materiaal zijn vermeld.
- Er zijn geen nieuwe afhankelijkheden van de private Ivy Core toegevoegd.

Lees ook de [bijdragehandleiding](contributing.md) voor het openen van een issue of pull request.
