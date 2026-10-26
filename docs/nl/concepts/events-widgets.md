<!-- doc-key: events-widgets -->
<!-- parity-version: 2 -->
<!-- sections: events,widgets,preview,status -->

# Events en widgets

**Nederlands** | [English](../../en/concepts/events-widgets.md)

<!-- section: events -->
## Events zijn feiten

Events melden iets dat al gebeurd is. Het zijn geen commands en ze kunnen
capabilitygrants niet omzeilen. Private Core beheert generieke registratie,
duurzame delivery, acknowledgement, retry, retention en begrenzing; domeinmodules
bezitten de betekenis en payloadschema's.

<!-- section: widgets -->
## Widgets zijn presentatie

Widgets zijn declaratieve presentatieresources die Prism rendert. Zichtbaarheid
geeft geen bevoegdheid. Een widgetactie wordt uiteindelijk een capabilityaanroep
onder het normale beleid.

<!-- section: preview -->
## Publieke previewtool

De meegeleverde tool rendert een kleine JSON-fixture veilig naar offline HTML:

```bash
cargo run -p ivy-widget-preview -- examples/hello-clock/widget.json
```

<!-- section: status -->
## Status

De publieke preview is geïmplementeerd en escapet HTML in fixturetekst. Het is
niet de production Prism-renderer. Een publieke eventharness en stabiele
widgetwiregeneratie zijn gepland.
