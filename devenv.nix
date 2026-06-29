{
  pkgs,
  ...
}:
{
  languages.rust.enable = true;

  packages = [
    pkgs.cargo-machete
    pkgs.cargo-tauri
  ];

  scripts.x = {
    exec = ''
      bun tauri "$@";
    '';
  };
}
