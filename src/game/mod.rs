#![allow(unused_imports, dead_code)]
pub mod shapes;
use shapes::{Dir, Shape, ShapeTrait, ShapeType};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    time::{self, Duration, Instant},
};
pub const WIDTH: usize = 12;
pub const HEIGHT: usize = 20;
const FALL_RATE: u128 = 750;

#[derive(Debug, PartialEq)]
pub enum Move {
    Left,
    Right,
    Down,
    Rotate,
}

pub struct Game {
    board: [[u8; WIDTH]; HEIGHT],
    pub active_shape: Option<Shape>,
    game_over: bool,
    fall_timer: Instant,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: [[0; WIDTH]; HEIGHT],
            active_shape: None,
            game_over: false,
            fall_timer: Instant::now(),
        }
    }

    pub fn spawn(&mut self, shape_type: ShapeType, x: usize, y: usize) {
        let shape = Shape::new(x, y, shape_type);
        if !self.valid_move(&shape.to_coords(shape.dir), &shape.to_coords(shape.dir)) {
            self.game_over = true;
            return;
        }
        self.active_shape = Some(shape);
    }

    pub fn update(&mut self) {
        if self.game_over {
            return;
        }
        // Handle logic
        if self.active_shape.is_some() {
            if self.fall_timer.elapsed().as_millis() > FALL_RATE {
                self.fall_timer = Instant::now();
                let old_coords = self
                    .active_shape
                    .as_ref()
                    .unwrap()
                    .to_coords(self.active_shape.as_ref().unwrap().dir);
                self.move_shape(Move::Down);
                let new_coords = self
                    .active_shape
                    .as_ref()
                    .unwrap()
                    .to_coords(self.active_shape.as_ref().unwrap().dir);
                if old_coords == new_coords {
                    self.shape_to_board();
                    self.clear_completed();
                } else {
                    self.clear_coords(&old_coords);
                }
            }
        } else {
            let random_shape = ShapeType::random();
            self.spawn(random_shape, WIDTH / 2, 0);
        }
        // Update shape position
        if let Some(shape) = &mut self.active_shape {
            let coords = shape.to_coords(shape.dir);
            for (x, y) in coords {
                if x < 0 || x >= WIDTH as isize || y < 0 || y >= HEIGHT as isize {
                    continue;
                }
                self.board[y as usize][x as usize] = shape.shape_type as u8;
            }
        }
    }

    pub fn valid_move(
        &mut self,
        old_coords: &Vec<(isize, isize)>,
        new_coords: &Vec<(isize, isize)>,
    ) -> bool {
        for (xv, yv) in new_coords {
            let (x, y) = (*xv, *yv);
            if y < 0 {
                continue;
            }
            if x < 0 || x >= WIDTH as isize || y >= HEIGHT as isize {
                return false;
            }
            if old_coords.contains(&(x, y)) {
                continue;
            }
            if self.board[y as usize][x as usize] != 0 {
                return false;
            }
        }
        true
    }

    pub fn move_shape(&mut self, dir: Move) {
        if self.game_over {
            return;
        }
        if self.active_shape.is_none() {
            return;
        }
        let mut shape = self.active_shape.take().unwrap();
        let old_coords = shape.to_coords(shape.dir);
        let mut new_coords = old_coords.clone();
        match dir {
            Move::Left => {
                for (x, _) in &mut new_coords {
                    *x -= 1;
                }
            }
            Move::Right => {
                for (x, _) in &mut new_coords {
                    *x += 1;
                }
            }
            Move::Down => {
                for (_, y) in &mut new_coords {
                    *y += 1;
                }
            }
            Move::Rotate => {
                let new_dir = shape.next_dir(shape.dir);
                new_coords = shape.to_coords(new_dir);
            }
        }
        if self.valid_move(&old_coords, &new_coords) {
            match dir {
                Move::Rotate => {
                    shape.dir = shape.next_dir(shape.dir);
                    self.clear_coords(&old_coords);
                }
                _ => {
                    shape.x = new_coords[0].0 as usize;
                    shape.y = new_coords[0].1 as usize;
                    self.clear_coords(&old_coords);
                }
            }
        }
        self.active_shape = Some(shape);
    }

    pub fn shape_to_board(&mut self) {
        if self.active_shape.is_none() {
            return;
        }
        let shape = self.active_shape.take().unwrap();
        let coords = shape.to_coords(shape.dir);
        for (x, y) in coords {
            if x < 0 || x >= WIDTH as isize || y < 0 || y >= HEIGHT as isize {
                continue;
            }
            self.board[y as usize][x as usize] = shape.shape_type as u8;
        }
    }

    pub fn clear_coords(&mut self, coords: &Vec<(isize, isize)>) {
        for (xv, yv) in coords {
            let (x, y) = (*xv, *yv);
            if x < 0 || x >= WIDTH as isize || y < 0 || y >= HEIGHT as isize {
                continue;
            }
            self.board[y as usize][x as usize] = 0;
        }
    }

    pub fn clear_completed(&mut self) {
        let mut completed = Vec::new();
        for (y, row) in self.board.iter().enumerate() {
            if row.iter().all(|&x| x != 0) {
                completed.push(y);
            }
        }
        for y in completed.clone() {
            for x in 0..WIDTH {
                self.board[y][x] = 0;
            }
        }
        for y in completed {
            for y2 in (0..y).rev() {
                for x in 0..WIDTH {
                    self.board[y2 + 1][x] = self.board[y2][x];
                }
            }
        }
    }

    pub fn board_ref(&self) -> &[[u8; WIDTH]; HEIGHT] {
        &self.board
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        let mut game = Game::new();
        let mut shape = Shape::new(3, 0, ShapeType::I);
        let coords = shape.to_coords(shape.dir);
        assert!(game.valid_move(&coords, &coords));
        game.board[1][3] = 1;
        assert!(game.valid_move(&coords, &coords));
        game.board[1][3] = 1;
        shape.y += 1;
        let new_coords = shape.to_coords(shape.dir);
        assert!(!game.valid_move(&coords, &new_coords));
    }

    #[test]
    fn test_move_shape_pos() {
        let mut game = Game::new();
        game.spawn(ShapeType::I, 3, 0);
        game.move_shape(Move::Down);
        assert_eq!(game.active_shape.unwrap().y, 1);
        game.move_shape(Move::Left);
        assert_eq!(game.active_shape.unwrap().x, 2);
        game.move_shape(Move::Right);
        assert_eq!(game.active_shape.unwrap().x, 3);
        game.move_shape(Move::Rotate);
        assert_eq!(game.active_shape.unwrap().dir, Dir::Down);
    }

    #[test]
    fn test_move_shape_coords() {
        let mut game = Game::new();
        game.spawn(ShapeType::I, 3, 0);
        let coords = game
            .active_shape
            .unwrap()
            .to_coords(game.active_shape.unwrap().dir);
        assert_eq!(coords, vec![(3, 0), (2, 0), (4, 0), (5, 0)]);
        game.move_shape(Move::Down);
        let coords = game
            .active_shape
            .unwrap()
            .to_coords(game.active_shape.unwrap().dir);
        assert_eq!(coords, vec![(3, 1), (2, 1), (4, 1), (5, 1)]);
        game.move_shape(Move::Left);
        let coords = game
            .active_shape
            .unwrap()
            .to_coords(game.active_shape.unwrap().dir);
        assert_eq!(coords, vec![(2, 1), (1, 1), (3, 1), (4, 1)]);
        game.move_shape(Move::Right);
        let coords = game
            .active_shape
            .unwrap()
            .to_coords(game.active_shape.unwrap().dir);
        assert_eq!(coords, vec![(3, 1), (2, 1), (4, 1), (5, 1)]);
    }

    #[test]
    fn test_move_shape_rotation() {
        let mut game = Game::new();
        game.spawn(ShapeType::I, 3, 0);
        let coords = game
            .active_shape
            .unwrap()
            .to_coords(game.active_shape.unwrap().dir);
        assert_eq!(coords, vec![(3, 0), (2, 0), (4, 0), (5, 0)]);
        game.move_shape(Move::Rotate);
        let coords = game
            .active_shape
            .unwrap()
            .to_coords(game.active_shape.unwrap().dir);
        assert_eq!(coords, vec![(3, 0), (3, -3), (3, -2), (3, -1)]);
        game.move_shape(Move::Rotate);
        let coords = game
            .active_shape
            .unwrap()
            .to_coords(game.active_shape.unwrap().dir);
        assert_eq!(coords, vec![(3, 0), (2, 0), (4, 0), (5, 0)]);
        game.move_shape(Move::Rotate);
        let coords = game
            .active_shape
            .unwrap()
            .to_coords(game.active_shape.unwrap().dir);
        assert_eq!(coords, vec![(3, 0), (3, -3), (3, -2), (3, -1)]);
    }
}
