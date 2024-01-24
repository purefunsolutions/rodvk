# SPDX-FileCopyrightText: 2024 Mika Tammi
#
# SPDX-License-Identifier: MIT
#
# rodvk package
{
  darwin,
  lib,
  rustPlatform,
  stdenv,
}:
rustPlatform.buildRustPackage {
  pname = "rodvk";
  version = "0.1.0";

  src = ./.;

  buildInputs = lib.optionals stdenv.isDarwin (
    with darwin.apple_sdk.frameworks; [
      AppKit
    ]
  );

  postInstall = ''
    ln -s ${darwin.moltenvk}/lib/libMoltenVK.dylib $out/bin/libMoltenVK.dylib
  '';
  cargoLock.lockFile = ./Cargo.lock;
}
