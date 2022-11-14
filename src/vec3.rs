use core::fmt;
use std::ops;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub e0: f64,
    pub e1: f64,
    pub e2: f64,
}

impl Vec3 {
    pub fn zero() -> Vec3 {
        Vec3 { e0: 0.0, e1: 0.0, e2: 0.0 }
    }

    pub fn mul_scalar(self, rhs: f64) -> Vec3 {
        Vec3 { e0: self.e0 * rhs, e1: self.e1 * rhs, e2: self.e2 * rhs }
    }

    pub fn div_scalar(self, rhs: f64) -> Vec3 {
        let numer = 1.0/rhs;
        self.mul_scalar(numer)
    }

    pub fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            e0: self.e0 * rhs,
            e1: self.e1 * rhs,
            e2: self.e2 * rhs
        }
    }

    pub fn div_assign(&mut self, rhs: f64) {
        let numer = 1.0/rhs;
        *self = Self {
            e0: self.e0 * numer,
            e1: self.e1 * numer,
            e2: self.e2 * numer
        }
    }

    pub fn dot(self, rhs: Vec3) -> f64 {
        self.e0 * rhs.e0 + self.e1 * rhs.e1 + self.e2 * rhs.e2
    }

    pub fn cross(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e0: self.e1 * rhs.e2 - self.e2 * rhs.e1,
            e1: self.e2 * rhs.e0 - self.e0 * rhs.e2,
            e2: self.e0 * rhs.e1 - self.e1 * rhs.e0
        }
    }

    pub fn unit_vector(self) -> Vec3 {
        self.div_scalar(self.length())
    }

    pub fn x(self) -> f64 {
        self.e0
    }

    pub fn y(self) -> f64 {
        self.e1
    }

    pub fn z(self) -> f64 {
        self.e2
    }

    pub fn length_squared(self) -> f64 {
        self.e0 * self.e0 + self.e1 * self.e1 + self.e2 * self.e2
    }

    pub fn length(self) -> f64 {
        f64::sqrt(self.length_squared())
    }

}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 { e0: self.e0 + rhs.e0, e1: self.e1 + rhs.e1, e2: self.e2 + rhs.e2 }
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 { e0: self.e0 - rhs.e0, e1: self.e1 - rhs.e1, e2: self.e2 - rhs.e2 }
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 { e0: self.e0 * rhs.e0, e1: self.e1 * rhs.e1, e2: self.e2 * rhs.e2 }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            e0: self.e0 + rhs.e0,
            e1: self.e1 + rhs.e1,
            e2: self.e2 + rhs.e2
        };
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 { e0: -self.e0, e1: -self.e1, e2: -self.e2 }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(e0: {:.1}, e1: {:.1}, e2: {:.1})", self.e0, self.e1, self.e2)
    }
}

//use Vec3 as Point3;

#[test]
fn test_vec3() {
  
    let vec3_a = Vec3 {e0: 1.0, e1: 2.0, e2: 3.0};
    let vec3_b = Vec3 {e0: 0.5, e1: 0.5, e2: 0.5};

    let mut vec3_mut_a = Vec3 {e0: 1.5, e1: 2.5, e2: 3.5};
    
    vec3_mut_a += vec3_b;
    let vec3_neg = -vec3_a;
    
    assert_eq!(Vec3::zero(), Vec3 {e0: 0.0, e1: 0.0, e2: 0.0});
    assert_eq!(vec3_a + vec3_b, Vec3 {e0: 1.5, e1: 2.5, e2: 3.5});
    assert_eq!(vec3_a - vec3_b, Vec3 {e0: 0.5, e1: 1.5, e2: 2.5});
    assert_eq!(vec3_a * vec3_b, Vec3 {e0: 0.5, e1: 1.0, e2: 1.5});
    assert_eq!(vec3_mut_a, Vec3 {e0: 2.0, e1: 3.0, e2: 4.0});

    vec3_mut_a.mul_assign(0.5);

    assert_eq!(vec3_mut_a, Vec3 {e0: 1.0, e1: 1.5, e2: 2.0});

    vec3_mut_a.div_assign(2.0);

    assert_eq!(vec3_mut_a, Vec3 {e0: 0.5, e1: 0.75, e2: 1.0});

    assert_eq!(vec3_neg, Vec3 {e0: -1.0, e1: -2.0, e2: -3.0});
    assert_eq!(vec3_a.mul_scalar(2.0), Vec3 {e0: 2.0, e1: 4.0, e2: 6.0});
    assert_eq!(vec3_a.div_scalar(2.0), Vec3 {e0: 0.5, e1: 1.0, e2: 1.5});

    assert_eq!(vec3_a.x(), 1.0);
    assert_eq!(vec3_a.y(), 2.0);
    assert_eq!(vec3_a.z(), 3.0);

    assert_eq!(vec3_a.length_squared(), 14.0);
    assert_eq!(vec3_a.length(), 3.7416573867739413);

    assert_eq!(vec3_a.dot(vec3_b), 3.0);

    assert_eq!(vec3_a.cross(vec3_b), Vec3 {e0: -0.5, e1: 1.0, e2: -0.5});

    assert_eq!(vec3_a.unit_vector(), Vec3 {e0: 0.2672612419124244, e1: 0.5345224838248488, e2: 0.8017837257372732});

    assert_eq!(format!("{vec3_a}"), "(e0: 1.0, e1: 2.0, e2: 3.0)");
}

