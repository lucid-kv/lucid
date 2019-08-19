# About Lucid

An High performance and distributed KV store accessible through a HTTP API. Written in Rust. 🦀

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
 ██╗    ██╗   ██╗ ██████╗██╗██████╗     ██╗  ██╗██╗   ██╗
 ██║    ██║   ██║██╔════╝██║██╔══██╗    ██║ ██╔╝██║   ██║
 ██║    ██║   ██║██║     ██║██║  ██║    ██╔═██╗ ╚██╗ ██╔╝
 ██████╗╚██████╔╝╚██████╗██║██████╔╝    ██║  ██╗ ╚████╔╝
 ╚═════╝ ╚═════╝  ╚═════╝╚═╝╚═════╝     ╚═╝  ╚═╝  ╚═══╝

A Fast, Secure and Distributed KV store with a HTTP API.
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

Lucid propose a Web UI to manage objects, issue tokens, organize nodes and configure instances.

## About the Author

Lucid is powered by [Clint.Network](https://twitter.com/clint_network) and published under the [MIT License](LICENSE.md).

If you want to make a little donation, use this Bitcoin address: 3NhdjiGrpzH5geVrDHa173EuXxnAVhghtZ or my [Patreon](https://www.patreon.com/clintnetwork).
