# SPDX-FileCopyrightText: 2024 Mika Tammi
#
# SPDX-License-Identifier: MIT
#
# Packages to be exported from the flake
{
  perSystem = {pkgs, ...}: {
    packages = let
      rodvk = pkgs.callPackage ../rodvk {};
      default = rodvk;
    in {
      inherit default rodvk;
    };
  };
}
