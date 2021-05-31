use num::{Num, Zero};

pub struct Triangle<T> {
    a: T,
    b: T,
    c: T,
}

impl<T> Triangle<T>
where
    T: Num + PartialOrd + Copy,
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        if sides[0] > Zero::zero()
            && sides[1] > Zero::zero()
            && sides[2] > Zero::zero()
            && !(sides[0] + sides[1] < sides[2]
                || sides[1] + sides[2] < sides[0]
                || sides[0] + sides[2] < sides[1])
        {
            Some(Triangle {
                a: sides[0],
                b: sides[1],
                c: sides[2],
            })
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.a == self.b && self.b == self.c
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_equilateral() && !self.is_isosceles()
    }

    pub fn is_isosceles(&self) -> bool {
        self.a == self.b || self.b == self.c || self.a == self.c
    }
}
