use macroquad::prelude::*;
use crate::SCREEN_H;

// g_ohkan.png: 89x70 native, Flash stage = 540x960 → display at native size
pub const CROWN_W: f32 = 89.0;
pub const CROWN_H: f32 = 70.0;
const GRAVITY: f32 = 45.0;

// 0=falling, 1=mounted, 2=ejected
pub struct Crown {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub rotation: f32,
    pub mode: u8,
    pub mount_index: usize,
    pub alive: bool,
}

impl Crown {
    pub fn new(x: f32) -> Self {
        Self { x, y: -CROWN_H, vx: 0.0, vy: 0.0, rotation: 0.0, mode: 0, mount_index: 0, alive: true }
    }

    pub fn mount(&mut self, index: usize) {
        self.mode = 1;
        self.mount_index = index;
        self.vx = 0.0;
        self.vy = 0.0;
        self.rotation = 0.0;
    }

    pub fn eject(&mut self, vx: f32, vy: f32) {
        self.mode = 2;
        self.vx = vx;
        self.vy = vy;
    }

    pub fn update(&mut self, dt: f32, head_x: f32, head_y: f32) {
        match self.mode {
            0 => {
                self.vy += GRAVITY * dt;
                self.y += self.vy * dt;
                if self.y > SCREEN_H + CROWN_H { self.alive = false; }
            }
            1 => {
                self.x = head_x - CROWN_W / 2.0;
                self.y = head_y - CROWN_H - self.mount_index as f32 * (CROWN_H + 2.0);
            }
            2 => {
                self.vy += GRAVITY * dt;
                self.vx *= 1.0 - dt * 0.8;
                self.x += self.vx * dt;
                self.y += self.vy * dt;
                self.rotation += 270.0 * dt;  // 45°/frame @6fps equiv
                if self.y > SCREEN_H + CROWN_H { self.alive = false; }
            }
            _ => {}
        }
    }

    pub fn collision_rect(&self) -> Rect {
        Rect::new(self.x + 3.0, self.y + 2.0, CROWN_W - 6.0, CROWN_H - 4.0)
    }
}
