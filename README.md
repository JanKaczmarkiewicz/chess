# Rust canvas app starter

Quick starter for rust canvas based app. Uses https://github.com/Rust-SDL2/rust-sdl2

## Quick start Mac

prerequisites: 
* [homebrew](https://brew.sh/)
* [rust](https://www.rust-lang.org/tools/install)

```
cargo generate --git https://github.com/JanKaczmarkiewicz/rust-canvas-app

brew install sdl2 sdl2_image sdl2_ttf

# Add this line to your ~/.zshenv or ~/.bash_profile depending on whether you use ZSH or Bash.
export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"

cargo run
```

> in case trobuleshooting please refere to https://github.com/Rust-SDL2/rust-sdl2

## Quick start Linux

prerequisites: 
* [rust](https://www.rust-lang.org/tools/install)

```
cargo generate --git https://github.com/JanKaczmarkiewicz/rust-canvas-app

sudo apt-get install libsdl2-dev libsdl2-image-dev libsdl2-ttf-dev

cargo run
```

## vscode features

For best experiece please install recommeded extentions.