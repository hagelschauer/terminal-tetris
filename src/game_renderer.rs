use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

use crate::{
    game_state::{BOARD_HEIGHT, BOARD_WIDTH, GameState, NEXT_HEIGHT, NEXT_WIDTH},
    tetromino::{TETROMINO_SIZE, TETROMINOS},
};

pub const PHYSICAL_BOARD_WIDTH: u16 = BOARD_WIDTH as u16 * 2;
pub const PHYSICAL_BOARD_HEIGHT: u16 = BOARD_HEIGHT as u16;
pub const PHYSICAL_NEXT_WIDTH: u16 = NEXT_WIDTH as u16 * 2;
pub const PHYSICAL_NEXT_HEIGHT: u16 = NEXT_HEIGHT as u16;
const EMPTY_FIELD: &str = ". ";
const PIECE: &str = "██";
const GHOST_PIECE: &str = "░░";

pub fn render_board(state: &GameState) -> Vec<Line<'_>> {
    let mut canvas = Vec::with_capacity(PHYSICAL_BOARD_HEIGHT as usize);
    for y in 0..state.board.len() {
        let mut spans = Vec::with_capacity(PHYSICAL_BOARD_WIDTH as usize);
        for x in 0..state.board[y].len() {
            let cell = state.board[y][x];
            let span = if cell == 0 {
                Span::from(EMPTY_FIELD)
            } else {
                Span::styled(PIECE, cell_style(cell))
            };
            spans.push(span);
        }
        canvas.push(Line::from(spans));
    }

    let distance_to_ground = state.distance_to_ground();

    for (x, y) in state.active_cells() {
        if y < canvas.len() as i16 && (x as usize) < canvas[0].spans.len() {
            if (y + distance_to_ground) >= 0 {
                canvas[(y + distance_to_ground) as usize].spans[x as usize] =
                    Span::styled(GHOST_PIECE, cell_style(state.active_tetromino));
            }

            if y >= 0 {
                canvas[y as usize].spans[x as usize] =
                    Span::styled(PIECE, cell_style(state.active_tetromino));
            }
        }
    }

    canvas
}

pub fn render_next(state: &GameState) -> Vec<Line<'_>> {
    let color = state.next;
    let mut tetromino = TETROMINOS[state.next as usize].rotations[0];
    let mut canvas = Vec::with_capacity(PHYSICAL_NEXT_HEIGHT as usize);
    for _ in 0..TETROMINO_SIZE {
        let mut spans = Vec::with_capacity(PHYSICAL_NEXT_WIDTH as usize);
        for _ in 0..TETROMINO_SIZE {
            let span = if tetromino & 1 == 0 {
                Span::from(EMPTY_FIELD)
            } else {
                Span::styled(PIECE, cell_style(color))
            };
            tetromino >>= 1;
            spans.push(span);
        }
        canvas.push(Line::from(spans));
    }
    canvas
}

fn cell_style(cell: u8) -> Style {
    match cell {
        1 => Style::default().fg(Color::Cyan),
        2 => Style::default().fg(Color::Yellow),
        3 => Style::default().fg(Color::Magenta),
        4 => Style::default().fg(Color::Green),
        5 => Style::default().fg(Color::Red),
        6 => Style::default().fg(Color::Blue),
        7 => Style::default().fg(Color::LightRed),
        _ => Style::default(),
    }
}
