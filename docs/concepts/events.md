# Events

Events communicate facts that already occurred. They do not bypass capability
grants and must not be used as hidden commands.

Production Ivy owns generic registration, durable delivery, acknowledgement,
retry, retention and bounds. Domain modules own event meaning and payload
schemas. The public Developer Kit does not yet include an event simulator.
