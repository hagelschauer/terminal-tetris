mod events;
mod game_phase;
mod game_renderer;
mod game_state;
mod terminal_guard;
mod tetromino;

use std::time::Duration;
use std::vec;

use crossterm::event;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::*;
use ratatui::text::{Line, ToText};
use ratatui::widgets::Paragraph;
use ratatui::widgets::{Block, BorderType, Borders, Clear};

use crate::game_phase::GamePhase;
use crate::game_state::GameState;
use crate::terminal_guard::TerminalGuard;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal_guard = TerminalGuard::new()?;

    let mut game_state = GameState::initial_state();

    while game_state.game_phase != GamePhase::Quitting {
        terminal_guard
            .terminal
            .draw(|frame| render(frame, &game_state))?;

        if game_state.game_phase == GamePhase::Running {
            let gravity_rate = game_state.gravity_rate();

            let timeout = gravity_rate
                .checked_sub(game_state.last_gravity_tick.elapsed())
                .unwrap_or(Duration::ZERO);

            if event::poll(timeout)? {
                events::on_event(&mut game_state)?;
            }

            if game_state.last_gravity_tick.elapsed() >= gravity_rate {
                game_state.tick_gravity();
            }
        } else {
            events::on_event(&mut game_state)?;
        }
    }

    Ok(())
}

fn render(frame: &mut Frame, game_state: &GameState) {
    render_gui(frame, game_state);

    match game_state.game_phase {
        GamePhase::Paused => render_pause_popup(frame),
        GamePhase::GameOver => render_gameover_popup(frame, game_state),
        _ => {}
    }
}

fn render_gameover_popup(frame: &mut Frame, game_state: &GameState) {
    let popup_area = frame
        .area()
        .centered(Constraint::Length(30), Constraint::Length(7));

    let block = Block::default()
        .title(" Game Over ")
        .title_alignment(HorizontalAlignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Double);

    let content_areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Fill(1), Constraint::Min(2)])
        .split(block.inner(popup_area));

    let scoreboard = Paragraph::new(format!("Score: {}", game_state.score)).centered();

    let text = Paragraph::new("Press q to quit\nPress r to restart").centered();

    frame.render_widget(Clear, popup_area);
    frame.render_widget(block, popup_area);
    frame.render_widget(scoreboard, content_areas[0]);
    frame.render_widget(text, content_areas[1]);
}

fn render_pause_popup(frame: &mut Frame) {
    let popup_area = frame
        .area()
        .centered(Constraint::Length(30), Constraint::Length(5));

    let block = Block::default()
        .title(" Paused ")
        .title_alignment(HorizontalAlignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Double);

    let text = Paragraph::new("Press c to continue\nPress q to quit\nPress r to restart")
        .centered()
        .block(block);

    frame.render_widget(Clear, popup_area);
    frame.render_widget(text, popup_area);
}

fn render_gui(frame: &mut Frame, game_state: &GameState) {
    let root_area = frame.area().centered(
        Constraint::Length(
            2 + 4
                + 8
                + 2
                + game_renderer::PHYSICAL_BOARD_WIDTH
                + 2
                + game_renderer::PHYSICAL_NEXT_WIDTH
                + 2,
        ),
        Constraint::Length(game_renderer::PHYSICAL_BOARD_HEIGHT + 2 + 2),
    );

    let root = Block::default()
        .borders(Borders::all())
        .border_type(BorderType::Double);

    frame.render_widget(&root, root_area);

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Fill(0),
            Constraint::Length(20 + 2),
            Constraint::Length(8 + 2),
        ])
        .split(root.inner(root_area));

    let scoreboard_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Fill(1), Constraint::Fill(1)])
        .split(columns[0])[1];

    let next_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(4 + 2), Constraint::Fill(0)])
        .split(columns[2])[0];

    let board_block = Block::default()
        .borders(Borders::all())
        .border_type(BorderType::Rounded);

    let next_block = Block::default()
        .title_top(Line::from(" Next ").centered())
        .borders(Borders::all())
        .border_type(BorderType::Rounded);

    let scoreboard_block = Block::default()
        .borders(Borders::all())
        .border_type(BorderType::Rounded);

    let board = Paragraph::new(game_renderer::render_board(game_state)).block(board_block);

    let next = Paragraph::new(game_renderer::render_next(game_state)).block(next_block);

    render_scoreboard(frame, game_state, scoreboard_block.inner(scoreboard_area));
    frame.render_widget(scoreboard_block, scoreboard_area);
    frame.render_widget(board, columns[1]);
    frame.render_widget(next, next_area);
}

fn render_scoreboard(frame: &mut Frame, game_state: &GameState, area: Rect) {
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Fill(0),
            Constraint::Fill(0),
            Constraint::Fill(0),
        ])
        .split(area);

    let block = Block::default()
        .borders(Borders::all())
        .border_type(BorderType::Rounded);

    let score_display = Paragraph::new(game_state.score.to_text())
        .centered()
        .block(block.clone().title(" Score: "));

    let lines_display = Paragraph::new(game_state.lines.to_text())
        .centered()
        .block(block.clone().title(" Lines: "));

    let level = game_state.level();
    let level_display = Paragraph::new(level.to_text())
        .centered()
        .block(block.title(" Level: "));

    frame.render_widget(score_display, areas[0]);
    frame.render_widget(lines_display, areas[1]);
    frame.render_widget(level_display, areas[2]);
}
