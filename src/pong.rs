use macroquad::prelude::*;
use crate::Game;

pub struct PongGame {
    left_paddle: (f32, f32),
    right_paddle: (f32, f32),
    ball: (f32, f32, f32, f32), // x, y, dx, dy
    score_left: i32,
    score_right: i32,
    paddle_speed: f32,
    ball_speed: f32,
    paddle_width: f32,
    paddle_height: f32,
    ball_size: f32,
}

impl PongGame {
    pub fn new() -> Self {
        let mut game = Self {
            left_paddle: (10.0, 200.0),
            right_paddle: (620.0, 200.0),
            ball: (320.0, 240.0, 5.0, 5.0),
            score_left: 0,
            score_right: 0,
            paddle_speed: 5.0,
            ball_speed: 5.0,
            paddle_width: 10.0,
            paddle_height: 80.0,
            ball_size: 16.0,
        };
        game.reset_ball(1.0);
        game
    }

    fn handle_input(&mut self) {
        // W/S for left paddle
        if is_key_down(KeyCode::W) && self.left_paddle.1 > 0.0 {
            self.left_paddle.1 -= self.paddle_speed;
        }
        if is_key_down(KeyCode::S) && self.left_paddle.1 + self.paddle_height < screen_height() {
            self.left_paddle.1 += self.paddle_speed;
        }

        // Up/Down for right paddle
        if is_key_down(KeyCode::Up) && self.right_paddle.1 > 0.0 {
            self.right_paddle.1 -= self.paddle_speed;
        }
        if is_key_down(KeyCode::Down) && self.right_paddle.1 + self.paddle_height < screen_height() {
            self.right_paddle.1 += self.paddle_speed;
        }
    }

    fn update_ball(&mut self) {
        // Move ball
        self.ball.0 += self.ball.2;
        self.ball.1 += self.ball.3;

        // Top/bottom collision
        if self.ball.1 <= 0.0 || self.ball.1 + self.ball_size >= screen_height() {
            self.ball.3 *= -1.0;
        }

        // Paddle collision detection
        let ball_rect = Rect::new(self.ball.0, self.ball.1, self.ball_size, self.ball_size);
        let left_paddle_rect = Rect::new(
            self.left_paddle.0,
            self.left_paddle.1,
            self.paddle_width,
            self.paddle_height,
        );
        let right_paddle_rect = Rect::new(
            self.right_paddle.0,
            self.right_paddle.1,
            self.paddle_width,
            self.paddle_height,
        );

        if ball_rect.overlaps(&left_paddle_rect) || ball_rect.overlaps(&right_paddle_rect) {
            self.ball.2 *= -1.0;
        }

        // Scoring
        if self.ball.0 <= 0.0 {
            self.score_right += 1;
            self.reset_ball(1.0);
        }
        if self.ball.0 + self.ball_size >= screen_width() {
            self.score_left += 1;
            self.reset_ball(-1.0);
        }
    }

    fn reset_ball(&mut self, direction: f32) {
        self.ball.0 = screen_width() / 2.0 - self.ball_size / 2.0;
        self.ball.1 = screen_height() / 2.0 - self.ball_size / 2.0;
        self.ball.2 = self.ball_speed * direction;
        self.ball.3 = self.ball_speed;
    }

    fn draw(&self) {
        clear_background(BLACK);

        // Draw paddles
        draw_rectangle(
            self.left_paddle.0,
            self.left_paddle.1,
            self.paddle_width,
            self.paddle_height,
            Color::new(0.8, 0.8, 0.8, 1.0),
        );
        draw_rectangle(
            self.right_paddle.0,
            self.right_paddle.1,
            self.paddle_width,
            self.paddle_height,
            Color::new(0.8, 0.8, 0.8, 1.0),
        );

        // Draw ball
        draw_circle(
            self.ball.0 + self.ball_size / 2.0,
            self.ball.1 + self.ball_size / 2.0,
            self.ball_size / 2.0,
            Color::new(0.8, 0.8, 0.8, 1.0),
        );

        // Draw net
        for y in (0..screen_height() as i32).step_by(20) {
            draw_rectangle(
                screen_width() / 2.0 - 1.0,
                y as f32,
                2.0,
                10.0,
                Color::new(0.4, 0.4, 0.4, 1.0),
            );
        }

        // Draw scores
        draw_text(
            &self.score_left.to_string(),
            screen_width() / 4.0,
            50.0,
            48.0,
            WHITE,
        );
        draw_text(
            &self.score_right.to_string(),
            screen_width() * 3.0 / 4.0,
            50.0,
            48.0,
            WHITE,
        );

        // Draw instructions
        draw_text(
            "Left: W/S  Right: UP/DOWN  ESC to return",
            10.0,
            screen_height() - 20.0,
            20.0,
            GRAY,
        );
    }
}

impl Game for PongGame {
    fn run(&mut self) -> bool {
        if is_key_pressed(KeyCode::Escape) {
            return true;
        }

        self.handle_input();
        self.update_ball();
        self.draw();

        false
    }

    fn reset(&mut self) {
        self.left_paddle = (10.0, screen_height() / 2.0 - self.paddle_height / 2.0);
        self.right_paddle = (
            screen_width() - 10.0 - self.paddle_width,
            screen_height() / 2.0 - self.paddle_height / 2.0,
        );
        self.score_left = 0;
        self.score_right = 0;
        self.reset_ball(1.0);
    }
}
