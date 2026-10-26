<!-- doc-key: first-issues -->
<!-- parity-version: 2 -->
<!-- sections: rules,good-first,help-wanted,done -->

# Voorbereide bijdragetaken

**Nederlands** | [English](../../en/community/first-issues.md)

<!-- section: rules -->
## Scoperegels

Elke taak moet onafhankelijk van proprietary Core blijven, indien nodig de
tweetalige documentatie bijwerken en de volledige repositorycheck doorstaan. De
GitHub-issues verwijzen terug naar dit document.

<!-- section: good-first -->
## Good first issues

1. Voeg grensfixtures toe voor cijfers en underscores in namespacesegmenten.
2. Voeg een toegankelijke tweede widgetfixture toe en test de geëscapete HTML-output.
3. Verbeter foutmeldingen voor twee ongeldige manifestfixtures zonder geaccepteerde input te wijzigen.
4. Voeg alt-tekstvalidatie toe voor toekomstige SVG-diagrammen.

<!-- section: help-wanted -->
## Help wanted

1. Ontwerp een Go- of TypeScript-prototype van contracttypes uit de publieke
   developerproto, zonder runtimeclaims.
2. Voeg deterministische foutinjectie voor providererrors aan de simulator toe.
3. Stel via een RFC een publieke event-testharness voor.
4. Audit keyboard- en screen-readervereisten voor de toekomstige widgetpreview.

<!-- section: done -->
## Definition of done

Een taak vereist tests, overeenkomende Engelse/Nederlandse documentatie, geen
nieuwe private dependency, gedocumenteerde licentieherkomst en een schone
`./scripts/check.sh`-uitkomst.
