use std::fmt;

use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color(pub Vec3);

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(r: {:.1}, g: {:.1}, b: {:.1})", (self.0.e0 * 255.999) as u8, (self.0.e1 * 255.999) as u8, (self.0.e2 * 255.999) as u8)
    }
}

impl Color {
    pub fn r(self) -> u8 {
        (self.0.e0 * 255.999) as u8
    }

    pub fn g(self) -> u8 {
        (self.0.e1 * 255.999) as u8
    }

    pub fn b(self) -> u8 {
        (self.0.e2 * 255.999) as u8
    }
}

#[test]
fn test_color() {

    let color = Color(Vec3{ e0: 0.5, e1: 0.5, e2: 0.5 });

    assert_eq!(format!("{color}"), "(r: 127, g: 127, b: 127)");
    assert_eq!(color.r(), 127);
    assert_eq!(color.g(), 127);
    assert_eq!(color.b(), 127);
}