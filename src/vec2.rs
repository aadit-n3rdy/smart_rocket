#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn get_distance(&self, other: &Vec2) -> f64 {
        let dx: f64 = other.x - self.x;
        let dy: f64 = other.y - self.y;
        return (dx * dx + dy * dy).powf(0.5);
    }
}

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;
    fn add(self, _rhs: Vec2) -> Vec2 {
        return Vec2 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        };
    }
}

impl std::ops::Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, _rhs: f64) -> Vec2 {
        return Vec2 {
            x: self.x * _rhs,
            y: self.y * _rhs,
        };
    }
}
