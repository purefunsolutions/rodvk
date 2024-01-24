# SPDX-FileCopyrightText: 2024 Mika Tammi
#
# SPDX-License-Identifier: MIT
#
# Packages to be exported from the flake
{
  perSystem = {pkgs, ...}: {
    packages = {
      rodvk = pkgs.callPackage ../rodvk {};
    };
  };
}
