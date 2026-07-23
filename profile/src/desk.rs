//! A desk and an open laptop, drawn in front of the character during a
//! `desk` beat — a custom "at work" scene instead of the stock `stats`
//! terminal. Reuses the `present` pose (stationary, centred) underneath: the
//! desk just masks his lower half, the same trick `wall` and `stats` already
//! use to dress up a beat without touching the core rig.

use image::RgbaImage;

use crate::draw::fill;

const DESK_TOP: [u8; 3] = [150, 110, 78];
const DESK_FACE: [u8; 3] = [120, 84, 58];
const DESK_LEG: [u8; 3] = [90, 62, 42];
const LAPTOP_BODY: [u8; 3] = [40, 42, 50];
const LAPTOP_KEYS: [u8; 3] = [58, 61, 72];
const SCREEN_GLOW: [u8; 3] = [140, 190, 255];

/// Paint the desk and laptop. `w` is the canvas width, `ground` the y of the
/// ground line — the same two values `sky::paint` already gets.
pub fn draw(img: &mut RgbaImage, w: u32, ground: u32) {
    let cx = w / 2;

    // Desk: a tabletop with a front face and two legs, centred under him —
    // low enough to clear his eyes, so the laptop screen only ever covers
    // his chest and below.
    let top_y = ground.saturating_sub(40);
    let (dw, dh) = (380u32, 14u32);
    let dx = cx.saturating_sub(dw / 2);
    fill(img, dx, top_y, dw, dh, DESK_TOP);
    fill(img, dx, top_y + dh, dw, 6, DESK_FACE);
    fill(
        img,
        dx + 14,
        top_y + dh + 6,
        10,
        ground - (top_y + dh + 6),
        DESK_LEG,
    );
    fill(
        img,
        dx + dw - 24,
        top_y + dh + 6,
        10,
        ground - (top_y + dh + 6),
        DESK_LEG,
    );

    // Laptop: keyboard deck resting on the desk, screen standing up in front
    // of him, glowing — he's looking down at it, not through it.
    let base_y = top_y - 4;
    let (base_w, base_h) = (112u32, 8u32);
    let bx = cx.saturating_sub(base_w / 2);
    fill(img, bx, base_y, base_w, base_h, LAPTOP_KEYS);

    let screen_h = 40u32;
    let screen_w = 96u32;
    let sx = cx.saturating_sub(screen_w / 2);
    let sy = base_y.saturating_sub(screen_h);
    fill(img, sx, sy, screen_w, screen_h, LAPTOP_BODY);
    fill(
        img,
        sx + 6,
        sy + 6,
        screen_w - 12,
        screen_h - 12,
        SCREEN_GLOW,
    );
}
