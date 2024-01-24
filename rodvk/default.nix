# SPDX-FileCopyrightText: 2024 Mika Tammi
#
# SPDX-License-Identifier: MIT
#
# rodvk package
{rustPlatform}:
rustPlatform.buildRustPackage {
  pname = "rodvk";
  version = "0.1.0";

  src = ./.;

  cargoLock.lockFile = ./Cargo.lock;
}
