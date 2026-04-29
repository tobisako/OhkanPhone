use macroquad::prelude::*;
use crate::SCREEN_H;

// g_kokeshi.png: 99x96 native, Flash stage = 540x960 → display at native size
pub const KOKESHI_W: f32 = 99.0;
pub const KOKESHI_H: f32 = 96.0;
const GRAVITY: f32 = 45.0;

// 0=falling, 1=fixed, 2=knockback
pub struct Kokeshi {
    pub x: f32,
    pub y: f32,
    vx: f32,
    vy: f32,
    pub rotation: f32,
    pub mode: u8,
    pub alive: bool,
}

impl Kokeshi {
    pub fn new(x: f32) -> Self {
        Self { x, y: -KOKESHI_H, vx: 0.0, vy: 0.0, rotation: 0.0, mode: 0, alive: true }
    }

    pub fn knockback(&mut self, dir: f32) {
        self.mode = 2;
        self.vx = dir * 180.0;
        self.vy = -300.0;
    }

    pub fn update(&mut self, dt: f32) {
        match self.mode {
            0 => {
                self.vy += GRAVITY * dt;
                self.y += self.vy * dt;
                self.rotation += 90.0 * dt;
                if self.y > SCREEN_H + KOKESHI_H { self.alive = false; }
            }
            1 => {}
            2 => {
                self.vy += GRAVITY * dt;
                self.vx *= 1.0 - dt * 1.5;
                self.x += self.vx * dt;
                self.y += self.vy * dt;
                self.rotation += 360.0 * dt;
                if self.y > SCREEN_H + KOKESHI_H { self.alive = false; }
            }
            _ => {}
        }
    }

    pub fn collision_rect(&self) -> Rect {
        Rect::new(self.x + 4.0, self.y + 2.0, KOKESHI_W - 8.0, KOKESHI_H - 4.0)
    }
}
