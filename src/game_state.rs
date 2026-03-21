use std::time::{Duration, Instant};

use rand::{prelude::*, rng};

use crate::{
    GamePhase,
    tetromino::{N_TETROMINOS, TETROMINO_SIZE, TETROMINOS},
};

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;
pub const NEXT_WIDTH: usize = 4;
pub const NEXT_HEIGHT: usize = 4;

pub struct GameState {
    pub game_phase: GamePhase,
    pub board: [[u8; BOARD_WIDTH]; BOARD_HEIGHT],
    pub next: u8,

    pub active_tetromino: u8,
    pub rotation: u8,
    pub position: (i16, i16),

    pub lines: u32,
    pub score: u64,

    rng: ThreadRng,
    pub last_gravity_tick: Instant,
}

impl GameState {
    pub fn initial_state() -> Self {
        let mut rng = rng();

        Self {
            game_phase: GamePhase::Running,
            board: [[0; BOARD_WIDTH]; BOARD_HEIGHT],
            next: rng.random_range(1..N_TETROMINOS),
            active_tetromino: rng.random_range(1..=N_TETROMINOS),
            rotation: 0,
            position: (3, -4),
            lines: 0,
            score: 0,
            rng,
            last_gravity_tick: Instant::now(),
        }
    }

    pub fn level(&self) -> u32 {
        (self.lines / 10) + 1
    }

    pub fn gravity_rate(&self) -> Duration {
        const LIMIT: u64 = 200;
        Duration::from_millis((1000 - LIMIT) / self.level() as u64 + LIMIT)
    }

    pub fn rotate_clockwise(&mut self) {
        let old_rotation = self.rotation;
        if self.rotation == 0 {
            self.rotation = 3;
        } else {
            self.rotation -= 1;
        }

        if self.detect_collision() {
            self.rotation = old_rotation;
        }
    }

    pub fn rotate_counter_clockwise(&mut self) {
        let old_rotation = self.rotation;
        if self.rotation >= 3 {
            self.rotation = 0;
        } else {
            self.rotation += 1;
        }

        if self.detect_collision() {
            self.rotation = old_rotation;
        }
    }

    pub fn move_left(&mut self) {
        if !self.collides_at(-1, 0) {
            self.position.0 -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if !self.collides_at(1, 0) {
            self.position.0 += 1;
        }
    }

    pub fn tick_gravity(&mut self) {
        if self.collides_at(0, 1) {
            let cell_outside_board = self.handle_ground_contact();
            if cell_outside_board {
                self.game_phase = GamePhase::GameOver;
            }
        } else {
            self.position.1 += 1;
        }
        self.last_gravity_tick = Instant::now();
    }

    pub fn drop(&mut self) {
        while !self.collides_at(0, 1) {
            self.position.1 += 1;
        }
        let cell_outside_board = self.handle_ground_contact();
        if cell_outside_board {
            self.game_phase = GamePhase::GameOver;
        }
    }

    pub fn active_cells(&self) -> impl Iterator<Item = (i16, i16)> + use<> {
        let mut bits = TETROMINOS[self.active_tetromino as usize].rotations[self.rotation as usize];

        let (bx, by) = self.position;

        (0..TETROMINO_SIZE as i16)
            .flat_map(move |dy| (0..TETROMINO_SIZE as i16).map(move |dx| (dx, dy)))
            .filter_map(move |(dx, dy)| {
                let occupied = (bits & 1) != 0;
                bits >>= 1;
                occupied.then_some((bx + dx, by + dy))
            })
    }

    fn detect_collision(&self) -> bool {
        self.collides_at(0, 0)
    }

    fn collides_at(&self, dx: i16, dy: i16) -> bool {
        for (x, y) in self.active_cells() {
            let x = x + dx;
            let y = y + dy;
            if !((0..BOARD_WIDTH).contains(&(x as usize)) && y < (BOARD_HEIGHT as i16)) {
                return true;
            }
            if y >= 0 && self.board[y as usize][x as usize] != 0 {
                return true;
            }
        }

        false
    }

    fn handle_ground_contact(&mut self) -> bool {
        let mut cell_outside_board = false;

        for (x, y) in self.active_cells() {
            if y < 0 {
                cell_outside_board = true;
            } else {
                self.board[y as usize][x as usize] = self.active_tetromino;
            }
        }

        self.clear_lines();
        self.swap_next();

        cell_outside_board
    }

    fn clear_lines(&mut self) {
        let mut lines_cleared = 0;
        for y in 0..self.board.len() {
            let full = self.board[y].iter().all(|cell| *cell != 0);
            if full {
                for yp in (0..y).rev() {
                    // Just move every line above one line down...
                    self.board[yp + 1] = self.board[yp]
                }
                lines_cleared += 1;
            }
        }
        if lines_cleared > 0 {
            self.board[0] = [0; BOARD_WIDTH]; //... and clear the topmost line.
            self.reward_lines(lines_cleared);
        }
    }

    fn gen_next(&mut self) {
        self.next = self.rng.random_range(1..N_TETROMINOS);
    }

    fn swap_next(&mut self) {
        self.active_tetromino = self.next;
        self.rotation = 0;
        self.position = (3, -4);
        self.gen_next();
    }

    fn reward_lines(&mut self, lines_cleared: u32) {
        self.lines += lines_cleared;
        let score_raw = lines_cleared as u64 * 10;
        self.score += score_raw * score_raw;
    }

    pub fn distance_to_ground(&self) -> i16 {
        let mut test_offset = 0;

        loop {
            if self.collides_at(0, test_offset + 1) {
                break test_offset;
            }
            test_offset += 1;
        }
    }
}
