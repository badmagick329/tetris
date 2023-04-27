mod game;
mod tui;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::{event, terminal};
use game::sound::Player;
use std::path::Path;
use std::time::Duration;
use tui::Tui;
use tokio::sync::mpsc::{self, Sender};

const TICK_RATE: u64 = 250;
const SOUND_FILE: &str = "soundtrack.mp3";

pub async fn run() {
    let (tx, rx) = mpsc::channel::<usize>(1);
    let sound_handle = tokio::spawn(async move {
        sound_loop(rx);
    });
    game_loop(tx.clone()).await;
    tx.send(1).await.ok();
    sound_handle.await.ok();
}

async fn game_loop(tx: Sender<usize>) {
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
                Event::Key(KeyEvent {
                    code: KeyCode::Char(' '),
                    modifiers: KeyModifiers::NONE,
                    ..
                }) => {
                    game.drop_shape();
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('p'),
                    modifiers: KeyModifiers::NONE,
                    ..
                }) => {
                    game.paused = !game.paused;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('d'),
                    modifiers: KeyModifiers::NONE,
                    ..
                }) => {
                    tx.send(1).await.ok();
                }
                _ => {}
            }
        }
        game.update();
        term.draw_board(game.board_ref(), game.preview_board_ref(), game.score);
        if game.game_over {
            break;
        }
    }
    terminal::disable_raw_mode().unwrap();
}

fn sound_loop(rx: mpsc::Receiver<usize>) {
    let mut player = match Path::new(SOUND_FILE).exists() {
        true => Some(Player::new(SOUND_FILE, rx)),
        false => None,
    };
    if let Some(player) = &mut player {
        player.play();
    }
}
