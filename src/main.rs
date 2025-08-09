use macroquad::prelude::*;

mod snake;
mod tetris;
mod pong;
mod pong_ai;

use snake::SnakeGame;
use tetris::TetrisGame;
use pong::PongGame;
use pong_ai::PongGameAI;

#[derive(Clone, PartialEq)]
enum GameState {
    Menu,
    Snake,
    Tetris,
    Pong,
    PongAI,
}

struct GameManager {
    snake_game: SnakeGame,
    tetris_game: TetrisGame,
    pong_game: PongGame,
    pong_ai_game: PongGameAI,
    selected: usize,
    state: GameState,
    game_names: Vec<&'static str>,
}

trait Game {
    fn run(&mut self) -> bool; // returns true if should return to menu
    fn reset(&mut self);
}

impl GameManager {
    fn new() -> Self {
        Self {
            snake_game: SnakeGame::new(),
            tetris_game: TetrisGame::new(),
            pong_game: PongGame::new(),
            pong_ai_game: PongGameAI::new(),
            selected: 0,
            state: GameState::Menu,
            game_names: vec!["Snake", "Tetris", "Pong", "Pong AI"],
        }
    }

    fn draw_menu(&self) {
        clear_background(Color::new(0.12, 0.12, 0.12, 1.0));
        
        let title = "Mini Games Hub";
        let title_size = 60.0;
        let title_width = measure_text(title, None, title_size as u16, 1.0).width;
        draw_text(
            title,
            screen_width() / 2.0 - title_width / 2.0,
            100.0,
            title_size,
            WHITE,
        );

        for (idx, name) in self.game_names.iter().enumerate() {
            let color = if idx == self.selected {
                Color::new(0.8, 0.8, 0.2, 1.0) // Yellow when selected
            } else {
                Color::new(0.8, 0.8, 0.8, 1.0) // Light gray
            };
            
            let y_pos = 200.0 + idx as f32 * 60.0;
            let text_width = measure_text(name, None, 48, 1.0).width;
            draw_text(
                name,
                screen_width() / 2.0 - text_width / 2.0,
                y_pos,
                48.0,
                color,
            );
        }
        
        // Instructions
        let instructions = "Use UP/DOWN to navigate, ENTER to select";
        let inst_width = measure_text(instructions, None, 24, 1.0).width;
        draw_text(
            instructions,
            screen_width() / 2.0 - inst_width / 2.0,
            screen_height() - 50.0,
            24.0,
            GRAY,
        );
    }

    fn handle_menu_input(&mut self) {
        if is_key_pressed(KeyCode::Up) {
            self.selected = if self.selected == 0 {
                self.game_names.len() - 1
            } else {
                self.selected - 1
            };
        }
        
        if is_key_pressed(KeyCode::Down) {
            self.selected = (self.selected + 1) % self.game_names.len();
        }
        
        if is_key_pressed(KeyCode::Enter) {
            self.state = match self.selected {
                0 => GameState::Snake,
                1 => GameState::Tetris,
                2 => GameState::Pong,
                3 => GameState::PongAI,
                _ => GameState::Menu,
            };
        }
    }

    async fn run(&mut self) {
        loop {
            match self.state {
                GameState::Menu => {
                    self.handle_menu_input();
                    self.draw_menu();
                }
                GameState::Snake => {
                    let should_return = self.snake_game.run();
                    if should_return {
                        self.snake_game.reset();
                        self.state = GameState::Menu;
                    }
                }
                GameState::Tetris => {
                    let should_return = self.tetris_game.run();
                    if should_return {
                        self.tetris_game.reset();
                        self.state = GameState::Menu;
                    }
                }
                GameState::Pong => {
                    let should_return = self.pong_game.run();
                    if should_return {
                        self.pong_game.reset();
                        self.state = GameState::Menu;
                    }
                }
                GameState::PongAI => {
                    let should_return = self.pong_ai_game.run();
                    if should_return {
                        self.pong_ai_game.reset();
                        self.state = GameState::Menu;
                    }
                }
            }
            
            next_frame().await;
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Mini Games Hub".to_owned(),
        window_width: 640,
        window_height: 480,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_manager = GameManager::new();
    game_manager.run().await;
}
