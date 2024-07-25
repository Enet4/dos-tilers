//! Module for the logic of setting up and moving the tiles of the puzzle
use alloc::vec::Vec;

use tinyrand::RandRange;

/// The full state of the tiles,
/// including dimensions and how the tiles are arranged in them.
///
/// Tiles are arranged like this:
/// ```none
/// +---+---+---+---+
/// | 0 | 1 | 2 | 3 |
/// +---+---+---+---+
/// | 4 | 5 | 6 | 7 |
/// +---+---+---+---+
/// | 8 | 9 |10 |11 |
/// +---+---+---+---+
/// ```
/// When the tiles are in this order, the game is won.
///
/// The tile of index 0 is assumed to be the empty tile,
/// but the x and y coordinate is saved separately.
#[derive(Debug, PartialEq)]
pub struct Tiles {
    /// the indices of the individual tiles,
    /// in column-first order
    pub tiles: Vec<u8>,
    pub rows: u8,
    pub cols: u8,

    /// the x coordinate of the empty tile
    pub empty_x: u8,
    /// the y coordinate of the empty tile
    pub empty_y: u8,
}

impl Tiles {
    /// Create a new tile state with the given dimensions,
    /// shuffling it in the process.
    ///
    /// More iterations means more randomness.
    pub fn new_shuffled(
        cols: u8,
        rows: u8,
        rng: &mut impl RandRange<u16>,
        iterations: u32,
    ) -> Self {
        let mut tiles = Tiles::new(cols, rows);
        tiles.shuffle(rng, iterations);
        tiles
    }

    /// Create a new tile state with the given dimensions,
    /// in a winning condition.
    ///
    /// Remember to shuffle afterwards.
    pub fn new(cols: u8, rows: u8) -> Self {
        // disallow 0 rows or 0 columns
        assert!(cols > 0);
        assert!(rows > 0);
        // disallow more than 16 rows or 16 columns
        assert!(cols <= 16);
        assert!(rows <= 16);

        Tiles {
            tiles: (0..rows * cols).collect(),
            rows,
            cols,
            empty_x: 0,
            empty_y: 0,
        }
    }

    /// Get the x,y coordinates that the tile
    /// currently at the given coordinates is supposed to be at
    /// when the puzzle is solved.
    ///
    /// # Panic
    ///
    /// Panics if the index is out of bounds.
    pub fn position_of(&self, current_x: u8, current_y: u8) -> (u8, u8) {
        let index = current_y as u32 * self.cols as u32 + current_x as u32;
        let tile_num = self.tiles[index as usize];

        (tile_num % self.cols, tile_num / self.cols)
    }

    /// Get the x,y coordinates of the tile with the given index.
    ///
    /// This does a linear search over the tiles.
    #[inline]
    pub fn where_is(&self, tile_num: u16) -> (u8, u8) {
        let index = self
            .tiles
            .iter()
            .position(|&tile| tile == tile_num as u8)
            .unwrap();
        (index as u8 % self.cols, index as u8 / self.cols)
    }

    pub fn is_won(&self) -> bool {
        self.tiles
            .iter()
            .enumerate()
            .all(|(i, &tile)| i == tile as usize)
    }

    /// Test whether a move can be done in the current state.
    pub fn is_valid_move(&mut self, r#move: Move) -> bool {
        match r#move {
            Move::Up => self.empty_y < self.rows - 1,
            Move::Down => self.empty_y > 0,
            Move::Left => self.empty_x < self.cols - 1,
            Move::Right => self.empty_x > 0,
        }
    }

    /// Apply a move to the tiles.
    ///
    /// If the operation is invalid,
    /// the tiles are left unchanged
    /// and `false` is returned.
    pub fn do_move(&mut self, r#move: Move) -> bool {
        if !self.is_valid_move(r#move) {
            return false;
        }

        let i = self.empty_y as usize * self.cols as usize + self.empty_x as usize;
        match r#move {
            Move::Up => {
                self.tiles.swap(i, i + self.cols as usize);
                self.empty_y += 1;
            }
            Move::Down => {
                self.tiles.swap(i, i - self.cols as usize);
                self.empty_y -= 1;
            }
            Move::Left => {
                self.tiles.swap(i, i + 1);
                self.empty_x += 1;
            }
            Move::Right => {
                self.tiles.swap(i, i - 1);
                self.empty_x -= 1;
            }
        }
        true
    }

    /// Shuffle the tiles by performing random moves.
    ///
    /// More iterations means more randomness.
    pub fn shuffle(&mut self, rng: &mut impl RandRange<u16>, iterations: u32) {
        for _ in 0..iterations {
            let r#move = match rng.next_range(0..4) {
                0 => Move::Up,
                1 => Move::Down,
                2 => Move::Left,
                3 => Move::Right,
                _ => unreachable!(),
            };
            self.do_move(r#move);
        }
    }
}

/// A player movement of a tile towards the empty slot.
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum Move {
    /// move the lower tile upwards
    Up,
    /// move the upper tile downwards
    Down,
    /// move the right tile to the left
    Left,
    /// move the left tile to the right
    Right,
}
