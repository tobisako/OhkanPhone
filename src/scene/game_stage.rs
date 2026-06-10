use macroquad::prelude::*;
use macroquad::audio::{play_sound, play_sound_once, stop_sound, PlaySoundParams};
use crate::{SCREEN_W, SCREEN_H, PLAY_X, PLAY_Y, PLAY_W, PLAY_H, Resources};
use crate::game::{
    Player, Crown, Kokeshi, Attacker, AttackerKind,
    PLAYER_W, PLAYER_H, CROWN_W, CROWN_H, KOKESHI_W, KOKESHI_H,
    BIG_KOKESHI_W, BIG_KOKESHI_H, MEGAMI_W, MEGAMI_H, MEGAMI_PIVOT_OX, MEGAMI_PIVOT_OY,
};
use super::{Scene, SceneResult};

const GAME_TIME: f32 = 90.0;
const WIN_CROWNS: usize = 10;
const MAX_DAMAGE: usize = 3;

pub struct GameStageScene {
    player: Player,
    crowns: Vec<Crown>,
    kokeshis: Vec<Kokeshi>,
    attackers: Vec<Attacker>,
    game_time: f32,
    damage: usize,
    mounted_count: usize,
    crown_spawn_timer: f32,
    kokeshi_spawn_timer: f32,
    bgm_started: bool,
    play_catch: bool,
    play_crown_hit: bool,
    play_damage: bool,
    win_timer: Option<f32>,
}

impl GameStageScene {
    pub fn new() -> Self {
        Self {
            player: Player::new(),
            crowns: Vec::new(),
            kokeshis: Vec::new(),
            attackers: vec![
                Attacker::new(AttackerKind::BigKokeshi, 20.0),
                Attacker::new(AttackerKind::Megami, 35.0),
            ],
            game_time: GAME_TIME,
            damage: 0,
            mounted_count: 0,
            crown_spawn_timer: 1.5,
            kokeshi_spawn_timer: 4.0,
            bgm_started: false,
            play_catch: false,
            play_crown_hit: false,
            play_damage: false,
            win_timer: None,
        }
    }

    // Obstacle hits player body directly → life loss
    fn take_damage(&mut self) {
        self.damage += 1;
        self.play_damage = true;
    }

    // Obstacle hits crown stack → top crown ejects, no life loss
    fn eject_top_crown(&mut self) {
        self.play_crown_hit = true;
        if self.mounted_count > 0 {
            let idx = self.mounted_count - 1;
            if let Some(i) = self.crowns.iter().position(|c| c.mode == 1 && c.mount_index == idx) {
                let vx = macroquad::rand::gen_range(-120.0f32, 120.0);
                let vy = macroquad::rand::gen_range(-240.0f32, -90.0);
                self.crowns[i].eject(vx, vy);
                self.mounted_count -= 1;
            }
        }
    }
}

impl Scene for GameStageScene {
    fn on_exit(&mut self, res: &Resources) {
        stop_sound(&res.snd_bgm);
    }

    fn update_draw(&mut self, res: &Resources) -> Option<SceneResult> {
        let dt = get_frame_time();

        if !self.bgm_started {
            play_sound(&res.snd_bgm, PlaySoundParams { looped: true, volume: 0.6 });
            self.bgm_started = true;
        }

        // --- INPUT ---
        self.player.moving_left  = is_key_down(KeyCode::Left)  || is_key_down(KeyCode::A);
        self.player.moving_right = is_key_down(KeyCode::Right) || is_key_down(KeyCode::D);

        let btn_y = PLAY_Y + PLAY_H + 50.0;
        let left_btn  = Rect::new(20.0,             btn_y, 140.0, 110.0);
        let right_btn = Rect::new(SCREEN_W - 160.0, btn_y, 140.0, 110.0);
        if is_mouse_button_down(MouseButton::Left) {
            let p = crate::viewport::pointer_world();
            if left_btn.contains(p)  { self.player.moving_left  = true; }
            if right_btn.contains(p) { self.player.moving_right = true; }
        }
        // Multi-touch: each finger checked separately, so both buttons can be held
        for p in crate::viewport::touches_world() {
            if left_btn.contains(p)  { self.player.moving_left  = true; }
            if right_btn.contains(p) { self.player.moving_right = true; }
        }

        // --- UPDATE ---
        self.player.update(dt);
        self.game_time -= dt;

        self.crown_spawn_timer -= dt;
        if self.crown_spawn_timer <= 0.0 {
            let x = macroquad::rand::gen_range(PLAY_X + 20.0, PLAY_X + PLAY_W - 70.0);
            self.crowns.push(Crown::new(x));
            self.crown_spawn_timer = macroquad::rand::gen_range(1.8f32, 3.2);
        }
        self.kokeshi_spawn_timer -= dt;
        if self.kokeshi_spawn_timer <= 0.0 {
            let x = macroquad::rand::gen_range(PLAY_X + 20.0, PLAY_X + PLAY_W - 60.0);
            self.kokeshis.push(Kokeshi::new(x));
            self.kokeshi_spawn_timer = macroquad::rand::gen_range(2.8f32, 5.0);
        }

        for a in &mut self.attackers {
            a.update(dt);
            if a.just_activated && a.kind == AttackerKind::BigKokeshi {
                play_sound_once(&res.snd_beamgun);
                a.just_activated = false;
            } else {
                a.just_activated = false;
            }
        }

        let head_x = self.player.head_x();
        let head_y = self.player.head_y();
        for c in &mut self.crowns { c.update(dt, head_x, head_y); }

        // Crown mount: AS3-accurate width (center - width/4 - 3 = +32.75, w=74.5),
        // stack-following height so crowns visually stack up
        let stack_top_y = self.player.head_y()
            - CROWN_H
            - self.mounted_count as f32 * (CROWN_H + 2.0);
        let mount_rect = Rect::new(
            self.player.x + 32.75,
            stack_top_y - 20.0,
            74.5,
            CROWN_H + 20.0,
        );
        let mut new_mounts = 0usize;
        for c in &mut self.crowns {
            if c.mode == 0 && mount_rect.overlaps(&c.collision_rect()) {
                c.mount(self.mounted_count + new_mounts);
                new_mounts += 1;
                self.play_catch = true;
            }
        }
        self.mounted_count += new_mounts;

        // Small kokeshi: hits PLAYER BODY only → damage
        // (does NOT interact with crown stack in original AS3)
        let body_rect = self.player.body_rect();
        let mut damage_events = 0usize;

        for k in &mut self.kokeshis {
            k.update(dt);
            if k.mode != 0 || !k.alive { continue; }
            if body_rect.overlaps(&k.collision_rect()) {
                let dir = if k.x + KOKESHI_W / 2.0 < body_rect.x + body_rect.w / 2.0 { -1.0 } else { 1.0 };
                k.knockback(dir);
                damage_events += 1;
            }
        }

        // Attacker: eject every mounted crown that overlaps collision rect (AS3-accurate, no cooldown)
        let attacker_rects: Vec<Option<Rect>> = self.attackers.iter()
            .map(|a| if a.is_colliding() { a.collision_rect() } else { None })
            .collect();
        for ar_opt in &attacker_rects {
            if let Some(ar) = ar_opt {
                for c in self.crowns.iter_mut() {
                    // AS3 onHitCheckPoint: point test at crown center, not rect overlap
                    let cx = c.x + CROWN_W / 2.0;
                    let cy = c.y + CROWN_H / 2.0;
                    if c.mode == 1 && ar.contains(Vec2::new(cx, cy)) {
                        let vx = macroquad::rand::gen_range(-120.0f32, 120.0);
                        let vy = macroquad::rand::gen_range(-240.0f32, -90.0);
                        c.eject(vx, vy);
                        self.play_crown_hit = true;
                    }
                }
            }
        }
        self.mounted_count = self.crowns.iter().filter(|c| c.mode == 1).count();

        // Re-compact mount_index after ejections to close visual gaps in the stack
        let mut idx = 0usize;
        for c in self.crowns.iter_mut() {
            if c.mode == 1 {
                c.mount_index = idx;
                idx += 1;
            }
        }

        for _ in 0..damage_events { self.take_damage(); }

        self.crowns.retain(|c| c.alive);
        self.kokeshis.retain(|k| k.alive);

        // --- AUDIO ---
        if self.play_catch     { play_sound_once(&res.snd_catch);     self.play_catch     = false; }
        if self.play_crown_hit { play_sound_once(&res.snd_crown_hit); self.play_crown_hit = false; }
        if self.play_damage    { play_sound_once(&res.snd_damage);    self.play_damage    = false; }

        // --- WIN / LOSE ---
        if self.mounted_count >= WIN_CROWNS {
            let t = self.win_timer.get_or_insert(1.0);
            *t -= dt;
            if *t <= 0.0 { return Some(SceneResult::GameClear); }
        }
        if self.damage >= MAX_DAMAGE || self.game_time <= 0.0         { return Some(SceneResult::GameOver); }

        // ======== DRAW ========

        // White frame background + game playfield inset
        draw_texture_ex(
            &res.tex_bg_game, 0.0, 0.0, WHITE,
            DrawTextureParams { dest_size: Some(vec2(SCREEN_W, SCREEN_H)), ..Default::default() },
        );
        draw_texture_ex(
            &res.tex_bg_gameback, PLAY_X, PLAY_Y, WHITE,
            DrawTextureParams { dest_size: Some(vec2(PLAY_W, PLAY_H)), ..Default::default() },
        );

        // Attackers (behind player)
        for a in &self.attackers {
            if !a.active { continue; }
            match a.kind {
                AttackerKind::BigKokeshi => {
                    draw_texture_ex(&res.tex_big_kokeshi, a.x, a.y, WHITE,
                        DrawTextureParams { dest_size: Some(vec2(BIG_KOKESHI_W, BIG_KOKESHI_H)), ..Default::default() });
                }
                AttackerKind::Megami => {
                    // Pivot = sprite registration point (AS3 origin), at offset (PIVOT_OX, PIVOT_OY) from draw top-left
                    let pivot = vec2(a.x + MEGAMI_PIVOT_OX, a.y + MEGAMI_PIVOT_OY);
                    draw_texture_ex(&res.tex_megami, a.x, a.y, WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(MEGAMI_W, MEGAMI_H)),
                            rotation: a.rotation.to_radians(),
                            pivot: Some(pivot),
                            ..Default::default()
                        });
                }
            }
        }

        // Falling / ejected crowns
        for c in &self.crowns {
            if c.mode == 1 { continue; }
            let rot = if c.mode == 2 { c.rotation.to_radians() } else { 0.0 };
            draw_texture_ex(
                &res.tex_ohkan, c.x, c.y, WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(CROWN_W, CROWN_H)),
                    rotation: rot,
                    pivot: if rot != 0.0 { Some(vec2(c.x + CROWN_W / 2.0, c.y + CROWN_H / 2.0)) } else { None },
                    ..Default::default()
                },
            );
        }

        // Player
        draw_texture_ex(
            &res.tex_abe, self.player.x, self.player.y, WHITE,
            DrawTextureParams { dest_size: Some(vec2(PLAYER_W, PLAYER_H)), ..Default::default() },
        );

        // Mounted crown stack
        for c in &self.crowns {
            if c.mode != 1 { continue; }
            draw_texture_ex(
                &res.tex_ohkan, c.x, c.y, WHITE,
                DrawTextureParams { dest_size: Some(vec2(CROWN_W, CROWN_H)), ..Default::default() },
            );
        }

        // Kokeshis
        for k in &self.kokeshis {
            let rot = k.rotation.to_radians();
            draw_texture_ex(
                &res.tex_kokeshi, k.x, k.y, WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(KOKESHI_W, KOKESHI_H)),
                    rotation: rot,
                    pivot: Some(vec2(k.x + KOKESHI_W / 2.0, k.y + KOKESHI_H / 2.0)),
                    ..Default::default()
                },
            );
        }

        // ---- White border overlay: masks game objects outside play area ----
        let border = WHITE;
        draw_rectangle(0.0, 0.0, SCREEN_W, PLAY_Y, border);
        draw_rectangle(0.0, PLAY_Y, PLAY_X, PLAY_H, border);
        draw_rectangle(PLAY_X + PLAY_W, PLAY_Y, SCREEN_W - PLAY_X - PLAY_W, PLAY_H, border);
        draw_rectangle(0.0, PLAY_Y + PLAY_H, SCREEN_W, SCREEN_H - PLAY_Y - PLAY_H, border);

        // ---- UI inside play area (top strip) ----
        let t_col = if self.game_time < 15.0 { RED } else { WHITE };
        draw_text(&format!("{:.0}s", self.game_time.max(0.0)), PLAY_X + PLAY_W - 72.0, PLAY_Y + 42.0, 36.0, t_col);

        let cs = format!("x{}/{}", self.mounted_count, WIN_CROWNS);
        draw_texture_ex(&res.tex_ohkan, PLAY_X + 8.0, PLAY_Y + 8.0, WHITE,
            DrawTextureParams { dest_size: Some(vec2(36.0, 28.0)), ..Default::default() });
        draw_text(&cs, PLAY_X + 50.0, PLAY_Y + 36.0, 30.0, GOLD);

        // ---- Life marks in bottom white area (right side) ----
        let life_w = 40.0;
        let life_h = 40.0;
        let life_src_w = 342.0 / 3.0;
        let life_y = PLAY_Y + PLAY_H + 10.0;
        for i in 0..MAX_DAMAGE {
            let lx = SCREEN_W - 45.0 - (MAX_DAMAGE - 1 - i) as f32 * (life_w + 4.0);
            if i < MAX_DAMAGE - self.damage {
                draw_texture_ex(
                    &res.tex_life_ok, lx, life_y, WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(life_w, life_h)),
                        source: Some(Rect::new(i as f32 * life_src_w, 0.0, life_src_w, 147.0)),
                        ..Default::default()
                    },
                );
            } else {
                draw_texture_ex(
                    &res.tex_life_ng, lx, life_y, WHITE,
                    DrawTextureParams { dest_size: Some(vec2(life_w, life_h)), ..Default::default() },
                );
            }
        }

        // ---- Buttons in bottom white area ----
        draw_texture_ex(&res.tex_btn_left, left_btn.x, left_btn.y, WHITE,
            DrawTextureParams { dest_size: Some(vec2(left_btn.w, left_btn.h)), ..Default::default() });
        draw_texture_ex(&res.tex_btn_right, right_btn.x, right_btn.y, WHITE,
            DrawTextureParams { dest_size: Some(vec2(right_btn.w, right_btn.h)), ..Default::default() });

        None
    }
}
