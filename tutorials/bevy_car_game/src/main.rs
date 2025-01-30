use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState{
    high_score: u32,
    current_socre: u32,
    enemy_labels: Vec<String>,
    spawn_timer: Timer
}

impl Default for GameState {
    fn default() -> Self {
        Self { 
            high_score: 0, 
            current_socre: 0, 
            enemy_labels: Vec::new(), 
            spawn_timer: Timer::from_seconds(1.0, TimerMode::Once) 
        }
    }
}

fn main() {
    let mut game = Game::new();    //  setup game here

    let player= game.add_sprite("player", SpritePreset::RacingCarBlack);
    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState){
    game_state.current_socre+=1;
    println!("Current Socre: {}", game_state.current_socre);
}