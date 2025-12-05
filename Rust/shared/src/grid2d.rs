#![allow(unused)]

use std::{
    fmt::{Debug, Display},
    iter::StepBy,
    ops::{Index, IndexMut},
    slice::Iter,
    vec::Drain,
};

use crate::{v2i::Vector2Int, v2u::Vector2Usize};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Size2D {
    pub x: usize,
    pub y: usize,
}

impl Size2D {
    pub fn to_vector2(&self) -> Vector2Int {
        return Vector2Int {
            x: self.x as isize,
            y: self.y as isize,
        };
    }
}

#[derive(Clone)]
pub struct Grid2D<T> {
    inner: Vec<T>,
    size: Size2D,
}

impl Grid2D<char> {
    /*
    Creates Grid2D from chars in input string
    Rows must be separated by newline chars, which will not be included in the grid
     */
    pub fn from_string(input: &String) -> Self {
        let lines: Vec<&str> = input.lines().collect();

        let mut g: Grid2D<char> = Grid2D {
            inner: Vec::<char>::with_capacity(lines[0].len() * lines.len()),
            size: Size2D {
                x: lines[0].len(),
                y: lines.len(),
            },
        };

        for line in lines {
            g.inner.extend(line.chars());
        }

        return g;
    }
}

impl Grid2D<u8> {
    /*
    Creates Grid2D from bytes in input string
    Rows must be separated by newline chars, which will not be included in the grid
     */
    pub fn from_string(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();

        let mut g: Grid2D<u8> = Grid2D {
            inner: Vec::<u8>::with_capacity(input.len()),
            size: Size2D {
                x: lines[0].len(),
                y: lines.len(),
            },
        };

        for line in lines {
            g.inner.extend(line.bytes());
        }

        return g;
    }

    pub fn print(&self) {
        for y in 0..self.size.y {
            println!(
                "{}",
                self.iter_row(y).map(|x| *x as char).collect::<String>()
            );
        }
    }
}

impl<T> Grid2D<T> {
    pub fn new(x_len: usize, y_len: usize) -> Self
    where
        T: Default,
    {
        let mut v = Vec::new();
        v.resize_with(x_len * y_len, T::default);
        return Grid2D {
            inner: v,
            size: Size2D { x: x_len, y: y_len },
        };
    }

    pub fn from_iter<I: IntoIterator<Item = T>>(
        x_len: usize,
        y_len: usize,
        iter: I,
    ) -> Option<Self> {
        let v: Vec<T> = iter.into_iter().collect();
        if v.len() != x_len * y_len {
            return None;
        } else {
            return Some(Self {
                inner: v,
                size: Size2D { x: x_len, y: y_len },
            });
        }
    }

    pub fn size(&self) -> Size2D {
        return self.size;
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        return &self.inner[self.xy_to_ind(x, y)];
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        let ind = self.xy_to_ind(x, y);
        self.inner[ind] = value;
    }

    /// Converts x,y coords to linear coords
    fn xy_to_ind(&self, x: usize, y: usize) -> usize {
        return x + (y * self.size.x);
    }

    fn ind_to_xy(&self, ind: usize) -> (usize, usize) {
        return (ind % self.size.y, ind / self.size.x);
    }

    /// Returns iterator over selected Y column in grid
    pub fn iter_col(&self, column: usize) -> StepBy<Iter<'_, T>> {
        return self.inner[column..].iter().step_by(self.size.x);
    }

    /// Returns iterator over selected X row in grid
    pub fn iter_row(&self, row: usize) -> Iter<'_, T> {
        let start = (row * self.size.x);
        return self.inner[start..(start + self.size.x)].iter();
    }

    pub fn find_all<'a>(&'a self, item: &'a T) -> impl Iterator<Item = (usize, usize)> + use<'a, T>
    where
        T: PartialEq,
    {
        return self
            .inner
            .iter()
            .enumerate()
            .filter(move |(_, t)| *t == item)
            .map(|(i, _)| self.ind_to_xy(i));
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

    pub fn remove_row(&mut self, row: usize) {
        let start = row * self.size.x;
        let end = start + self.size.x - 1;
        self.size.y -= 1;
        self.inner.drain(start..=end);
    }

    pub fn remove_col(&mut self, col: usize) {
        let mut n = col;
        let step = self.size.x;
        self.size.x -= 1;

        while n < self.inner.len() {
            self.inner.remove(n);
            n += self.size.x;
        }
    }
}

impl<T> Grid2D<T>
where
    T: ToString,
{
    pub fn to_string(&self) -> String {
        let mut s = String::new();
        for y in 0..self.size.y {
            s.push_str(&self.iter_row(y).map(|x| x.to_string()).collect::<String>());
            s.push('\n');
        }
        s.pop();
        return s;
    }
}

impl<T> Grid2D<T>
where
    T: Copy,
{
    /// Mutates inner array to flip grid left/right
    pub fn flip_horizontal(&mut self) {
        let mut row_start = 0;
        while row_start < self.inner.len() {
            for i in 0..(self.size.x / 2) {
                let tmp = self.inner[row_start + i];
                self.inner[row_start + i] = self.inner[row_start + self.size.x - 1 - i];
                self.inner[row_start + self.size.x - 1 - i] = tmp;
            }
            row_start += self.size.x;
        }
    }

    /// Mutates inner array to flip grid top/bottom
    /*
        0  1  2  3  4  5  6  7  8  9     90 91 92 93 94 95 96 97 98 99
       10 11 12 13 14 15 16 17 18 19     80 81 82 83 84 85 86 87 88 89
       20 21 22 23 24 25 26 27 28 29     70 71 72 73 74 75 76 77 78 79
       30 31 32 33 34 35 36 37 38 39     60 61 62 63 64 65 66 67 68 69
       40 41 42 43 44 45 46 47 48 49  -> 50 51 52 53 54 55 56 57 58 59
       50 51 52 53 54 55 56 57 58 59     40 41 42 43 44 45 46 47 48 49
       60 61 62 63 64 65 66 67 68 69     30 31 32 33 34 35 36 37 38 39
       70 71 72 73 74 75 76 77 78 79     20 21 22 23 24 25 26 27 28 29
       80 81 82 83 84 85 86 87 88 89     10 11 12 13 14 15 16 17 18 19
       90 91 92 93 94 95 96 97 98 99      0  1  2  3  4  5  6  7  8  9

    */
    pub fn flip_vertical(&mut self)
    where
        T: Display,
        T: Debug,
    {
        for column in 0..self.size.x {
            for row in 0..(self.size.y / 2) {
                let a = column + (row * self.size.x);
                let b = column + ((self.size.y - row - 1) * self.size.x);
                let tmp = self.inner[a];
                self.inner[a] = self.inner[b];
                self.inner[b] = tmp;
            }
        }
    }

    /// Rotates inner array 90 degrees counter-clockwise (left), changing the size of the
    /// grid if required
    pub fn rotate_ccw(&mut self) -> bool {
        if self.size.x != self.size.y {
            return false;
        }

        let n = self.size.x;
        for y in 0..(n / 2) {
            for x in y..(n - y - 1) {
                let i_a = self.xy_to_ind(x, y);
                let i_b = self.xy_to_ind(n - y - 1, x);
                let i_c = self.xy_to_ind(n - x - 1, n - y - 1);
                let i_d = self.xy_to_ind(y, n - x - 1);
                let tmp = self.inner[i_a];

                self.inner[i_a] = self.inner[i_b];
                self.inner[i_b] = self.inner[i_c];
                self.inner[i_c] = self.inner[i_d];
                self.inner[i_d] = tmp;
            }
        }
        return true;
    }

    /// Rotates inner array 90 degrees counter-clockwise (left), changing the size of the
    /// grid if required
    pub fn rotate_cw(&mut self) -> bool {
        if self.size.x != self.size.y {
            return false;
        }

        let n = self.size.x;
        for y in 0..(n / 2) {
            for x in y..(n - y - 1) {
                let i_d = self.xy_to_ind(x, y);
                let i_c = self.xy_to_ind(n - y - 1, x);
                let i_b = self.xy_to_ind(n - x - 1, n - y - 1);
                let i_a = self.xy_to_ind(y, n - x - 1);

                let tmp = self.inner[i_a];

                self.inner[i_a] = self.inner[i_b];
                self.inner[i_b] = self.inner[i_c];
                self.inner[i_c] = self.inner[i_d];
                self.inner[i_d] = tmp;
            }
        }
        return true;
    }
}

impl<T> Index<&Vector2Int> for Grid2D<T> {
    type Output = T;

    fn index(&self, index: &Vector2Int) -> &Self::Output {
        return &self.inner[self.xy_to_ind(index.x as usize, index.y as usize)];
    }
}

impl<T> Index<&Vector2Usize> for Grid2D<T> {
    type Output = T;

    fn index(&self, index: &Vector2Usize) -> &Self::Output {
        return &self.inner[self.xy_to_ind(index.x, index.y)];
    }
}

impl<T> Index<(usize, usize)> for Grid2D<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        return &self.inner[self.xy_to_ind(index.0 as usize, index.1 as usize)];
    }
}

impl<T> IndexMut<&Vector2Int> for Grid2D<T> {
    fn index_mut(&mut self, index: &Vector2Int) -> &mut Self::Output {
        let i = self.xy_to_ind(index.x as usize, index.y as usize);
        return &mut self.inner[i];
    }
}

#[cfg(test)]
mod tests {
    use crate::{grid2d::Size2D, v2i::Vector2Int};

    use super::Grid2D;

    const TEST_DATA_5X7: [[usize; 5]; 7] = [
        [1, 2, 3, 4, 5],
        [6, 7, 8, 9, 10],
        [11, 12, 13, 14, 15],
        [16, 17, 18, 19, 20],
        [21, 22, 23, 24, 25],
        [26, 27, 28, 29, 30],
        [31, 32, 33, 34, 35],
    ];

    const TEST_DATA_3X3: [[usize; 3]; 3] = [[0, 1, 2], [3, 4, 5], [6, 7, 8]];

    #[test]
    fn test_get_simple() {
        // Test simple 3x3x3 grid
        let mut grid: Grid2D<usize> = Grid2D::new(3, 3);
        grid.inner = (0..(3 * 3)).collect();

        for x in 0..3 {
            for y in 0..3 {
                assert_eq!(TEST_DATA_3X3[y][x], *grid.get(x, y));
            }
        }
    }

    #[test]
    fn test_get_asymmetrical() {
        let mut grid: Grid2D<usize> = Grid2D::new(5, 7);
        grid.inner = (1..=(5 * 7)).collect();

        for x in 0..5 {
            for y in 0..7 {
                assert_eq!(TEST_DATA_5X7[y][x], *grid.get(x, y));
            }
        }
    }

    #[test]
    fn test_v2i_index_asymmetrical() {
        let mut grid: Grid2D<usize> = Grid2D::new(5, 7);
        grid.inner = (1..=(5 * 7)).collect();

        let mut ind = Vector2Int::ZERO;
        for x in 0..5 {
            ind.x = x as isize;
            for y in 0..7 {
                ind.y = y as isize;

                assert_eq!(TEST_DATA_5X7[y][x], grid[&ind]);
            }
        }
    }

    #[test]
    fn test_iter_col() {
        let mut grid: Grid2D<usize> = Grid2D::new(5, 7);
        grid.inner = (1..=(5 * 7)).collect();

        for col in 0..TEST_DATA_5X7[0].len() {
            let grid_col_vec: Vec<&usize> = grid.iter_col(col).collect();
            for row in 0..grid_col_vec.len() {
                assert_eq!(TEST_DATA_5X7[row][col], *grid_col_vec[row]);
            }
        }
    }

    #[test]
    fn test_iter_row() {
        let mut grid: Grid2D<usize> = Grid2D::new(5, 7);
        grid.inner = (1..=(5 * 7)).collect();
        for row in 0..TEST_DATA_5X7.len() {
            let grid_row_vec: Vec<&usize> = grid.iter_row(row).collect();
            for col in 0..grid_row_vec.len() {
                assert_eq!(TEST_DATA_5X7[row][col], *grid_row_vec[col]);
            }
        }
    }

    #[test]
    fn test_rotate_ccw() {
        /*
        012    258
        345 -> 147
        678    036
         */
        let mut grid: Grid2D<usize> = Grid2D::new(3, 3);
        grid.inner = (0..(3 * 3)).collect();

        grid.rotate_ccw();
        for i in 0..grid.inner.len() {
            assert_eq!(grid.inner[i], [2, 5, 8, 1, 4, 7, 0, 3, 6][i]);
        }

        for (i, v) in grid.iter_row(1).enumerate() {
            assert_eq!(*v, [1, 4, 7][i]);
        }
    }

    #[test]
    fn test_rotate_cw() {
        /*
        012    630
        345 -> 741
        678    852
         */
        let mut grid: Grid2D<usize> = Grid2D::new(3, 3);
        grid.inner = (0..(3 * 3)).collect();

        grid.rotate_cw();

        for i in 0..grid.inner.len() {
            assert_eq!(grid.inner[i], [6, 3, 0, 7, 4, 1, 8, 5, 2][i]);
        }

        for (i, v) in grid.iter_row(1).enumerate() {
            assert_eq!(*v, [7, 4, 1][i]);
        }
    }

    #[test]
    fn test_flip_vertical() {
        /*
        Flip ODD grid

         1,  2,  3,  4,  5      31, 32, 33, 34, 35
         6,  7,  8,  9, 10      26, 27, 28, 29, 30
        11, 12, 13, 14, 15      21, 22, 23, 24, 25
        16, 17, 18, 19, 20  ->  16, 17, 18, 19, 20
        21, 22, 23, 24, 25      11, 12, 13, 14, 15
        26, 27, 28, 29, 30       6,  7,  8,  9, 10
        31, 32, 33, 34, 35       1,  2,  3,  4,  5
         */
        // let mut grid: Grid2D<u8> = Grid2D::new(5, 7);
        // grid.inner = (1..=(5 * 7)).collect();
        // grid.flip_vertical();

        // let flipped_inner = [
        //     31, 32, 33, 34, 35, 26, 27, 28, 29, 30, 21, 22, 23, 24, 25, 16, 17, 18, 19, 20, 11, 12, 13, 14, 15, 6,
        //     7, 8, 9, 10, 1, 2, 3, 4, 5,
        // ];

        // for (i, v) in grid.inner.iter().enumerate() {
        //     assert_eq!(grid.inner[i], flipped_inner[i]);
        // }

        /*
        Flip EVEN grid

         0  1  2  3  4  5  6  7  8  9     90 91 92 93 94 95 96 97 98 99
        10 11 12 13 14 15 16 17 18 19     80 81 82 83 84 85 86 87 88 89
        20 21 22 23 24 25 26 27 28 29     70 71 72 73 74 75 76 77 78 79
        30 31 32 33 34 35 36 37 38 39     60 61 62 63 64 65 66 67 68 69
        40 41 42 43 44 45 46 47 48 49  -> 50 51 52 53 54 55 56 57 58 59
        50 51 52 53 54 55 56 57 58 59     40 41 42 43 44 45 46 47 48 49
        60 61 62 63 64 65 66 67 68 69     30 31 32 33 34 35 36 37 38 39
        70 71 72 73 74 75 76 77 78 79     20 21 22 23 24 25 26 27 28 29
        80 81 82 83 84 85 86 87 88 89     10 11 12 13 14 15 16 17 18 19
        90 91 92 93 94 95 96 97 98 99      0  1  2  3  4  5  6  7  8  9

         */
        let mut grid: Grid2D<u8> = Grid2D::new(10, 10);
        grid.inner = (0..(10 * 10)).collect();
        grid.flip_vertical();

        let flipped_inner = [
            90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 70, 71,
            72, 73, 74, 75, 76, 77, 78, 79, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 50, 51, 52, 53,
            54, 55, 56, 57, 58, 59, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 30, 31, 32, 33, 34, 35,
            36, 37, 38, 39, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 10, 11, 12, 13, 14, 15, 16, 17,
            18, 19, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
        ];

        for (i, v) in grid.inner.iter().enumerate() {
            assert_eq!(grid.inner[i], flipped_inner[i]);
        }
    }

    #[test]
    fn test_remove_row() {
        /*
         0  1  2  3  4       0  1  2  3  4
         5  6  7  8  9       5  6  7  8  9
        10 11 12 13 14  ->  10 11 12 13 14
        15 16 17 18 19      20 21 22 23 24
        20 21 22 23 24



        */
        let mut grid: Grid2D<usize> = Grid2D::new(5, 5);
        grid.inner = (0..(5 * 5)).collect();

        grid.remove_row(3);

        assert_eq!(grid.inner.len(), 20);
        assert_eq!(grid.size, Size2D { x: 5, y: 4 });

        assert_eq!(
            grid.iter_row(3).cloned().collect::<Vec<usize>>(),
            vec![20, 21, 22, 23, 24]
        );
    }

    #[test]
    fn test_remove_col() {
        /*
         0  1  2  3  4      0  2  3  4
         5  6  7  8  9      5  7  8  9
        10 11 12 13 14  -> 10 12 13 14
        15 16 17 18 19     15 17 18 19
        20 21 22 23 24     20 22 23 24



        */
        let mut grid: Grid2D<usize> = Grid2D::new(5, 5);
        grid.inner = (0..(5 * 5)).collect();

        grid.remove_col(1);

        assert_eq!(grid.inner.len(), 20);
        assert_eq!(grid.size, Size2D { x: 4, y: 5 });

        assert_eq!(
            grid.iter_col(1).cloned().collect::<Vec<usize>>(),
            vec![2, 7, 12, 17, 22]
        );
    }
}
