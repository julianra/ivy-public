<!-- doc-key: events-widgets -->
<!-- parity-version: 2 -->
<!-- sections: events,widgets,preview,status -->

# Events and widgets: what happened, and what do you see?

[Nederlands](../../nl/concepts/events-widgets.md) | **English**

Ivy modules can exchange information about events and present information to users. Ivy separates these into two concepts:

- An **event** reports that something happened.
- A **widget** displays information or controls in a user interface.

**Important:** neither automatically gives a module permission to perform an action. Normal capability rules still apply.

Imagine smart lighting: a module reports that a light was switched on (**event**). Prism displays its status and perhaps a button to turn it off (**widget**). That button may control the light only if the corresponding capability invocation is authorized.

<!-- section: events -->
## Events: reports of things that already happened

An event is a message about a past fact, such as **“The living-room light was switched on.”** This differs from **“Switch on the living-room light.”** The latter is a *command* (a kind of capability invocation).

Keeping them separate prevents an event from silently acting as an instruction or permission. **Events never bypass capability grants** (explicit access permissions).

### Who is responsible for what?

The private Ivy Core manages generic event infrastructure:
- **Registration:** which event streams exist.
- **Durable delivery:** reliably processing and delivering messages.
- **Acknowledgement:** recording that a message was received or handled.
- **Retry:** handling certain delivery failures.
- **Retention and bounds:** limiting how long and how many messages are kept.

The **domain module** defines what its events mean and their data format (*payload schema*). For instance, the lighting module knows what “light switched on” means; Core does not need to understand lighting.

This describes the architecture, not a fully available public event API.

<!-- section: widgets -->
## Widgets: the information and controls you see

A widget is a declarative description of part of the user interface. **Declarative** means a module describes *what* to display, while **Prism** is responsible for rendering it.

For example, a widget may show a light's name and current state, along with a button to turn it off.

But **a visible button is not permission**. Pressing it eventually becomes a normal capability invocation. The usual checks for identity, declarations, ownership and grants still apply.

A widget can therefore appear on screen while its action is still denied if the required permission is missing or revoked.

<!-- section: preview -->
## Try a widget example yourself

The public Developer Kit includes a small preview tool. It reads a **JSON fixture** (an example file with widget data) and produces an offline HTML preview.

Run this from the repository root:

```bash
cargo run -p ivy-widget-preview -- examples/hello-clock/widget.json
```

This is useful for viewing the example and checking how text is displayed in HTML. The preview **HTML-escapes** fixture text: special characters are treated safely as text instead of executable HTML.

The tool is **not** the real Prism renderer and does not demonstrate working production integration with Core or events.

<!-- section: status -->
## What works today and what is planned?

| Component | Current public status |
|---|---|
| Render a small JSON widget fixture into offline HTML | Implemented in the preview tool |
| HTML escaping of fixture text | Implemented in the preview tool |
| Full production Prism renderer | Not publicly available through this Developer Kit |
| Public event test harness | Planned |
| Stable widget message generation for communication (*widget wire generation*) | Planned |

**In short:** events tell you *what happened*, widgets show *what you can see or control*, and capabilities with grants determine *what is actually allowed*.

See also [Capabilities and grants](capabilities.md) and the [architecture overview](../architecture/overview.md) for how Ivy checks permissions.
