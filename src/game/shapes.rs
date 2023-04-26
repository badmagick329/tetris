#![allow(unused_imports, dead_code, unused_variables)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Dir {
    Right,
    Down,
    Left,
    Up,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ShapeType {
    I = 1,
    J = 2,
    L = 3,
    O = 4,
    S = 5,
    T = 6,
    Z = 7,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Shape {
    pub dir: Dir,
    pub shape_type: ShapeType,
    pub x: usize,
    pub y: usize,
}

impl Shape {
    pub fn new(x: usize, y: usize, shape_type: ShapeType) -> Self {
        Shape {
            dir: Dir::Right,
            shape_type,
            x,
            y,
        }
    }

    fn to_coords_i(&self, dir: Dir) -> Vec<(isize, isize)> {
        let x = self.x as isize;
        let y = self.y as isize;
        match dir {
            Dir::Right | Dir::Left => vec![(x, y), (x - 1, y), (x + 1, y), (x + 2, y)],
            Dir::Down | Dir::Up => vec![(x, y), (x, y - 3), (x, y - 2), (x, y - 1)],
        }
    }

    fn to_coords_j(&self, dir: Dir) -> Vec<(isize, isize)> {
        let x = self.x as isize;
        let y = self.y as isize;
        match dir {
            Dir::Right => vec![(x, y), (x - 1, y - 1), (x - 1, y), (x + 1, y)],
            Dir::Down => vec![(x, y), (x, y - 1), (x + 1, y - 1), (x, y + 1)],
            Dir::Left => vec![(x, y), (x - 1, y), (x + 1, y), (x + 1, y + 1)],
            Dir::Up => vec![(x, y), (x, y - 1), (x - 1, y + 1), (x, y + 1)],
        }
    }

    fn to_coords_l(&self, dir: Dir) -> Vec<(isize, isize)> {
        let x = self.x as isize;
        let y = self.y as isize;
        match dir {
            Dir::Right => vec![(x, y), (x - 1, y), (x + 1, y), (x - 1, y + 1)],
            Dir::Down => vec![(x, y), (x - 1, y - 1), (x, y - 1), (x, y + 1)],
            Dir::Left => vec![(x, y), (x + 1, y - 1), (x - 1, y), (x + 1, y)],
            Dir::Up => vec![(x, y), (x, y - 1), (x, y + 1), (x + 1, y + 1)],
        }
    }

    fn to_coords_o(&self, dir: Dir) -> Vec<(isize, isize)> {
        let x = self.x as isize;
        let y = self.y as isize;
        vec![(x, y), (x + 1, y), (x, y + 1), (x + 1, y + 1)]
    }

    fn to_coords_s(&self, dir: Dir) -> Vec<(isize, isize)> {
        let x = self.x as isize;
        let y = self.y as isize;
        todo!()
    }

    fn to_coords_t(&self, dir: Dir) -> Vec<(isize, isize)> {
        let x = self.x as isize;
        let y = self.y as isize;
        todo!()
    }

    fn to_coords_z(&self, dir: Dir) -> Vec<(isize, isize)> {
        let x = self.x as isize;
        let y = self.y as isize;
        todo!()
    }
}

pub trait ShapeTrait {
    fn to_coords(&self, dir: Dir) -> Vec<(isize, isize)>;
    fn next_dir(&self, dir: Dir) -> Dir;
}

impl ShapeTrait for Shape {
    /// Returns a vector of coordinates for the shape
    /// in the given direction.
    /// The coordinates are relative to the shape's
    /// position.
    /// The home coords should be returned first in the vector. This
    /// is the x,y value that the position will be set to. The rest
    /// of the coords should be returned from left to right, top to bottom.
    ///
    /// For the following I shape at position (1, 3):
    /// 0 x 0 0
    /// 0 x 0 0
    /// 0 x 0 0
    /// 0 H 0 0
    ///
    /// The coords returned will be [(1, 3), (1, 2), (1, 1), (1, 0)]
    ///
    /// For the following J shape at position (1, 3):
    /// 0 x 0
    /// 0 H 0
    /// x x 0
    ///
    /// The coords returned will be [(1, 3), (1, 2), (0, 4), (1, 4)]
    ///
    /// # Arguments
    /// * `dir` - The direction to return the coordinates for
    fn to_coords(&self, dir: Dir) -> Vec<(isize, isize)> {
        let x = self.x as isize;
        let y = self.y as isize;
        match &self.shape_type {
            ShapeType::I => self.to_coords_i(dir),
            ShapeType::J => self.to_coords_j(dir),
            ShapeType::L => self.to_coords_l(dir),
            ShapeType::O => self.to_coords_o(dir),
            ShapeType::S => self.to_coords_s(dir),
            ShapeType::T => self.to_coords_t(dir),
            ShapeType::Z => self.to_coords_z(dir),
        }
    }

    fn next_dir(&self, dir: Dir) -> Dir {
        match dir {
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Up => Dir::Right,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_shape(shape_type: ShapeType) -> Shape {
        Shape {
            dir: Dir::Right,
            shape_type,
            x: 0,
            y: 0,
        }
    }

    #[test]
    fn test_shape() {
        let shape = create_shape(ShapeType::I);
        assert_eq!(shape.dir, Dir::Right);
        assert_eq!(shape.x, 0);
        assert_eq!(shape.y, 0);
    }

    #[test]
    fn test_shape_to_coords_i() {
        let shape = create_shape(ShapeType::I);
        assert_eq!(
            shape.to_coords(Dir::Right),
            vec![(0, 0), (-1, 0), (1, 0), (2, 0)]
        );
        assert_eq!(
            shape.to_coords(Dir::Down),
            vec![(0, 0), (0, -3), (0, -2), (0, -1)]
        );
        assert_eq!(
            shape.to_coords(Dir::Left),
            vec![(0, 0), (-1, 0), (1, 0), (2, 0)]
        );
        assert_eq!(
            shape.to_coords(Dir::Up),
            vec![(0, 0), (0, -3), (0, -2), (0, -1)]
        );
    }

    #[test]
    fn test_shape_to_coords_j() {
        let shape = create_shape(ShapeType::J);
        assert_eq!(
            shape.to_coords(Dir::Right),
            vec![(0, 0), (-1, -1), (-1, 0), (1, 0)]
        );
        assert_eq!(
            shape.to_coords(Dir::Down),
            vec![(0, 0), (0, -1), (1, -1), (0, 1)]
        );
        assert_eq!(
            shape.to_coords(Dir::Left),
            vec![(0, 0), (-1, 0), (1, 0), (1, 1)]
        );
        assert_eq!(
            shape.to_coords(Dir::Up),
            vec![(0, 0), (0, -1), (-1, 1), (0, 1)]
        );
    }

    #[test]
    fn test_shape_to_coords_l() {
        let shape = create_shape(ShapeType::L);
        assert_eq!(
            shape.to_coords(Dir::Right),
            vec![(0, 0), (-1, 0), (1, 0), (-1, 1)]
        );
        assert_eq!(
            shape.to_coords(Dir::Down),
            vec![(0, 0), (-1, -1), (0, -1), (0, 1)]
        );
        assert_eq!(
            shape.to_coords(Dir::Left),
            vec![(0, 0), (1, -1), (-1, 0), (1, 0)]
        );
        assert_eq!(
            shape.to_coords(Dir::Up),
            vec![(0, 0), (0, -1), (0, 1), (1, 1)]
        );
    }
}
