{ pkgs ? import <nixpkgs> {} }:

let
  # Собираем список библиотек, которые понадобятся для запуска
  libs = with pkgs; [
    wayland
    libxkbcommon
    libGL
    vulkan-loader
  ];
  # Формируем путь к библиотекам (через двоеточие)
  libPath = pkgs.lib.makeLibraryPath libs;
in
pkgs.mkShell {
  buildInputs = libs ++ (with pkgs; [
    pkg-config
    gcc
  ]);

  shellHook = ''
    export LD_LIBRARY_PATH="${libPath}:$LD_LIBRARY_PATH"
  '';
}
