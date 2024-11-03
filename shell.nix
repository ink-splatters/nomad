with import <nixpkgs> { };
let
  inherit (llvmPackages_19) libcxxStdenv bintools;
  stdenv = libcxxStdenv;
in
mkShell.override { inherit stdenv; } {

  buildInputs =
    with darwin.apple_sdk.frameworks;
    [
      Security
      SystemConfiguration
    ]
    ++ [

      iconv
      openssl
      pkg-config
    ];

  nativeBuildInputs = [
    bintools
  ];
}
