use std::{
    fmt::Display,
    fs::{self, File, OpenOptions},
    io::Write,
    ops::{Add, AddAssign, Mul, Rem},
};

pub fn input(name: &str) -> String {
    // recreate empty output file
    let _ = File::create("./output");
    fs::read_to_string(format!("./{name}")).unwrap()
}

pub fn output(result: impl Display) {
    println!("{result}");
    let mut file = OpenOptions::new().append(true).open("./output").unwrap();
    file.write_all(format!("{}\n", result).as_bytes()).unwrap();
}

/// A vector in Num^2.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec2D<T>
where
    T: Add<Output = T>,
{
    pub x: T,
    pub y: T,
}

impl<T> Add for Vec2D<T>
where
    T: Add<Output = T>,
{
    type Output = Vec2D<T>;

    fn add(self, rhs: Vec2D<T>) -> Self::Output {
        Vec2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> AddAssign for Vec2D<T>
where
    T: Add<Output = T> + AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T, I> Mul<I> for Vec2D<T>
where
    T: Add<Output = T> + Mul<I, Output = T>,
    I: Copy,
{
    type Output = Vec2D<T>;

    /// Scalar multiplication.
    fn mul(self, rhs: I) -> Self::Output {
        Vec2D {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> Rem for Vec2D<T>
where
    T: Add<Output = T> + Rem<Output = T> + Copy,
{
    type Output = Vec2D<T>;

    /// Calculates the elementwise modulus.
    fn rem(self, rhs: Self) -> Self::Output {
        Vec2D {
            x: ((self.x % rhs.x) + rhs.x) % rhs.x,
            y: ((self.y % rhs.y) + rhs.y) % rhs.y,
        }
    }
}

impl<T> Ord for Vec2D<T>
where
    T: Ord + Add<Output = T>,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.x.cmp(&other.x).then_with(|| self.y.cmp(&other.y))
    }
}

impl<T> PartialOrd for Vec2D<T>
where
    T: Ord + Add<Output = T>,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Vec2D<T>
where
    T: Add<Output = T>,
{
    pub const fn new(x: T, y: T) -> Self {
        Vec2D { x, y }
    }
}

impl<T> Vec2D<T>
where
    T: Add<Output = T> + PartialOrd,
{
    /// Work in progress...
    pub fn bounded_add(self, rhs: Vec2D<T>, w: T, h: T) -> Option<Vec2D<T>> {
        let nx = self.x + rhs.x;
        let ny = self.y + rhs.y;
        if nx < w && ny < h {
            Some(Vec2D { x: nx, y: ny })
        } else {
            None
        }
    }
}

// i32 types
pub const NORTH: Vec2D<i32> = Vec2D::new(0, -1);
pub const WEST: Vec2D<i32> = Vec2D::new(-1, 0);
pub const SOUTH: Vec2D<i32> = Vec2D::new(0, 1);
pub const EAST: Vec2D<i32> = Vec2D::new(1, 0);

pub const CARDINAL: [Vec2D<i32>; 4] = [NORTH, WEST, SOUTH, EAST];
