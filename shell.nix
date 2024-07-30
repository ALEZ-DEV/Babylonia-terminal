{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    rustup
    rustfmt
    clippy
    rust-analyzer
    gcc
    pkg-config
    flutter
    protoc-gen-prost
    mpv-unwrapped
    libass
    libplacebo
    libunwind
    shaderc
    vulkan-loader
    vulkan-headers
    lcms
    libdovi
    libdvdnav
    libdvdread
    libGL
    wayland
    openssl
    cabextract
    ffmpeg
  ];

  shellHook = ''
    export OPENSSL_DIR="${pkgs.openssl.dev}"
    export PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkgconfig:${pkgs.mpv-unwrapped.dev}/lib/pkgconfig:${pkgs.libass.dev}/lib/pkgconfig:${pkgs.ffmpeg.dev}/lib/pkgconfig:${pkgs.libplacebo}/lib/pkgconfig:${pkgs.libunwind.dev}/lib/pkgconfig:${pkgs.shaderc.dev}/lib/pkgconfig:${pkgs.vulkan-loader.dev}/lib/pkgconfig:${pkgs.lcms.dev}/lib/pkgconfig:${pkgs.libdovi}/lib/pkgconfig:${pkgs.libdvdnav}/lib/pkgconfig:${pkgs.libdvdread}/lib/pkgconfig"
    export OPENSSL_NO_VENDOR=1
    export OPENSSL_LIB_DIR="${pkgs.lib.getLib pkgs.openssl}/lib"
    export FLUTTER_ROOT="${pkgs.flutter}"
    export LD_LIBRARY_PATH="${pkgs.wayland}:$LD_LIBRARY_PATH"
  '';
}
