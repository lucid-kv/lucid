# About Lucid ᵏᵛ

High performance and distributed KV store accessible through an HTTP API. Written in Rust. 🦀

[![Build Status](https://github.com/lucid-kv/lucid/workflows/Lucid/badge.svg)](https://github.com/lucid-kv/lucid/actions?workflow=Lucid)
[![Made with Rust](https://img.shields.io/badge/Made%20With-Rust-dea584)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-lightgrey.svg)](https://github.com/lucid-kv/lucid/blob/master/LICENSE.md)
[![Gitter](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/lucidkv/community)
[![Twitter](https://img.shields.io/twitter/follow/lucid_kv.svg?style=social)](https://twitter.com/intent/follow?screen_name=lucid_kv)

## Introduction

Lucid is currently in a development stage but we want to achieve a fast, secure and distributed key-value store accessible through an HTTP API, we also want to propose persistence, encryption, WebSocket streaming, replication and more.

<a href="#azure_deploy" target="_blank">
    <img src="http://azuredeploy.net/deploybutton.png" height="32" />
</a><a href="https://heroku.com/deploy?template=https://github.com/lucid-kv/deploy-templates" target="_blank">
    <img src="https://www.herokucdn.com/deploy/button.svg" height="32"/>
</a>

## Getting Started

Get the latest binary from the [releases](https://github.com/lucid-kv/lucid/releases) page and run these commands:

```
$ ./lucid init
$ ./lucid server
```

### Docker Deployment

Or run a node with Docker, but you need to create a [lucid.yml](.github/lucid.yml) file locally before.
```
$ docker pull lucidkv/lucid
$ docker run -v lucid.yml:/etc/lucid/lucid.yml lucidkv/lucid
```

## Documentation

You can find the official documentation at https://docs.lucid-kv.store.

## Command Line Interface

You can take a look at the Lucid command line on at https://asciinema.org/a/277538.

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

## Development Credits

Lucid is developed by [@clintnetwork](https://twitter.com/clint_network), [@Slals](https://github.com/Slals), [@CephalonRho](https://github.com/CephalonRho), [@rigwild](https://github.com/rigwild) and published under the [MIT License](LICENSE.md).

| Name / Nickname | Email                 | Role               |
|-----------------|-----------------------|--------------------|
| Clint Mourlevat | me@clint.network      | Lucid Founder      |
| Jonathan Serra  | jonathan@blocs.fr     | Core Development   |
| CephalonRho     | CephalonRho@gmail.com | Core Development   |
| Rigwild         | me@rigwild.dev        | Web UI Development |

## Contribute to Lucid

See [CONTRIBUTING.md](CONTRIBUTING.md) for best practices and instructions on setting up your development environment to work on Lucid.
