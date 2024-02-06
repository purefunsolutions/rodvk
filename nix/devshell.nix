# SPDX-FileCopyrightText: 2024 Mika Tammi
#
# SPDX-License-Identifier: MIT
_: {
  perSystem = {
    lib,
    pkgs,
    self',
    ...
  }: {
    devShells.default = pkgs.mkShell {
      buildInputs =
        [
          pkgs.wget
        ]
        ++ self'.packages.default.buildInputs
        ++ self'.packages.default.nativeBuildInputs;
      shellHook =
        lib.optionalString pkgs.stdenv.isDarwin ''
          export DYLD_LIBRARY_PATH=${lib.makeLibraryPath [pkgs.darwin.moltenvk]}
        ''
        + lib.optionalString pkgs.stdenv.isLinux ''
          export LD_LIBRARY_PATH=${lib.makeLibraryPath [pkgs.vulkan-loader]}
        '';
    };
  };
}
