<p align="center">
  <p align="center">
    <img src="https://github.com/lucid-kv/deploy-templates/blob/master/lucid.png?raw=true" height="100" alt="Lucid KV" />
  </p>
  <h3 align="center">
    About Lucid KV
  </h3>
  <p align="center">
    High performance and distributed KV store w/ REST API. 🦀
  </p>
  <p align="center">
      <a href="https://github.com/lucid-kv/lucid/actions?workflow=Lucid"><img src="https://github.com/lucid-kv/lucid/workflows/Lucid/badge.svg" /></a>
      <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/Made%20With-Rust-dea584" /></a>
      <a href="https://github.com/lucid-kv/lucid/blob/master/LICENSE.md"><img src="https://img.shields.io/badge/license-MIT-lightgrey.svg" /></a>
      <a href="https://gitter.im/lucidkv/community"><img src="https://badges.gitter.im/Join%20Chat.svg" /></a>
  </p>
</p>

## Introduction

Lucid is currently in a development stage but we want to achieve a fast, secure and distributed key-value store accessible through an HTTP API, we also want to propose persistence, encryption, WebSocket streaming, replication and a lot of features.

## Getting Started

Get the latest binary from the [releases](https://github.com/lucid-kv/lucid/releases) page and run these commands:

```
$ ./lucid init
$ ./lucid server
```

Or run a node with Docker, but you need to create a [lucid.yml](.github/lucid.yml) file locally before.
```
$ docker pull lucidkv/lucid
$ docker run -v lucid.yml:/etc/lucid/lucid.yml lucidkv/lucid
```

A demonstration instance is accessible at http://lucid-kv.herokuapp.com (with authentication disabled).

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
