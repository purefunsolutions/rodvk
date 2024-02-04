# SPDX-FileCopyrightText: 2024 Mika Tammi
#
# SPDX-License-Identifier: MIT
#
# rodvk package
{
  darwin,
  lib,
  makeWrapper,
  rustPlatform,
  stdenv,
}:
rustPlatform.buildRustPackage {
  pname = "rodvk";
  version = "0.1.0";

  src = ./.;

  nativeBuildInputs = lib.optionals stdenv.isDarwin [makeWrapper];

  buildInputs = lib.optionals stdenv.isDarwin (
    with darwin.apple_sdk.frameworks; [
      AppKit
    ]
  );

  postInstall = lib.optionalString stdenv.isDarwin ''
    wrapProgram $out/bin/rodvk \
      --prefix DYLD_LIBRARY_PATH : ${lib.makeLibraryPath [darwin.moltenvk]}
  '';
  cargoLock.lockFile = ./Cargo.lock;
}
