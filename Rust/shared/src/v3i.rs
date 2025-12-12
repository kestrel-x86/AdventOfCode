use std::{
    fmt::Display,
    hash::Hash,
    ops::{self},
};

use crate::v2i::Vector2Int;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct Vector3Int {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

#[allow(unused)]
impl Vector3Int {
    pub const ZERO: Vector3Int = Vector3Int { x: 0, y: 0, z: 0 };
    pub const ONE: Vector3Int = Vector3Int { x: 1, y: 1, z: 1 };
    pub const MAX: Vector3Int = Vector3Int {
        x: isize::MAX,
        y: isize::MAX,
        z: isize::MAX,
    };

    pub const fn new(x: isize, y: isize, z: isize) -> Self {
        return Vector3Int { x: x, y: y, z: z };
    }

    pub fn v2(&self) -> Vector2Int {
        return Vector2Int { x: self.x, y: self.y };
    }

    /// Check if in range inclusive
    pub fn in_range(&self, min: &Vector3Int, max: &Vector3Int) -> bool {
        return self.x >= min.x
            && self.x <= max.x
            && self.y >= min.y
            && self.y <= max.y
            && self.z >= min.z
            && self.z <= max.z;
    }

    pub fn min(&self, other: &Vector3Int) -> Vector3Int {
        return Vector3Int {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
        };
    }

    pub fn max(&self, other: &Vector3Int) -> Vector3Int {
        return Vector3Int {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        };
    }

    pub const fn zero() -> Self {
        return Vector3Int { x: 0, y: 0, z: 0 };
    }

    pub fn up() -> Self {
        return Vector3Int { x: 0, y: 1, z: 0 };
    }

    pub fn down() -> Self {
        return Vector3Int { x: 0, y: -1, z: 0 };
    }

    pub fn left() -> Self {
        return Vector3Int { x: -1, y: 0, z: 0 };
    }

    pub fn right() -> Self {
        return Vector3Int { x: 1, y: 0, z: 0 };
    }

    pub fn foward() -> Self {
        return Vector3Int { x: 0, y: 0, z: 1 };
    }

    pub fn backward() -> Self {
        return Vector3Int { x: 0, y: 0, z: -1 };
    }

    pub fn manhattan(&self, rhs: Vector3Int) -> usize {
        return (self.x - rhs.x).abs() as usize
            + (self.y - rhs.y).abs() as usize
            + (self.z - rhs.z).abs() as usize;
    }

    /// Return euclidian distance between to Vector3s
    pub fn dist(&self, rhs: &Vector3Int) -> usize {
        let sqrmag = (self.x - rhs.x).pow(2) + (self.y - rhs.y).pow(2) + (self.z - rhs.z).pow(2);
        return (self.dist_sq(rhs) as f64).sqrt() as usize + 1;
    }

    /// Return squared euclidian distance between to Vector3s
    ///
    /// Sqrts are expensive so this should be used when all you need to do is compare distances
    pub fn dist_sq(&self, rhs: &Vector3Int) -> usize {
        ((self.x - rhs.x).pow(2) + (self.y - rhs.y).pow(2) + (self.z - rhs.z).pow(2)) as usize
    }
}

impl Display for Vector3Int {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}, {}, {}>", self.x, self.y, self.z)
    }
}

// +
impl ops::Add<Vector3Int> for Vector3Int {
    type Output = Vector3Int;
    fn add(self, rhs: Vector3Int) -> Self::Output {
        return Vector3Int::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z);
    }
}

// + Vector2Int
impl ops::Add<Vector2Int> for Vector3Int {
    type Output = Vector3Int;
    fn add(self, rhs: Vector2Int) -> Self::Output {
        return Vector3Int::new(self.x + rhs.x, self.y + rhs.y, self.z);
    }
}

// +=
impl ops::AddAssign<Vector3Int> for Vector3Int {
    fn add_assign(&mut self, rhs: Vector3Int) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

// -
impl ops::Sub<Vector3Int> for Vector3Int {
    type Output = Vector3Int;
    fn sub(self, rhs: Vector3Int) -> Self::Output {
        return Vector3Int::new(self.x - rhs.x, self.y - rhs.y, self.z + rhs.z);
    }
}

// -=
impl ops::SubAssign<Vector3Int> for Vector3Int {
    fn sub_assign(&mut self, rhs: Vector3Int) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

// *
impl ops::Mul<Vector3Int> for Vector3Int {
    type Output = Vector3Int;
    fn mul(self, rhs: Vector3Int) -> Self::Output {
        return Vector3Int::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.y);
    }
}

// * <isize>
impl ops::Mul<isize> for Vector3Int {
    type Output = Vector3Int;
    fn mul(self, rhs: isize) -> Self::Output {
        return Vector3Int::new(self.x * rhs, self.y * rhs, self.z * rhs);
    }
}

// *=
impl ops::MulAssign<Vector3Int> for Vector3Int {
    fn mul_assign(&mut self, rhs: Vector3Int) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

// /
impl ops::Div<Vector3Int> for Vector3Int {
    type Output = Vector3Int;
    fn div(self, rhs: Vector3Int) -> Self::Output {
        return Vector3Int::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z);
    }
}

// /=
impl ops::DivAssign<Vector3Int> for Vector3Int {
    fn div_assign(&mut self, rhs: Vector3Int) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}
