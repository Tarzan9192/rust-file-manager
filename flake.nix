{
  description = "An empty flake template that you can adapt to your own environment";

  # Flake inputs
  inputs.nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/*.tar.gz";

  # Flake outputs
  outputs = { self, nixpkgs }:
    let
      # The systems supported for this flake
      supportedSystems = [
        "x86_64-linux" # 64-bit Intel/AMD Linux
        "aarch64-linux" # 64-bit ARM Linux
        "x86_64-darwin" # 64-bit Intel macOS
        "aarch64-darwin" # 64-bit ARM macOS
      ];

      # Helper to provide system-specific attributes
      forEachSupportedSystem = f: nixpkgs.lib.genAttrs supportedSystems (system: f {
        pkgs = import nixpkgs { inherit system; };
      });
    in
    {
      devShells = forEachSupportedSystem ({ pkgs }: 
        let
          # Things i need at build time
          nativeBuildInputs = with pkgs; [];

          # Things i need at runtime
          buildInputs = with pkgs; [];

          # Define your package list once
          myPackages = with pkgs; [
            # common build inputs
            direnv
            xclip
            duf
            eza
            fd

            # project-specific build inputs go here
            rustup
            # sqlite
            # openssl
            # pkg-config
            # tailwindcss

            # {{{ Add these as needed

              ## NOTE: If you are building graphical applications, 
              # you will likely need wayland support from the below dependencies!
               
              #cmake
              #pkg-config
              #fontconfig
              #libxkbcommon
              #libGL

              ## WINIT_UNIX_BACKEND=wayland
              #wayland
              
              ## WINIT_UNIX_BACKEND=x11
              #xorg.libXcursor
              #xorg.libXrandr
              #xorg.libXi
              #xorg.libX11

              ## contains gcc compiler, among other things
              #libgccjit

            # }}}

            # for fun
            cowsay
            lolcat
          ];
        in {
          default = pkgs.mkShell {

            packages = myPackages;

            # Set any environment variables for your dev shell
            env = {
              GREETING = "Welcome to your Rust environment!";

              # Make sure all libraries are added to the PATH
              LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath myPackages;
            };

            # Add any shell logic you want executed any time the environment is activated
            shellHook = ''

              # Welcome message for shell
              echo $GREETING | cowsay | lolcat

              # Set the path where 'cargo install' installs packages
              # export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin

              #alias ls=eza
              #alias ls="ls -alh --color=auto"
              alias ls="eza --long --group --header -a --classify --links --level=3 --color=auto --sort=type --time-style=long-iso --extended"
              alias find=fd
              alias fd="fd --hidden --list-details --color=auto" # cannot be aliased to 'find' if using hlissner doom emacs
              #alias fd="fd --hidden --no-ignore --follow --list-details --color=auto" # cannot be aliased to 'find' if using hlissner doom emacs
              #alias fd="find -L" # cannot be aliased if using hlissner doom emacs
              alias du="duf"

              #git
              alias gst="git status"
              alias gc="git commit"
              alias gcm="git commit -m"
              alias ga="git add"
              alias gaa="git add --all"
              alias gcl="git clone -v --progress"
              alias gb="git branch"
              alias gp="git push -u"
              alias gpu="git push -u"

              # cargo
              alias cru="cargo run"
              alias cb="cargo build"
              alias chk="cargo check"

              #import parent shell config
              [ -x ~/.bashrc ] && source ~/.bashrc
              [ -x ~/.zshrc ] && source ~/.zshrc
            '';
          };
        });
    };
}
