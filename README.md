# Tilers

[![ci](https://github.com/Enet4/dos-tilers/actions/workflows/ci.yml/badge.svg)](https://github.com/Enet4/dos-tilers/actions/workflows/ci.yml)

A relaxing tile permutation puzzle game for MS-DOS.

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
  (default is `i486`)
- Run `./build.sh` or `./build.sh release`
  (the latter builds with optimizations)

You will find the .exe file in `build/debug/` or `build/release/`.

## Running

Add the resulting `TILERS.EXE` alongside `CWSDPMI.EXE`
to your DOS machine or emulator.
The absolute minimum requirements are
an i486 with a VGA display.

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

## Modding the game

You can add your own pictures too!
If you add PNG files with the name `#.PNG`
(where `#` is the level number starting with 1)
next to `TILERS.EXE`
they will be picked up automatically and used as the picture for that level.
For instance, if your directory structure is like this:

```none
dir

.                <DIR>
..               <DIR>
1         PNG
CWSDPMI   EXE
TILERS    EXE
```

Then the first level will use the image file in `1.png`
instead of the default picture for level 1
(the _Monet_ painting _La plage à Pourville, soleil couchant_).

Moreover, you can do this to extend the number of levels available!
Add `4.png` to unlock level 4,
`5.png` for a fifth level,
and so on.

Note however, that there are restrictions in place for the image:

- It must be a valid PNG image file in 8-bit indexed color mode
- It must have the exact resolution of 320x200
- It should not have more than 240 colors in total
  (the last few colors are reserved to the application
  and may affect presentation if this is ignored)
- And of course, not all pictures are visually appropriate
  for this kind of puzzle.
  The program will not check for identical tiles,
  so make it visually appealing as well as feasible to play!

You can use tools such as ImageMagick to adapt your image:

```sh
magick in.png -resize 320x200 +dither -colors 240 PNG8:4.png
```

The command above works best when the input image is 4:3 in aspect ratio.

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
