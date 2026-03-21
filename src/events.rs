use std::time::Instant;

use crossterm::event::{self, Event, KeyCode, MouseEventKind};

use crate::{GamePhase, GameState};

pub fn on_event(game_state: &mut GameState) -> Result<(), Box<dyn std::error::Error>> {
    match game_state.game_phase {
        GamePhase::Running => match event::read()? {
            Event::Mouse(mouse) => match mouse.kind {
                MouseEventKind::ScrollUp => game_state.rotate_counter_clockwise(),
                MouseEventKind::ScrollDown => game_state.rotate_clockwise(),
                _ => {}
            },
            Event::Key(key) => match key.code {
                KeyCode::Char('q') => game_state.game_phase = GamePhase::Quitting,
                KeyCode::Esc => game_state.game_phase = GamePhase::Paused,
                KeyCode::Left => game_state.move_left(),
                KeyCode::Right => game_state.move_right(),
                KeyCode::Up => game_state.rotate_clockwise(),
                KeyCode::Char('z') => game_state.rotate_counter_clockwise(),
                KeyCode::Down => {
                    game_state.tick_gravity();
                    game_state.last_gravity_tick = Instant::now();
                }
                KeyCode::Char(' ') => {
                    game_state.drop();
                    game_state.last_gravity_tick = Instant::now();
                }
                _ => {}
            },
            _ => {}
        },
        GamePhase::GameOver => {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => game_state.game_phase = GamePhase::Quitting,
                    KeyCode::Char('r') => *game_state = GameState::initial_state(),
                    _ => {}
                }
            }
        }
        GamePhase::Paused => {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('c') => game_state.game_phase = GamePhase::Running,
                    KeyCode::Char('q') => game_state.game_phase = GamePhase::Quitting,
                    KeyCode::Char('r') => *game_state = GameState::initial_state(),
                    _ => {}
                }
            }
        }
        _ => {}
    };

    Ok(())
}
