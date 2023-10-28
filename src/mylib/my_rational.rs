// まだスニペット化してない
// modint と似たインターフェースのもの
#[derive(Copy, Clone, Eq, PartialEq)]
struct MyRational(Rational64);

impl MyRational {
    fn new(x: i64) -> Self {
        MyRational(x.into())
    }

    fn inv(self) -> Self {
        MyRational(self.0.inv())
    }

    fn val(self) -> (i64, i64) {
        (*self.0.numer(), *self.0.denom())
    }
}

impl std::ops::Add for MyRational {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        MyRational(self.0 + rhs.0)
    }
}

impl std::ops::Sub for MyRational {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        MyRational(self.0 - rhs.0)
    }
}

impl std::ops::Mul for MyRational {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        MyRational(self.0 * rhs.0)
    }
}

impl std::ops::Div for MyRational {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        MyRational(self.0 / rhs.0)
    }
}

impl std::ops::AddAssign for MyRational {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl std::ops::SubAssign for MyRational {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl std::ops::MulAssign for MyRational {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

impl std::ops::DivAssign for MyRational {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs
    }
}

impl std::iter::Sum for MyRational {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        MyRational(iter.map(|x| x.0).sum())
    }
}

impl std::iter::Product for MyRational {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        MyRational(iter.map(|x| x.0).product())
    }
}
