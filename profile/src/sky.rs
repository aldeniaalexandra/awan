//! Optional backdrop theming: a day or night sky, and simple weather, painted
//! behind the character instead of the flat backdrop. Both fields default to
//! empty in `Profile`, and an empty `sky` skips this whole module — existing
//! profiles keep rendering exactly as they always have.
//!
//! No clouds drawn here: the reel already parallax-drifts its own ambient
//! clouds over every scene (see `awan_core::reel::clouds`), so a second,
//! static set would just double up. Cloudier weather instead dims and
//! greys the gradient itself.

use image::RgbaImage;

use crate::draw::{fill, mix};
use crate::gif::BG;

const NIGHT_TOP: [u8; 3] = [10, 10, 31];
const NIGHT_BOTTOM: [u8; 3] = [27, 24, 64];
const DAY_TOP: [u8; 3] = [61, 111, 214];
const DAY_BOTTOM: [u8; 3] = [169, 212, 255];
const OVERCAST_GREY: [u8; 3] = [90, 96, 110];
const STAR: [u8; 3] = [201, 188, 255];
const MOON: [u8; 3] = [242, 238, 203];
const SUN: [u8; 3] = [255, 217, 92];
const RAIN: [u8; 3] = [143, 184, 255];
const FOG: [u8; 3] = [207, 208, 230];

/// Paint the backdrop for the sky band `0..h` at tick `t`. Falls back to the
/// plain `BG` fill when `sky` is empty, so an unset profile is unaffected.
pub fn paint(img: &mut RgbaImage, w: u32, h: u32, sky: &str, weather: &str, t: i32) {
    if sky.is_empty() {
        fill(img, 0, 0, w, h, BG);
        return;
    }
    let night = sky == "night";
    let (mut top, mut bottom) = if night {
        (NIGHT_TOP, NIGHT_BOTTOM)
    } else {
        (DAY_TOP, DAY_BOTTOM)
    };
    // Overcast, rain, storm, and fog all mute the gradient toward grey — the
    // heavier the weather, the flatter the sky.
    let overcast_pct = match weather {
        "overcast" | "fog" => 55,
        "rain" => 45,
        "storm" => 65,
        "cloudy" => 20,
        _ => 0,
    };
    if overcast_pct > 0 {
        top = mix(top, OVERCAST_GREY, overcast_pct);
        bottom = mix(bottom, OVERCAST_GREY, overcast_pct / 2);
    }
    gradient(img, w, h, top, bottom);

    let raining = matches!(weather, "rain" | "storm");
    if night {
        stars(img, w, t, overcast_pct);
        moon(img, w);
    } else if !raining && overcast_pct < 50 {
        sun(img, w);
    }
    if raining {
        rain(img, w, h, t);
    }
    if weather == "fog" {
        fog(img, w, h);
    }
}

fn gradient(img: &mut RgbaImage, w: u32, h: u32, top: [u8; 3], bottom: [u8; 3]) {
    for y in 0..h {
        let pct = y * 100 / h.max(1);
        fill(img, 0, y, w, 1, mix(top, bottom, pct));
    }
}

/// Eight fixed points, each blinking on its own slow, tick-driven phase — no
/// randomness, so two runs of the same tick always match. Heavier weather
/// mutes them (or hides them under thick overcast).
fn stars(img: &mut RgbaImage, w: u32, t: i32, overcast_pct: u32) {
    if overcast_pct >= 55 {
        return;
    }
    let pts: [(u32, u32); 8] = [
        (40, 18),
        (95, 30),
        (160, 14),
        (230, 34),
        (40, 45),
        (300, 20),
        (340, 40),
        (200, 50),
    ];
    for (i, (fx, y)) in pts.iter().enumerate() {
        let x = fx * w / 360;
        let lit = (t / 6 + i as i32) % 4 != 0;
        let c = if lit { STAR } else { mix(STAR, NIGHT_TOP, 70) };
        fill(img, x, *y, 2, 2, c);
    }
}

fn moon(img: &mut RgbaImage, w: u32) {
    let x = w.saturating_sub(w / 8);
    fill(img, x, 10, 14, 14, MOON);
}

fn sun(img: &mut RgbaImage, w: u32) {
    let x = w / 2;
    fill(img, x, 10, 14, 14, SUN);
}

/// Straight falling drops, each column re-seeded by tick so they scroll down
/// the frame without needing any per-frame state.
fn rain(img: &mut RgbaImage, w: u32, h: u32, t: i32) {
    let cols: i32 = 14;
    let h_i = h as i32;
    for i in 0..cols {
        let x = ((i * w as i32 / cols + (i * 37) % 17) % w.max(1) as i32) as u32;
        let y = ((t * 5 + i * 23) % (h_i + 20) - 10).max(0) as u32;
        fill(img, x, y.min(h.saturating_sub(2)), 1, 6, RAIN);
    }
}

fn fog(img: &mut RgbaImage, w: u32, h: u32) {
    let y0 = h * 3 / 5;
    for y in y0..h {
        let pct = (y - y0) * 30 / (h - y0).max(1);
        for x in (0..w).step_by(2) {
            let existing = img.get_pixel(x, y).0;
            let c = mix([existing[0], existing[1], existing[2]], FOG, 20 + pct);
            fill(img, x, y, 2, 1, c);
        }
    }
}
