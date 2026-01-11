# Interax

Interax is a framework for building applications that can communicate with each other.

## Overview

The initial implementation focuses on binder-based IPC for one-way and two-way communication, utilizing an interaction model similar to the Matter Protocol.

The architecture, topology, and API are designed to scale to cross-device IPC in a mesh model or with the use of proxies.

## Network Topology

The network is composed of:

- **Endpoints** - Processes that communicate with each other
- **Proxies** (optional) - Intermediate relays that may offer additional functionality such as:
  - Caching
  - Broadcasting
  - Message transformation
  - Access control

## Vision

Interax is designed as a prototype building block for a future **Constellation OS** design - a multi-device distributed mesh operating system.

## Crates

| Crate | Description |
|-------|-------------|
| `interax-tui-fwk` | Async, event-driven TUI framework built on ratatui and tokio |
| `interax-tui-monitor` | TUI-based monitoring application |

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
