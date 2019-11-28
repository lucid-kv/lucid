#!/bin/bash
cd ..
cargo build --release
cd webui
yarn install
yarn build