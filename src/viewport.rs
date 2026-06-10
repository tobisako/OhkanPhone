use macroquad::prelude::*;
use crate::{SCREEN_W, SCREEN_H};

// Letterbox view: maps the fixed 600x1200 world onto any screen size, centered.
pub struct View {
    pub scale: f32,
    pub ox: f32,
    pub oy: f32,
}

pub fn view() -> View {
    let sw = screen_width();
    let sh = screen_height();
    let scale = (sw / SCREEN_W).min(sh / SCREEN_H);
    View {
        scale,
        ox: (sw - SCREEN_W * scale) * 0.5,
        oy: (sh - SCREEN_H * scale) * 0.5,
    }
}

// Camera mapping world (0,0,600,1200) onto the letterboxed viewport, y-down like screen space.
// zoom.y must be POSITIVE here: macroquad's screen pass already flips y, a negative
// value double-flips and renders the whole game upside down.
// Camera2D.viewport goes straight to glViewport (PHYSICAL px), while screen_width()
// and mouse_position() are logical px — multiply by dpi_scale or high-DPI phones
// render the game quarter-sized in the bottom-left corner.
// Viewport offsets are symmetric (centered), so the GL bottom-left origin needs no flip.
pub fn apply_camera() {
    let v = view();
    let dpi = macroquad::miniquad::window::dpi_scale();
    set_camera(&Camera2D {
        target: vec2(SCREEN_W / 2.0, SCREEN_H / 2.0),
        zoom: vec2(2.0 / SCREEN_W, 2.0 / SCREEN_H),
        viewport: Some((
            (v.ox * dpi).round() as i32,
            (v.oy * dpi).round() as i32,
            (SCREEN_W * v.scale * dpi).round() as i32,
            (SCREEN_H * v.scale * dpi).round() as i32,
        )),
        ..Default::default()
    });
}

fn to_world(p: Vec2) -> Vec2 {
    let v = view();
    vec2((p.x - v.ox) / v.scale, (p.y - v.oy) / v.scale)
}

// Pointer position in world coords. Touch is mirrored into mouse by macroquad,
// so this covers both mouse and single-touch.
pub fn pointer_world() -> Vec2 {
    let (mx, my) = mouse_position();
    to_world(vec2(mx, my))
}

// All currently active touch points in world coords (multi-touch: hold both buttons).
// touches() returns RAW physical px (macroquad divides by dpi for mouse_position
// but not for touches) — normalize to logical px before mapping to world.
pub fn touches_world() -> Vec<Vec2> {
    let dpi = macroquad::miniquad::window::dpi_scale();
    touches()
        .into_iter()
        .filter(|t| !matches!(t.phase, TouchPhase::Ended | TouchPhase::Cancelled))
        .map(|t| to_world(t.position / dpi))
        .collect()
}

// One-shot tap position this frame (touch start or mouse press), in world coords.
pub fn tap_world() -> Option<Vec2> {
    if let Some(t) = touches().iter().find(|t| t.phase == TouchPhase::Started) {
        let dpi = macroquad::miniquad::window::dpi_scale();
        return Some(to_world(t.position / dpi));
    }
    if is_mouse_button_pressed(MouseButton::Left) {
        return Some(pointer_world());
    }
    None
}
