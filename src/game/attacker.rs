use macroquad::prelude::*;

// Flash stage = 540x960, sprites displayed at native pixel size (no scaling)
// g_big_kokeshi.png: 665x578 native, center-registered in AS3
// AS3 init center: (1080+665, 1100)=(1745,1100), end: (-100,-578)
// draw top-left = center - (W/2, H/2)
pub const BIG_KOKESHI_W: f32 = 665.0;
pub const BIG_KOKESHI_H: f32 = 578.0;

// g_megami.png: 653x717 native
// AS3 img offset: x = -width/4 = -163, y = -(height*3/4) = -538
// sprite registration (rotation pivot) is at offset (163, 538) from image top-left
pub const MEGAMI_W: f32 = 653.0;
pub const MEGAMI_H: f32 = 717.0;
pub const MEGAMI_PIVOT_OX: f32 = 163.0;
pub const MEGAMI_PIVOT_OY: f32 = 538.0;

#[derive(Clone, Copy, PartialEq)]
pub enum AttackerKind {
    BigKokeshi,
    Megami,
}

pub struct Attacker {
    pub kind: AttackerKind,
    pub x: f32,   // draw top-left x
    pub y: f32,   // draw top-left y
    pub rotation: f32,
    cooldown: f32,
    max_cooldown: f32,
    pub attack_timer: f32,
    pub active: bool,
    pub just_activated: bool,
}

impl Attacker {
    pub fn new(kind: AttackerKind, initial_cooldown: f32) -> Self {
        Self {
            kind, x: -2000.0, y: -2000.0, rotation: 0.0,
            cooldown: 0.0, max_cooldown: initial_cooldown,
            attack_timer: 0.0, active: false, just_activated: false,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if !self.active {
            self.cooldown += dt;
            if self.cooldown >= self.max_cooldown {
                self.cooldown = 0.0;
                self.attack_timer = 0.0;
                self.active = true;
                self.just_activated = true;
                self.rotation = 0.0;
                self.max_cooldown = match self.kind {
                    AttackerKind::BigKokeshi => 30.0,
                    AttackerKind::Megami     => 35.0,
                };
            }
            return;
        }

        self.attack_timer += dt;

        match self.kind {
            AttackerKind::BigKokeshi => {
                // GameMain-local (1745,1100)→(-100,-578), scale x*540/972+30, y*960/984+80
                // center start=(1000,1153)→(586,1207), draw=center-(332.5,289)=(254,918) ... wait
                // Actually: local_center start=(1745,1100) → screen = 1745*540/972+30=999,1100*960/984+80=1155
                // draw top-left = screen_center - (W/2,H/2) = (999-332.5, 1155-289) = (667, 866)
                // local_center end=(-100,-578) → screen = -100*540/972+30=-26,-578*960/984+80=-484
                // draw top-left = (-26-332.5, -484-289) = (-359, -773)
                let t = (self.attack_timer / 5.0).min(1.0);
                let et = ease_out_back(t);
                self.x = 667.0 + (-359.0 - 667.0) * et;
                self.y = 866.0 + (-773.0 - 866.0) * et;
                if self.attack_timer >= 5.0 {
                    self.active = false;
                    self.x = -2000.0;
                    self.y = -2000.0;
                }
            }
            AttackerKind::Megami => {
                // GameMain-local pivot init=(-500,1100)→screen=(-248,1153), draw=pivot-(163,538)=(-411,615)
                // GameMain-local pivot target=(-80,550)→screen=(-14,617), draw=pivot-(163,538)=(-177,79)
                let t = self.attack_timer;
                if t < 0.5 {
                    let p = ease_out_back(t / 0.5);
                    self.x = -411.0 + (-177.0 - (-411.0)) * p;
                    self.y = 615.0  + (79.0 - 615.0) * p;
                    self.rotation = 0.0;
                } else if t < 2.5 {
                    self.x = -177.0;
                    self.y = 79.0;
                    let p = (t - 0.5) / 2.0;
                    self.rotation = ease_out_bounce(p) * 45.0;
                } else if t < 4.3 {
                    self.x = -177.0;
                    self.y = 79.0;
                    let p = (t - 2.5) / 1.8;
                    self.rotation = (1.0 - ease_in_quad(p)) * 45.0;
                } else if t < 4.8 {
                    let p = ease_in_expo((t - 4.3) / 0.5);
                    self.x = -177.0 + (-411.0 - (-177.0)) * p;
                    self.y = 79.0   + (615.0 - 79.0) * p;
                    self.rotation = 0.0;
                } else {
                    self.active = false;
                    self.x = -2000.0;
                    self.y = -2000.0;
                    self.rotation = 0.0;
                }
            }
        }
    }

    pub fn is_colliding(&self) -> bool {
        if !self.active { return false; }
        match self.kind {
            AttackerKind::BigKokeshi => {
                let t = self.attack_timer / 5.0;
                t > 0.08 && t < 0.92
            }
            AttackerKind::Megami => {
                self.attack_timer >= 0.5 && self.attack_timer < 4.3 && self.rotation > 5.0
            }
        }
    }

    pub fn collision_rect(&self) -> Option<Rect> {
        if !self.is_colliding() { return None; }
        match self.kind {
            AttackerKind::BigKokeshi => {
                // Narrow strip: pixel-accurate test would use sprite shape, not AABB.
                // 20% height centered on sprite approximates the kokeshi body's hit zone.
                Some(Rect::new(
                    self.x + BIG_KOKESHI_W * 0.05,
                    self.y + BIG_KOKESHI_H * 0.40,
                    BIG_KOKESHI_W * 0.9,
                    BIG_KOKESHI_H * 0.20,
                ))
            }
            AttackerKind::Megami => {
                // AS3 uses pixel-level hitTestPoint — approximated here as the arm midpoint.
                // Arm extends from pivot toward top-right of sprite.
                // Midpoint of arm local: (MEGAMI_W/2 - MEGAMI_PIVOT_OX, -MEGAMI_PIVOT_OY/2) ≈ (163, -269)
                let pivot_x = self.x + MEGAMI_PIVOT_OX;
                let pivot_y = self.y + MEGAMI_PIVOT_OY;
                let rot_rad = self.rotation.to_radians();
                let (sin_r, cos_r) = (rot_rad.sin(), rot_rad.cos());
                let lx = MEGAMI_W / 2.0 - MEGAMI_PIVOT_OX;   // ~163
                let ly = -MEGAMI_PIVOT_OY / 2.0;              // ~-269
                let arm_x = pivot_x + lx * cos_r - ly * sin_r;
                let arm_y = pivot_y + lx * sin_r + ly * cos_r;
                let r = 130.0_f32;
                Some(Rect::new(arm_x - r, arm_y - r, r * 2.0, r * 2.0))
            }
        }
    }
}

fn ease_out_back(t: f32) -> f32 {
    let c1: f32 = 1.70158;
    let c3 = c1 + 1.0;
    1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
}

fn ease_out_bounce(t: f32) -> f32 {
    let n1: f32 = 7.5625;
    let d1: f32 = 2.75;
    if t < 1.0 / d1 {
        n1 * t * t
    } else if t < 2.0 / d1 {
        let t = t - 1.5 / d1;
        n1 * t * t + 0.75
    } else if t < 2.5 / d1 {
        let t = t - 2.25 / d1;
        n1 * t * t + 0.9375
    } else {
        let t = t - 2.625 / d1;
        n1 * t * t + 0.984375
    }
}

fn ease_in_quad(t: f32) -> f32 { t * t }

fn ease_in_expo(t: f32) -> f32 {
    if t == 0.0 { 0.0 } else { (2.0_f32).powf(10.0 * t - 10.0) }
}
