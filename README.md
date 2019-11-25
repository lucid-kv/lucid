# About Lucid ᵏᵛ

High performance and distributed KV store accessible through an HTTP API. Written in Rust. 🦀

[![Build Status](https://github.com/lucid-kv/lucid/workflows/Lucid/badge.svg)](https://github.com/lucid-kv/lucid/actions?workflow=Lucid)
[![Made with Rust](https://img.shields.io/badge/Made%20With-Rust-dea584)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-lightgrey.svg)](https://github.com/clintnetwork/lucid/blob/master/LICENSE.md)
[![Gitter](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/lucidkv/community)
[![Twitter](https://img.shields.io/twitter/follow/lucid_kv.svg?style=social)](https://twitter.com/intent/follow?screen_name=lucid_kv)

## Introduction

Lucid is currently in an embryonic state but we wish to achieve a fast, secure and distributed key-value store accessible through an HTTP API, we also want to propose persistence, encryption, WebSocket streaming, replication and a lot of features.

## Works Progress

<ins>__**Warning: Empty project for now, the development is ensured in the [development](https://github.com/lucid-kv/lucid/tree/development) branch.**__</ins>

- [x] Minimum Viable Product (MVP)
  - [x] Initialization process
  - [x] Configuration files handling
  - [x] JWT token Issuing
  - [ ] HTTPS Support
  - [x] Rest API
     - [x] Adding CORS
     - [x] KV-Base, HashMap-based
     - [x] JWT Authentication
- [ ] Web UI in VueJS
- [ ] Persistence
- [ ] Encryption (with Serpent Cipher)
- [ ] Access Control List (ACL)
- [ ] Server Sent Events (SSE)

## Some Use Cases

- Private Keys Storing (for a wallet by example)
- IoT: collect and save statistics data
- A distributed cache for an application
- Service Discovery
- Distributed Configuration
- Blob Storage

## Some Features

- RESTful HTTP API
- Web UI
- Persistence
- Encryption on the Fly (Serpent)
- Access Control List (ACL)
- Server Sent Events (SSE)
- Named Pipes

## Command Line Interface

[![asciicast](https://asciinema.org/a/277538.svg)](https://asciinema.org/a/277538)

## Web Interface (UI)

Lucid wants to propose a web UI to manage data, issue tokens, organize nodes and configure instances.

## About the Author

Lucid is powered by [Clint.Network](https://twitter.com/clint_network) and published under the [MIT License](LICENSE.md).

**Donate to Clint.Network**
- ![Paypal](https://raw.githubusercontent.com/reek/anti-adblock-killer/gh-pages/images/paypal.png) Paypal: [Donate](http://paypal.me/clintnetwork)
- ![btc](https://raw.githubusercontent.com/reek/anti-adblock-killer/gh-pages/images/bitcoin.png) Bitcoin: 3AEqgvpiHC2LzPDunf6PBPBLeT98YruKmg
