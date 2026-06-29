{
  pkgs,
  ...
}:

{
  languages.rust.enable = true;

  packages = [
    pkgs.cargo-machete
    pkgs.cargo-tauri
    pkgs.biome
  ];

  scripts.x = {
    exec = ''
      bun tauri "$@";
    '';
  };
}
