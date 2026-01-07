{
  description = "Tech-Noir Personal Website: Zola + Rust WASM";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # 1. Define the specific Rust toolchain we need
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
          targets = [ "wasm32-unknown-unknown" ];
        };

        # 2. Build script wrapper to simplify the process
        # This allows you to run `nix run` to build everything locally
        buildScript = pkgs.writeShellScriptBin "build-site" ''
          echo ">> Building Rust Simulations..."
          # Assumes your crate is in a folder named 'simulations'
          # Adjust this path if your crate is elsewhere
          if [ -d "simulations" ]; then
            cd simulations

            # 1. Compile Rust to WASM
            cargo build --release --target wasm32-unknown-unknown

            # 2. Bindgen: Generate the JS glue code
            # We output directly to Zola's static folder
            ${pkgs.wasm-bindgen-cli}/bin/wasm-bindgen \
              --out-dir ../static/wasm \
              --target web \
              --no-typescript \
              target/wasm32-unknown-unknown/release/spinning_cube.wasm

              cd ..
          else 
            echo "No 'simulations' folder found, skipping WASM build."
          fi

          echo ">> Building Zola Site..."
          ${pkgs.zola}/bin/zola build
        '';

      in
      {
        # Development Shell
        # Run `nix develop` to get access to zola, cargo, and wasm-pack
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            zola
            rustToolchain
            wasm-pack
            wasm-bindgen-cli
            binaryen
            # Bevy dependencies
            pkg-config
            alsa-lib
            udev
            vulkan-loader
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
          ];

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (
            with pkgs;
            [
              alsa-lib
              udev
              vulkan-loader
              xorg.libX11
              xorg.libXcursor
              xorg.libXi
              xorg.libXrandr
            ]
          );

          shellHook = ''
            echo "------------------------------------------------"
            echo "Tools loaded: Zola, Rust (w/ WASM), wasm-pack"
            echo "------------------------------------------------"
          '';
        };

        # Run `nix run` to execute the build script defined above
        apps.default = flake-utils.lib.mkApp {
          drv = buildScript;
        };
      }
    );
}
