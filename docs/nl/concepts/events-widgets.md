<!-- doc-key: events-widgets -->
<!-- parity-version: 2 -->
<!-- sections: events,widgets,preview,status -->

# Events en widgets: wat gebeurt er en wat zie je?

**Nederlands** | [English](../../en/concepts/events-widgets.md)

Ivy-modules kunnen informatie over gebeurtenissen uitwisselen én informatie aan gebruikers tonen. Daarvoor gebruikt Ivy twee verschillende concepten:

- Een **event** vertelt dat er iets gebeurd is.
- Een **widget** laat informatie of bediening zien in een gebruikersinterface.

**Belangrijk:** geen van beide geeft een module automatisch toestemming om iets uit te voeren. Voor acties blijven de gewone capabilityregels gelden.

Denk aan slimme verlichting: een module meldt dat een lamp is ingeschakeld (**event**). In Prism zie je de toestand van de lamp en misschien een knop om haar uit te schakelen (**widget**). Die knop mag de lamp pas bedienen als de bijbehorende capabilityaanroep is toegestaan.

<!-- section: events -->
## Events: meldingen over wat al gebeurd is

Een event is een bericht over een feit uit het verleden, bijvoorbeeld: **“De woonkamerlamp is ingeschakeld.”** Het is niet hetzelfde als de opdracht **“Schakel de woonkamerlamp in.”** Die opdracht is een *command* (een soort capabilityaanroep).

Dit onderscheid voorkomt dat een gebeurtenis stiekem als bevoegdheid of opdracht wordt behandeld. **Een event omzeilt nooit capabilitygrants** (expliciete toegangsrechten).

### Wie regelt wat?

De private Ivy Core beheert de generieke infrastructuur voor events:
- **Registratie:** welke eventstromen er bestaan.
- **Duurzame aflevering (delivery):** het betrouwbaar verwerken en afleveren van berichten.
- **Bevestiging (acknowledgement):** vastleggen dat een bericht ontvangen of verwerkt is.
- **Opnieuw proberen (retry):** omgaan met bepaalde fouten bij aflevering.
- **Bewaartermijn (retention) en limieten:** begrenzen hoelang en hoeveel berichten worden bijgehouden.

De **domeinmodule** bepaalt wat haar events betekenen en welk gegevensformaat (*payloadschema*) ze gebruiken. De lampenmodule begrijpt bijvoorbeeld wat “lamp ingeschakeld” inhoudelijk betekent; Core hoeft geen verlichtingsexpert te zijn.

Dit beschrijft de architectuur, niet een volledig publiek beschikbare event-API.

<!-- section: widgets -->
## Widgets: de informatie en knoppen die je ziet

Een widget is een declaratieve beschrijving van een stukje gebruikersinterface. **Declaratief** betekent dat de module beschrijft *wat* moet worden getoond, terwijl **Prism** verantwoordelijk is voor de weergave.

Een widget kan bijvoorbeeld de naam en toestand van een lamp tonen, met een knop om de lamp uit te schakelen.

Maar **een zichtbare knop is geen toestemming**. Als de gebruiker erop drukt, wordt de actie uiteindelijk een normale capabilityaanroep. De gebruikelijke controles op identiteit, declaraties, eigendom en grants blijven van toepassing.

Dat betekent dat een widget er wel kan zijn, maar dat een actie toch geweigerd wordt wanneer de benodigde rechten ontbreken of ingetrokken zijn.

<!-- section: preview -->
## Zelf een widgetvoorbeeld bekijken

De publieke Developer Kit bevat een kleine previewtool. Die leest een **JSON-fixture** (een voorbeeldbestand met widgetgegevens) en maakt er een offline HTML-weergave van.

Voer dit vanuit de hoofdmap van de repository uit:

```bash
cargo run -p ivy-widget-preview -- examples/hello-clock/widget.json
```

Deze tool is nuttig om het voorbeeld te bekijken en te controleren hoe tekst in HTML terechtkomt. De preview **escapet HTML** in fixturetekst: speciale tekens worden veilig als tekst behandeld in plaats van als uitvoerbare HTML geïnterpreteerd.

De tool is **niet** de echte Prism-renderer en bewijst geen werkende productie-integratie met Core of events.

<!-- section: status -->
## Wat werkt al en wat is nog gepland?

| Onderdeel | Huidige publieke status |
|---|---|
| Kleine JSON-widgetfixture naar offline HTML omzetten | Geïmplementeerd in de previewtool |
| HTML-escaping van fixturetekst | Geïmplementeerd in de previewtool |
| Volledige productie-Prism-renderer | Niet publiek beschikbaar via deze Developer Kit |
| Publieke testomgeving voor events (*event harness*) | Gepland |
| Stabiele generatie van widgetberichten voor communicatie (*widget wire generation*) | Gepland |

**Kort gezegd:** events vertellen *wat er gebeurd is*, widgets tonen *wat je kunt zien of bedienen* en capabilities met grants bepalen *wat daadwerkelijk mag gebeuren*.

Lees ook [Capabilities en grants](capabilities.md) en de [architectuuruitleg](../architecture/overview.md) als je wilt weten hoe Ivy de toestemming controleert.
