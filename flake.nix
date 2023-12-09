{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }: flake-utils.lib.eachSystem [ "x86_64-linux" ] (system : let
    overlays = [ (import rust-overlay) ];
    pkgs = import nixpkgs { 
      inherit system overlays; 
      config.allowUnfree = true;
    };
    rust_toolchain = pkgs.rust-bin.beta.latest.default.override {
      extensions = ["rust-src"];
    };
    in
  {
    devShells.default = pkgs.mkShell rec {
      CARGO_INSTALL_ROOT = "${toString ./.}/.cargo";

      buildInputs = with pkgs; [ libxkbcommon wayland libGL xorg.libX11 xorg.libXcursor xorg.libXrandr xorg.libXi xorg.libXft
      xorg.libXinerama];
      nativeBuildInputs = with pkgs; [ rust_toolchain rust-analyzer git pkg-config lldb vscode-fhs] ++ buildInputs;
      LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
      XDG_DATA_DIRS = builtins.getEnv "XDG_DATA_DIRS";
    };
  });
}
