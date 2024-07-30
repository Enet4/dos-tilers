//! Simple game stats

/// Total number of tile moves
static mut TOTAL_MOVES: u32 = 0;

#[inline]
pub fn add_move() {
    // DOS is single threaded so...
    unsafe {
        TOTAL_MOVES += 1;
    }
}

#[inline]
pub fn total_moves() -> u32 {
    unsafe {
        TOTAL_MOVES
    }
}
