//! A desk and an open laptop, drawn in front of the character during a
//! `desk` beat — a custom "at work" scene instead of the stock `stats`
//! terminal. Reuses the `present` pose (stationary, centred) underneath: the
//! desk just masks his lower half, the same trick `wall` and `stats` already
//! use to dress up a beat without touching the core rig.
//!
//! Every block here snaps to the character's own pixel unit — 66×30, one
//! sprite-pixel of his own art — so the desk reads as part of the same scene
//! instead of a smoother, differently-scaled prop pasted over it. His eyes
//! sit in the 30px row right above the screen; the screen never reaches it.

use image::RgbaImage;

use crate::draw::fill;

/// One sprite-pixel, matching the character's own art (2 grid cells wide, 1
/// grid row tall).
const UNIT_W: u32 = 66;
const UNIT_H: u32 = 30;

const DESK_TOP: [u8; 3] = [150, 110, 78];
const DESK_FACE: [u8; 3] = [120, 84, 58];
const DESK_LEG: [u8; 3] = [90, 62, 42];
const BACKDROP: [u8; 3] = [70, 60, 66];
const LAPTOP_HINGE: [u8; 3] = [40, 42, 50];
const SCREEN_GLOW: [u8; 3] = [140, 190, 255];

/// Paint the desk and laptop. `w` is the canvas width, `ground` the y of the
/// ground line — the same two values `sky::paint` already gets.
pub fn draw(img: &mut RgbaImage, w: u32, ground: u32) {
    let cx = w / 2;

    // Three stacked unit-rows, ground up: legs, desk, screen. His eyes are
    // the row right above the screen row — this stops exactly at their
    // bottom edge, never into it.
    let legs_y = ground - UNIT_H;
    let desk_y = legs_y - UNIT_H;
    let screen_y = desk_y - UNIT_H;

    // Desk: 6 units wide, one unit tall — a lighter top strip over a darker
    // face, on two legs that fill the row below.
    let dw = UNIT_W * 6;
    let dx = cx.saturating_sub(dw / 2);
    fill(img, dx, desk_y, dw, UNIT_H / 3, DESK_TOP);
    fill(
        img,
        dx,
        desk_y + UNIT_H / 3,
        dw,
        UNIT_H - UNIT_H / 3,
        DESK_FACE,
    );
    fill(img, dx + UNIT_W / 2, legs_y, UNIT_W / 3, UNIT_H, DESK_LEG);
    fill(
        img,
        dx + dw - UNIT_W / 2 - UNIT_W / 3,
        legs_y,
        UNIT_W / 3,
        UNIT_H,
        DESK_LEG,
    );

    // A full-width backdrop behind the laptop, same width as the desk — so
    // his own body never shows either side of the screen. Without it the
    // screen reads as a mouth cut into him instead of a laptop in front of
    // him: it has to fully replace that row, not just sit inside it.
    fill(img, dx, screen_y, dw, UNIT_H, BACKDROP);

    // Laptop: the screen fills the free unit-row above the desk, a glowing
    // panel inside a dark hinge/bezel, sitting flush on the desktop.
    let sw = UNIT_W * 2;
    let sx = cx.saturating_sub(sw / 2);
    fill(img, sx, screen_y, sw, UNIT_H, LAPTOP_HINGE);
    fill(
        img,
        sx + UNIT_W / 6,
        screen_y + UNIT_H / 6,
        sw - UNIT_W / 3,
        UNIT_H - UNIT_H / 3,
        SCREEN_GLOW,
    );
}
