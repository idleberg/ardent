{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};
      craneLib = crane.mkLib pkgs;
      nsisFilter = path: _type:
        (pkgs.lib.hasSuffix ".nsi" path)
        || (pkgs.lib.hasSuffix ".nsh" path);
      src = pkgs.lib.cleanSourceWith {
        src = ./.;
        filter = path: type:
          (nsisFilter path type) || (craneLib.filterCargoSources path type);
      };
      ardent = craneLib.buildPackage {
        inherit src;
      };
    in {
      packages.default = ardent;
      devShells.default = craneLib.devShell {
        packages = [pkgs.cargo-watch];
      };
    });
}
