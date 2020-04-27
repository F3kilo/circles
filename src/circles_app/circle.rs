use glam::Vec2;
use std::f32::consts::PI;
use std::time::Duration;

const DENSITY: f32 = 1f32;

pub struct Circle {
    center: Vec2,
    radius: f32,
    speed: Vec2,
}

impl Circle {
    pub fn new(center: Vec2, radius: f32, speed: Vec2) -> Self {
        Self {
            center,
            radius,
            speed,
        }
    }

    pub fn mass(&self) -> f32 {
        DENSITY * PI * self.radius.powi(2)
    }

    pub fn center(&self) -> Vec2 {
        self.center
    }

    pub fn left(&self) -> f32 {
        self.center.x() - self.radius
    }

    pub fn top(&self) -> f32 {
        self.center.y() - self.radius
    }

    pub fn right(&self) -> f32 {
        self.center.x() + self.radius
    }

    pub fn bot(&self) -> f32 {
        self.center.y() + self.radius
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn speed(&self) -> Vec2 {
        self.speed
    }

    pub fn is_intersect(&self, other: &Self) -> bool {
        (self.center - other.center).length() < (self.radius + other.radius)
    }

    pub fn update(&mut self, elapsed_time: Duration) {
        self.center += elapsed_time.as_secs_f32() * self.speed;
    }

    pub fn reflect_x(&mut self) {
        self.speed *= Vec2::new(-1f32, 1f32);
    }

    pub fn reflect_y(&mut self) {
        self.speed *= Vec2::new(1f32, -1f32);
    }

    pub fn collide(&mut self, other: &mut Self) {
        let v1 = self.speed;
        let v2 = other.speed;
        let to_other = other.center - self.center;
        let to_self = -to_other;
        let cos_a1 = Self::v_dv_cos(v1, to_self);
        let cos_a2 = Self::v_dv_cos(v2, to_other);
        let m1 = self.mass();
        let m2 = other.mass();
        let v1_len = v1.length();
        let v2_len = v2.length();
        // From energy conservation law:
        let dv1_len = (2f32 * (v1_len * cos_a1 + v2_len * cos_a2) * m2 / (m1 + m2)).abs();
        let dv2_len = dv1_len * m1 / m2;
        let dv1 = to_self.normalize() * dv1_len;
        let dv2 = to_other.normalize() * dv2_len;
        self.speed += dv1;
        other.speed += dv2;
    }

    fn v_dv_cos(v: Vec2, dv: Vec2) -> f32 {
        if v == Vec2::new(0f32, 0f32) || dv == Vec2::new(0f32, 0f32) {
            return 0f32;
        }
        v.dot(dv) * v.length_reciprocal() * dv.length_reciprocal()
    }
}

#[cfg(test)]
mod tests {
    use super::Circle;
    use assert_approx_eq;
    use glam::Vec2;

    fn intersect_circles() -> Vec<Circle> {
        vec![
            Circle::new((0f32, 0f32).into(), 2f32, (0f32, 0f32).into()),
            Circle::new((3f32, 4f32).into(), 4f32, (0f32, 0f32).into()),
        ]
    }

    fn no_intersect_circles() -> Vec<Circle> {
        vec![
            Circle::new((0f32, 0f32).into(), 2f32, (0f32, 0f32).into()),
            Circle::new((3f32, 4f32).into(), 2.9f32, (0f32, 0f32).into()),
        ]
    }

    fn line_collided_circles() -> (Circle, Circle) {
        let a = Circle::new(Vec2::zero(), 2f32, (10f32, 0f32).into());
        let b = Circle::new((4f32, 0f32).into(), 2f32, (-10f32, 0f32).into());
        (a, b)
    }

    fn angle_collided_circles() -> (Circle, Circle) {
        let r = 1f32;
        let a = Circle::new(Vec2::zero(), r, (10f32, 0f32).into());
        let b_center = r * 2f32.sqrt();
        let b = Circle::new((b_center, b_center).into(), r, (-10f32, 0f32).into());
        (a, b)
    }

    #[test]
    fn intersect() {
        let cs = intersect_circles();
        assert!(cs[0].is_intersect(&cs[1]));

        let cs = no_intersect_circles();
        assert!(!cs[0].is_intersect(&cs[1]));
    }

    #[test]
    fn line_collide() {
        let (mut a, mut b) = line_collided_circles();
        a.collide(&mut b);
        assert_approx_eq::assert_approx_eq!(
            a.speed(),
            (-10f32, 0f32).into(),
            (std::f32::EPSILON, std::f32::EPSILON).into()
        );
        assert_approx_eq::assert_approx_eq!(
            b.speed(),
            (10f32, 0f32).into(),
            (std::f32::EPSILON, std::f32::EPSILON).into()
        );
    }

    #[test]
    fn angle_collide() {
        let (mut a, mut b) = angle_collided_circles();
        a.collide(&mut b);
        assert_approx_eq::assert_approx_eq!(
            a.speed(),
            (0f32, -10f32).into(),
            (1e-5f32, 1e-5f32).into()
        );
        assert_approx_eq::assert_approx_eq!(
            b.speed(),
            (0f32, 10f32).into(),
            (1e-5f32, 1e-5f32).into()
        );
    }
}
