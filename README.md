# About Lucid ᵏᵛ

High performance and distributed KV store accessible through an HTTP API. Written in Rust. 🦀

[![Build Status](https://github.com/lucid-kv/lucid/workflows/Lucid/badge.svg)](https://github.com/lucid-kv/lucid/actions?workflow=Lucid)
[![Made with Rust](https://img.shields.io/badge/Made%20With-Rust-dea584)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-lightgrey.svg)](https://github.com/lucid-kv/lucid/blob/master/LICENSE.md)
[![Gitter](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/lucidkv/community)
[![Twitter](https://img.shields.io/twitter/follow/lucid_kv.svg?style=social)](https://twitter.com/intent/follow?screen_name=lucid_kv)

## Introduction

Lucid is currently in an embryonic state but we wish to achieve a fast, secure and distributed key-value store accessible through an HTTP API, we also want to propose persistence, encryption, WebSocket streaming, replication and a lot of features.

## Getting Started

Get the latest binary from the [releases](https://github.com/lucid-kv/lucid/releases) page and run this commands:

```
$ ./lucid init
$ ./lucid server
```

Or run a node with Docker, but you need to create a [lucid.yml](lucid.yml) file localy before.
```
$ docker pull lucidkv/lucid
$ docker run -v lucid.yml:/etc/lucid/lucid.yml lucidkv/lucid
```

A demonstration instance is accessible at http://lucid-kv.herokuapp.com (with authentication disabled).

## Official Documentation

You can find the official documentation at https://docs.lucid-kv.store.

## Some Use Cases

* **IoT** - Health Checking enables Consul to quickly alert
  operators about any issues in a cluster. The integration with service
  discovery prevents routing traffic to unhealthy hosts and enables service
  level circuit breakers.

* **Private Keys Storing** - A flexible key/value store enables storing
  dynamic configuration, feature flagging, coordination, leader election and
  more. The simple HTTP API makes it easy to use anywhere.

* **Distributed Cache** - Consul makes it simple for services to register
  themselves and to discover other services via a DNS or HTTP interface.
  External services such as SaaS providers can be registered as well.

* **Service Discovery** - Consul is built to be datacenter aware, and can
  support any number of regions without complex configuration.

* **Distributed Configuration** - Consul Connect enables secure service-to-service
communication with automatic TLS encryption and identity-based authorization.

## Command Line Interface

You can take a look to the Lucid command line on at https://asciinema.org/a/277538.

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
    -h, --help       Prints help information
    -v, --version    Prints version information

SUBCOMMANDS:
    cli         Spawn to the command line interface
    help        Prints this message or the help of the given subcommand(s)
    init        Initialize Lucid and generate configuration file
    server      Run a new Lucid server instance
    settings    Manage Lucid configuration file
    store       Play with the KV store (get/set)
    tokens      Manage JWT Tokens (issue, revoke etc.)
```

## Web Interface (UI)

Lucid wants to propose a web UI to manage data, issue tokens, organize nodes and configure instances.

## Lucid Credits

Lucid is developed by [@clintnetwork](https://twitter.com/clint_network), [@Slals](https://github.com/Slals), [@CephalonRho](https://github.com/CephalonRho), [@rigwild](https://github.com/rigwild) and published under the [MIT License](LICENSE.md).

| Name / Nickname | Email                 | Role               |
|-----------------|-----------------------|--------------------|
| Clint Mourlevat | me@clint.network      | Lucid Founder      |
| Jonathan Serra  | jonathan@blocs.fr     | Core Development   |
| CephalonRho     | CephalonRho@gmail.com | Core Development   |
| Rigwild         | me@rigwild.dev        | Web UI Development |

## Contribute on Lucid

See [CONTRIBUTING.md](CONTRIBUTING.md) for best practices and instructions on setting up your development environment to work on Lucid.
