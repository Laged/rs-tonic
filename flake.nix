{
  description = "Description for the project";

  inputs = {
    devenv-root = { url = "file+file:///dev/null"; flake = false; };
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:cachix/devenv-nixpkgs/rolling";
    devenv.url = "github:cachix/devenv";
    nix2container.url = "github:nlewo/nix2container";
    nix2container.inputs.nixpkgs.follows = "nixpkgs";
    mk-shell-bin.url = "github:rrbutani/nix-mk-shell-bin";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  nixConfig = {
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };

  outputs = inputs@{ flake-parts, devenv-root, fenix, nixpkgs, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ inputs.devenv.flakeModule ];
      
      systems = nixpkgs.lib.systems.flakeExposed;

      perSystem = { config, self', inputs', pkgs, system, ... }: {
        # Use the Fenix overlay
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [ fenix.overlays.default ];
        };

        packages.default = pkgs.hello;

        devenv.shells.default = {
          devenv.root = let
            devenvRootFileContent = builtins.readFile devenv-root.outPath;
          in
            pkgs.lib.mkIf (devenvRootFileContent != "") devenvRootFileContent;

          name = "my-project";
          imports = [
            # This is just like the imports in devenv.nix.
            # See https://devenv.sh/guides/using-with-flake-parts/#import-a-devenv-module
            # ./devenv-foo.nix
          ];

          # https://devenv.sh/reference/options/
          packages = with pkgs; [
            config.packages.default
            # Use the stable Rust toolchain from Fenix
            fenix.packages.${system}.stable.toolchain
            # You can also add other Rust-related tools here
            rust-analyzer
          ] ++ lib.optionals stdenv.isDarwin [
            libiconv
            darwin.apple_sdk.frameworks.Security
          ];

          enterShell = ''
            hello
            rustc --version
            cargo --version
          '';

          processes.hello.exec = "hello";

          # Add LIBRARY_PATH and other necessary environment variables for macOS
          env = pkgs.lib.optionalAttrs pkgs.stdenv.isDarwin {
            LIBRARY_PATH = "${pkgs.lib.makeLibraryPath [
              pkgs.libiconv
              pkgs.darwin.apple_sdk.frameworks.Security
            ]}";
            RUSTFLAGS = "-L ${pkgs.libiconv}/lib";
          };
        };
      };

      flake = {
        # The usual flake attributes can be defined here, including system-
        # agnostic ones like nixosModule and system-enumerating ones, although
        # those are more easily expressed in perSystem.
      };
    };
}
