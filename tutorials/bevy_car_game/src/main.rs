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

    let player= game.add_sprite("player", SpritePreset::RacingCarBlue);
    player.translation= Vec2::new(0.0, 0.0);
    player.rotation= SOUTH_WEST;
    player.scale= 1.0;
    player.collision= true;

    
    let car1= game.add_sprite("car1", SpritePreset::RacingCarRed);
    car1.translation= Vec2::new(300.0, 0.0);
    car1.collision= true;

    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState){
    for event in engine.collision_events.drain(..) {
        if event.state == CollisionState::Begin && event.pair.one_starts_with("player"){
            for label in [event.pair.0, event.pair.1] {
                if label != "palyer"{
                    engine.sprites.remove(&label);
                }
            }
            game_state.current_socre+= 1;
            println!("Current Socre: {}", game_state.current_socre);
        }
    }

    let palyer= engine.sprites.get_mut("player").unwrap();
    palyer.translation.x+= 100.0 * engine.delta_f32;
}