# wgas-rs

Simple WireGuard Access Server


## Quickstart

```
sudo add-apt-repository ppa:wireguard/wireguard -y
sudo apt install wireguard -y
sudo umask 077
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
git clone https://github.com/lalanza808/wgas-rs && cd wgas-rs
rustup override set nightly
cargo run
```
