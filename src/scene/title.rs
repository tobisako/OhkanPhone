use macroquad::prelude::*;
use macroquad::audio::{play_sound, stop_sound, PlaySoundParams};
use crate::{SCREEN_W, SCREEN_H, Resources};
use super::{Scene, SceneResult};

pub struct TitleScene {
    timer: f32,
    bgm_started: bool,
}

impl TitleScene {
    pub fn new() -> Self {
        Self { timer: 30.0, bgm_started: false }
    }
}

impl Scene for TitleScene {
    fn on_exit(&mut self, res: &Resources) {
        stop_sound(&res.snd_title);
    }

    fn update_draw(&mut self, res: &Resources) -> Option<SceneResult> {
        if !self.bgm_started {
            play_sound(&res.snd_title, PlaySoundParams { looped: true, volume: 0.7 });
            self.bgm_started = true;
        }
        let dt = get_frame_time();
        self.timer -= dt;

        // Background
        draw_texture_ex(
            &res.tex_bg_title,
            0.0, 0.0, WHITE,
            DrawTextureParams { dest_size: Some(vec2(SCREEN_W, SCREEN_H)), ..Default::default() },
        );

        // START button — positioned near bottom, matching original y≈1680/3.5 ≈ 480 scaled
        let bx = 144.0;
        let by = 1050.0;
        let bw = 260.0;
        let bh = 80.0;
        draw_texture_ex(
            &res.tex_btn_start,
            bx, by, WHITE,
            DrawTextureParams { dest_size: Some(vec2(bw, bh)), ..Default::default() },
        );

        // Auto-start timer (small text overlay)
        let s = format!("Auto-start: {:.0}s", self.timer.max(0.0));
        let sw = measure_text(&s, None, 20, 1.0).width;
        draw_text(&s, SCREEN_W / 2.0 - sw / 2.0, SCREEN_H * 0.88, 20.0, Color::from_rgba(200, 200, 200, 200));

        let btn = Rect::new(bx, by, bw, bh);
        let tapped = crate::viewport::tap_world().is_some_and(|p| btn.contains(p));
        if tapped
            || is_key_pressed(KeyCode::Space)
            || is_key_pressed(KeyCode::Enter)
            || self.timer <= 0.0
        {
            return Some(SceneResult::StartGame);
        }

        None
    }
}
