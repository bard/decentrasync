This project explores:

1. state synchronization among intermittently-connected instances of an app (e.g. mobile and desktop versions of a calendar) through [dumb](https://www.hyperorg.com/misc/stupidnet.html), resilient channels instead of centralized servers;
2. from-scratch implementation of [CQRS](https://martinfowler.com/bliki/CQRS.html), [Event Sourcing](https://martinfowler.com/eaaDev/EventSourcing.html), and the [Hexagonal (aka Ports & Adapters) architecture](https://alistair.cockburn.us/hexagonal-architecture/) in Rust.

The sample implementation is a basic bookmark manager. To launch:

```sh
$ cargo run
```

Then open `http://localhost:9111`.

## Status

Work in progress.

## Synchronization as interpretation of history

In Event Sourcing, individual events are immutable, and the representation of history (the event log) changes linearly, in parallel with time.

In this experiment, individual events are immutable, but not necessarily known: some of them may have been recorded in the past by another instance that is still offline. The representation of history isn't definitive, and may be amended by taking into account previously unknown events, much like in human history. Rather than a derivation of a perfectly known history, application state is an interpretation of a partially unknown one.

How this may look like in practice:

1. In response to user actions, an app instance adds an event to a log (e.g. `BookmarkCreated`).
2. The event log is serialized to a disk folder, one file per event.
3. When possible, each instance makes its log available to other instances via e.g. [Syncthing](https://syncthing.net/)), Dropbox, or even a USB stick.
4. Each instance computes its own interpretation of the thus-far known history by replaying events from all logs.
