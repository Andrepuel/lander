pub type Vector = Point;

#[derive(Clone, Copy)]
pub struct Point(pub f32, pub f32);
impl Point {
    pub fn dot_div(self, rhs: Point) -> Point {
        Point(self.0 / rhs.0, self.1 / rhs.1)
    }

    pub fn zero() -> Point {
        Point(0.0, 0.0)
    }

    pub fn len(self) -> f32 {
        (self.0 * self.0 + self.1 * self.1).sqrt()
    }

    pub fn rot90(self) -> Point {
        Point(self.1, -self.0)
    }

    pub fn unit(self) -> Point {
        self * (1.0 / self.len())
    }
}
impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl std::ops::Mul<f32> for Point {
    type Output = Point;

    fn mul(self, rhs: f32) -> Self::Output {
        Point(self.0 * rhs, self.1 * rhs)
    }
}
impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Clone, Copy)]
pub struct Line(pub Point, pub Point);
impl Line {
    pub fn intersects(self, rhs: Line) -> bool {
        let Point(x1, y1) = self.0;
        let Point(x2, y2) = self.1;
        let Point(x3, y3) = rhs.0;
        let Point(x4, y4) = rhs.1;

        let t_dividend = (x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4);
        let t_divisor = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

        if t_divisor == 0.0 && t_dividend == 0.0 {
            return self.colinear_intersect(rhs);
        }

        let t = t_dividend / t_divisor;

        let u_dividend = (x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3);
        let u_divisor = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        let u = u_dividend / u_divisor;

        (t >= 0.0 && t <= 1.0 && u >= 0.0 && u <= 1.0)
            || (t_dividend == 0.0 && t_divisor == 0.0 && u_dividend == 0.0 && u_divisor == 0.0)
    }

    fn colinear_intersect(self, rhs: Line) -> bool {
        let t0 = (rhs.0 - self.0).dot_div(self.1).0;
        let t1 = (rhs.1 - self.0).dot_div(self.1).0;

        (t0 >= 0.0 && t0 <= 1.0) || (t1 >= 0.0 && t1 <= 1.0)
    }
}
impl std::ops::Add<Point> for Line {
    type Output = Line;

    fn add(self, rhs: Point) -> Self::Output {
        Line(self.0 + rhs, self.1 + rhs)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn line_intersects_on_zero() {
        let a = Line(Point(-1.0, -1.0), Point(1.0, 1.0));
        let b = Line(Point(-1.0, 1.0), Point(1.0, -1.0));

        assert_eq!(a.intersects(b), true);
    }

    #[test]
    fn line_doesnt_intersect() {
        let a = Line(Point(-1.0, -1.0), Point(1.0, 1.0));
        let b = Line(Point(-1.0, 1.0), Point(1.0, -1.0));
        let a = a + Point(-3.0, -3.0);

        assert_eq!(a.intersects(b), false);
    }

    #[test]
    fn line_with_many_intersections() {
        let a = Line(Point(-1.0, 0.0), Point(0.5, 0.0));
        let b = Line(Point(-0.5, 0.0), Point(1.0, 0.0));

        assert_eq!(a.intersects(b), true);
    }

    #[test]
    fn colinear_that_doesnt_intersect() {
        let a = Line(Point(-1.0, 0.0), Point(-0.5, 0.0));
        let b = Line(Point(0.5, 0.0), Point(1.0, 0.0));

        assert_eq!(a.intersects(b), false);
    }
}
