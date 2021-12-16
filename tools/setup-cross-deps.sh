#!/bin/bash

sudo apt-get install -y qemu binfmt-support
docker run --privileged --rm tonistiigi/binfmt --install all
curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal
rustup target add $TARGETS
cargo install cross
