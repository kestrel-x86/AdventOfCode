#![allow(unused)]

use std::{
    iter::StepBy,
    ops::{Index, IndexMut},
    slice::Iter,
};

use crate::v3i::Vector3Int;

#[derive(Clone, Copy)]
pub struct Size3D {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

pub struct Grid3D<T> {
    inner: Vec<T>,
    size: Size3D,
}

impl<T> Grid3D<T> {
    pub fn new(x_len: usize, y_len: usize, z_len: usize) -> Self
    where
        T: Default,
    {
        let mut v = Vec::new();
        v.resize_with(x_len * y_len * z_len, T::default);
        return Grid3D {
            inner: v,
            size: Size3D {
                x: x_len,
                y: y_len,
                z: z_len,
            },
        };
    }

    pub fn from_iter<I: IntoIterator<Item = T>>(
        x_len: usize,
        y_len: usize,
        z_len: usize,
        iter: I,
    ) -> Option<Self> {
        let v: Vec<T> = iter.into_iter().collect();
        if v.len() != x_len * y_len * z_len {
            return None;
        } else {
            return Some(Self {
                inner: v,
                size: Size3D {
                    x: x_len,
                    y: y_len,
                    z: z_len,
                },
            });
        }
    }

    pub fn size(&self) -> Size3D {
        return self.size;
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> &T {
        return &self.inner[self.xyz_to_ind(x, y, z)];
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, value: T) {
        let i = self.xyz_to_ind(x, y, z);
        self.inner[i] = value;
    }

    fn xyz_to_ind(&self, x: usize, y: usize, z: usize) -> usize {
        return x + (y * self.size.x) + (z * self.size.y * self.size.x);
    }

    // todo: test
    /// Returns iterator over selected Y column in grid
    fn iter_col(&self, column: usize) -> StepBy<Iter<'_, T>> {
        return self.inner[column..].iter().step_by(self.size.x);
    }

    // todo: test
    /// Returns iterator over selected X row in grid
    fn iter_row(&self, row: usize) -> Iter<'_, T> {
        let start = (row * self.size.x);
        return self.inner[start..(start + self.size.x)].iter();
    }

    pub fn count<P>(&self, mut predicate: P) -> usize
    where
        P: FnMut(&T) -> bool,
    {
        let mut c = 0;
        for elem in &self.inner {
            if predicate(&elem) {
                c += 1;
            }
        }
        return c;
    }
}

impl<T> Index<&Vector3Int> for Grid3D<T> {
    type Output = T;

    fn index(&self, index: &Vector3Int) -> &Self::Output {
        return &self.inner[self.xyz_to_ind(index.x as usize, index.y as usize, index.z as usize)];
    }
}

impl<T> IndexMut<&Vector3Int> for Grid3D<T> {
    fn index_mut(&mut self, index: &Vector3Int) -> &mut Self::Output {
        let i = self.xyz_to_ind(index.x as usize, index.y as usize, index.z as usize);
        return &mut self.inner[i];
    }
}

#[cfg(test)]
mod tests {
    use crate::v3i::Vector3Int;

    use super::Grid3D;

    const TEST_DATA_5X7X3: [[[usize; 5]; 7]; 3] = [
        [
            [1, 2, 3, 4, 5],
            [6, 7, 8, 9, 10],
            [11, 12, 13, 14, 15],
            [16, 17, 18, 19, 20],
            [21, 22, 23, 24, 25],
            [26, 27, 28, 29, 30],
            [31, 32, 33, 34, 35],
        ],
        [
            [36, 37, 38, 39, 40],
            [41, 42, 43, 44, 45],
            [46, 47, 48, 49, 50],
            [51, 52, 53, 54, 55],
            [56, 57, 58, 59, 60],
            [61, 62, 63, 64, 65],
            [66, 67, 68, 69, 70],
        ],
        [
            [71, 72, 73, 74, 75],
            [76, 77, 78, 79, 80],
            [81, 82, 83, 84, 85],
            [86, 87, 88, 89, 90],
            [91, 92, 93, 94, 95],
            [96, 97, 98, 99, 100],
            [101, 102, 103, 104, 105],
        ],
    ];

    const TEST_DATA_3X3X3: [[[usize; 3]; 3]; 3] = [
        [[0, 1, 2], [3, 4, 5], [6, 7, 8]],
        [[9, 10, 11], [12, 13, 14], [15, 16, 17]],
        [[18, 19, 20], [21, 22, 23], [24, 25, 26]],
    ];

    #[test]
    fn test_get_simple() {
        // Test simple 3x3x3 grid
        let mut grid: Grid3D<usize> = Grid3D::new(3, 3, 3);
        grid.inner = (0..(3 * 3 * 3)).collect();

        for x in 0..3 {
            for y in 0..3 {
                for z in 0..3 {
                    assert_eq!(TEST_DATA_3X3X3[z][y][x], *grid.get(x, y, z));
                }
            }
        }
    }

    #[test]
    fn test_get_asymmetrical() {
        let mut grid: Grid3D<usize> = Grid3D::new(5, 7, 3);
        grid.inner = (1..=(5 * 7 * 3)).collect();

        for x in 0..5 {
            for y in 0..7 {
                for z in 0..3 {
                    assert_eq!(TEST_DATA_5X7X3[z][y][x], *grid.get(x, y, z));
                }
            }
        }
    }

    #[test]
    fn test_v3i_index_asymmetrical() {
        let mut grid: Grid3D<usize> = Grid3D::new(5, 7, 3);
        grid.inner = (1..=(5 * 7 * 3)).collect();

        let mut ind = Vector3Int::ZERO;
        for x in 0..5 {
            ind.x = x as isize;
            for y in 0..7 {
                ind.y = y as isize;
                for z in 0..3 {
                    ind.z = z as isize;
                    assert_eq!(TEST_DATA_5X7X3[z][y][x], grid[&ind]);
                }
            }
        }
    }
}
