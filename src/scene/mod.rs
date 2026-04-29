mod title;
mod game_stage;
mod game_result;

pub use title::TitleScene;
pub use game_stage::GameStageScene;
pub use game_result::GameResultScene;

use crate::Resources;

pub enum SceneResult {
    Title,
    StartGame,
    GameClear,
    GameOver,
}

pub trait Scene {
    fn update_draw(&mut self, res: &Resources) -> Option<SceneResult>;
    fn on_exit(&mut self, _res: &Resources) {}
}
