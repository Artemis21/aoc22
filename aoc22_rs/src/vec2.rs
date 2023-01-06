use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Vec2<T = ()>(pub isize, pub isize, pub PhantomData<T>);

pub const CARDINALS: [Vec2; 4] = [
    Vec2::new(0, -1),
    Vec2::new(1, 0),
    Vec2::new(0, 1),
    Vec2::new(-1, 0),
];

impl<T> Vec2<T> {
    pub const fn new(x: isize, y: isize) -> Self {
        Self(x, y, PhantomData)
    }

    pub fn map_coords<U>(map: &[Vec<U>]) -> impl Iterator<Item = Self> {
        let width = map[0].len() as isize;
        (0..map.len() as isize).flat_map(move |y| (0..width).map(move |x| Self::new(x, y)))
    }

    pub fn index<'a, U>(&self, map: &'a [Vec<U>]) -> &'a U {
        &map[self.1 as usize][self.0 as usize]
    }

    pub fn index_mut<'a, U>(&self, map: &'a mut [Vec<U>]) -> &'a mut U {
        &mut map[self.1 as usize][self.0 as usize]
    }

    pub const fn as_base_n(&self, base: isize) -> isize {
        self.1 * base + self.0
    }

    pub const fn signum(&self) -> Self {
        Self::new(self.0.signum(), self.1.signum())
    }

    pub const fn manhattan(&self) -> usize {
        (self.0.abs() + self.1.abs()) as usize
    }
}

impl<T: Eq + Copy> Vec2<T> {
    pub fn range_inclusive(&self, other: Self) -> impl Iterator<Item = Self> {
        let delta = (other - *self).signum();
        let mut point = *self;
        let mut done = false;
        std::iter::from_fn(move || {
            if done {
                return None;
            }
            if point == other {
                done = true;
                Some(point)
            } else {
                let old_point = point;
                point += delta;
                Some(old_point)
            }
        })
    }
}

impl<T> std::fmt::Debug for Vec2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec2({}, {})", self.0, self.1)
    }
}

impl<T> Add for Vec2<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.0 + other.0, self.1 + other.1)
    }
}

impl<T> AddAssign for Vec2<T> {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
    }
}

impl<T> Sub for Vec2<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.0 - other.0, self.1 - other.1)
    }
}

impl<T> SubAssign for Vec2<T> {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
        self.1 -= other.1;
    }
}

impl<T> Mul<usize> for Vec2<T> {
    type Output = Self;

    fn mul(self, other: usize) -> Self {
        Self::new(self.0 * other as isize, self.1 * other as isize)
    }
}

impl<T> MulAssign<usize> for Vec2<T> {
    fn mul_assign(&mut self, other: usize) {
        self.0 *= other as isize;
        self.1 *= other as isize;
    }
}

impl<T> Mul<isize> for Vec2<T> {
    type Output = Self;

    fn mul(self, other: isize) -> Self {
        Self::new(self.0 * other, self.1 * other)
    }
}

impl<T> MulAssign<isize> for Vec2<T> {
    fn mul_assign(&mut self, other: isize) {
        self.0 *= other;
        self.1 *= other;
    }
}

impl<T> Div<isize> for Vec2<T> {
    type Output = Self;

    fn div(self, other: isize) -> Self {
        Self::new(self.0 / other, self.1 / other)
    }
}

impl<T> DivAssign<isize> for Vec2<T> {
    fn div_assign(&mut self, other: isize) {
        self.0 /= other;
        self.1 /= other;
    }
}

impl<T> Div<usize> for Vec2<T> {
    type Output = Self;

    fn div(self, other: usize) -> Self {
        Self::new(self.0 / other as isize, self.1 / other as isize)
    }
}

impl<T> DivAssign<usize> for Vec2<T> {
    fn div_assign(&mut self, other: usize) {
        self.0 /= other as isize;
        self.1 /= other as isize;
    }
}

impl<T> Rem<isize> for Vec2<T> {
    type Output = Self;

    fn rem(self, other: isize) -> Self {
        Self::new(self.0 % other, self.1 % other)
    }
}

impl<T> RemAssign<isize> for Vec2<T> {
    fn rem_assign(&mut self, other: isize) {
        self.0 %= other;
        self.1 %= other;
    }
}

impl<T> Rem<usize> for Vec2<T> {
    type Output = Self;

    fn rem(self, other: usize) -> Self {
        Self::new(self.0 % other as isize, self.1 % other as isize)
    }
}

impl<T> RemAssign<usize> for Vec2<T> {
    fn rem_assign(&mut self, other: usize) {
        self.0 %= other as isize;
        self.1 %= other as isize;
    }
}
