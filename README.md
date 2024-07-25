# Tilers

[![ci](https://github.com/Enet4/dos-tilers/actions/workflows/ci.yml/badge.svg)](https://github.com/Enet4/dos-tilers/actions/workflows/ci.yml)

A relaxing tile-based puzzle game for MS-DOS.

Created for the [DOS Games July 2024 Jam](https://itch.io/jam/dos-games-july-2024-jam).

## Playing

Use the arrow keys (or WASD)
to move a tile to the empty slot.
Rearrange the pieces until the picture is fully organized.
Press ESC to exit.


## Building

First you need:

- Rust, preferably installed via Rustup.
  It will install the right toolchain via [rust-toolchain.toml](./rust-toolchain.toml).
- `elf2djgpp`, available on [this repository](https://github.com/cknave/elf2djgpp)
- The [DJGPP GCC toolchain](https://www.delorie.com/djgpp)
  (version 14.1.0 is known to work, but it should also work with v12).

Then:

- If your DJGPP toolchain is not named `i686-pc-msdosdjgpp-gcc`,
  set the environment variable `CC` to the right path.
- Set `ARCH` depending on the target architecture intended
  (default is `i386`)
- Run `./build.sh` or `./build.sh release`
  (the latter builds with optimizations)

You will find the .exe file in `build/debug/` or `build/release/`.

## Running

Add the resulting `TILERS.EXE` alongside `CWSDPMI.EXE`
to your DOS machine or emulator.
The absolute minimum requirements are
an i386 with a VGA display.

```bat
TILERS
```

To run the game without PC speaker sound,
append `nosound` to the command line arguments:

```bat
TILERS nosound
```

To change the initial disposition of the tiles,
append an integer to the command line arguments for the seed.

```bat
TILERS 123456
```
