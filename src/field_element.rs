use std::fmt;
use std::fmt::{Debug, Display};
use std::ops::{Add, Div, Mul, Rem, Sub};

#[derive(Clone, Copy, Debug)]
pub struct FieldElement<T> {
    pub num: T,
    pub prime: T,
}

impl<T> FieldElement<T>
where
    T: PartialOrd + Debug,
{
    pub fn new(num: T, prime: T) -> Self {
        if num >= prime {
            panic!("Num {:?} not in field range o to {:?}", num, prime);
        }
        Self { num, prime }
    }
}

impl<T> fmt::Display for FieldElement<T>
where
    T: fmt::Display + Add<Output = T> + Sub<Output = T> + Rem<Output = T>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FieldElement_{}({})", self.prime, self.num)
    }
}

impl<T> Add for FieldElement<T>
where
    T: PartialEq + Add<Output = T> + Rem<Output = T> + PartialOrd + Debug + Copy,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            panic!("Prime number should be same")
        }
        if self.num + other.num >= self.prime {
            Self::new((self.num + other.num) % self.prime, self.prime)
        } else {
            Self::new(self.num + other.num, self.prime)
        }
    }
}

impl<T> Sub for FieldElement<T>
where
    T: PartialEq + Sub<Output = T> + Rem<Output = T> + PartialOrd + Debug + Copy,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            panic!("Prime number should be same")
        }
        Self::new((self.num - other.num) % self.prime, self.prime)
    }
}

impl<T> Mul for FieldElement<T>
where
    T: PartialEq
        + Add<Output = T>
        + Sub<Output = T>
        + Rem<Output = T>
        + Div<Output = T>
        + PartialOrd
        + Debug
        + Copy,
{
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            panic!("Prime number should be same")
        }

        let zero = self.prime - other.prime;
        let one = self.prime / other.prime;
        let mut counter = other.num;

        let mut ret = FieldElement::new(zero, self.prime);
        while counter > zero {
            ret = ret + self;
            counter = counter - one;
        }
        ret
    }
}

impl<T> Div for FieldElement<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Div<Output = T>
        + Mul<Output = T>
        + Rem<Output = T>
        + PartialOrd
        + Debug
        + Display
        + Copy,
{
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        let p = self.prime;
        let one = p / p;
        self * other.pow(p - one - one)
    }
}

impl<T> PartialEq for FieldElement<T>
where
    T: PartialEq + Add<Output = T>,
{
    fn eq(&self, other: &Self) -> bool {
        self.prime == other.prime && self.num == other.num
    }
}

impl<T> Eq for FieldElement<T> where T: Eq + Add<Output = T> {}

impl<T> FieldElement<T>
where
    T: Add<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + PartialOrd
        + Debug
        + Copy,
{
    fn pow(self, exponent: T) -> Self {
        let zero = self.prime - self.prime;
        let one = self.prime / self.prime;
        let mut ret = FieldElement::new(one, self.prime);
        let mut counter = exponent % (self.prime - one);

        while counter > zero {
            ret = ret * self;
            counter = counter - one;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::FieldElement;
    use primitive_types::U256;

    #[test]
    fn eq() {
        let a = FieldElement::new(U256::from(2), U256::from(3));
        let b = FieldElement::new(U256::from(2), U256::from(3));
        let c = FieldElement::new(U256::from(1), U256::from(3));

        println!("FieldElement A = {}", a);

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn add() {
        let a = FieldElement::new(U256::from(2), U256::from(7));
        let b = FieldElement::new(U256::from(5), U256::from(7));
        let c = FieldElement::new(U256::from(0), U256::from(7));

        println!("FieldElement A = {}", a);

        assert_eq!(a + b, c);
    }

    #[test]
    fn mul() {
        let a = FieldElement::new(U256::from(3), U256::from(13));
        let b = FieldElement::new(U256::from(12), U256::from(13));
        let c = FieldElement::new(U256::from(10), U256::from(13));

        println!("FieldElement A = {}", a);

        assert_eq!(a * b, c);
    }

    #[test]
    fn pow() {
        let a = FieldElement::new(U256::from(3), U256::from(13));
        let b = FieldElement::new(U256::from(1), U256::from(13));

        assert_eq!(a.pow(U256::from(3)), b);
    }

    #[test]
    fn div() {
        let a = FieldElement::new(U256::from(7), U256::from(19));
        let b = FieldElement::new(U256::from(5), U256::from(19));
        let c = FieldElement::new(U256::from(9), U256::from(19));

        assert_eq!(a / b, c);
    }
}
