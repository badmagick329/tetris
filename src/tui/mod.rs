#![allow(unused_imports, dead_code)]
use super::game::{Game, HEIGHT, WIDTH};
use crossterm::{
    cursor, queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, size},
};
use std::{
    collections::HashMap,
    io::{stdout, Read, Write},
};

pub struct Tui {
    colors: HashMap<u8, Color>,
    border: &'static str,
    space: &'static str,
    pub message: String,
}

impl Tui {
    pub fn new() -> Self {
        Tui {
            colors: [
                Color::Black, // 0 is empty
                Color::Cyan,
                Color::Blue,
                Color::Rgb {
                    r: 255,
                    g: 165,
                    b: 0,
                }, // Orange
                Color::Yellow,
                Color::Rgb {
                    r: 102,
                    g: 255,
                    b: 102,
                }, // Light Green
                Color::Magenta,
                Color::Red,
            ]
            .iter()
            .enumerate()
            .map(|(i, c)| (i as u8, *c))
            .collect(),
            border: "─",
            space: " ",
            message: "Press 'q' to quit.".to_string(),
        }
    }

    pub fn draw_board(
        &mut self,
        board: &[[u8; WIDTH]; HEIGHT],
        preview_board: &[[u8; 4]; 4],
        score: usize,
    ) {
        let mut stdout = stdout();
        let (width, height) = size().unwrap();
        let (width, height) = (width as usize, height as usize);
        let (x, y) = (width / 2 - WIDTH, height / 2 - (HEIGHT / 2));
        queue!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
        // Board borders
        queue!(
            stdout,
            cursor::MoveTo(x as u16, y as u16),
            SetForegroundColor(Color::White),
            Print(format!("┌{}┐", self.border.repeat(WIDTH * 2 + 2))), // board width * 2 + 2 lines
        )
        .unwrap();
        for i in 0..HEIGHT {
            queue!(
                stdout,
                cursor::MoveTo(x as u16, (y + i + 1) as u16),
                Print(format!("│{}│", self.space.repeat(WIDTH * 2 + 2))), // board width * 2 + 2 spaces
            )
            .unwrap();
        }
        queue!(
            stdout,
            cursor::MoveTo(x as u16, (y + HEIGHT + 1) as u16),
            Print(format!("└{}┘", self.border.repeat(WIDTH * 2 + 2))),
            ResetColor,
        )
        .unwrap();
        // Board content
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                if board[i][j] != 0 {
                    queue!(
                        stdout,
                        cursor::MoveTo((x + j * 2 + 2) as u16, (y + i + 1) as u16),
                        SetForegroundColor(self.colors[&board[i][j]]),
                        Print("██"),
                        ResetColor,
                    )
                    .unwrap();
                }
            }
        }
        // Message
        self.message = format!("Score: {}", score);
        queue!(
            stdout,
            cursor::MoveTo((x + 2) as u16, 1 as u16),
            SetForegroundColor(Color::White),
            Print(&self.message),
            ResetColor,
        )
        .unwrap();
        // Preview Board borders
        queue!(
            stdout,
            cursor::MoveTo((x + WIDTH * 2 + 4) as u16, (y + 1) as u16),
            SetForegroundColor(Color::White),
            Print("Next:"),
            ResetColor,
        )
        .unwrap();
        queue!(
            stdout,
            cursor::MoveTo((x + WIDTH * 2 + 4) as u16, (y + 2) as u16),
            Print(format!("┌{}┐", self.border.repeat(10))), // 4 * 2 + 2
        )
        .unwrap();
        for i in 0..4 {
            queue!(
                stdout,
                cursor::MoveTo((x + WIDTH * 2 + 4) as u16, (y + i + 3) as u16),
                Print(format!("│{}│", self.space.repeat(10))),
            )
            .unwrap();
        }
        queue!(
            stdout,
            cursor::MoveTo((x + WIDTH * 2 + 4) as u16, (y + 7) as u16),
            Print(format!("└{}┘", self.border.repeat(10))),
        )
        .unwrap();
        // Preview Board content
        for i in 0..4 {
            for j in 0..4 {
                if preview_board[i][j] != 0 {
                    queue!(
                        stdout,
                        cursor::MoveTo(((x + WIDTH * 2 + 6) + j * 2) as u16, (y + i + 3) as u16),
                        SetForegroundColor(self.colors[&preview_board[i][j]]),
                        // SetForegroundColor(Color::Red),
                        Print("██"),
                        ResetColor,
                    )
                    .unwrap();
                }
            }
        }
        queue!(stdout, cursor::MoveTo(0, height as u16)).unwrap();
        stdout.flush().unwrap();
    }
}
