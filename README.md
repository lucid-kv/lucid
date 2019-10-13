# About Lucid ᵏᵛ

High performance and distributed KV store accessible through an HTTP API. Written in Rust. 🦀

[![Build Status](https://travis-ci.com/clintnetwork/lucid.svg)](https://travis-ci.com/clintnetwork/lucid)
[![Made with Rust](https://img.shields.io/badge/Made%20With-Rust-dea584)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/clintnetwork/lucid/blob/master/LICENSE.md)
[![Twitter](https://img.shields.io/twitter/follow/clint_network.svg?style=social)](https://twitter.com/intent/follow?screen_name=clint_network)

## Introduction

Lucid is currently in an embryonic state but we wish to achieve a fast, secure and distributed key-value store accessible through an HTTP API, we also want to propose persistence, encryption, WebSocket streaming, replication and a lot of features.

## Works Progress

<ins>__**Warning: Empty project for now, the development is ensured in the [development](https://github.com/clintnetwork/lucid/tree/development) branch.**__</ins>

- [x] Minimum Viable Product (MVP)
  - [x] Initialization process
  - [x] Configuration files handling
  - [x] JWT token Issuing
  - [ ] HTTPS Support
  - [ ] Rest API
     - [ ] JWT Authentication
- [ ] WebUI in VueJS
- [ ] Persistence
- [ ] Encryption on the Fly (AES-256)
- [ ] Access Control List (ACL)
- [ ] WebSocket or Event Source (SSE)

## Some Use Cases

- Private Keys Storing (for a wallet by example)
- IoT: collect and save statistics data
- A distributed cache for an application
- Service Discovery
- Distributed Configuration
- Blob Storage

## Command Line Interface

```
 ██╗    ██╗   ██╗ ██████╗██╗██████╗     ██╗  ██╗██╗   ██╗
 ██║    ██║   ██║██╔════╝██║██╔══██╗    ██║ ██╔╝██║   ██║
 ██║    ██║   ██║██║     ██║██║  ██║    ██╔═██╗ ╚██╗ ██╔╝
 ██████╗╚██████╔╝╚██████╗██║██████╔╝    ██║  ██╗ ╚████╔╝
 ╚═════╝ ╚═════╝  ╚═════╝╚═╝╚═════╝     ╚═╝  ╚═╝  ╚═══╝

A Fast, Secure and Distributed KV store with an HTTP API.
Written in Rust by Clint.Network (twitter.com/clint_network)

USAGE:
    lucid.exe [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help informations
    -v, --version    Print version information

SUBCOMMANDS:
    cli         Spawn to the command line interface
    help        Prints this message or the help of the given subcommand(s)
    init        Initialize the Lucid cluster and generate configuration file
    members     Manage members of the cluster
    server      Run a new Lucid server instance
    settings    Manage Lucid configuration file
    store       Play with the KV store (get/set)
    tokens      Manage JWT Tokens (issue, revoke etc.)
```

## Web Interface (UI)

Lucid wants to propose a web UI to manage data, issue tokens, organize nodes and configure instances.

## About the Author

Lucid is powered by [Clint.Network](https://twitter.com/clint_network) and published under the [MIT License](LICENSE.md).

If you want to make a little donation, use this Bitcoin address: 3NhdjiGrpzH5geVrDHa173EuXxnAVhghtZ or my [Patreon](https://www.patreon.com/clintnetwork).
