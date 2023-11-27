use rand::Rng;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Vec2 {
        Vec2::from_one(0.0)
    }

    pub fn from_one(v: f64) -> Vec2 {
        Vec2 { x: v, y: v }
    }

    pub fn magnitude(&self) -> f64 {
        self.norm().sqrt()
    }
    pub fn norm(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    /// Create a vector in the same direction as this vector, but with magnitude 1.
    pub fn normalize(&self) -> Vec2 {
        let inv_mag = self.magnitude().recip();
        Vec2 {
            x: self.x * inv_mag,
            y: self.y * inv_mag,
        }
    }

    pub fn dot(&self, other: &Vec2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn near_zero(&self) -> bool {
        const SMALL: f64 = 0.00000008;
        self.x.abs() < SMALL && self.y.abs() < SMALL
    }

    pub fn random(min: f64, max: f64) -> Vec2 {
        let mut rng = rand::thread_rng();
        Vec2 {
            x: rng.gen_range(min..max),
            y: rng.gen_range(min..max),
        }
    }

    pub fn random_in_unit_sphere() -> Vec2 {
        loop {
            let p = Vec2::random(-1.0, 1.0);
            if p.norm() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn random_unit_vector() -> Vec2 {
        Vec2::random_in_unit_sphere().normalize()
    }

    pub fn random_in_hemisphere(normal: &Vec2) -> Vec2 {
        let p = Vec2::random_in_unit_sphere();
        if p.dot(normal) > 0.0 {
            p
        } else {
            -p
        }
    }

    pub fn random_in_unit_disk() -> Vec2 {
        let mut rng = rand::thread_rng();
        loop {
            let p = Vec2 {
                x: rng.gen_range(-1.0..1.0),
                y: rng.gen_range(-1.0..1.0),
            };
            if p.norm() < 1.0 {
                return p;
            }
        }
    }
}

impl Add for Vec2 {
    type Output = Vec2;
    #[inline(always)]
    fn add(self, other: Vec2) -> Self::Output {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    #[inline(always)]
    fn sub(self, other: Vec2) -> Self::Output {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul for Vec2 {
    type Output = Vec2;
    #[inline(always)]
    fn mul(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;
    #[inline(always)]
    fn mul(self, other: f64) -> Vec2 {
        Vec2 {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Div<f64> for Vec2 {
    type Output = Vec2;
    #[inline(always)]
    fn div(self, other: f64) -> Vec2 {
        self * (1.0 / other)
    }
}

impl Mul<Vec2> for f64 {
    type Output = Vec2;
    #[inline(always)]
    fn mul(self, other: Vec2) -> Vec2 {
        other * self
    }
}

impl Neg for Vec2 {
    type Output = Vec2;
    #[inline(always)]
    fn neg(self) -> Vec2 {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[test]
fn test_can_add_vectors() {
    let v1 = Vec2 { x: 1.0, y: 2.0 };
    let v2 = Vec2 { x: 11.0, y: 12.0 };

    assert_eq!(Vec2 { x: 12.0, y: 14.0 }, v1 + v2);
    assert_eq!(Vec2 { x: -1.0, y: -2.0 }, -v1);
}

#[test]
fn test_can_negate_vectors() {
    let v1 = Vec2 { x: 1.0, y: 2.0 };
    assert_eq!(Vec2 { x: -1.0, y: -2.0 }, -v1);
}
