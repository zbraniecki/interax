# Interax

Interax is a unified communication framework spanning from in-device IPC to cross-device mesh networking and internet-scale distributed systems.

## Vision

Interax is designed as the foundational communication fabric for **Constellation OS** - a distributed mesh operating system built for continuous interaction sessions that span across devices: wearables, wall displays, TVs, per-room smart devices (Nest/Echo-like), phones, tablets, and more.

The core paradigm is:

- **Offline-first**: Sessions and state persist locally, syncing when connectivity is available
- **Local mesh enabled**: Devices in proximity form ad-hoc networks for low-latency communication
- **Seamless scaling**: The same API abstracts in-process calls, cross-process IPC, local mesh RPC, and internet-scale communication

### The Communication Continuum

Interax provides a single programming model that scales across all communication boundaries:

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                           INTERAX COMMUNICATION FABRIC                          │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  IN-DEVICE (Binder IPC)       LOCAL MESH (mDNS/DNS-SD)       INTERNET (IP)     │
│  ──────────────────────       ────────────────────────       ─────────────     │
│                                                                                 │
│  ┌──────────────────┐         ┌──────────────────┐          ┌──────────────┐   │
│  │     Device 1     │         │     Device 1     │          │   Device 1   │   │
│  │  ┌────────────┐  │         │                  │          │              │   │
│  │  │ Process A  │  │         │  ┌────────────┐  │          │  ┌────────┐  │   │
│  │  │ (Endpoint) │  │         │  │  Endpoint  │  │          │  │Endpoint│  │   │
│  │  └─────┬──────┘  │         │  └──────┬─────┘  │          │  └───┬────┘  │   │
│  │        │         │         │         │        │          │      │       │   │
│  │    Binder IPC    │         │   Thread/WiFi    │          │   TCP/UDP    │   │
│  │        │         │         │      /BLE        │          │      │       │   │
│  │  ┌─────┴──────┐  │         │         │        │          │      ▼       │   │
│  │  │ Process B  │  │         │         ▼        │          │ ┌────────┐   │   │
│  │  │ (Endpoint) │  │         │  ┌────────────┐  │          │ │ Relay  │   │   │
│  │  └────────────┘  │         │  │  Device 2  │  │          │ │ Server │   │   │
│  │                  │         │  │ (Endpoint) │  │          │ │(Known) │   │   │
│  └──────────────────┘         │  └────────────┘  │          │ └───┬────┘   │   │
│                               │                  │          │     │        │   │
│                               └──────────────────┘          │     ▼        │   │
│                                                             │ ┌────────┐   │   │
│                                                             │ │ Remote │   │   │
│                                                             │ │ Device │   │   │
│                                                             │ └────────┘   │   │
│                                                             └──────────────┘   │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

## Architecture

### Interaction Model

Interax uses an interaction model inspired by the **Matter Protocol**, enabling convergence between in-device and cross-device communication. This model provides:

- **Endpoints**: Addressable entities that expose attributes and accept commands
- **Clusters**: Logical groupings of related functionality
- **Attributes**: State that can be read, written, and subscribed to
- **Commands**: Actions that can be invoked with request/response semantics
- **Events**: Asynchronous notifications pushed to subscribers

### Extended Fabric Topology

The topology extends Matter's fabric concept across multiple layers:

| Layer | Scope | Discovery | Transport | Example |
|-------|-------|-----------|-----------|---------|
| **Intra-device** | Process ↔ Process | Binder Service Manager | Binder IPC | App ↔ System Service |
| **Local mesh** | Device ↔ Device | mDNS/DNS-SD | Thread, WiFi, BLE | Phone ↔ Smart Speaker |
| **Internet** | Device ↔ Relay ↔ Device | Known relay IP | TCP/UDP, QUIC | Home ↔ Cloud ↔ Remote |

### Pluggable Transport Layer

The underlying protocols are abstracted behind a unified connectivity API:

**Discovery:**
- **In-device**: Binder Service Manager (hub)
- **Local mesh**: mDNS/DNS-SD for topology discovery
- **Internet**: Known relay server addresses

**Transport:**
- **In-device**: Android Binder, Unix sockets
- **Local mesh**: Thread, WiFi, BLE
- **Internet**: TCP/UDP sockets, QUIC, WebRTC

Applications interact with a single API regardless of where the target endpoint resides.

## Network Topology

### Nodes and Endpoints

- **Endpoint**: A logical communication target within a process (similar to a Binder interface)
- **Node**: A device containing one or more endpoints
- **Fabric**: A trust domain spanning multiple nodes with shared credentials

### Proxies

Proxies are optional intermediaries that provide value-added services:

- **Caching**: Store and serve frequently accessed attributes
- **Broadcasting**: Fan-out notifications to multiple subscribers
- **Routing**: Bridge between network segments or fabrics
- **Transformation**: Protocol translation, data format conversion
- **Access control**: Enforce policies at network boundaries

## Initial Building Blocks

The initial implementation operates within a single Android device:

### Hub (Service Manager)

A central registry modeled after Android's Binder Service Manager:

- Manages in-device endpoint topology
- Handles endpoint registration and discovery
- Routes connection requests
- Eventually scales to persistent hub managing local and global node topology

### Client Library

For applications that consume services:

- Connection management with automatic reconnection
- Subscription handling for attributes and events
- Request/response correlation
- Timeout and retry policies

### Server Library

For applications that expose services:

- Endpoint definition and registration
- Attribute storage with change notification
- Command handler registration
- Access control integration

### Proxy Library

For building intermediary applications:

- Register as a target endpoint
- Act as in-device intermediate
- Implement caching, broadcasting, or routing logic
- Chain with other proxies

### Monitoring Tools

- TUI-based monitoring application (`interax-tui-monitor`)
- Real-time view of endpoint topology
- Message tracing and debugging
- Performance metrics

## Security and Authentication

### In-Device Communication

- **Sandboxed environments**: Communication can be unencrypted when Android's process sandbox provides isolation
- **Sensitive data**: Optional encryption layer for high-security use cases
- **Permission gating**: Access controlled at endpoint, proxy, or hub level based on Android permissions

### Local Mesh Discovery and Access

Follows the Matter Protocol provisioning model:

- **Device commissioning**: Secure onboarding of new nodes to a fabric
- **PAKE (Password-Authenticated Key Exchange)**: Initial pairing without pre-shared secrets
- **CASE (Certificate Authenticated Session Establishment)**: Subsequent connections use fabric credentials
- **Access Control Lists**: Fine-grained control over which nodes can access which endpoints

### Internet Communication

Security model depends on the communication pattern:

| Pattern | Authentication | Encryption |
|---------|---------------|------------|
| **Proxy-relayed** | OAuth/OIDC with relay service | TLS 1.3 to proxy, E2E optional |
| **Peer-to-peer** | Fabric certificates, CASE | DTLS 1.3 / Noise Protocol |
| **Hybrid** | Relay for signaling, P2P for data | Mixed per-channel |

### Trust Hierarchy

```
┌──────────────────────────────────────────────────────────┐
│                     Root of Trust                         │
│              (Fabric Certificate Authority)               │
└─────────────────────────┬────────────────────────────────┘
                          │
          ┌───────────────┼───────────────┐
          ▼               ▼               ▼
    ┌───────────┐   ┌───────────┐   ┌───────────┐
    │  Node 1   │   │  Node 2   │   │  Node 3   │
    │   Cert    │   │   Cert    │   │   Cert    │
    └─────┬─────┘   └─────┬─────┘   └─────┬─────┘
          │               │               │
    ┌─────┴─────┐   ┌─────┴─────┐   ┌─────┴─────┐
    │ Endpoint  │   │ Endpoint  │   │ Endpoint  │
    │   ACLs    │   │   ACLs    │   │   ACLs    │
    └───────────┘   └───────────┘   └───────────┘
```

## Crates

| Crate | Description |
|-------|-------------|
| `interax-tui-fwk` | Async, event-driven TUI framework built on ratatui and tokio |
| `interax-tui-monitor` | TUI-based monitoring application for Interax topology |

## Roadmap

1. **Phase 1**: In-device IPC on Android Binder
   - Hub, client, server libraries
   - Basic monitoring tools

2. **Phase 2**: Local mesh networking
   - Thread/WiFi transport plugins
   - Matter-style commissioning
   - Multi-node topology

3. **Phase 3**: Internet connectivity
   - Relay server infrastructure
   - P2P with NAT traversal
   - Cross-fabric federation

4. **Phase 4**: Constellation OS integration
   - Session continuity across devices
   - Distributed state management
   - Unified interaction experience

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
