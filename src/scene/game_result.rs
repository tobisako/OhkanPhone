use macroquad::prelude::*;
use macroquad::audio::{play_sound_once};
use crate::{SCREEN_W, SCREEN_H, Resources};
use super::{Scene, SceneResult};

pub struct GameResultScene {
    won: bool,
    timer: f32,
    sound_played: bool,
}

impl GameResultScene {
    pub fn new(won: bool) -> Self {
        Self { won, timer: 30.0, sound_played: false }
    }
}

impl Scene for GameResultScene {
    fn update_draw(&mut self, res: &Resources) -> Option<SceneResult> {
        let dt = get_frame_time();
        self.timer -= dt;

        if !self.sound_played {
            if self.won {
                play_sound_once(&res.snd_clear);
            }
            self.sound_played = true;
        }

        if self.won {
            draw_texture_ex(
                &res.tex_bg_clear,
                0.0, 0.0, WHITE,
                DrawTextureParams { dest_size: Some(vec2(SCREEN_W, SCREEN_H)), ..Default::default() },
            );
        } else {
            draw_texture_ex(
                &res.tex_bg_gameover, 0.0, 0.0, WHITE,
                DrawTextureParams { dest_size: Some(vec2(SCREEN_W, SCREEN_H)), ..Default::default() },
            );
            // Gameover message overlay — centered, occupying upper 2/3
            let mw = SCREEN_W * 0.85;
            let mh = mw * (400.0 / 680.0);  // approximate aspect of gameover image
            draw_texture_ex(
                &res.tex_gameover_msg,
                (SCREEN_W - mw) / 2.0, SCREEN_H * 0.15, WHITE,
                DrawTextureParams { dest_size: Some(vec2(mw, mh)), ..Default::default() },
            );
        }

        // Back button
        let bx = SCREEN_W / 2.0 - 120.0;
        let by = SCREEN_H * 0.72;
        let bw = 240.0;
        let bh = 80.0;
        draw_texture_ex(
            &res.tex_btn_back,
            bx, by, WHITE,
            DrawTextureParams { dest_size: Some(vec2(bw, bh)), ..Default::default() },
        );

        let s = format!("Back to title: {:.0}s", self.timer.max(0.0));
        let sw = measure_text(&s, None, 20, 1.0).width;
        draw_text(&s, SCREEN_W / 2.0 - sw / 2.0, SCREEN_H * 0.88, 20.0, Color::from_rgba(200, 200, 200, 200));

        let btn = Rect::new(bx, by, bw, bh);
        let tapped = crate::viewport::tap_world().is_some_and(|p| btn.contains(p));
        if tapped
            || is_key_pressed(KeyCode::Enter)
            || is_key_pressed(KeyCode::Space)
            || self.timer <= 0.0
        {
            return Some(SceneResult::Title);
        }

        None
    }
}
