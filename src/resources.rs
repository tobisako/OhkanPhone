use macroquad::prelude::*;
use macroquad::audio::{load_sound, Sound};

pub struct Resources {
    // Character sprites
    pub tex_abe: Texture2D,
    pub tex_ohkan: Texture2D,
    pub tex_kokeshi: Texture2D,
    pub tex_megami: Texture2D,
    pub tex_big_kokeshi: Texture2D,
    // Backgrounds
    pub tex_bg_title: Texture2D,
    pub tex_bg_game: Texture2D,
    pub tex_bg_clear: Texture2D,
    pub tex_bg_gameback: Texture2D,
    pub tex_bg_gameover: Texture2D,
    pub tex_gameover_msg: Texture2D,
    // UI
    pub tex_life_ok: Texture2D,
    pub tex_life_ng: Texture2D,
    pub tex_btn_left: Texture2D,
    pub tex_btn_right: Texture2D,
    pub tex_btn_start: Texture2D,
    pub tex_btn_back: Texture2D,
    // Sounds
    pub snd_title: Sound,
    pub snd_bgm: Sound,
    pub snd_catch: Sound,
    pub snd_crown_hit: Sound,
    pub snd_damage: Sound,
    pub snd_clear: Sound,
    pub snd_beamgun: Sound,
}

impl Resources {
    pub async fn load() -> Self {
        let tex_abe          = load_texture("img/g_abe.png").await.unwrap();
        let tex_ohkan        = load_texture("img/g_ohkan.png").await.unwrap();
        let tex_kokeshi      = load_texture("img/g_kokeshi.png").await.unwrap();
        let tex_megami       = load_texture("img/g_megami.png").await.unwrap();
        let tex_big_kokeshi  = load_texture("img/g_big_kokeshi.png").await.unwrap();
        let tex_bg_title     = load_texture("img/bg_gametitle.png").await.unwrap();
        let tex_bg_game      = load_texture("img/bg_gamebase.png").await.unwrap();
        let tex_bg_clear     = load_texture("img/bg_gameclear.png").await.unwrap();
        let tex_bg_gameback  = load_texture("img/bg_gameback.png").await.unwrap();
        let tex_bg_gameover  = load_texture("img/bg_haikei.png").await.unwrap();
        let tex_gameover_msg = load_texture("img/gameover_mouhitoiki.png").await.unwrap();
        let tex_life_ok      = load_texture("img/g_lifemark.png").await.unwrap();
        let tex_life_ng      = load_texture("img/g_life_batsu.png").await.unwrap();
        let tex_btn_left     = load_texture("img/btn_left.png").await.unwrap();
        let tex_btn_right    = load_texture("img/btn_right.png").await.unwrap();
        let tex_btn_start    = load_texture("img/btn_gamestart.png").await.unwrap();
        let tex_btn_back     = load_texture("img/btn_back.png").await.unwrap();

        // Set nearest filter for crisp sprites
        for t in [&tex_abe, &tex_ohkan, &tex_kokeshi, &tex_megami, &tex_big_kokeshi,
                  &tex_btn_left, &tex_btn_right, &tex_btn_start, &tex_btn_back,
                  &tex_life_ok, &tex_life_ng] {
            t.set_filter(FilterMode::Linear);
        }

        // Web build uses mp3 (17MB BGM wav → 1.5MB; iOS Safari can't decode ogg).
        // Native keeps wav: quad-snd's native backend has no mp3 decoder.
        #[cfg(target_arch = "wasm32")]
        const SE_EXT: &str = "mp3";
        #[cfg(not(target_arch = "wasm32"))]
        const SE_EXT: &str = "wav";
        let se = |name: &str| format!("se/{name}.{SE_EXT}");

        let snd_title     = load_sound(&se("se_setsumei")).await.unwrap();
        let snd_bgm       = load_sound(&se("se_gamebgm")).await.unwrap();
        let snd_catch     = load_sound(&se("se_catch")).await.unwrap();
        // crown_hit has no mp3 source — wav is small (654KB), used on all platforms
        let snd_crown_hit = load_sound("se/se_crown_hit.wav").await.unwrap();
        let snd_damage    = load_sound(&se("se_damage")).await.unwrap();
        let snd_clear     = load_sound(&se("se_gameclear")).await.unwrap();
        let snd_beamgun   = load_sound(&se("se_beamgun")).await.unwrap();

        Self {
            tex_abe, tex_ohkan, tex_kokeshi, tex_megami, tex_big_kokeshi,
            tex_bg_title, tex_bg_game, tex_bg_clear, tex_bg_gameback, tex_bg_gameover, tex_gameover_msg,
            tex_life_ok, tex_life_ng,
            tex_btn_left, tex_btn_right, tex_btn_start, tex_btn_back,
            snd_title, snd_bgm, snd_catch, snd_crown_hit, snd_damage, snd_clear, snd_beamgun,
        }
    }
}
