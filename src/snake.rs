use macroquad::prelude::*;
use crate::Game;

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct SnakeGame {
    snake: Vec<(i32, i32)>,
    direction: Direction,
    food: (i32, i32),
    score: i32,
    block_size: i32,
    last_update: f64,
}

impl SnakeGame {
    pub fn new() -> Self {
        let mut game = Self {
            snake: vec![(320, 240)],
            direction: Direction::Right,
            food: (0, 0),
            score: 0,
            block_size: 20,
            last_update: 0.0,
        };
        game.spawn_food();
        game
    }

    fn spawn_food(&mut self) {
        let max_x = (screen_width() as i32 / self.block_size) * self.block_size;
        let max_y = (screen_height() as i32 / self.block_size) * self.block_size;
        
        loop {
            self.food = (
                (rand::gen_range(0, max_x / self.block_size) * self.block_size) as i32,
                (rand::gen_range(0, max_y / self.block_size) * self.block_size) as i32,
            );
            
            // Make sure food doesn't spawn on snake
            if !self.snake.contains(&self.food) {
                break;
            }
        }
    }

    fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::Up) && self.direction != Direction::Down {
            self.direction = Direction::Up;
        }
        if is_key_pressed(KeyCode::Down) && self.direction != Direction::Up {
            self.direction = Direction::Down;
        }
        if is_key_pressed(KeyCode::Left) && self.direction != Direction::Right {
            self.direction = Direction::Left;
        }
        if is_key_pressed(KeyCode::Right) && self.direction != Direction::Left {
            self.direction = Direction::Right;
        }
    }

    fn update(&mut self) -> bool {
        let current_time = get_time();
        if current_time - self.last_update < 0.1 {
            return false; // Don't update yet
        }
        self.last_update = current_time;

        // Calculate new head position
        let head = self.snake[0];
        let new_head = match self.direction {
            Direction::Up => (head.0, head.1 - self.block_size),
            Direction::Down => (head.0, head.1 + self.block_size),
            Direction::Left => (head.0 - self.block_size, head.1),
            Direction::Right => (head.0 + self.block_size, head.1),
        };

        // Check wall collisions
        if new_head.0 < 0 
            || new_head.0 >= screen_width() as i32
            || new_head.1 < 0 
            || new_head.1 >= screen_height() as i32 {
            return true; // Game over
        }

        // Check self collision
        if self.snake.contains(&new_head) {
            return true; // Game over
        }

        // Add new head
        self.snake.insert(0, new_head);

        // Check food collision
        if new_head == self.food {
            self.score += 1;
            self.spawn_food();
        } else {
            self.snake.pop(); // Remove tail if no food eaten
        }

        false
    }

    fn draw(&self) {
        clear_background(BLACK);

        // Draw snake
        for segment in &self.snake {
            draw_rectangle(
                segment.0 as f32,
                segment.1 as f32,
                self.block_size as f32,
                self.block_size as f32,
                GREEN,
            );
        }

        // Draw food
        draw_rectangle(
            self.food.0 as f32,
            self.food.1 as f32,
            self.block_size as f32,
            self.block_size as f32,
            RED,
        );

        // Draw score
        draw_text(
            &format!("Score: {}", self.score),
            10.0,
            30.0,
            30.0,
            WHITE,
        );

        // Draw instructions
        draw_text(
            "ESC to return to menu",
            10.0,
            screen_height() - 20.0,
            20.0,
            GRAY,
        );
    }
}

impl Game for SnakeGame {
    fn run(&mut self) -> bool {
        if is_key_pressed(KeyCode::Escape) {
            return true;
        }

        self.handle_input();
        let game_over = self.update();
        self.draw();

        if game_over {
            // Show game over message
            let game_over_text = format!("Game Over! Score: {} - Press SPACE to continue", self.score);
            let text_width = measure_text(&game_over_text, None, 32, 1.0).width;
            draw_text(
                &game_over_text,
                screen_width() / 2.0 - text_width / 2.0,
                screen_height() / 2.0,
                32.0,
                WHITE,
            );
            
            if is_key_pressed(KeyCode::Space) {
                return true;
            }
        }

        false
    }

    fn reset(&mut self) {
        self.snake = vec![(320, 240)];
        self.direction = Direction::Right;
        self.score = 0;
        self.spawn_food();
        self.last_update = 0.0;
    }
}
