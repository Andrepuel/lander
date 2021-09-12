pub type Vector = Point;

#[derive(Clone, Copy)]
pub struct Point(pub f32, pub f32);
impl Point {
    pub fn dot(self, rhs: Point) -> f32 {
        self.0 * rhs.0 + self.1 * rhs.1
    }

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

    pub fn sin(self) -> f32 {
        self.1 / self.len()
    }

    pub fn cos(self) -> f32 {
        self.0 / self.len()
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

    pub fn len(self) -> f32 {
        (self.1 - self.0).len()
    }

    pub fn center(self) -> Point {
        (self.0 + self.1) * 0.5
    }

    pub fn direction(self) -> Vector {
        (self.1 - self.0).unit()
    }

    pub fn projection(self, point: Point) -> Point {
        let direction = self.direction();
        let point = point - self.0;

        direction * point.dot(direction) + self.0
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

#[derive(Clone, Copy, Debug)]
pub struct Mat3((f32, f32, f32), (f32, f32, f32), (f32, f32, f32));
impl Mat3 {
    pub fn identity() -> Mat3 {
        Mat3((1.0, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0))
    }

    pub fn scale(sx: f32, sy: f32) -> Mat3 {
        Mat3((sx, 0.0, 0.0), (0.0, sy, 0.0), (0.0, 0.0, 1.0))
    }

    pub fn scale_x(amount: f32) -> Mat3 {
        Mat3((amount, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0))
    }

    pub fn scale_y(amount: f32) -> Mat3 {
        Mat3((1.0, 0.0, 0.0), (0.0, amount, 0.0), (0.0, 0.0, 1.0))
    }

    pub fn rotate(angle: f32) -> Mat3 {
        let s = angle.sin();
        let c = angle.cos();

        Mat3((c, -s, 0.0), (s, c, 0.0), (0.0, 0.0, 1.0))
    }

    pub fn rotate_y_to(direction: Vector) -> Mat3 {
        let direction = direction.rot90();
        let s = direction.sin();
        let c = direction.cos();

        Mat3((c, -s, 0.0), (s, c, 0.0), (0.0, 0.0, 1.0))
    }

    pub fn translate(tx: f32, ty: f32) -> Mat3 {
        Mat3((1.0, 0.0, tx), (0.0, 1.0, ty), (0.0, 0.0, 1.0))
    }

    pub fn as_f32(&self) -> [f32; 12] {
        [
            self.0 .0, self.1 .0, self.2 .0, 0.0, self.0 .1, self.1 .1, self.2 .1, 0.0, self.0 .2,
            self.1 .2, self.2 .2, 0.0,
        ]
    }

    pub fn as_f32_packed(&self) -> [f32; 9] {
        [
            self.0 .0, self.1 .0, self.2 .0, self.0 .1, self.1 .1, self.2 .1, self.0 .2, self.1 .2,
            self.2 .2,
        ]
    }
}
impl std::ops::Mul for Mat3 {
    type Output = Mat3;

    fn mul(self, rhs: Self) -> Self::Output {
        let lhs = self;
        Mat3(
            (
                lhs.0 .0 * rhs.0 .0 + lhs.0 .1 * rhs.1 .0 + lhs.0 .2 * rhs.2 .0,
                lhs.0 .0 * rhs.0 .1 + lhs.0 .1 * rhs.1 .1 + lhs.0 .2 * rhs.2 .1,
                lhs.0 .0 * rhs.0 .2 + lhs.0 .1 * rhs.1 .2 + lhs.0 .2 * rhs.2 .2,
            ),
            (
                lhs.1 .0 * rhs.0 .0 + lhs.1 .1 * rhs.1 .0 + lhs.1 .2 * rhs.2 .0,
                lhs.1 .0 * rhs.0 .1 + lhs.1 .1 * rhs.1 .1 + lhs.1 .2 * rhs.2 .1,
                lhs.1 .0 * rhs.0 .2 + lhs.1 .1 * rhs.1 .2 + lhs.1 .2 * rhs.2 .2,
            ),
            (
                lhs.2 .0 * rhs.0 .0 + lhs.2 .1 * rhs.1 .0 + lhs.2 .2 * rhs.2 .0,
                lhs.2 .0 * rhs.0 .1 + lhs.2 .1 * rhs.1 .1 + lhs.2 .2 * rhs.2 .1,
                lhs.2 .0 * rhs.0 .2 + lhs.2 .1 * rhs.1 .2 + lhs.2 .2 * rhs.2 .2,
            ),
        )
    }
}
