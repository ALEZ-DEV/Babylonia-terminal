#!/bin/bash

while true; do
	read -p "Are you on steam deck? (y/n): " confirm
	if [[ $confirm =~ ^yes$|^y$|^Y$|^no$|^n$|^N$ ]]; then
		if [[ $confirm =~ ^yes$|^y$|^Y$ ]]; then
			isSteamdeck=true
		else
			isSteamdeck=false
		fi
		break
	fi
done

if $isSteamdeck; then
	sudo steamos-readonly disable
fi

if ! command -v distrobox &>/dev/null; then
	curl -s https://raw.githubusercontent.com/89luca89/distrobox/main/install | sudo sh
fi

if $isSteamdeck; then
	sudo steamos-readonly enable
fi

xhost +si:localuser:$USER >/dev/null

distroname="babylonia-terminal-image"
distrobox create --image registry.fedoraproject.org/fedora-toolbox:38 -Y --name $distroname
distrobox enter --name $distroname -- sudo dnf install -y pkg-config openssl-devel llvm-devel gcc-c++.x86_64
distrobox enter --name $distroname -- rustup update
distrobox enter --name $distroname -- rustup toolchain install 1.78.0
distrobox enter --name $distroname -- cargo install --git https://github.com/ALEZ-DEV/Babylonia-terminal --bin
