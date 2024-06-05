sudo steamos-readonly disable
curl -s https://raw.githubusercontent.com/89luca89/distrobox/main/install | sudo sh -s -- --prefix /usr
sudo steamos-readonly enable
xhost +si:localuser:$USER >/dev/null
distrobox create && distrobox enter
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo dnf install pkg-config openssl-devel
sudo dnf install llvm-devel
sudo dnf install gcc-c++.x86_64
rustup update
rustup toolchain install 1.78.0
cargo install --git https://github.com/ALEZ-DEV/Babylonia-terminal --bin
