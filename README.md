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
    <a href="https://discord.gg/mZz67M6"><img src="https://img.shields.io/badge/Discord-Server-7289DA" /></a>
  </p>
</p>

## Introduction

Lucid is currently in a development stage but we want to achieve a fast, secure and distributed key-value store accessible through an HTTP API, we also want to propose persistence, encryption, WebSocket streaming, replication and more.

[Read the complete Medium article →](https://medium.com/@clintnetwork/lucid-an-http-key-value-store-c0e734586e26)

## Getting Started

Get the latest binary from the [releases](https://github.com/lucid-kv/lucid/releases) page and run these commands:

```
$ ./lucid init
$ ./lucid --config lucid.yml server
```

### Docker Deployment

Or run a node with Docker, but you need to create a [lucid.yml](.github/lucid.yml) file locally before.

```
$ docker pull lucidkv/lucid
$ docker run -p 7020:7020 -v lucid.yml:/etc/lucid/lucid.yml lucidkv/lucid
```

A demonstration instance is accessible at <http://lucid-kv.herokuapp.com> (with authentication disabled).

<a href="https://heroku.com/deploy?template=https://github.com/lucid-kv/lucid/tree/deploy-buttons" target="_blank">
  <img src="https://www.herokucdn.com/deploy/button.svg" height="32"/>
</a>

## Documentation

You can start quickly [here](https://github.com/lucid-kv/lucid/wiki) or get the complete documentation at <https://docs.lucid-kv.store>.

## Works in Progress

You can take a look at the roadmap [here](https://github.com/lucid-kv/lucid/issues/46), we are working on the implementation of persistence and encryption.

## Command Line Interface

You can take a look at the Lucid command line on at <https://asciinema.org/a/277538>.

```
 ██╗    ██╗   ██╗ ██████╗██╗██████╗     ██╗  ██╗██╗   ██╗
 ██║    ██║   ██║██╔════╝██║██╔══██╗    ██║ ██╔╝██║   ██║
 ██║    ██║   ██║██║     ██║██║  ██║    ██╔═██╗ ╚██╗ ██╔╝
 ██████╗╚██████╔╝╚██████╗██║██████╔╝    ██║  ██╗ ╚████╔╝
 ╚═════╝ ╚═════╝  ╚═════╝╚═╝╚═════╝     ╚═╝  ╚═╝  ╚═══╝

A Fast, Secure and Distributed KV store with an HTTP API.
Written in Rust, Fork us on GitHub (https://github.com/lucid-kv)

FLAGS:
    -h, --help         Prints help information
        --no-banner    Disable showing the banner on start
    -V, --version      Prints version information

OPTIONS:
    -c, --config <config>    Specify the Lucid configuration file

SUBCOMMANDS:
    help        Prints this message or the help of the given subcommand(s)
    init        Initialize Lucid and generate configuration file
    server      Run a new Lucid server instance
    settings    Manage the Lucid configuration file
```

## Web Interface (UI)

Lucid wants to propose a web UI to manage data, issue tokens, organize nodes and configure instances.

## Development Credits

Lucid is developed by [@clintnetwork](https://github.com/clintnetwork), [@Slals](https://github.com/Slals), [@CephalonRho](https://github.com/CephalonRho), [@rigwild](https://github.com/rigwild) and published under the [MIT License](LICENSE.md).

| Name / Nickname | Email                 | Role               |
|-----------------|-----------------------|--------------------|
| Clint Mourlevat | me@clint.network      | Lucid Founder      |
| Jonathan Serra  | jonathan@blocs.fr     | Core Development   |
| CephalonRho     | CephalonRho@gmail.com | Core Development   |
| Rigwild         | me@rigwild.dev        | Web UI Development |

## Contribute to Lucid

See [CONTRIBUTING.md](CONTRIBUTING.md) for best practices and instructions on setting up your development environment to work on Lucid.
