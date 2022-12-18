This project explores an idea: decentralized synchronization of state among instances of the same application (e.g mobile and desktop versions of a notetaking app) by relying on "dumb" channels only (e.g. filesystem folders shared over the network) and supporting disconnected operation.

The sample implementation is in Rust and loosely follows the CQRS, Event Sourcing, and Ports/Adapters approaches.

## Status

Half a prototype. "Physical" event logs not added yet.

## Mode of operation

1. State is stored as event logs, one per application instance.
2. An event log is serialized to a folder, one event per file.
3. Each instance exposes its log to other instances by sharing the log folder, using a service like Dropbox or a tool like Syncthing.
4. Each instance computes its own "interpretation" of history by replaying events from all logs.
   - Alternatively, each instance "imports" events from other logs, adding suitable metadata to differentiate sources.
5. As in Event Sourcing, events are immutable, but contrary to it, new events can appear in the history. Application state is an interpretation of history and thus can change when previously unknown "facts" become known (e.g. an instance reconnects and share events generated while offline).
