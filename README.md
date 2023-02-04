This project explores an idea: instances of an application (e.g. mobile, web, and desktop versions of a notetaking app) belonging to the same user can synchronize their state using [dumb](https://www.hyperorg.com/misc/stupidnet.html) channels (e.g. shared folders) instead of centralized servers, without requiring to be permanently connected.

The sample implementation is a simple bookmark manager written in Rust, loosely following CQRS, Event Sourcing, and Ports/Adapters approaches.

## Status

Basic logic in place. Still lacks a physical log implementation.

## Mode of operation

1. In response to user actions, an application instance adds a business event to a log (e.g. `BookmarkCreatedEvent`).
   - The log is the _local_ source of truth and the application state is a derivation of it.
2. The event log is serialized to a disk folder, one file per event.
3. Each instance makes its log available to other instances whenever possible by sharing the folder with a synchronization tool (e.g. [Syncthing](https://syncthing.net/)) or even a simple USB stick.
4. Each instance computes its own interpretation of the collective history by replaying events from all logs.
5. Like in Event Sourcing, events are immutable, but contrary to it, new events can appear at any point in the timeline, rather than just its tail (e.g. an instance reconnects and shares events that were stored to the log while offline). This is similar to the real world, where previously unknown facts become known and lead to different "conclusions" (a different application state).

##

Synchronization strategies such as [CRDT](https://en.wikipedia.org/wiki/Conflict-free_replicated_data_type) strive to provide the illusion real-time concurrent editing.
