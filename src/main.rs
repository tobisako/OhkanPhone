use macroquad::prelude::*;

mod game;
mod resources;
mod scene;
mod viewport;

pub use resources::Resources;
pub const SCREEN_W: f32 = 600.0;
pub const SCREEN_H: f32 = 1200.0;
// Game playfield inset within the white frame — play area matches original "perfect" 540×960
pub const PLAY_X: f32 = 30.0;
pub const PLAY_Y: f32 = 80.0;
pub const PLAY_W: f32 = 540.0;
pub const PLAY_H: f32 = 960.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "OhkanGame".to_owned(),
        window_width: 600,
        window_height: 1200,
        window_resizable: false,
        high_dpi: true,
        // Android emulators (swiftshader) expose no multisample EGL configs;
        // EGL_SAMPLES>=1 then matches nothing and miniquad panics (egl.rs cfg_count>0).
        // 2D sprites don't need MSAA anyway.
        sample_count: if cfg!(target_os = "android") { 0 } else { 1 },
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // When launched as .app bundle, CWD is "/" — set it to the binary's directory so assets/ is found
    #[cfg(not(target_arch = "wasm32"))]
    if !std::path::Path::new("assets").exists() {
        if let Ok(exe) = std::env::current_exe() {
            if let Some(dir) = exe.parent() {
                let _ = std::env::set_current_dir(dir);
            }
        }
    }

    // Resource paths are written WITHOUT the assets/ prefix: Android's AssetManager
    // roots at the assets dir itself; desktop/wasm get the prefix from here.
    set_pc_assets_folder("assets");

    macroquad::rand::srand(macroquad::miniquad::date::now() as u64);

    use scene::{Scene, SceneResult};

    let res = Resources::load().await;
    let mut active: Box<dyn Scene> = Box::new(scene::TitleScene::new());

    loop {
        clear_background(BLACK);
        viewport::apply_camera();
        if let Some(next) = active.update_draw(&res) {
            active.on_exit(&res);
            active = match next {
                SceneResult::Title     => Box::new(scene::TitleScene::new()),
                SceneResult::StartGame => Box::new(scene::GameStageScene::new()),
                SceneResult::GameClear => Box::new(scene::GameResultScene::new(true)),
                SceneResult::GameOver  => Box::new(scene::GameResultScene::new(false)),
            };
        }
        next_frame().await;
    }
}
