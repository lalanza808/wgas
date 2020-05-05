# wgas-rs

Simple WireGuard Access Server


## Installation

The application depends on the `WireGuard` package for the `wg` and `wg-quick` command line tools:

```
# Ubuntu
sudo add-apt-repository ppa:wireguard/wireguard -y
sudo apt install wireguard -y

# macOS
brew install wireguard-tools
```

This is a Rust application so install via `rustup`:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```


Install the repo:

```
git clone https://github.com/lalanza808/wgas-rs && cd wgas-rs
rustup override set nightly
cargo run
```

```

From there set environment variables:

```
export WGAS_ENDPOINT=127.0.0.1
export WGAS_SUDO=false
export WGAS_DNS=1.1.1.1 # set to dns you control if possible
export WGAS_ROUTE=0.0.0.0/0
export WGAS_PORT=51820
export WGAS_PUBKEY=$(wg genkey | wg pubkey)
```

and use `cargo` to run the dev server or build production release binaries:

```
cargo run
cargo build --release # outputs to ./target/release/<package_nae>
```
