use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Div, Mul, Sub};

// Elliptic Curve: y^2 = x^3 + a*x + b
#[derive(Clone, Debug, PartialEq)]
pub enum Point<T> {
    Coordinate { x: T, y: T, a: T, b: T },
    Infinity,
}
impl<T> fmt::Display for Point<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            &Point::Coordinate { x, y, a, b } => {
                write!(f, "Point({}, {})_{}_{}", x, y, a, b)
            }
            &Point::Infinity => {
                write!(f, "Point(Infinity)")
            }
        }
    }
}

impl<T> Point<T>
where
    T: Add<Output = T> + Mul<Output = T> + PartialEq + Copy,
{
    pub fn new(x: T, y: T, a: T, b: T) -> Self {
        if y * y != x * x * x + a * x + b {
            panic!("This is invalid number")
        }
        Self::Coordinate { x, y, a, b }
    }
}

impl<T> Add for Point<T>
where
    T: PartialEq + Add<Output = T> + Sub<Output = T> + Div<Output = T> + Mul<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        use Point::*;
        match (self, other) {
            (
                Coordinate {
                    x: x0,
                    y: y0,
                    a: a0,
                    b: b0,
                },
                Coordinate {
                    x: x1,
                    y: y1,
                    a: a1,
                    b: b1,
                },
            ) => {
                if a0 != a1 || b0 != b1 {
                    panic!("Points are not on the same curve.")
                }
                if x0 == x1 {
                    // 参考にした実装と違う実装をした
                    // xが同じでyが違う場合は垂直線が生じるため無限遠点（単位元）を返す
                    if y0 != y1 {
                        return Infinity;
                    }
                    // self == other の場合
                    let one = a0 / a0;
                    let two = one + one;
                    let three = one + two;

                    //  微分して傾きを求める
                    let s = (three * x0 * x0 + a0) / (two * y0);

                    // 公式
                    let x2 = s * s - two * x0;
                    let y2 = s * (x0 - x2) - y0;

                    return Coordinate {
                        x: x2,
                        y: y2,
                        a: a0,
                        b: b0,
                    };
                }

                // 傾き = x の増加量分の y の増加量
                let s = (y1 - y0) / (x1 - x0);
                // 公式
                let x2 = s * s - x0 - x1;
                let y2 = s * (x0 - x2) - y0;
                return Coordinate {
                    x: x2,
                    y: y2,
                    a: a0,
                    b: b0,
                };
            }
            (Coordinate { x, y, a, b }, Infinity) => Coordinate { x, y, a, b },
            (Infinity, Coordinate { x, y, a, b }) => Coordinate { x, y, a, b },
            (Infinity, Infinity) => Infinity,
        }
    }
}

impl<T, U> Mul<U> for Point<T>
where
    T: Add<Output = T> + Sub<Output = T> + Div<Output = T> + Mul<Output = T> + PartialOrd + Copy,
    U: Sub<Output = U> + Div<Output = U> + Mul<Output = U> + PartialOrd + Copy,
{
    type Output = Self;

    fn mul(self, other: U) -> Self::Output {
        let zero = other - other;
        let one = other / other;

        let mut counter = other;
        let mut ret = Self::Infinity;

        while counter > zero {
            ret = ret + self.clone();
            counter = counter - one;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Point;
    use crate::field_element::FieldElement;
    use primitive_types::{U256, U512};
    use sha2::Sha256;

    #[test]
    fn new() {
        let _ = Point::new(U256::from(18), U256::from(77), U256::from(5), U256::from(7));
    }

    #[test]
    fn eq_elliptic() {
        let a = Point::new(U256::from(18), U256::from(77), U256::from(5), U256::from(7));
        let b = Point::new(U256::from(18), U256::from(77), U256::from(5), U256::from(7));

        assert_eq!(a, b);
    }

    #[test]
    fn point_on_elliptic_curve() {
        let a = FieldElement::new(U256::from(0), U256::from(223));
        let b = FieldElement::new(U256::from(7), U256::from(223));
        let x = FieldElement::new(U256::from(192), U256::from(223));
        let y = FieldElement::new(U256::from(105), U256::from(223));

        assert_eq!(y * y, x * x * x + a * x + b);
    }

    #[test]
    fn mul() {
        let p0 = Point::new(2, 5, 5, 7);
        let p1 = Point::new(2, -5, 5, 7);

        assert_ne!(p0, p1);
        assert_eq!(p0.clone() * 3, p1);
        assert_eq!(p0 * U256::from(3), p1);
    }

    #[test]
    fn on_the_curve() {
        let p = U512::from_str_radix(
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
            16,
        )
        .unwrap();
        let x = U512::from_str_radix(
            "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",
            16,
        )
        .unwrap();
        let y = U512::from_str_radix(
            "483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8",
            16,
        )
        .unwrap();
        let n = U512::from_str_radix(
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141",
            16,
        )
        .unwrap();

        let a = FieldElement::new(U512::from(0), p);
        let b = FieldElement::new(U512::from(7), p);
        let gx = FieldElement::new(x, p);
        let gy = FieldElement::new(y, p);

        fn make_hash(source: &[u8]) -> U512 {
            let mut hasher = Sha256::new();
            hasher.update(source);
            U512::from(&hasher.finalize()[..])
        }

        // 署名ハッシュ作成
        let z = FieldElement::new(make_hash(b"This is my sign"), n);

        // 秘密鍵作成
        let e = FieldElement::new(make_hash(b"This is my secret"), n);

        // 乱数kを生成
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let i: i32 = rng.gen();
        let k = FieldElement::new(U512::from(rng.gen::<i32>()), n);

        let powed = FieldElement::new(n - U512::from(2), n);

        let G = Point::new(gx, gy, a, b);
        let r = (G * k).x;
        let mut k_inv = (FieldElement::new(k, n)).pow(powed);
        let s = (z + r * e) * k_inv;

        let P = G * e;
    }
}
