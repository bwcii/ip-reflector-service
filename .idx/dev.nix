# To learn more about how to use Nix to configure your environment
# see: https://developers.google.com/idx/guides/customize-idx-env
{ pkgs, ... }: {
  channel = "stable-23.11"; # "stable-23.11" or "unstable"
  # Use https://search.nixos.org/packages to  find packages
  packages = [
    pkgs.cargo
    pkgs.rustc
    pkgs.stdenv.cc
    pkgs.cargo-watch
    pkgs.openssl
    pkgs.perl
    pkgs.gnumake
  ];
  # Sets environment variables in the workspace
  env = {
    RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
  };
  # search for the extension on https://open-vsx.org/ and use "publisher.id"
  idx.extensions = [
    "rust-lang.rust-analyzer"
    "tamasfe.even-better-toml"
    "serayuzgur.crates"
    "vadimcn.vscode-lldb"
    "WakaTime.vscode-wakatime"
    "rangav.vscode-thunder-client"
    "ritwickdey.LiveServer"
  ];
}