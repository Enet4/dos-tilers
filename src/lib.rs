#![no_std]
#![no_main]
extern crate alloc;

mod audio;
mod tiles;

use audio::{is_sound_on, play_click, play_tune, sound_off};
use dos_x::djgpp::dos::delay;
use dos_x::key;
use dos_x::vga::Palette;
use tiles::{Move, Tiles};

use alloc::vec;
use core::panic::PanicInfo;
use dos_x::vga::vsync;
use dos_x::{djgpp::stdlib::exit, println};
use tinyrand::{RandRange, Seeded};

// embed images into the binary
static IMAGE_1_DATA: &[u8] = include_bytes!("../resources/1.png");
static IMAGE_2_DATA: &[u8] = include_bytes!("../resources/2.png");
static IMAGE_3_DATA: &[u8] = include_bytes!("../resources/3.png");

#[no_mangle]
fn dos_main() {
    // process inputs
    let mut starting_level = 0;
    let mut seed = 1;
    for arg in dos_x::argv() {
        unsafe {
            let arg = core::ffi::CStr::from_ptr(*arg);
            if arg.to_bytes() == b"nosound" {
                sound_off();
            } else if arg.to_bytes() == b"iknowwhatimdoing" {
                starting_level = 2;
            } else {
                // try to interpret it as an integer
                if let Ok(s) = core::str::from_utf8(arg.to_bytes()).unwrap().parse::<u64>() {
                    // use it as a seed for the rng
                    seed = s;
                    break;
                }
            }
        }
    }

    let rng = tinyrand::Xorshift::seed(seed);
    run(rng, starting_level);
}

fn run(mut rng: impl RandRange<u16>, starting_level: u8) {
    println!("Tilers by E_net4 (2024)");

    play_tune();

    unsafe {
        delay(100);
    }

    println!("Loading...");

    // load image for the first puzzle

    let mut buf = vec![0; 80_000];

    let mut image = match minipng::decode_png(IMAGE_1_DATA, &mut buf) {
        Ok(image) => image,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    dos_x::vga::set_video_mode_13h();

    // grab palette and apply it to VGA display
    let mut palette = Palette::new([0u8; 768]);

    let mut win = false;
    let mut level = starting_level;
    loop {
        unsafe {
            dos_x::vga::draw_rect(0, 0, 320, 200, 255);
        }

        if level > 0 {
            // load the next image

            let img_data = match level {
                0 => IMAGE_1_DATA,
                1 => IMAGE_2_DATA,
                _ => IMAGE_3_DATA,
            };

            image = match minipng::decode_png(img_data, &mut buf) {
                Ok(image) => image,
                Err(e) => {
                    unsafe {
                        dos_x::vga::set_video_mode(0x02);
                    }
                    println!("Error: {}", e);
                    unsafe {
                        exit(2);
                        unreachable!();
                    }
                }
            };
        }

        // set up palette
        let mut k = 0;
        for i in 0..=254 {
            let [r, g, b, _a] = image.palette(i);
            palette.0[k] = r >> 2;
            k += 1;
            palette.0[k] = g >> 2;
            k += 1;
            palette.0[k] = b >> 2;
            k += 1;
        }
        // ensure that the last color (#255) is always black.
        palette.0[765] = 0;
        palette.0[766] = 0;
        palette.0[767] = 0;
        // ensure that the second last color (#254) is always white.
        palette.0[762] = 63;
        palette.0[763] = 63;
        palette.0[764] = 63;
        palette.set();

        match game_level(&mut rng, level, &image.pixels()) {
            LevelOutcome::Exit => break,
            LevelOutcome::NextLevel => {
                // paint the whole picture without the empty slot
                unsafe {
                    vsync();
                    dos_x::vga::draw_buffer(&image.pixels());
                }
                unsafe {
                    if is_sound_on() {
                        play_tune();
                        delay(500);
                    } else {
                        delay(2_000);
                    }
                }

                level += 1;

                // fade out
                for _ in 0..64 {
                    unsafe {
                        for p in &mut palette.0 {
                            *p = p.saturating_sub(1);
                        }
                        vsync();
                        palette.set();
                    }
                }
            }
            LevelOutcome::Complete => {
                win = true;
                break;
            }
        }
    }

    // fade out

    for _ in 0..64 {
        unsafe {
            for p in &mut palette.0 {
                *p = p.saturating_sub(1);
            }
            vsync();
            palette.set();
        }
    }

    // set back to text mode
    unsafe {
        dos_x::vga::set_video_mode(0x02);
    }

    if win {
        println!("Congratulations! You have completed the game!");
    }

    println!("Thank you for playing Tilers (2024)");
}

/// What the game should do as the level ends
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
enum LevelOutcome {
    /// Exit the game
    Exit,
    /// Proceed to the next level
    /// (or just end with a congratulatory message if there are no more)
    NextLevel,
    /// Exit the game with a congraulatory message
    Complete,
}

fn game_level(rng: &mut impl RandRange<u16>, level: u8, picture: &[u8]) -> LevelOutcome {
    let (cols, rows) = match level {
        0 => (3, 2),
        1 => (4, 3),
        2 => (5, 4),
        _ => return LevelOutcome::Complete,
    };

    // decide the total width & height of the puzzle
    // as well as the size of the pieces
    let (width, tile_width) = (320, 320 / cols as u32);
    let (height, tile_height) = if rows == 3 {
        (198, 198 / rows as u32)
    } else {
        (200, 200 / rows as u32)
    };

    let picture = &picture[0..width as usize * height as usize];

    let mut tiles = Tiles::new_shuffled(cols, rows, &mut *rng, 512);

    unsafe {
        vsync();
    }

    draw_tiles_animated(&tiles, picture, (width, height), (tile_width, tile_height));

    loop {
        // - implement user input & tile movements

        // detect Left, Right, Up, Down key presses
        // (also W, A, S, D)
        let key = key::get_keypress();
        let m = match key {
            0x4b | 0x1e => Some(Move::Left),
            0x4d | 0x20 => Some(Move::Right),
            0x48 | 0x11 => Some(Move::Up),
            0x50 | 0x1f => Some(Move::Down),
            _ => None,
        };
        if let Some(m) = m {
            if !tiles.is_valid_move(m) {
                continue;
            }

            // move the tile with an animation
            animate_tile_move(
                &tiles,
                picture,
                (width, height),
                (tile_width, tile_height),
                m,
            );
            // apply the move proper
            tiles.do_move(m);
            // click!
            play_click();

            // check if the puzzle is solved
            if tiles.is_won() {
                return LevelOutcome::NextLevel;
            }

            // draw the updated tiles
            unsafe {
                vsync();
            }
        }

        unsafe {
            vsync();

            // implement exit (ESC)
            let key = key::get_keypress();
            if key == 0x01 {
                return LevelOutcome::Exit;
            }
        }
    }
}

fn animate_tile_move(
    tiles: &Tiles,
    picture: &[u8],
    (width, height): (u32, u32),
    (tile_width, tile_height): (u32, u32),
    m: Move,
) {
    // get the x,y coordinates of the tile being moved
    // (plus the direction of movement)
    let (x, y, delta_x, delta_y) = match m {
        Move::Up => (tiles.empty_x, tiles.empty_y + 1, 0, -1),
        Move::Down => (tiles.empty_x, tiles.empty_y - 1, 0, 1),
        Move::Left => (tiles.empty_x + 1, tiles.empty_y, -1, 0),
        Move::Right => (tiles.empty_x - 1, tiles.empty_y, 1, 0),
    };

    // determine the position of the tile to move in the picture
    let (pic_x, pic_y) = tiles.position_of(x, y);

    let origin = (
        pic_x as u32 * tile_width,
        pic_y as u32 * tile_height,
        tile_width,
        tile_height,
    );

    let x = x as i32 * tile_width as i32;
    let y = y as i32 * tile_height as i32;

    let mut d_x = 0;
    let mut d_y = 0;

    let amount = match m {
        Move::Up | Move::Down => tile_height,
        Move::Left | Move::Right => tile_width,
    };

    for _ in (1..=amount).step_by(2) {
        d_x += delta_x + delta_x;
        d_y += delta_y + delta_y;
        let target = (x + d_x, y + d_y);

        unsafe {
            vsync();
            dos_x::vga::blit_rect(picture, (width as u32, height as u32), origin, target);
        }
        // clear out the trailing space
        match m {
            Move::Up => unsafe {
                dos_x::vga::draw_hline(x, y + d_y as i32 + tile_height as i32, tile_width, 255);
                dos_x::vga::draw_hline(x, y + d_y as i32 + tile_height as i32 + 1, tile_width, 255);
            },
            Move::Down => unsafe {
                dos_x::vga::draw_hline(x, y + d_y as i32 - 1, tile_width, 255);
                dos_x::vga::draw_hline(x, y + d_y as i32 - 2, tile_width, 255);
            },
            Move::Left => unsafe {
                dos_x::vga::draw_vline(x + d_x as i32 + tile_width as i32, y, tile_height, 255);
                dos_x::vga::draw_vline(x + d_x as i32 + tile_width as i32 + 1, y, tile_height, 255);
            },
            Move::Right => unsafe {
                dos_x::vga::draw_vline(x + d_x as i32 - 1, y, tile_height, 255);
                dos_x::vga::draw_vline(x + d_x as i32 - 2, y, tile_height, 255);
            },
        }
        unsafe {
            delay(5);
        }
    }
}

/// Draw all tiles of the puzzle
///
/// Needed when the level starts.
fn draw_tiles_animated(
    tiles: &Tiles,
    picture: &[u8],
    (width, height): (u32, u32),
    (tile_width, tile_height): (u32, u32),
) {
    let cols = tiles.cols as u16;

    // draw puzzle tiles to the screen
    for k in 0..cols as u16 * tiles.rows as u16 {
        let (i, j) = tiles.where_is(k);
        let x = i as i32 * tile_width as i32;
        let y = j as i32 * tile_height as i32;

        if k == 0 {
            // draw a black rectangle instead
            unsafe {
                vsync();
                dos_x::vga::draw_rect(x, y, width as u32, height as u32, 255);
            }

            continue;
        }

        let tile_n = (k % cols) as u32;
        let tile_m = (k / cols) as u32;

        let target = (x, y);
        let origin = (
            tile_n * tile_width,
            tile_m * tile_height,
            tile_width as u32,
            tile_height as u32,
        );
        unsafe {
            vsync();
            dos_x::vga::blit_rect(picture, (width as u32, height as u32), origin, target);

            delay(50);
        }
    }
}

#[panic_handler]
fn handle_panic(_info: &PanicInfo) -> ! {
    unsafe {
        // reset video mode
        dos_x::vga::set_video_mode(0x02);
        // exit using libc
        exit(-1);
        core::hint::unreachable_unchecked()
    }
}
