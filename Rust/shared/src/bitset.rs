#![allow(unused)]

const UWIDTH: usize = std::mem::size_of::<usize>() * 8;

#[derive(Clone)]
pub struct Bitset {
    len: usize,
    bits: Vec<usize>,
}

impl Bitset {
    pub fn new(len: usize) -> Self {
        return Self {
            len: len,
            bits: vec![0; len.div_ceil(UWIDTH)],
        };
    }

    pub fn clear(&mut self) {
        self.bits.fill(0);
    }

    pub fn len(&self) -> usize {
        return self.len;
    }

    pub fn set(&mut self, index: usize) {
        let vi = index / UWIDTH;
        let i = index % UWIDTH;
        self.bits[vi] |= 1 << i;
    }

    pub fn unset(&mut self, index: usize) {
        let vi = index / UWIDTH;
        let i = index % UWIDTH;
        self.bits[vi] &= !(1 << i);
    }

    pub fn is_set(&self, index: usize) -> bool {
        let vi = index / UWIDTH;
        let i = index % UWIDTH;
        return self.bits.get(vi).is_some_and(|v| ((v >> i) & 1) == 1);
    }

    pub fn count_set(&self) -> usize {
        let mut count = 0;
        for num in &self.bits {
            let mut n = *num;
            while n > 0 {
                count += n & 1;
                n >>= 1;
            }
        }
        return count;
    }
}

#[derive(Clone)]
pub struct Bitset2D {
    cols: usize,
    rows: usize,
    bits: Vec<Bitset>,
}

impl Bitset2D {
    pub fn new(cols: usize, rows: usize) -> Self {
        return Self {
            cols,
            rows,
            bits: vec![Bitset::new(cols); rows],
        };
    }

    pub fn print(&self) {
        for row in 0..self.rows {
            let mut r = Vec::with_capacity(self.cols);
            for col in 0..self.cols {
                if self.is_set(col, row) {
                    r.push('#');
                } else {
                    r.push('.');
                }
            }
            println!("{}", r.iter().collect::<String>());
        }
    }

    pub fn clear(&mut self) {
        self.bits.iter_mut().for_each(|b| b.clear());
    }

    pub fn rows(&self) -> usize {
        return self.rows;
    }

    pub fn cols(&self) -> usize {
        return self.cols;
    }

    pub fn set(&mut self, col: usize, row: usize) {
        let vi = col / UWIDTH;
        let i = col % UWIDTH;
        self.bits[row].bits[vi] |= 1 << i;
    }

    pub fn unset(&mut self, col: usize, row: usize) {
        let vi = col / UWIDTH;
        let i = col % UWIDTH;
        self.bits[row].bits[vi] &= !(1 << i);
    }

    pub fn is_set(&self, col: usize, row: usize) -> bool {
        let vi = col / UWIDTH;
        let i = col % UWIDTH;
        return self
            .bits
            .get(row)
            .is_some_and(|r| r.bits.get(vi).is_some_and(|v| ((v >> i) & 1) == 1));
    }

    pub fn is_set_i(&self, col: isize, row: isize) -> bool {
        if col < 0 || row < 0 || col as usize > self.cols || row as usize > self.rows {
            return false;
        }
        let col = col as usize;
        let row = row as usize;
        let vi = col / UWIDTH;
        let i = col % UWIDTH;
        return self
            .bits
            .get(row)
            .is_some_and(|r| r.bits.get(vi).is_some_and(|v| ((v >> i) & 1) == 1));
    }

    /// Returns an iterator of (x,y) coords of set bits
    pub fn iter_set(&self) -> impl Iterator<Item = (usize, usize)> + use<'_> {
        let mut x = 0;
        let mut y = 0;

        std::iter::from_fn(move || loop {
            let (xx, yy) = (x, y);

            x += 1;
            if x == self.cols {
                x = 0;
                y += 1;
                if y == self.rows {
                    return None;
                }
            }

            if self.is_set(xx, yy) {
                return Some((xx, yy));
            }
        })
    }

    pub fn count_set(&self) -> usize {
        let mut count = 0;
        for r in &self.bits {
            count += r.count_set();
        }
        return count;
    }
}

pub struct Bitset2DIterator<'a> {
    bs: &'a Bitset2D,
    cx: usize,
    cy: usize,
}

impl<'a> Iterator for Bitset2DIterator<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let mut ret = None;
        while ret == None {
            if self.bs.is_set(self.cx, self.cy) {
                ret = Some((self.cx, self.cy));
            }
            self.cx += 1;
            if self.cx >= self.bs.cols {
                self.cx = 0;
                self.cy += 1;
                if self.cy >= self.bs.rows {
                    break;
                }
            }
        }
        return ret;
    }
}

impl<'a> IntoIterator for &'a Bitset2D {
    type Item = (usize, usize);
    type IntoIter = Bitset2DIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Bitset2DIterator {
            bs: self,
            cx: 0,
            cy: 0,
        }
    }
}

mod tests {
    use std::collections::HashSet;

    use super::Bitset2D;

    #[test]
    fn test_iterator_2d() {
        let mut bs = Bitset2D::new(113, 131);
        let set = [
            (2, 3),
            (5, 7),
            (11, 13),
            (17, 19),
            (23, 29),
            (31, 37),
            (41, 43),
            (47, 53),
            (59, 61),
            (67, 71),
            (73, 79),
            (83, 89),
            (97, 101),
            (103, 107),
            (109, 113),
        ];

        for (x, y) in set {
            bs.set(x, y);
        }

        for (i, xy) in bs.into_iter().enumerate() {
            assert_eq!(set[i], xy);
        }
    }
    #[test]
    fn test_clone() {
        let mut bs = Bitset2D::new(113, 131);
        let mut bs2 = Bitset2D::new(bs.cols(), bs.rows());
        let set = [
            (2, 3),
            (5, 7),
            (11, 13),
            (17, 19),
            (23, 29),
            (31, 37),
            (41, 43),
            (47, 53),
            (59, 61),
            (67, 71),
            (73, 79),
            (83, 89),
            (97, 101),
            (103, 107),
            (109, 113),
        ];

        for (x, y) in set {
            bs.set(x, y);
        }

        let bs2 = bs.clone();

        for (x, y) in set {
            assert!(bs2.is_set(x, y));
        }
    }
}
