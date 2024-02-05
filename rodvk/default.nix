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
  vulkan-loader,
}:
rustPlatform.buildRustPackage {
  pname = "rodvk";
  version = "0.1.0";

  src = ./.;

  nativeBuildInputs = lib.optionals (stdenv.isDarwin || stdenv.isLinux) [makeWrapper];

  buildInputs = lib.optionals stdenv.isDarwin (
    with darwin.apple_sdk.frameworks; [
      AppKit
    ]
  );

  postInstall =
    lib.optionalString stdenv.isDarwin ''
      wrapProgram $out/bin/rodvk \
        --prefix DYLD_LIBRARY_PATH : ${lib.makeLibraryPath [darwin.moltenvk]}
    ''
    + lib.optionalString stdenv.isLinux ''
      wrapProgram $out/bin/rodvk \
        --prefix LD_LIBRARY_PATH : ${lib.makeLibraryPath [vulkan-loader]}
    '';
  cargoLock.lockFile = ./Cargo.lock;
}
