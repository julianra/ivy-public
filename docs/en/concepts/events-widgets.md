<!-- doc-key: events-widgets -->
<!-- parity-version: 2 -->
<!-- sections: events,widgets,preview,status -->

# Events and widgets

[Nederlands](../../nl/concepts/events-widgets.md) | **English**

<!-- section: events -->
## Events are facts

Events report something that already happened. They are not commands and cannot
bypass capability grants. Private Core owns generic registration, durable
delivery, acknowledgement, retry, retention and bounds; domain modules own event
meaning and payload schemas.

<!-- section: widgets -->
## Widgets are presentation

Widgets are declarative presentation resources rendered by Prism. Visibility
does not grant authority. A widget action ultimately maps to a capability
invocation under normal policy.

<!-- section: preview -->
## Public preview tool

The included tool safely renders a small JSON fixture to offline HTML:

```bash
cargo run -p ivy-widget-preview -- examples/hello-clock/widget.json
```

<!-- section: status -->
## Status

The public preview is implemented and HTML-escapes fixture text. It is not the
production Prism renderer. A public event harness and stable widget wire
generation are planned.
