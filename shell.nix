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
    
    wayland
    openssl
    cabextract

    #mpv dependecies
    mpv
    mpv-unwrapped
    libass
    libplacebo
    libunwind
    shaderc
    vulkan-loader
    lcms
    libdovi
    libdvdnav
    libdvdread
    libGL
    mujs
    libbluray
    lua
    rubberband
    SDL2
    libuchardet
    zimg
    alsa-lib
    openal
    ffmpeg
    libcaca
    libdrm
    libpulseaudio
    libva
    libvdpau
    mesa
    nv-codec-headers-11
    pipewire
    xorg.libXpresent
    xorg.libXScrnSaver
    xorg.libXv
  ];

  shellHook = ''
    export OPENSSL_DIR="${pkgs.openssl.dev}"
    export PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkgconfig:${pkgs.mpv-unwrapped.dev}/lib/pkgconfig:${pkgs.libass.dev}/lib/pkgconfig:${pkgs.ffmpeg.dev}/lib/pkgconfig:${pkgs.libplacebo}/lib/pkgconfig:${pkgs.libunwind.dev}/lib/pkgconfig:${pkgs.shaderc.dev}/lib/pkgconfig:${pkgs.vulkan-loader.dev}/lib/pkgconfig:${pkgs.lcms.dev}/lib/pkgconfig:${pkgs.libdovi}/lib/pkgconfig:${pkgs.libdvdnav}/lib/pkgconfig:${pkgs.libdvdread}/lib/pkgconfig:${pkgs.mujs}/lib/pkgconfig:${pkgs.pipewire.dev}/lib/pkgconfig:${pkgs.libbluray}/lib/pkgconfig:${pkgs.lua}/lib/pkgconfig:${pkgs.rubberband}/lib/pkgconfig:${pkgs.SDL2.dev}/lib/pkgconfig:${pkgs.libuchardet.dev}/lib/pkgconfig:${pkgs.zimg.dev}/lib/pkgconfig:${pkgs.alsa-lib.dev}/lib/pkgconfig:${pkgs.openal}/lib/pkgconfig:${pkgs.libcaca.dev}/lib/pkgconfig:${pkgs.libdrm.dev}/lib/pkgconfig:${pkgs.libpulseaudio.dev}/lib/pkgconfig:${pkgs.libva.dev}/lib/pkgconfig:${pkgs.libvdpau.dev}/lib/pkgconfig:${pkgs.mesa.dev}/lib/pkgconfig:${pkgs.nv-codec-headers-11}/lib/pkgconfig:${pkgs.pipewire.dev}/lib/pkgconfig:${pkgs.xorg.libXpresent}/lib/pkgconfig:${pkgs.xorg.libXpresent}/lib/pkgconfig:${pkgs.xorg.libXScrnSaver}/lib/pkgconfig:${pkgs.xorg.libXv.dev}/lib/pkgconfig"
    export OPENSSL_NO_VENDOR=1
    export OPENSSL_LIB_DIR="${pkgs.lib.getLib pkgs.openssl}/lib"
    export FLUTTER_ROOT="${pkgs.flutter}"
    export LD_LIBRARY_PATH="${pkgs.wayland}:$LD_LIBRARY_PATH"
  '';
}
