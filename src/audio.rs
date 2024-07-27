use dos_x::djgpp::{dos::delay, pc::{inportb, outportb}};

static mut NO_SOUND: bool = false;

// Hz
const PIT_FREQUENCY: u32 = 0x1234DD;

// helper macro for defining countdowns of notes as constants
macro_rules! const_note {
    ($name: ident, $freq: literal) => {
        const $name: u16 = (PIT_FREQUENCY / $freq) as u16;
    };
}

/// disable sound
pub fn sound_off() {
    unsafe {
        NO_SOUND = true;
    }
}

pub fn is_sound_on() -> bool {
    unsafe {
        !NO_SOUND
    }
}

/// Play a tune using PC speaker
pub fn play_tune() {
    if unsafe { NO_SOUND } {
        return;
    }

    // setting up a few constants for note countdowns
    const_note!(NOTE_A5, 880);
    const_note!(NOTE_CS6, 1109);
    const_note!(NOTE_D6, 1175);
    const_note!(NOTE_E6, 1328);
    const_note!(NOTE_GS6, 1661);
    const_note!(NOTE_A6, 1760);
    const_note!(NOTE_B6, 1975);

    const NOTE_VOID: u16 = 3;

    // use PC speaker
    unsafe {
        pc_speaker_on();

        // String Quintet in E Major, Op. 11, No. 5, G. 275: III. Minuetto by Boccherini
        play_note(NOTE_A6);
        delay(75);
        play_note(NOTE_B6);
        delay(75);
        play_note(NOTE_A6);
        delay(140);
        play_note(NOTE_GS6);
        delay(140);
        play_note(NOTE_A6);
        delay(140);
        play_note(NOTE_B6);
        delay(140);
        play_note(NOTE_A6);
        delay(140);

        play_note(NOTE_VOID);
        delay(140);

        play_note(NOTE_A5);
        delay(280);

        play_note(NOTE_VOID);
        delay(280);

        play_note(NOTE_CS6);
        delay(280);

        play_note(NOTE_VOID);
        delay(280);

        play_note(NOTE_E6);
        delay(140);
        play_note(NOTE_VOID);
        delay(140);
        play_note(NOTE_E6);
        delay(280);
        play_note(NOTE_D6);
        delay(140);
        play_note(NOTE_VOID);
        delay(140);
        play_note(NOTE_D6);
        delay(280);

        // turn off
        pc_speaker_off();
        delay(140);
    }
}

/// Play a click sound
pub fn play_click() {
    if unsafe { NO_SOUND } {
        return;
    }

    // use PC speaker
    unsafe {
        pc_speaker_on();

        // a note played fast enough will sound like a click
        let countdown = 1500;
        play_note(countdown);
        delay(4);

        // turn off
        pc_speaker_off();
    }
}

#[inline]
unsafe fn play_note(countdown: u16) {
    outportb(0x42, (countdown & 0xff) as u8);
    outportb(0x42, (countdown >> 8) as u8);
}

#[inline]
unsafe fn pc_speaker_on() {
    unsafe {
        let inb = inportb(0x61);
        outportb(0x61, inb | 3); // enable speaker
        outportb(0x43, 0xb6); // set PIT
    }
}

#[inline(always)]
unsafe fn pc_speaker_off() {
    outportb(0x61, 0);
}
