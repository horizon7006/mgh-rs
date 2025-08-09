use macroquad::prelude::*;
use crate::Game;

#[derive(Clone)]
struct Tetromino {
    shape: Vec<Vec<bool>>,
    color: Color,
}

pub struct TetrisGame {
    grid: Vec<Vec<Color>>,
    current_piece: Tetromino,
    piece_pos: (i32, i32),
    score: i32,
    last_drop: f64,
    drop_interval: f64,
    rows: usize,
    cols: usize,
    block_size: f32,
}

const SHAPES: &[(&[&[u8]], Color)] = &[
    // I piece
    (&[&[1, 1, 1, 1]], Color::new(0.0, 0.94, 0.94, 1.0)),
    // O piece
    (&[&[1, 1], &[1, 1]], Color::new(0.94, 0.94, 0.0, 1.0)),
    // T piece
    (&[&[0, 1, 0], &[1, 1, 1]], Color::new(0.63, 0.0, 0.94, 1.0)),
    // S piece
    (&[&[0, 1, 1], &[1, 1, 0]], Color::new(0.0, 0.94, 0.0, 1.0)),
    // Z piece
    (&[&[1, 1, 0], &[0, 1, 1]], Color::new(0.94, 0.0, 0.0, 1.0)),
    // J piece
    (&[&[1, 0, 0], &[1, 1, 1]], Color::new(0.0, 0.0, 0.94, 1.0)),
    // L piece
    (&[&[0, 0, 1], &[1, 1, 1]], Color::new(0.94, 0.63, 0.0, 1.0)),
];

impl TetrisGame {
    pub fn new() -> Self {
        let rows = 20;
        let cols = 10;
        let block_size = 24.0;
        
        let mut game = Self {
            grid: vec![vec![BLACK; cols]; rows],
            current_piece: Self::random_tetromino(),
            piece_pos: (0, cols as i32 / 2 - 2),
            score: 0,
            last_drop: 0.0,
            drop_interval: 0.5,
            rows,
            cols,
            block_size,
        };
        
        game.spawn_piece();
        game
    }

    fn random_tetromino() -> Tetromino {
        let (shape_data, color) = SHAPES[rand::gen_range(0, SHAPES.len())];
        let shape = shape_data
            .iter()
            .map(|row| row.iter().map(|&cell| cell == 1).collect())
            .collect();
        
        Tetromino { shape, color }
    }

    fn spawn_piece(&mut self) {
        self.current_piece = Self::random_tetromino();
        self.piece_pos = (0, self.cols as i32 / 2 - 2);
    }

    fn rotate_piece(&mut self) {
        let rows = self.current_piece.shape.len();
        let cols = self.current_piece.shape[0].len();
        let mut rotated = vec![vec![false; rows]; cols];
        
        for (i, row) in self.current_piece.shape.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                rotated[j][rows - 1 - i] = cell;
            }
        }
        
        let old_shape = self.current_piece.shape.clone();
        self.current_piece.shape = rotated;
        
        if !self.is_valid_position(0, 0) {
            self.current_piece.shape = old_shape; // Undo rotation
        }
    }

    fn is_valid_position(&self, offset_y: i32, offset_x: i32) -> bool {
        for (y, row) in self.current_piece.shape.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell {
                    let nx = self.piece_pos.1 + x as i32 + offset_x;
                    let ny = self.piece_pos.0 + y as i32 + offset_y;
                    
                    if nx < 0 || nx >= self.cols as i32 || ny >= self.rows as i32 {
                        return false;
                    }
                    
                    if ny >= 0 && self.grid[ny as usize][nx as usize] != BLACK {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn lock_piece(&mut self) {
        for (y, row) in self.current_piece.shape.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell {
                    let grid_x = (self.piece_pos.1 + x as i32) as usize;
                    let grid_y = (self.piece_pos.0 + y as i32) as usize;
                    self.grid[grid_y][grid_x] = self.current_piece.color;
                }
            }
        }
        
        self.clear_lines();
        self.spawn_piece();
    }

    fn clear_lines(&mut self) {
        let mut lines_cleared = 0;
        let mut new_grid = Vec::new();
        
        for row in &self.grid {
            if row.iter().any(|&cell| cell == BLACK) {
                new_grid.push(row.clone());
            } else {
                lines_cleared += 1;
            }
        }
        
        // Add empty rows at the top
        while new_grid.len() < self.rows {
            new_grid.insert(0, vec![BLACK; self.cols]);
        }
        
        self.grid = new_grid;
        self.score += lines_cleared * lines_cleared * 100;
    }

    fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::Left) {
            if self.is_valid_position(0, -1) {
                self.piece_pos.1 -= 1;
            }
        }
        
        if is_key_pressed(KeyCode::Right) {
            if self.is_valid_position(0, 1) {
                self.piece_pos.1 += 1;
            }
        }
        
        if is_key_pressed(KeyCode::Down) {
            if self.is_valid_position(1, 0) {
                self.piece_pos.0 += 1;
            }
        }
        
        if is_key_pressed(KeyCode::Up) {
            self.rotate_piece();
        }
    }

    fn update(&mut self) -> bool {
        let current_time = get_time();
        if current_time - self.last_drop >= self.drop_interval {
            self.last_drop = current_time;
            
            if self.is_valid_position(1, 0) {
                self.piece_pos.0 += 1;
            } else {
                self.lock_piece();
                
                // Check game over - if new piece can't be placed
                if !self.is_valid_position(0, 0) {
                    return true;
                }
            }
        }
        
        false
    }

    fn draw(&self) {
        clear_background(BLACK);

        // Draw grid
        for (y, row) in self.grid.iter().enumerate() {
            for (x, &cell_color) in row.iter().enumerate() {
                let rect_x = x as f32 * self.block_size;
                let rect_y = y as f32 * self.block_size;
                
                draw_rectangle(rect_x, rect_y, self.block_size, self.block_size, cell_color);
                draw_rectangle_lines(
                    rect_x,
                    rect_y,
                    self.block_size,
                    self.block_size,
                    1.0,
                    Color::new(0.16, 0.16, 0.16, 1.0),
                );
            }
        }

        // Draw current piece
        for (y, row) in self.current_piece.shape.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell {
                    let px = (self.piece_pos.1 + x as i32) as f32 * self.block_size;
                    let py = (self.piece_pos.0 + y as i32) as f32 * self.block_size;
                    
                    draw_rectangle(px, py, self.block_size, self.block_size, self.current_piece.color);
                    draw_rectangle_lines(px, py, self.block_size, self.block_size, 1.0, GRAY);
                }
            }
        }

        // Draw score
        draw_text(
            &format!("Score: {}", self.score),
            self.cols as f32 * self.block_size + 10.0,
            30.0,
            30.0,
            WHITE,
        );

        // Draw instructions
        let instructions = vec![
            "Arrow keys to move",
            "UP to rotate",
            "ESC to return to menu",
        ];
        
        for (i, instruction) in instructions.iter().enumerate() {
            draw_text(
                instruction,
                self.cols as f32 * self.block_size + 10.0,
                80.0 + i as f32 * 25.0,
                20.0,
                GRAY,
            );
        }
    }
}

impl Game for TetrisGame {
    fn run(&mut self) -> bool {
        if is_key_pressed(KeyCode::Escape) {
            return true;
        }

        self.handle_input();
        let game_over = self.update();
        self.draw();

        if game_over {
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
        self.grid = vec![vec![BLACK; self.cols]; self.rows];
        self.score = 0;
        self.spawn_piece();
        self.last_drop = 0.0;
    }
}
