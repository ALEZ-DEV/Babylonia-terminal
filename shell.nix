{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    rustup
    rustfmt
    clippy
    rust-analyzer
    gcc
    pkg-config
  ];

  buildInputs = with pkgs; [
    openssl
    cabextract
  ];

  shellHook = ''
    export OPENSSL_DIR="${pkgs.openssl.dev}"
    export PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkgconfig"
    export OPENSSL_NO_VENDOR=1
    export OPENSSL_LIB_DIR="${pkgs.lib.getLib pkgs.openssl}/lib"
  '';
}
