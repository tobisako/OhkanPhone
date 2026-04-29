use macroquad::prelude::*;
use crate::{PLAY_X, PLAY_W, PLAY_Y, PLAY_H};

// g_abe.png: 143x186 native, Flash stage = 540x960 → display at native size
pub const PLAYER_W: f32 = 143.0;
pub const PLAYER_H: f32 = 186.0;
const SPEED: f32 = 180.0;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub moving_left: bool,
    pub moving_right: bool,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: PLAY_X + PLAY_W / 2.0 - PLAYER_W / 2.0,
            y: PLAY_Y + PLAY_H - PLAYER_H - 20.0,
            moving_left: false,
            moving_right: false,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.moving_left  { self.x -= SPEED * dt; }
        if self.moving_right { self.x += SPEED * dt; }
        self.x = self.x.clamp(PLAY_X, PLAY_X + PLAY_W - PLAYER_W);
    }

    // Crown lands centered on player's horizontal center, 9px below sprite top
    pub fn head_x(&self) -> f32 { self.x + PLAYER_W / 2.0 }
    pub fn head_y(&self) -> f32 { self.y + 9.0 }

    pub fn mount_rect(&self) -> Rect {
        Rect::new(self.x + 10.0, self.y - 10.0, PLAYER_W - 20.0, 22.0)
    }

    pub fn body_rect(&self) -> Rect {
        Rect::new(self.x + 5.0, self.y, PLAYER_W - 10.0, PLAYER_H)
    }
}
