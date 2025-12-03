use std::ops::*;

pub trait BitMask<T>
where
    T: Sized
        + Copy
        + PartialOrd
        + Ord
        + Eq
        + Not<Output = Self>
        + BitAnd<Output = Self>
        + BitOr<Output = Self>
        + BitXor<Output = Self>
        + Shl<usize, Output = Self>
        + Shr<usize, Output = Self>,
{
    fn set_bit(&mut self, index: T);
    fn is_set(&self, index: T) -> bool;
}

impl BitMask<u8> for u8 {
    fn set_bit(&mut self, index: u8) {
        *self |= 1 << index;
    }

    fn is_set(&self, index: u8) -> bool {
        self & (1 << index) == 1
    }
}

impl BitMask<u16> for u16 {
    fn set_bit(&mut self, index: u16) {
        *self |= 1 << index;
    }

    fn is_set(&self, index: u16) -> bool {
        self & (1 << index) == 1
    }
}
