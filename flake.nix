{
  description = "Personal Website + Resume PDF Development Environment and Automation";

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

        # Rust toolchain
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
          targets = [ "wasm32-unknown-unknown" ];
        };

        # Build WASM Simulations
        buildSimulations = pkgs.writeShellScriptBin "build-simulations" ''
          set -euo pipefail
          # Outside a `nix develop` shell (e.g. plain `nix run` in CI) nothing
          # puts the pinned toolchain on PATH, so a bare `cargo` falls back to
          # whatever the runner happens to have, which lacks the
          # wasm32-unknown-unknown std. cargo also shells out to `rustc` by
          # bare name internally, so it must be on PATH too, not just cargo.
          export PATH="${rustToolchain}/bin:$PATH"
          echo ">> Building Rust Simulations..."
          if [ -d "simulations" ]; then
            cd simulations

            # Compile every workspace member (spinning_cube, game_of_life) in one pass.
            cargo build --release --target wasm32-unknown-unknown

            # Bindgen: Generate the JS glue code for each simulation.
            ${pkgs.wasm-bindgen-cli}/bin/wasm-bindgen \
              --out-dir ../static/wasm \
              --target web \
              --no-typescript \
              target/wasm32-unknown-unknown/release/spinning_cube.wasm

            ${pkgs.wasm-bindgen-cli}/bin/wasm-bindgen \
              --out-dir ../static/wasm \
              --target web \
              --no-typescript \
              target/wasm32-unknown-unknown/release/game_of_life.wasm

              cd ..
          else
            echo "No 'simulations' folder found, skipping WASM build."
          fi
        '';

        # Build Resume
        buildResume = pkgs.writeShellScriptBin "build-resume" ''
          set -euo pipefail
          echo ">> Building Resume..."
          TYPST_FONT_PATHS="${pkgs.font-awesome}/share/fonts" ${pkgs.typst}/bin/typst compile --root . \
            --input RESUME_NAME="''${RESUME_NAME:-}" \
            --input RESUME_EMAIL="''${RESUME_EMAIL:-}" \
            --input RESUME_PHONE="''${RESUME_PHONE:-}" \
            resume/resume.typ static/resume.pdf
        '';

        # Build Zola Site
        buildZola = pkgs.writeShellScriptBin "build-zola" ''
          set -euo pipefail
          echo ">> Building Zola Site..."
          ${pkgs.zola}/bin/zola build
        '';

        # Default build script (runs all)
        buildSite = pkgs.writeShellScriptBin "build-site" ''
          set -euo pipefail
          ${buildSimulations}/bin/build-simulations
          ${buildResume}/bin/build-resume
          ${buildZola}/bin/build-zola
        '';

      in
      {
        # Development shell
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [

            # Task runner
            just

            # Website
            zola

            # Resume
            typst
            tinymist
            typstyle

            # Rust
            rustToolchain
            wasm-pack
            wasm-bindgen-cli
            binaryen
            # Bevy dependencies
            pkg-config
            alsa-lib
            udev
            vulkan-loader
            libx11
            libxcursor
            libxi
            libxrandr
          ];

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (
            with pkgs;
            [
              alsa-lib
              udev
              vulkan-loader
              libx11
              libxcursor
              libxi
              libxrandr
            ]
          );

          TYPST_FONT_PATHS = "${pkgs.font-awesome}/share/fonts";

          shellHook = ''
            echo "----------------------------------------------------"
            echo "Tools loaded: Zola, Rust (w/ WASM), wasm-pack, Typst"
            echo "----------------------------------------------------"
          '';
        };

        apps = {
          simulations = flake-utils.lib.mkApp {
            drv = buildSimulations;
          };
          resume = flake-utils.lib.mkApp {
            drv = buildResume;
          };
          zola = flake-utils.lib.mkApp {
            drv = buildZola;
          };
          default = flake-utils.lib.mkApp {
            drv = buildSite;
          };
        };
      }
    );
}
