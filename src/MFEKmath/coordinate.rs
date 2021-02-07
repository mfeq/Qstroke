use std::cmp::PartialEq;
use std::ops::*;

pub trait Coordinate : Sized+Copy+Add<Self, Output=Self>+Mul<Self, Output=Self>+Sub<Self, Output=Self>+PartialEq {
    fn magnitude(self) -> f64;
    fn distance(self, v1: Self) -> f64;
    fn lerp(self, v1: Self, t: f64) -> Self;
}

pub trait Coordinate2D: Coordinate {
    fn x(self) -> f64;
    fn y(self) -> f64;
    fn normalize(self) -> Self;
}

impl Coordinate for f64 {
    fn magnitude(self) -> Self {
        return f64::abs(self);
    }

    fn distance(self, v1: Self) -> Self {
        return f64::abs(self - v1);
    }

    fn lerp(self, v1: Self, t: f64) -> Self {
        return (1. - t) * self + t * v1;

    }
}