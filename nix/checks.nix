# SPDX-FileCopyrightText: 2024 Mika Tammi
#
# SPDX-License-Identifier: MIT
{inputs, ...}: {
  perSystem = {pkgs, ...}: {
    checks = {
      reuse =
        pkgs.runCommandLocal "reuse-lint" {
          buildInputs = [pkgs.reuse];
        } ''
          cd ${../.}
          reuse lint
          touch $out
        '';
      nix-build-all = pkgs.writeShellApplication {
        name = "nix-build-all";
        runtimeInputs = let
          devour-flake = pkgs.callPackage inputs.devour-flake {};
        in [
          pkgs.nix
          devour-flake
        ];
        text = ''
          # Make sure that flake.lock is sync
          nix flake lock --no-update-lock-file

          # Do a full nix build (all outputs)
          devour-flake . "$@"
        '';
      };
    };
  };
}
