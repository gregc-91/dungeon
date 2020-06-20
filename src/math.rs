
use std::ops::{Add, Sub, Mul, Div};

pub type Vec2i = Vec2<i32>;
pub type _Vec2f = Vec2<f32>;

#[derive(Debug, Clone, Copy)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T
}

impl<T: Add<Output = T>> Add for Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2::<T> { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl<T: Sub<Output = T>> Sub for Vec2<T> {
    type Output = Vec2<T>;

    fn sub(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2::<T> { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl<T: Mul<Output = T>> Mul for Vec2<T> {
    type Output = Vec2<T>;

    fn mul(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2::<T> { x: self.x * rhs.x, y: self.y * rhs.y }
    }
}

impl<T: Div<Output = T>> Div for Vec2<T> {
    type Output = Vec2<T>;

    fn div(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2::<T> { x: self.x / rhs.x, y: self.y / rhs.y }
    }
}
