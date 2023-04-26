#![allow(unused_imports, dead_code, unused_mut)]
mod game;
mod tui;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::{event, terminal};
use game::shapes::ShapeType;
use std::time::Duration;
use tui::Tui;

const TICK_RATE: u64 = 250;

pub fn run() {
    let mut term = Tui::new();
    let mut game = game::Game::new();
    // game.spawn(ShapeType::I, 5, 5);
    terminal::enable_raw_mode().unwrap();
    loop {
        if event::poll(Duration::from_millis(TICK_RATE)).unwrap() {
            match event::read().unwrap() {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::NONE,
                    ..
                }) => {
                    break;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Left,
                    modifiers: KeyModifiers::NONE,
                    ..
                }) => {
                    game.move_shape(game::Move::Left);
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Right,
                    modifiers: KeyModifiers::NONE,
                    ..
                }) => {
                    game.move_shape(game::Move::Right);
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Down,
                    modifiers: KeyModifiers::NONE,
                    ..
                }) => {
                    game.move_shape(game::Move::Down);
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Up,
                    modifiers: KeyModifiers::NONE,
                    ..
                }) => {
                    game.move_shape(game::Move::Rotate);
                }
                _ => {}
            }
        }
        game.update();
        term.message = format!("Fall timer: {:?}", game.fall_timer);
        term.draw_board(game.board_ref());
    }
    terminal::disable_raw_mode().unwrap();
}
