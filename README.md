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
$ ./lucid init   # To initialize the node and create a new configuration file
$ ./lucid server

 ██╗    ██╗   ██╗ ██████╗██╗██████╗     ██╗  ██╗██╗   ██╗
 ██║    ██║   ██║██╔════╝██║██╔══██╗    ██║ ██╔╝██║   ██║
 ██║    ██║   ██║██║     ██║██║  ██║    ██╔═██╗ ╚██╗ ██╔╝
 ██████╗╚██████╔╝╚██████╗██║██████╔╝    ██║  ██╗ ╚████╔╝
 ╚═════╝ ╚═════╝  ╚═════╝╚═╝╚═════╝     ╚═╝  ╚═╝  ╚═══╝

2019/12/01 12:58:16 INFO [warp::server] warp drive engaged: listening on http://127.0.0.1:7021
```

A demonstration instance is accessible at http://lucid-kv.herokuapp.com (authentication disabled).

## Documentation

You can find the official documentation at https://docs.lucid-kv.store.

## Some Use Cases

- Private Keys Storing (for a wallet by example)
- IoT: collect and save statistics data
- A distributed cache for an application
- Service Discovery
- Distributed Configuration
- Blob Storage

## Command Line Interface

[![asciicast](https://asciinema.org/a/277538.svg)](https://asciinema.org/a/277538)

## Web Interface (UI)

Lucid wants to propose a web UI to manage data, issue tokens, organize nodes and configure instances.

## Lucid Credits

Lucid is developed by [@clintnetwork](https://twitter.com/clint_network), [@Slals](https://github.com/Slals), [@CephalonRho](https://github.com/CephalonRho), [@rigwild](https://github.com/rigwild) and published under the [MIT License](LICENSE.md).

| Clint Mourlevat | me@clint.network      | Lucid Founder      |
|-----------------|-----------------------|--------------------|
| Jonathan Serra  | jonathan@blocs.fr     | Core Development   |
| CephalonRho     | CephalonRho@gmail.com | Core Development   |
| Rigwild         | me@rigwild.dev        | Web UI Development |

## Contribute on Lucid
See CONTRIBUTING.md for best practices and instructions on setting up your development environment to work on Lucid.
