# About Lucid

<ins>__**Warning: Empty project for now, the development is ensured in the [development](https://github.com/clintnetwork/lucid/tree/developement) branch.**__</ins>

An High performance and distributed KV ledger accessible through HTTP API. Written in Rust. 🦀

[![Build Status](https://travis-ci.com/clintnetwork/lucid.svg?branch=developement)](https://travis-ci.com/clintnetwork/lucid)
[![Made with Rust](https://img.shields.io/badge/Made%20With-Rust-dea584)](https://www.rust-lang.org/)
[![Clint.Network](https://img.shields.io/badge/Powered%20by-Clint.Network-blue.svg)](https://twitter.com/clint_network)

## Introduction

Lucid is currently in an embryonic state but we wish to achieve a fast, secure and distributed key-value store accessible through a HTTP API, we also want to propose persistence, encryption, websocket streaming, replication and a lots of features.

## Some Use Cases

- Private Keys Storing (for a wallet by example)
- IoT: collect and save statistics data
- A distributed cache for an application
- Service Discovery
- Distributed Configuration

## Command Line Interface

```
  _   _   _  ___ ___ ___  _
 | | | | | |/ __|_ _|   \| |
 | |_| |_| | (__ | || |) |_|
 |____\___/ \___|___|___/(_)

High performance and distributed KV ledger.
Written in Rust by Clint.Network

USAGE:
    lucid.exe [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    cli         Spawn to the command line interface
    help        Prints this message or the help of the given subcommand(s)
    members     Manage members of the cluster
    monitor     Display logs in realtime.
    server      Run a Lucid server instance
    token       Manage JWT tokens issuing
    store       Play with the KV store (get/set)
    settings    Configure the instance
```

## Web Interface (UI)

Lucid want to propose an web UI to manage data, issue tokens, organize nodes and configure instances.

## About the Author

Lucid is powered by [Clint.Network](https://twitter.com/clint_network) and published under the [MIT License](LICENSE.md).

If you want to make a little donation, use this Bitcoin address: 3NhdjiGrpzH5geVrDHa173EuXxnAVhghtZ or my [Patreon](https://www.patreon.com/clintnetwork).
