{ pkgs, lib, config, inputs, ... }:

{
  # Package dependencies
  packages = with pkgs; [
    jujutsu
  ];

  # Rust toolchain
  languages.rust = {
    enable = true;
    toolchainFile = ./rust-toolchain.toml;
  };

  languages.c.debugger = null;

  # Environment variables matching defs.bzl
  env = {
    # Logging
    RUST_LOG = "trace";
    RUST_BACKTRACE = 1;
  };

  # Shell hook for initialization
  enterShell = ''
    echo "Spacey Development Environment"
  '';
}
